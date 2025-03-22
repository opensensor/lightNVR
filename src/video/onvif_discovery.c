#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200112L

#include "video/onvif_discovery.h"
#include "core/logger.h"
#include "core/config.h"
#include "video/stream_manager.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/select.h>
#include <sys/time.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <ifaddrs.h>
#include <errno.h>
#include <time.h>
#include <net/if.h>

// Maximum number of networks to detect
#define MAX_DETECTED_NETWORKS 10

// ONVIF WS-Discovery message template
static const char *ONVIF_DISCOVERY_MSG = 
    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>"
    "<e:Envelope xmlns:e=\"http://www.w3.org/2003/05/soap-envelope\" "
    "xmlns:w=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" "
    "xmlns:d=\"http://schemas.xmlsoap.org/ws/2005/04/discovery\" "
    "xmlns:dn=\"http://www.onvif.org/ver10/network/wsdl\">"
    "<e:Header>"
    "<w:MessageID>uuid:%s</w:MessageID>"
    "<w:To>urn:schemas-xmlsoap-org:ws:2005:04:discovery</w:To>"
    "<w:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/Probe</w:Action>"
    "</e:Header>"
    "<e:Body>"
    "<d:Probe>"
    "<d:Types>dn:NetworkVideoTransmitter</d:Types>"
    "</d:Probe>"
    "</e:Body>"
    "</e:Envelope>";

// Discovery thread data
typedef struct {
    pthread_t thread;
    int running;
    char network[64];
    int interval;
} discovery_thread_t;

// Global discovery thread
static discovery_thread_t g_discovery_thread = {0};

// Mutex for thread safety
static pthread_mutex_t g_discovery_mutex = PTHREAD_MUTEX_INITIALIZER;

// Array of discovered devices
#define MAX_DISCOVERED_DEVICES 32
static onvif_device_info_t g_discovered_devices[MAX_DISCOVERED_DEVICES];
static int g_discovered_device_count = 0;

// Generate a random UUID for WS-Discovery message ID
static void generate_uuid(char *uuid, size_t size) {
    snprintf(uuid, size, "%x%x-%x-%x-%x-%x%x%x",
             rand() & 0xffff, rand() & 0xffff,
             rand() & 0xffff,
             ((rand() & 0x0fff) | 0x4000),
             ((rand() & 0x3fff) | 0x8000),
             rand() & 0xffff, rand() & 0xffff, rand() & 0xffff);
}

// Parse network string (e.g., "192.168.1.0/24") into base address and subnet mask
static int parse_network(const char *network, uint32_t *base_addr, uint32_t *subnet_mask) {
    char network_copy[64];
    char *slash;
    int prefix_len;
    struct in_addr addr;
    
    if (!network || !base_addr || !subnet_mask) {
        return -1;
    }
    
    // Make a copy of the network string
    strncpy(network_copy, network, sizeof(network_copy) - 1);
    network_copy[sizeof(network_copy) - 1] = '\0';
    
    // Find the slash
    slash = strchr(network_copy, '/');
    if (!slash) {
        log_error("Invalid network format: %s (expected format: x.x.x.x/y)", network);
        return -1;
    }
    
    // Split the string
    *slash = '\0';
    prefix_len = atoi(slash + 1);
    
    // Validate prefix length
    if (prefix_len < 0 || prefix_len > 32) {
        log_error("Invalid prefix length: %d (must be between 0 and 32)", prefix_len);
        return -1;
    }
    
    // Convert IP address
    if (inet_aton(network_copy, &addr) == 0) {
        log_error("Invalid IP address: %s", network_copy);
        return -1;
    }
    
    // Calculate subnet mask
    *base_addr = ntohl(addr.s_addr);
    *subnet_mask = prefix_len == 0 ? 0 : ~((1 << (32 - prefix_len)) - 1);
    
    return 0;
}

// Send WS-Discovery probe message to a specific IP address
static int send_discovery_probe(const char *ip_addr) {
    int sock;
    struct sockaddr_in addr;
    char uuid[64];
    char message[1024];
    int message_len;
    int ret;
    
    // Create socket
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }
    
    // Set socket options
    int broadcast = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_BROADCAST, &broadcast, sizeof(broadcast)) < 0) {
        log_error("Failed to set socket options: %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    // Set up address
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(3702); // WS-Discovery port
    
    // Convert IP address
    if (inet_aton(ip_addr, &addr.sin_addr) == 0) {
        log_error("Invalid IP address: %s", ip_addr);
        close(sock);
        return -1;
    }
    
    // Generate UUID
    generate_uuid(uuid, sizeof(uuid));
    
    // Format message
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
    
    // Send message
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
        log_error("Failed to send discovery probe to %s: %s", ip_addr, strerror(errno));
        close(sock);
        return -1;
    }
    
    log_debug("Sent discovery probe to %s", ip_addr);
    
    // Close socket
    close(sock);
    
    return 0;
}

// Parse ONVIF device information from discovery response
static int parse_device_info(const char *response, onvif_device_info_t *device_info) {
    // This is a simplified implementation. In a real implementation, you would
    // use an XML parser to extract the device information from the response.
    // For now, we'll just extract the device service URL.
    
    const char *xaddr_start = strstr(response, "<d:XAddrs>");
    const char *xaddr_end = strstr(response, "</d:XAddrs>");
    
    if (!xaddr_start || !xaddr_end) {
        return -1;
    }
    
    xaddr_start += 10; // Skip "<d:XAddrs>"
    
    size_t len = xaddr_end - xaddr_start;
    if (len >= MAX_URL_LENGTH) {
        len = MAX_URL_LENGTH - 1;
    }
    
    strncpy(device_info->device_service, xaddr_start, len);
    device_info->device_service[len] = '\0';
    
    // Extract IP address from device service URL
    const char *http = strstr(device_info->device_service, "http://");
    if (http) {
        const char *ip_start = http + 7;
        const char *ip_end = strchr(ip_start, ':');
        if (!ip_end) {
            ip_end = strchr(ip_start, '/');
        }
        
        if (ip_end) {
            len = ip_end - ip_start;
            if (len >= sizeof(device_info->ip_address)) {
                len = sizeof(device_info->ip_address) - 1;
            }
            
            strncpy(device_info->ip_address, ip_start, len);
            device_info->ip_address[len] = '\0';
        }
    }
    
    // Set discovery time
    device_info->discovery_time = time(NULL);
    
    // Set online status
    device_info->online = true;
    
    return 0;
}

// Receive and process discovery responses
static int receive_discovery_responses(onvif_device_info_t *devices, int max_devices) {
    int sock;
    struct sockaddr_in addr;
    socklen_t addr_len = sizeof(addr);
    char buffer[8192];
    int ret;
    int count = 0;
    fd_set readfds;
    struct timeval timeout;
    
    log_info("Setting up socket to receive discovery responses");
    
    // Create socket
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }
    
    // Set socket options
    int reuse = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &reuse, sizeof(reuse)) < 0) {
        log_error("Failed to set socket options (SO_REUSEADDR): %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    // Set broadcast option
    int broadcast = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_BROADCAST, &broadcast, sizeof(broadcast)) < 0) {
        log_error("Failed to set socket options (SO_BROADCAST): %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    // Bind to WS-Discovery port
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(3702);
    addr.sin_addr.s_addr = htonl(INADDR_ANY);
    
    if (bind(sock, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        log_error("Failed to bind socket: %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    log_info("Waiting for discovery responses (timeout: 5 seconds, attempts: 5)");
    
    // Set timeout for select
    timeout.tv_sec = 25;  // Increased timeout to 25 seconds
    timeout.tv_usec = 0;
    
    // Wait for responses
    for (int i = 0; i < 5; i++) { // Increased attempts to 5
        log_info("Waiting for responses, attempt %d/5", i+1);
        
        FD_ZERO(&readfds);
        FD_SET(sock, &readfds);
        
        ret = select(sock + 1, &readfds, NULL, NULL, &timeout);
        if (ret < 0) {
            log_error("Select failed: %s", strerror(errno));
            close(sock);
            return -1;
        } else if (ret == 0) {
            // Timeout, no data available
            log_info("Timeout waiting for responses, no data available");
            continue;
        }
        
        // Receive data
        ret = recvfrom(sock, buffer, sizeof(buffer) - 1, 0, (struct sockaddr *)&addr, &addr_len);
        if (ret < 0) {
            log_error("Failed to receive data: %s", strerror(errno));
            continue;
        }
        
        // Log the source IP
        char ip_str[INET_ADDRSTRLEN];
        inet_ntop(AF_INET, &(addr.sin_addr), ip_str, INET_ADDRSTRLEN);
        log_info("Received %d bytes from %s:%d", ret, ip_str, ntohs(addr.sin_port));
        
        // Null-terminate the buffer
        buffer[ret] = '\0';
        
        // Parse device information
        if (count < max_devices) {
            if (parse_device_info(buffer, &devices[count]) == 0) {
                log_info("Discovered ONVIF device: %s", devices[count].device_service);
                count++;
            } else {
                log_error("Failed to parse device info from response");
            }
        }
        
        // Reset timeout for next attempt
        timeout.tv_sec = 25;
        timeout.tv_usec = 0;
    }
    
    // Close socket
    close(sock);
    
    log_info("Discovery response collection completed, found %d devices", count);
    
    return count;
}

// Discovery thread function
static void *discovery_thread_func(void *arg) {
    discovery_thread_t *thread_data = (discovery_thread_t *)arg;
    uint32_t base_addr, subnet_mask;
    char ip_addr[16];
    struct in_addr addr;
    onvif_device_info_t devices[MAX_DISCOVERED_DEVICES];
    int count;
    
    log_info("ONVIF discovery thread started");
    
    // Parse network
    if (parse_network(thread_data->network, &base_addr, &subnet_mask) != 0) {
        log_error("Failed to parse network: %s", thread_data->network);
        return NULL;
    }
    
    // Main discovery loop
    while (thread_data->running) {
        log_info("Starting ONVIF discovery scan on network %s", thread_data->network);
        
        // Calculate network range
        uint32_t network = base_addr & subnet_mask;
        uint32_t broadcast = network | ~subnet_mask;
        
        // Send discovery probes to all addresses in the range
        for (uint32_t ip = network + 1; ip < broadcast && thread_data->running; ip++) {
            addr.s_addr = htonl(ip);
            strcpy(ip_addr, inet_ntoa(addr));
            
            // Send discovery probe
            send_discovery_probe(ip_addr);
            
            // Sleep a bit to avoid flooding the network
            usleep(10000); // 10ms
        }
        
        // Send multiple probes to broadcast address
        addr.s_addr = htonl(broadcast);
        strcpy(ip_addr, inet_ntoa(addr));
        log_info("Sending multiple discovery probes to broadcast address: %s", ip_addr);
        
        for (int i = 0; i < 5; i++) {
            send_discovery_probe(ip_addr);
            usleep(100000); // 100ms between broadcast probes
        }
        
        // Also try sending to specific multicast address used by some ONVIF devices
        log_info("Sending discovery probe to ONVIF multicast address: 239.255.255.250");
        send_discovery_probe("239.255.255.250");
        
        // Receive responses
        count = receive_discovery_responses(devices, MAX_DISCOVERED_DEVICES);
        
        // Update discovered devices list
        pthread_mutex_lock(&g_discovery_mutex);
        
        // Clear old devices
        memset(g_discovered_devices, 0, sizeof(g_discovered_devices));
        g_discovered_device_count = 0;
        
        // Add new devices
        for (int i = 0; i < count && i < MAX_DISCOVERED_DEVICES; i++) {
            memcpy(&g_discovered_devices[i], &devices[i], sizeof(onvif_device_info_t));
            g_discovered_device_count++;
        }
        
        pthread_mutex_unlock(&g_discovery_mutex);
        
        log_info("ONVIF discovery scan completed, found %d devices", count);
        
        // Sleep until next scan
        for (int i = 0; i < thread_data->interval && thread_data->running; i++) {
            sleep(1);
        }
    }
    
    log_info("ONVIF discovery thread stopped");
    
    return NULL;
}

// Detect local networks
static int detect_local_networks(char networks[][64], int max_networks) {
    struct ifaddrs *ifaddr, *ifa;
    int family, s;
    char host[NI_MAXHOST];
    int count = 0;
    bool found_wireless = false;
    
    log_info("Starting network interface detection");
    
    // Add known wireless network as first option (highest priority)
    if (count < max_networks) {
        strncpy(networks[count], "192.168.50.0/24", 64);
        log_info("Added known wireless network: %s", networks[count]);
        count++;
        found_wireless = true;
    }
    
    if (getifaddrs(&ifaddr) == -1) {
        log_error("Failed to get interface addresses: %s", strerror(errno));
        if (found_wireless) {
            // Return the known wireless network if we can't get interfaces
            return count;
        }
        return -1;
    }
    
    // Walk through linked list, maintaining head pointer so we can free list later
    for (ifa = ifaddr; ifa != NULL && count < max_networks; ifa = ifa->ifa_next) {
        if (ifa->ifa_addr == NULL) {
            continue;
        }
        
        family = ifa->ifa_addr->sa_family;
        
        // Log all interfaces for debugging
        if (ifa->ifa_name) {
            log_info("Found interface: %s, family: %d", ifa->ifa_name, family);
        }
        
        // Only consider IPv4 addresses
        if (family == AF_INET) {
            // Get IP address for logging
            s = getnameinfo(ifa->ifa_addr, sizeof(struct sockaddr_in),
                           host, NI_MAXHOST, NULL, 0, NI_NUMERICHOST);
            if (s != 0) {
                log_error("Failed to get IP address: %s", gai_strerror(s));
                continue;
            }
            
            log_info("IPv4 interface: %s, IP: %s, flags: 0x%x", 
                    ifa->ifa_name, host, ifa->ifa_flags);
            
            // Skip loopback interfaces
            if (ifa->ifa_flags & IFF_LOOPBACK) {
                log_info("Skipping loopback interface: %s", ifa->ifa_name);
                continue;
            }
            
            // Skip interfaces that are not up
            if (!(ifa->ifa_flags & IFF_UP)) {
                log_info("Skipping interface that is not up: %s", ifa->ifa_name);
                continue;
            }
            
            // Skip Docker and virtual interfaces
            if (strstr(ifa->ifa_name, "docker") || 
                strstr(ifa->ifa_name, "veth") || 
                strstr(ifa->ifa_name, "br-") ||
                strstr(ifa->ifa_name, "lxc")) {
                log_info("Skipping Docker/virtual interface: %s", ifa->ifa_name);
                continue;
            }
            
            // Get IP address
            s = getnameinfo(ifa->ifa_addr, sizeof(struct sockaddr_in),
                           host, NI_MAXHOST, NULL, 0, NI_NUMERICHOST);
            if (s != 0) {
                log_error("Failed to get IP address: %s", gai_strerror(s));
                continue;
            }
            
            // Get netmask
            struct sockaddr_in *netmask = (struct sockaddr_in *)ifa->ifa_netmask;
            if (!netmask) {
                continue;
            }
            
            // Calculate prefix length from netmask
            uint32_t mask = ntohl(netmask->sin_addr.s_addr);
            int prefix_len = 0;
            while (mask & 0x80000000) {
                prefix_len++;
                mask <<= 1;
            }
            
            // Calculate network address
            struct sockaddr_in *addr = (struct sockaddr_in *)ifa->ifa_addr;
            uint32_t ip = ntohl(addr->sin_addr.s_addr);
            uint32_t network = ip & ntohl(netmask->sin_addr.s_addr);
            
            // Convert network address to string
            struct in_addr network_addr;
            network_addr.s_addr = htonl(network);
            
            // Format network in CIDR notation
            snprintf(networks[count], 64, "%s/%d", inet_ntoa(network_addr), prefix_len);
            
            log_info("Detected network: %s (interface: %s)", networks[count], ifa->ifa_name);
            count++;
        }
    }
    
    freeifaddrs(ifaddr);
    
    if (count == 0) {
        log_warn("No suitable network interfaces found");
    }
    
    return count;
}

// Initialize ONVIF discovery module
int init_onvif_discovery(void) {
    // Initialize random number generator for UUID generation
    srand(time(NULL));
    
    // Initialize discovered devices array
    memset(g_discovered_devices, 0, sizeof(g_discovered_devices));
    g_discovered_device_count = 0;
    
    // Initialize discovery thread data
    memset(&g_discovery_thread, 0, sizeof(g_discovery_thread));
    
    log_info("ONVIF discovery module initialized");
    
    return 0;
}

// Shutdown ONVIF discovery module
void shutdown_onvif_discovery(void) {
    // Stop discovery thread if running
    stop_onvif_discovery();
    
    log_info("ONVIF discovery module shutdown");
}

// Start ONVIF discovery process
int start_onvif_discovery(const char *network, int interval) {
    char detected_networks[MAX_DETECTED_NETWORKS][64];
    int network_count = 0;
    char selected_network[64] = {0};
    
    // Validate interval parameter
    if (interval <= 0) {
        log_error("Invalid interval parameter for start_onvif_discovery");
        return -1;
    }
    
    // Check if we need to auto-detect networks
    if (!network || strlen(network) == 0 || strcmp(network, "auto") == 0) {
        log_info("Auto-detecting networks for ONVIF discovery");
        
        // Detect local networks
        network_count = detect_local_networks(detected_networks, MAX_DETECTED_NETWORKS);
        
        if (network_count <= 0) {
            log_error("Failed to auto-detect networks for ONVIF discovery");
            return -1;
        }
        
        // Use the first detected network
        strncpy(selected_network, detected_networks[0], sizeof(selected_network) - 1);
        selected_network[sizeof(selected_network) - 1] = '\0';
        
        log_info("Auto-detected network for ONVIF discovery: %s", selected_network);
    } else {
        // Use the provided network
        strncpy(selected_network, network, sizeof(selected_network) - 1);
        selected_network[sizeof(selected_network) - 1] = '\0';
    }
    
    // Stop existing discovery thread if running
    stop_onvif_discovery();
    
    // Initialize thread data
    pthread_mutex_lock(&g_discovery_mutex);
    
    g_discovery_thread.running = 1;
    strncpy(g_discovery_thread.network, selected_network, sizeof(g_discovery_thread.network) - 1);
    g_discovery_thread.network[sizeof(g_discovery_thread.network) - 1] = '\0';
    g_discovery_thread.interval = interval;
    
    // Create thread
    if (pthread_create(&g_discovery_thread.thread, NULL, discovery_thread_func, &g_discovery_thread) != 0) {
        log_error("Failed to create ONVIF discovery thread: %s", strerror(errno));
        g_discovery_thread.running = 0;
        pthread_mutex_unlock(&g_discovery_mutex);
        return -1;
    }
    
    pthread_mutex_unlock(&g_discovery_mutex);
    
    log_info("ONVIF discovery started on network %s with interval %d seconds", selected_network, interval);
    
    return 0;
}

// Get discovery mutex
pthread_mutex_t* get_discovery_mutex(void) {
    return &g_discovery_mutex;
}

// Get current discovery network
const char* get_current_discovery_network(void) {
    if (g_discovery_thread.running) {
        return g_discovery_thread.network;
    }
    return NULL;
}

// Stop ONVIF discovery process
int stop_onvif_discovery(void) {
    pthread_mutex_lock(&g_discovery_mutex);
    
    if (g_discovery_thread.running) {
        // Signal thread to stop
        g_discovery_thread.running = 0;
        
        // Wait for thread to exit
        pthread_mutex_unlock(&g_discovery_mutex);
        pthread_join(g_discovery_thread.thread, NULL);
        pthread_mutex_lock(&g_discovery_mutex);
        
        // Clear thread data
        memset(&g_discovery_thread, 0, sizeof(g_discovery_thread));
        
        log_info("ONVIF discovery stopped");
    }
    
    pthread_mutex_unlock(&g_discovery_mutex);
    
    return 0;
}

// Get discovered ONVIF devices
int get_discovered_onvif_devices(onvif_device_info_t *devices, int max_devices) {
    int count;
    
    if (!devices || max_devices <= 0) {
        return -1;
    }
    
    pthread_mutex_lock(&g_discovery_mutex);
    
    count = g_discovered_device_count < max_devices ? g_discovered_device_count : max_devices;
    
    for (int i = 0; i < count; i++) {
        memcpy(&devices[i], &g_discovered_devices[i], sizeof(onvif_device_info_t));
    }
    
    pthread_mutex_unlock(&g_discovery_mutex);
    
    return count;
}

// Get ONVIF device profiles
int get_onvif_device_profiles(const char *device_url, const char *username, 
                             const char *password, onvif_profile_t *profiles, 
                             int max_profiles) {
    // This is a placeholder implementation. In a real implementation, you would
    // use ONVIF SOAP calls to get the device profiles.
    
    log_info("Getting profiles for ONVIF device: %s", device_url);
    
    // For now, just return a dummy profile
    if (max_profiles > 0) {
        strncpy(profiles[0].token, "Profile_1", sizeof(profiles[0].token) - 1);
        profiles[0].token[sizeof(profiles[0].token) - 1] = '\0';
        
        strncpy(profiles[0].name, "Main Stream", sizeof(profiles[0].name) - 1);
        profiles[0].name[sizeof(profiles[0].name) - 1] = '\0';
        
        snprintf(profiles[0].stream_uri, sizeof(profiles[0].stream_uri),
                 "rtsp://%s:554/onvif/profile1/media.smp", 
                 strstr(device_url, "://") ? strstr(device_url, "://") + 3 : device_url);
        
        profiles[0].width = 1920;
        profiles[0].height = 1080;
        strncpy(profiles[0].encoding, "H264", sizeof(profiles[0].encoding) - 1);
        profiles[0].encoding[sizeof(profiles[0].encoding) - 1] = '\0';
        profiles[0].fps = 30;
        profiles[0].bitrate = 4000;
        
        return 1;
    }
    
    return 0;
}

// Get ONVIF stream URL for a specific profile
int get_onvif_stream_url(const char *device_url, const char *username, 
                        const char *password, const char *profile_token, 
                        char *stream_url, size_t url_size) {
    // This is a placeholder implementation. In a real implementation, you would
    // use ONVIF SOAP calls to get the stream URL.
    
    log_info("Getting stream URL for ONVIF device: %s, profile: %s", device_url, profile_token);
    
    // For now, just return a dummy URL
    snprintf(stream_url, url_size, "rtsp://%s:554/onvif/%s/media.smp", 
             strstr(device_url, "://") ? strstr(device_url, "://") + 3 : device_url,
             profile_token);
    
    return 0;
}

// Add discovered ONVIF device as a stream
int add_onvif_device_as_stream(const onvif_device_info_t *device_info, 
                              const onvif_profile_t *profile, 
                              const char *username, const char *password, 
                              const char *stream_name) {
    stream_config_t config;
    
    if (!device_info || !profile || !stream_name) {
        log_error("Invalid parameters for add_onvif_device_as_stream");
        return -1;
    }
    
    // Initialize stream configuration
    memset(&config, 0, sizeof(config));
    
    // Set stream name
    strncpy(config.name, stream_name, MAX_STREAM_NAME - 1);
    config.name[MAX_STREAM_NAME - 1] = '\0';
    
    // Set stream URL
    strncpy(config.url, profile->stream_uri, MAX_URL_LENGTH - 1);
    config.url[MAX_URL_LENGTH - 1] = '\0';
    
    // Set stream parameters
    config.enabled = true;
    config.width = profile->width;
    config.height = profile->height;
    config.fps = profile->fps;
    strncpy(config.codec, profile->encoding, sizeof(config.codec) - 1);
    config.codec[sizeof(config.codec) - 1] = '\0';
    
    // Set default values
    config.priority = 5;
    config.record = false;
    config.segment_duration = 60;
    config.detection_based_recording = false;
    config.detection_interval = 10;
    config.detection_threshold = 0.5;
    config.pre_detection_buffer = 5;
    config.post_detection_buffer = 10;
    config.streaming_enabled = true;
    
    // Set protocol to ONVIF
    config.protocol = STREAM_PROTOCOL_ONVIF;
    
    // Set ONVIF-specific fields
    if (username) {
        strncpy(config.onvif_username, username, sizeof(config.onvif_username) - 1);
        config.onvif_username[sizeof(config.onvif_username) - 1] = '\0';
    }
    
    if (password) {
        strncpy(config.onvif_password, password, sizeof(config.onvif_password) - 1);
        config.onvif_password[sizeof(config.onvif_password) - 1] = '\0';
    }
    
    strncpy(config.onvif_profile, profile->token, sizeof(config.onvif_profile) - 1);
    config.onvif_profile[sizeof(config.onvif_profile) - 1] = '\0';
    
    config.onvif_discovery_enabled = true;
    
    // Add stream
    stream_handle_t handle = add_stream(&config);
    if (!handle) {
        log_error("Failed to add ONVIF device as stream: %s", stream_name);
        return -1;
    }
    
    log_info("Added ONVIF device as stream: %s", stream_name);
    
    return 0;
}


// Extended version of receive_discovery_responses with configurable timeouts
static int receive_extended_discovery_responses(onvif_device_info_t *devices, int max_devices,
                                               int timeout_sec, int max_attempts) {
    int sock;
    struct sockaddr_in addr;
    socklen_t addr_len = sizeof(addr);
    char buffer[8192];
    int ret;
    int count = 0;
    fd_set readfds;
    struct timeval timeout;

    log_info("Setting up socket to receive discovery responses (extended timeouts)");

    // Create socket
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }

    // Set socket options
    int reuse = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &reuse, sizeof(reuse)) < 0) {
        log_error("Failed to set socket options (SO_REUSEADDR): %s", strerror(errno));
        close(sock);
        return -1;
    }

    // Set broadcast option
    int broadcast = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_BROADCAST, &broadcast, sizeof(broadcast)) < 0) {
        log_error("Failed to set socket options (SO_BROADCAST): %s", strerror(errno));
        close(sock);
        return -1;
    }

    // Join multicast group for ONVIF discovery
    struct ip_mreq mreq;
    memset(&mreq, 0, sizeof(mreq));
    mreq.imr_multiaddr.s_addr = inet_addr("239.255.255.250");
    mreq.imr_interface.s_addr = htonl(INADDR_ANY);

    if (setsockopt(sock, IPPROTO_IP, IP_ADD_MEMBERSHIP, &mreq, sizeof(mreq)) < 0) {
        log_warn("Failed to join multicast group: %s", strerror(errno));
        // Continue anyway, unicast and broadcast might still work
    }

    // Bind to WS-Discovery port
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(3702);
    addr.sin_addr.s_addr = htonl(INADDR_ANY);

    if (bind(sock, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        log_error("Failed to bind socket: %s", strerror(errno));
        close(sock);
        return -1;
    }

    log_info("Waiting for discovery responses (timeout: %d seconds, attempts: %d)",
             timeout_sec, max_attempts);

    // Set timeout for select
    timeout.tv_sec = timeout_sec;
    timeout.tv_usec = 0;

    // Wait for responses, processing multiple responses per attempt
    for (int attempt = 0; attempt < max_attempts; attempt++) {
        log_info("Waiting for responses, attempt %d/%d", attempt+1, max_attempts);

        FD_ZERO(&readfds);
        FD_SET(sock, &readfds);

        // Reset timeout for each attempt
        timeout.tv_sec = timeout_sec;
        timeout.tv_usec = 0;

        ret = select(sock + 1, &readfds, NULL, NULL, &timeout);

        if (ret < 0) {
            log_error("Select failed: %s", strerror(errno));
            close(sock);
            return count;  // Return any devices found so far
        } else if (ret == 0) {
            // Timeout, no data available
            log_info("Timeout waiting for responses, no data available");
            continue;
        }

        // Process all available responses without blocking
        while (count < max_devices) {
            // Use MSG_DONTWAIT to avoid blocking
            ret = recvfrom(sock, buffer, sizeof(buffer) - 1, MSG_DONTWAIT,
                         (struct sockaddr *)&addr, &addr_len);

            if (ret < 0) {
                if (errno == EAGAIN || errno == EWOULDBLOCK) {
                    // No more data available without blocking
                    break;
                }
                log_error("Failed to receive data: %s", strerror(errno));
                break;
            }

            // Log the source IP
            char ip_str[INET_ADDRSTRLEN];
            inet_ntop(AF_INET, &(addr.sin_addr), ip_str, INET_ADDRSTRLEN);
            log_info("Received %d bytes from %s:%d", ret, ip_str, ntohs(addr.sin_port));

            // Null-terminate the buffer
            buffer[ret] = '\0';

            // Dump the first 200 characters of the response for debugging
            char debug_buffer[201];
            strncpy(debug_buffer, buffer, 200);
            debug_buffer[200] = '\0';
            log_debug("Response sample: %s", debug_buffer);

            // Parse device information
            if (parse_device_info(buffer, &devices[count]) == 0) {
                log_info("Discovered ONVIF device: %s (%s)",
                        devices[count].device_service, devices[count].ip_address);

                // Check if this is a duplicate device
                bool duplicate = false;
                for (int i = 0; i < count; i++) {
                    if (strcmp(devices[i].ip_address, devices[count].ip_address) == 0) {
                        duplicate = true;
                        break;
                    }
                }

                if (!duplicate) {
                    count++;
                } else {
                    log_debug("Skipping duplicate device: %s", devices[count].ip_address);
                }
            } else {
                log_debug("Failed to parse device info from response");
            }

            // If we've filled our device buffer, stop receiving
            if (count >= max_devices) {
                break;
            }
        }
    }

    // Close socket
    close(sock);

    log_info("Discovery response collection completed, found %d devices", count);

    return count;
}

// Manually discover ONVIF devices on a specific network
int discover_onvif_devices(const char *network, onvif_device_info_t *devices,
                          int max_devices) {
    uint32_t base_addr, subnet_mask;
    char ip_addr[16];
    struct in_addr addr;
    int count = 0;
    char detected_networks[MAX_DETECTED_NETWORKS][64];
    int network_count = 0;
    char selected_network[64] = {0};
    int discovery_sock = -1;
    int broadcast_enabled = 1;
    int result = -1;

    if (!devices || max_devices <= 0) {
        log_error("Invalid parameters for discover_onvif_devices");
        return -1;
    }

    // Check if we need to auto-detect networks
    if (!network || strlen(network) == 0 || strcmp(network, "auto") == 0) {
        log_info("Auto-detecting networks for ONVIF discovery");

        // Detect local networks
        network_count = detect_local_networks(detected_networks, MAX_DETECTED_NETWORKS);

        if (network_count <= 0) {
            log_error("Failed to auto-detect networks for ONVIF discovery");
            return -1;
        }

        // Use the first detected network
        strncpy(selected_network, detected_networks[0], sizeof(selected_network) - 1);
        selected_network[sizeof(selected_network) - 1] = '\0';

        log_info("Auto-detected network for ONVIF discovery: %s", selected_network);

        // Use the selected network
        network = selected_network;
    }

    log_info("Starting manual ONVIF discovery on network %s", network);

    // Parse network
    if (parse_network(network, &base_addr, &subnet_mask) != 0) {
        log_error("Failed to parse network: %s", network);
        return -1;
    }

    // Create a single socket for all discovery probes
    discovery_sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (discovery_sock < 0) {
        log_error("Failed to create discovery socket: %s", strerror(errno));
        return -1;
    }

    // Set socket options for broadcast
    if (setsockopt(discovery_sock, SOL_SOCKET, SO_BROADCAST, &broadcast_enabled, sizeof(broadcast_enabled)) < 0) {
        log_error("Failed to set SO_BROADCAST option: %s", strerror(errno));
        close(discovery_sock);
        return -1;
    }

    // Try binding to any interface to improve reliability
    struct sockaddr_in bind_addr;
    memset(&bind_addr, 0, sizeof(bind_addr));
    bind_addr.sin_family = AF_INET;
    bind_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    bind_addr.sin_port = htons(0);  // Use any available port for sending

    if (bind(discovery_sock, (struct sockaddr *)&bind_addr, sizeof(bind_addr)) < 0) {
        log_warn("Failed to bind discovery socket: %s", strerror(errno));
        // Continue anyway, might still work
    }

    // Calculate network range
    uint32_t network_addr = base_addr & subnet_mask;
    uint32_t broadcast = network_addr | ~subnet_mask;

    // Set up destination address structure (reused for all sends)
    struct sockaddr_in dest_addr;
    memset(&dest_addr, 0, sizeof(dest_addr));
    dest_addr.sin_family = AF_INET;
    dest_addr.sin_port = htons(3702);  // WS-Discovery port

    // Prepare discovery message once
    char uuid[64];
    char message[1024];
    int message_len;

    // Generate UUID
    generate_uuid(uuid, sizeof(uuid));

    // Format message
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
    if (message_len < 0 || message_len >= sizeof(message)) {
        log_error("Failed to format discovery message");
        close(discovery_sock);
        return -1;
    }

    // First send a few probes to broadcast
    addr.s_addr = htonl(broadcast);
    strcpy(ip_addr, inet_ntoa(addr));
    log_info("Sending discovery probes to broadcast address: %s", ip_addr);

    dest_addr.sin_addr.s_addr = htonl(broadcast);
    for (int i = 0; i < 3; i++) {
        result = sendto(discovery_sock, message, message_len, 0,
                     (struct sockaddr *)&dest_addr, sizeof(dest_addr));
        if (result < 0) {
            log_warn("Failed to send broadcast probe %d: %s", i, strerror(errno));
        }
        usleep(100000); // 100ms between broadcast probes
    }

    // Send to multicast address
    log_info("Sending discovery probes to ONVIF multicast address: 239.255.255.250");
    inet_pton(AF_INET, "239.255.255.250", &dest_addr.sin_addr);
    for (int i = 0; i < 3; i++) {
        result = sendto(discovery_sock, message, message_len, 0,
                     (struct sockaddr *)&dest_addr, sizeof(dest_addr));
        if (result < 0) {
            log_warn("Failed to send multicast probe %d: %s", i, strerror(errno));
        }
        usleep(100000); // 100ms between multicast probes
    }

    // Now send to individual addresses, but only if broadcast failed
    if (result < 0) {
        log_info("Broadcast/multicast might be restricted, trying unicast to each IP");
        // Send discovery probes to all addresses in the range (with lower frequency)
        for (uint32_t ip = network_addr + 1; ip < broadcast; ip++) {
            // Skip addresses too close to network or broadcast addresses
            if (ip == network_addr + 1 || ip == broadcast - 1) {
                continue;
            }

            addr.s_addr = htonl(ip);
            strcpy(ip_addr, inet_ntoa(addr));

            // Set destination address
            dest_addr.sin_addr.s_addr = htonl(ip);

            // Send discovery probe
            if (sendto(discovery_sock, message, message_len, 0,
                       (struct sockaddr *)&dest_addr, sizeof(dest_addr)) < 0) {
                // Don't log every failure to avoid log spam
                if (errno != EHOSTUNREACH && errno != ENETUNREACH) {
                    log_debug("Failed to send unicast probe to %s: %s", ip_addr, strerror(errno));
                }
            }

            // Sleep less between unicast probes (we have more to send)
            usleep(5000); // 5ms
        }
    }

    // Always close the sending socket before receiving
    close(discovery_sock);

    // Try to receive with different timeouts
    log_info("Waiting for discovery responses with extended timeouts...");

    // First try with default timeout
    count = receive_discovery_responses(devices, max_devices);

    // If no devices found, try again with longer timeout
    if (count == 0) {
        log_info("No devices found with standard timeout, trying with extended timeout");
        // This assumes you can modify receive_discovery_responses to accept a timeout parameter
        // Otherwise, you might need to create a new function with longer timeouts

        // Here we're simulating a longer overall wait by calling again
        // Ideally you'd modify receive_discovery_responses to accept a timeout parameter
        count = receive_extended_discovery_responses(devices, max_devices, 10, 8); // 10 sec timeout, 8 attempts
    }

    log_info("Manual ONVIF discovery completed, found %d devices", count);

    return count;
}


// Test connection to an ONVIF device
int test_onvif_connection(const char *url, const char *username, const char *password) {
    // This is a placeholder implementation. In a real implementation, you would
    // use ONVIF SOAP calls to test the connection.
    
    log_info("Testing connection to ONVIF device: %s", url);
    
    // For now, just return success
    return 0;
}
