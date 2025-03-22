#include "video/onvif_discovery_network.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <ifaddrs.h>
#include <errno.h>
#include <net/if.h>
#include <stdbool.h>

// Define constants if not already defined
#ifndef NI_MAXHOST
#define NI_MAXHOST 1025
#endif

#ifndef NI_NUMERICHOST
#define NI_NUMERICHOST 0x01
#endif

// Define interface flags if not already defined
#ifndef IFF_LOOPBACK
#define IFF_LOOPBACK 0x8
#endif

#ifndef IFF_UP
#define IFF_UP 0x1
#endif

// Maximum number of networks to detect
#define MAX_DETECTED_NETWORKS 10

// Parse network string (e.g., "192.168.1.0/24") into base address and subnet mask
int parse_network(const char *network, uint32_t *base_addr, uint32_t *subnet_mask) {
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

// Detect local networks
int detect_local_networks(char networks[][64], int max_networks) {
    struct ifaddrs *ifaddr, *ifa;
    int family, s;
    char host[NI_MAXHOST];
    int count = 0;
    
    log_info("Starting network interface detection");
    
    if (getifaddrs(&ifaddr) == -1) {
        log_error("Failed to get interface addresses: %s", strerror(errno));
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
