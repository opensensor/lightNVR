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
    
    // Create socket
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }
    
    // Set socket options
    int reuse = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &reuse, sizeof(reuse)) < 0) {
        log_error("Failed to set socket options: %s", strerror(errno));
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
    
    // Set timeout for select
    timeout.tv_sec = 2;
    timeout.tv_usec = 0;
    
    // Wait for responses
    for (int i = 0; i < 3; i++) { // Try a few times to get responses
        FD_ZERO(&readfds);
        FD_SET(sock, &readfds);
        
        ret = select(sock + 1, &readfds, NULL, NULL, &timeout);
        if (ret < 0) {
            log_error("Select failed: %s", strerror(errno));
            close(sock);
            return -1;
        } else if (ret == 0) {
            // Timeout, no data available
            continue;
        }
        
        // Receive data
        ret = recvfrom(sock, buffer, sizeof(buffer) - 1, 0, (struct sockaddr *)&addr, &addr_len);
        if (ret < 0) {
            log_error("Failed to receive data: %s", strerror(errno));
            continue;
        }
        
        // Null-terminate the buffer
        buffer[ret] = '\0';
        
        // Parse device information
        if (count < max_devices) {
            if (parse_device_info(buffer, &devices[count]) == 0) {
                log_info("Discovered ONVIF device: %s", devices[count].device_service);
                count++;
            }
        }
    }
    
    // Close socket
    close(sock);
    
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
        
        // Also send to broadcast address
        addr.s_addr = htonl(broadcast);
        strcpy(ip_addr, inet_ntoa(addr));
        send_discovery_probe(ip_addr);
        
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
    // Validate parameters
    if (!network || interval <= 0) {
        log_error("Invalid parameters for start_onvif_discovery");
        return -1;
    }
    
    // Stop existing discovery thread if running
    stop_onvif_discovery();
    
    // Initialize thread data
    pthread_mutex_lock(&g_discovery_mutex);
    
    g_discovery_thread.running = 1;
    strncpy(g_discovery_thread.network, network, sizeof(g_discovery_thread.network) - 1);
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
    
    log_info("ONVIF discovery started on network %s with interval %d seconds", network, interval);
    
    return 0;
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

// Manually discover ONVIF devices on a specific network
int discover_onvif_devices(const char *network, onvif_device_info_t *devices, 
                          int max_devices) {
    uint32_t base_addr, subnet_mask;
    char ip_addr[16];
    struct in_addr addr;
    int count;
    
    if (!network || !devices || max_devices <= 0) {
        log_error("Invalid parameters for discover_onvif_devices");
        return -1;
    }
    
    log_info("Starting manual ONVIF discovery on network %s", network);
    
    // Parse network
    if (parse_network(network, &base_addr, &subnet_mask) != 0) {
        log_error("Failed to parse network: %s", network);
        return -1;
    }
    
    // Calculate network range
    uint32_t network_addr = base_addr & subnet_mask;
    uint32_t broadcast = network_addr | ~subnet_mask;
    
    // Send discovery probes to all addresses in the range
    for (uint32_t ip = network_addr + 1; ip < broadcast; ip++) {
        addr.s_addr = htonl(ip);
        strcpy(ip_addr, inet_ntoa(addr));
        
        // Send discovery probe
        send_discovery_probe(ip_addr);
        
        // Sleep a bit to avoid flooding the network
        usleep(10000); // 10ms
    }
    
    // Also send to broadcast address
    addr.s_addr = htonl(broadcast);
    strcpy(ip_addr, inet_ntoa(addr));
    send_discovery_probe(ip_addr);
    
    // Receive responses
    count = receive_discovery_responses(devices, max_devices);
    
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
