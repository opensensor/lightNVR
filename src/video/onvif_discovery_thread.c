#include "video/onvif_discovery_thread.h"
#include "video/onvif_discovery_network.h"
#include "video/onvif_discovery_probe.h"
#include "video/onvif_discovery_response.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <time.h>

// Maximum number of discovered devices
#define MAX_DISCOVERED_DEVICES 32

// Global discovery thread
discovery_thread_t g_discovery_thread = {0};

// Mutex for thread safety (defined in onvif_discovery.c)
extern pthread_mutex_t g_discovery_mutex;

// Array of discovered devices
static onvif_device_info_t g_discovered_devices[MAX_DISCOVERED_DEVICES];
static int g_discovered_device_count = 0;

// Discovery thread function
void *discovery_thread_func(void *arg) {
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
