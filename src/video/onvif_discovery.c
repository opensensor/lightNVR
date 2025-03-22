#define _GNU_SOURCE
#define _POSIX_C_SOURCE 200112L

#include "video/onvif_discovery.h"
#include "video/onvif_discovery_messages.h"
#include "video/onvif_discovery_network.h"
#include "video/onvif_discovery_probe.h"
#include "video/onvif_discovery_response.h"
#include "video/onvif_discovery_thread.h"
#include "video/onvif_device_management.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <time.h>
#include <stdbool.h>
#include <fcntl.h>

// Maximum number of networks to detect
#define MAX_DETECTED_NETWORKS 10

// Maximum number of discovered devices
#define MAX_DISCOVERED_DEVICES 32

// Array of discovered devices
static onvif_device_info_t g_discovered_devices[MAX_DISCOVERED_DEVICES];
static int g_discovered_device_count = 0;

// Mutex for thread safety
pthread_mutex_t g_discovery_mutex = PTHREAD_MUTEX_INITIALIZER;

// Initialize ONVIF discovery module
int init_onvif_discovery(void) {
    // Initialize random number generator for UUID generation
    srand(time(NULL));
    
    // Initialize discovered devices array
    memset(g_discovered_devices, 0, sizeof(g_discovered_devices));
    g_discovered_device_count = 0;
    
    log_info("ONVIF discovery module initialized");
    
    return 0;
}

// Shutdown ONVIF discovery module
void shutdown_onvif_discovery(void) {
    log_info("ONVIF discovery module shutdown");
}

// Get discovery mutex
pthread_mutex_t* get_discovery_mutex(void) {
    return &g_discovery_mutex;
}

// Get current discovery network
const char* get_current_discovery_network(void) {
    return NULL; // No longer using a persistent network
}

// Start ONVIF discovery process - deprecated, use discover_onvif_devices directly
int start_onvif_discovery(const char *network, int interval) {
    log_warn("start_onvif_discovery is deprecated, use discover_onvif_devices directly");
    return -1;
}

// Stop ONVIF discovery process - deprecated, use discover_onvif_devices directly
int stop_onvif_discovery(void) {
    log_warn("stop_onvif_discovery is deprecated, use discover_onvif_devices directly");
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

// Check if a port is open on a given IP address
static int is_port_open(const char *ip_addr, int port, int timeout_ms) {
    int sock;
    struct sockaddr_in addr;
    struct timeval tv;
    fd_set fdset;
    int res;
    
    // Create socket
    sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        return 0; // Failed to create socket
    }
    
    // Set non-blocking
    long arg = fcntl(sock, F_GETFL, NULL);
    arg |= O_NONBLOCK;
    fcntl(sock, F_SETFL, arg);
    
    // Set up address
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    
    // Convert IP address
    if (inet_aton(ip_addr, &addr.sin_addr) == 0) {
        close(sock);
        return 0; // Invalid IP address
    }
    
    // Try to connect
    res = connect(sock, (struct sockaddr *)&addr, sizeof(addr));
    
    if (res < 0) {
        if (errno == EINPROGRESS) {
            // Connection in progress, wait for result
            tv.tv_sec = timeout_ms / 1000;
            tv.tv_usec = (timeout_ms % 1000) * 1000;
            
            FD_ZERO(&fdset);
            FD_SET(sock, &fdset);
            
            // Wait for socket to be writable (connected)
            res = select(sock + 1, NULL, &fdset, NULL, &tv);
            
            if (res > 0) {
                // Socket is writable, check if there was an error
                int so_error;
                socklen_t len = sizeof(so_error);
                
                getsockopt(sock, SOL_SOCKET, SO_ERROR, &so_error, &len);
                
                if (so_error == 0) {
                    close(sock);
                    return 1; // Port is open
                }
            }
        }
    } else {
        // Connected immediately
        close(sock);
        return 1; // Port is open
    }
    
    close(sock);
    return 0; // Port is closed or connection timed out
}

// Discover ONVIF devices on a specific network
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

    log_info("Starting ONVIF discovery on network %s", network);

    // Parse network
    if (parse_network(network, &base_addr, &subnet_mask) != 0) {
        log_error("Failed to parse network: %s", network);
        return -1;
    }

    // Calculate network range
    uint32_t network_addr = base_addr & subnet_mask;
    uint32_t broadcast = network_addr | ~subnet_mask;
    
    // First scan for open ports on the network
    log_info("Scanning network for open ONVIF ports (3702 and 80)");
    
    // Array to store IPs with open ports
    #define MAX_CANDIDATE_IPS 256
    char candidate_ips[MAX_CANDIDATE_IPS][16];
    int candidate_count = 0;
    
    // Scan all IPs in the range
    for (uint32_t ip = network_addr + 1; ip < broadcast && candidate_count < MAX_CANDIDATE_IPS; ip++) {
        // Skip addresses too close to network or broadcast addresses
        if (ip == network_addr + 1 || ip == broadcast - 1) {
            continue;
        }

        addr.s_addr = htonl(ip);
        strcpy(ip_addr, inet_ntoa(addr));
        
        // Check if port 3702 (ONVIF) or port 80 (HTTP) is open
        if (is_port_open(ip_addr, 3702, 100) || is_port_open(ip_addr, 80, 100)) {
            log_debug("Found potential ONVIF device at %s", ip_addr);
            
            // Add to candidate list
            strncpy(candidate_ips[candidate_count], ip_addr, 16);
            candidate_count++;
        }
    }
    
    log_info("Found %d potential ONVIF devices with open ports", candidate_count);
    
    // If no candidates found, try broadcast and multicast
    if (candidate_count == 0) {
        log_info("No devices with open ports found, trying broadcast and multicast");
        
        // Create a socket for discovery probes
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

        // Set up destination address structure
        struct sockaddr_in dest_addr;
        memset(&dest_addr, 0, sizeof(dest_addr));
        dest_addr.sin_family = AF_INET;
        dest_addr.sin_port = htons(3702);  // WS-Discovery port

        // Send probes to broadcast address
        addr.s_addr = htonl(broadcast);
        strcpy(ip_addr, inet_ntoa(addr));
        log_info("Sending discovery probes to broadcast address: %s", ip_addr);

        dest_addr.sin_addr.s_addr = htonl(broadcast);
        send_all_discovery_probes(discovery_sock, ip_addr, &dest_addr);

        // Send to multicast address
        log_info("Sending discovery probes to ONVIF multicast address: 239.255.255.250");
        inet_pton(AF_INET, "239.255.255.250", &dest_addr.sin_addr);
        send_all_discovery_probes(discovery_sock, "239.255.255.250", &dest_addr);

        // Close the sending socket
        close(discovery_sock);
        
        // Try to receive responses
        log_info("Waiting for discovery responses...");
        count = receive_discovery_responses(devices, max_devices);
        
        // If no devices found, try with a slightly longer timeout but still keep it under 30 seconds total
        if (count == 0) {
            log_info("No devices found with standard timeout, trying with slightly longer timeout");
            count = receive_extended_discovery_responses(devices, max_devices, 3, 3); // 3 sec timeout, 3 attempts
        }
    } else {
        // Create a socket for discovery probes
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

        // Set up destination address structure
        struct sockaddr_in dest_addr;
        memset(&dest_addr, 0, sizeof(dest_addr));
        dest_addr.sin_family = AF_INET;
        
        // Send probes to each candidate IP
        log_info("Sending discovery probes to %d candidate IPs", candidate_count);
        
        for (int i = 0; i < candidate_count; i++) {
            // Try port 3702 (ONVIF discovery)
            dest_addr.sin_port = htons(3702);
            if (inet_aton(candidate_ips[i], &dest_addr.sin_addr) != 0) {
                log_debug("Sending discovery probe to %s:3702", candidate_ips[i]);
                send_all_discovery_probes(discovery_sock, candidate_ips[i], &dest_addr);
            }
            
            // Try port 80 (HTTP)
            dest_addr.sin_port = htons(80);
            if (inet_aton(candidate_ips[i], &dest_addr.sin_addr) != 0) {
                log_debug("Sending discovery probe to %s:80", candidate_ips[i]);
                send_all_discovery_probes(discovery_sock, candidate_ips[i], &dest_addr);
            }
        }
        
        // Close the sending socket
        close(discovery_sock);
        
        // Try to receive responses
        log_info("Waiting for discovery responses...");
        count = receive_discovery_responses(devices, max_devices);
        
        // If no devices found, try with a slightly longer timeout but still keep it under 30 seconds total
        if (count == 0) {
            log_info("No devices found with standard timeout, trying with slightly longer timeout");
            count = receive_extended_discovery_responses(devices, max_devices, 3, 3); // 3 sec timeout, 3 attempts
        }
    }

    // Store the discovered devices for later retrieval
    pthread_mutex_lock(&g_discovery_mutex);
    g_discovered_device_count = 0;
    
    for (int i = 0; i < count && i < MAX_DISCOVERED_DEVICES; i++) {
        memcpy(&g_discovered_devices[i], &devices[i], sizeof(onvif_device_info_t));
        g_discovered_device_count++;
    }
    
    pthread_mutex_unlock(&g_discovery_mutex);

    log_info("ONVIF discovery completed, found %d devices", count);

    return count;
}
