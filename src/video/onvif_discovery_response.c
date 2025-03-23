#include "video/onvif_discovery_response.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/select.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <ctype.h>
#include <time.h>
#include <stdbool.h>

// Parse ONVIF device information from discovery response
int parse_device_info(const char *response, onvif_device_info_t *device_info) {
    // This is a more robust implementation for parsing ONVIF discovery responses
    // without using a full XML parser
    
    // Initialize device info
    memset(device_info, 0, sizeof(onvif_device_info_t));
    
    // Log the first 200 characters of the response for debugging
    char debug_buffer[201];
    strncpy(debug_buffer, response, 200);
    debug_buffer[200] = '\0';
    log_debug("Parsing response: %s...", debug_buffer);
    
    // Check if this is a valid ONVIF response
    if (!strstr(response, "NetworkVideoTransmitter") && 
        !strstr(response, "Device") && 
        !strstr(response, "ONVIF")) {
        log_debug("Not an ONVIF response (missing required keywords)");
        return -1;
    }
    
    // Extract XAddrs (device service URLs)
    const char *xaddr_start = strstr(response, "<d:XAddrs>");
    const char *xaddr_end = NULL;
    
    // Try alternative tag formats if the first one fails
    if (!xaddr_start) {
        xaddr_start = strstr(response, "<XAddrs>");
        if (xaddr_start) {
            xaddr_start += 8; // Skip "<XAddrs>"
            xaddr_end = strstr(xaddr_start, "</XAddrs>");
        }
    } else {
        xaddr_start += 10; // Skip "<d:XAddrs>"
        xaddr_end = strstr(xaddr_start, "</d:XAddrs>");
    }
    
    // If we still don't have XAddrs, try one more format
    if (!xaddr_start) {
        xaddr_start = strstr(response, ":XAddrs>");
        if (xaddr_start) {
            xaddr_start = strchr(xaddr_start, '>') + 1;
            xaddr_end = strstr(xaddr_start, "</");
        }
    }
    
    if (!xaddr_start || !xaddr_end) {
        log_debug("Failed to find XAddrs in response");
        return -1;
    }
    
    // Extract and trim the XAddrs value
    size_t len = xaddr_end - xaddr_start;
    if (len >= MAX_URL_LENGTH) {
        len = MAX_URL_LENGTH - 1;
    }
    
    char xaddrs[MAX_URL_LENGTH];
    strncpy(xaddrs, xaddr_start, len);
    xaddrs[len] = '\0';
    
    // Trim leading/trailing whitespace
    char *start = xaddrs;
    char *end = xaddrs + len - 1;
    
    while (*start && isspace(*start)) start++;
    while (end > start && isspace(*end)) *end-- = '\0';
    
    // Split multiple URLs if present (some devices return multiple space-separated URLs)
    char *url = strtok(start, " \t\n\r");
    if (url) {
        strncpy(device_info->device_service, url, MAX_URL_LENGTH - 1);
        device_info->device_service[MAX_URL_LENGTH - 1] = '\0';
        
        // Also store as endpoint
        strncpy(device_info->endpoint, url, MAX_URL_LENGTH - 1);
        device_info->endpoint[MAX_URL_LENGTH - 1] = '\0';
        
        log_debug("Found device service URL: %s", device_info->device_service);
    } else {
        log_debug("No valid URL found in XAddrs");
        return -1;
    }
    
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
            log_debug("Extracted IP address: %s", device_info->ip_address);
        }
    }
    
    // Try to extract device type/model information
    const char *types_start = strstr(response, "<d:Types>");
    const char *types_end = NULL;
    
    if (!types_start) {
        types_start = strstr(response, "<Types>");
        if (types_start) {
            types_start += 7; // Skip "<Types>"
            types_end = strstr(types_start, "</Types>");
        }
    } else {
        types_start += 9; // Skip "<d:Types>"
        types_end = strstr(types_start, "</d:Types>");
    }
    
    if (types_start && types_end) {
        len = types_end - types_start;
        if (len > 0 && len < 64) {
            char types[64];
            strncpy(types, types_start, len);
            types[len] = '\0';
            
            // Extract model information if available
            const char *model_start = strstr(types, "NetworkVideoTransmitter");
            if (model_start) {
                strncpy(device_info->model, "NetworkVideoTransmitter", sizeof(device_info->model) - 1);
                device_info->model[sizeof(device_info->model) - 1] = '\0';
            }
        }
    }
    
    // Set discovery time
    device_info->discovery_time = time(NULL);
    
    // Set online status
    device_info->online = true;
    
    log_info("Successfully parsed device info: %s (%s)", 
             device_info->device_service, device_info->ip_address);
    
    return 0;
}

// Receive and process discovery responses
int receive_discovery_responses(onvif_device_info_t *devices, int max_devices) {
    int sock;
    struct sockaddr_in addr;
    socklen_t addr_len = sizeof(addr);
    char buffer[16384]; // Larger buffer for responses
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
    
    // Increase socket buffer size
    int rcvbuf = 1024 * 1024; // 1MB buffer
    if (setsockopt(sock, SOL_SOCKET, SO_RCVBUF, &rcvbuf, sizeof(rcvbuf)) < 0) {
        log_warn("Failed to increase receive buffer size: %s", strerror(errno));
        // Continue anyway
    }
    
    // Get actual buffer size for logging
    int actual_rcvbuf;
    socklen_t optlen = sizeof(actual_rcvbuf);
    if (getsockopt(sock, SOL_SOCKET, SO_RCVBUF, &actual_rcvbuf, &optlen) == 0) {
        log_info("Socket receive buffer size: %d bytes", actual_rcvbuf);
    }
    
    // Join multicast group for ONVIF discovery
    // Use a char buffer to avoid struct ip_mreq issues
    char mreq_buf[8];
    struct in_addr *imr_multiaddr = (struct in_addr *)mreq_buf;
    struct in_addr *imr_interface = (struct in_addr *)(mreq_buf + sizeof(struct in_addr));
    
    imr_multiaddr->s_addr = inet_addr("239.255.255.250");
    imr_interface->s_addr = htonl(INADDR_ANY);
    
    if (setsockopt(sock, IPPROTO_IP, IP_ADD_MEMBERSHIP, mreq_buf, sizeof(mreq_buf)) < 0) {
        log_warn("Failed to join multicast group: %s", strerror(errno));
        // Continue anyway, unicast and broadcast might still work
    }
    
    // Set multicast TTL
    int ttl = 4;
    if (setsockopt(sock, IPPROTO_IP, IP_MULTICAST_TTL, &ttl, sizeof(ttl)) < 0) {
        log_warn("Failed to set multicast TTL: %s", strerror(errno));
    }
    
    // Set multicast loopback
    int loopback = 1;
    if (setsockopt(sock, IPPROTO_IP, IP_MULTICAST_LOOP, &loopback, sizeof(loopback)) < 0) {
        log_warn("Failed to enable multicast loopback: %s", strerror(errno));
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
    
    log_info("Waiting for discovery responses (timeout: 1 second, attempts: 2)");
    
    // Set timeout for select
    timeout.tv_sec = 1;
    timeout.tv_usec = 0;
    
    // Wait for responses
    for (int i = 0; i < 2; i++) {
        log_info("Waiting for responses, attempt %d/2", i+1);
        
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
            
            // On every attempt, try sending a new probe to the broadcast address
            // This helps speed up discovery
            {
                log_info("Sending additional discovery probe to broadcast address");
                struct sockaddr_in broadcast_addr;
                memset(&broadcast_addr, 0, sizeof(broadcast_addr));
                broadcast_addr.sin_family = AF_INET;
                broadcast_addr.sin_port = htons(3702);
                broadcast_addr.sin_addr.s_addr = inet_addr("255.255.255.255"); // Use general broadcast
                
                char uuid[64];
                char message[1024];
                extern const char *ONVIF_DISCOVERY_MSG;
                extern void generate_uuid(char *uuid, size_t size);
                
                generate_uuid(uuid, sizeof(uuid));
                int message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
                
                sendto(sock, message, message_len, 0, (struct sockaddr *)&broadcast_addr, sizeof(broadcast_addr));
            }
            
            continue;
        }
        
        // Process all available responses without blocking
        while (1) {
            // Receive data with MSG_DONTWAIT to avoid blocking
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
            if (count < max_devices) {
                if (parse_device_info(buffer, &devices[count]) == 0) {
                    log_info("Discovered ONVIF device: %s (%s)", 
                            devices[count].device_service, devices[count].ip_address);
                    
                    // Check if this is a duplicate device
                    bool duplicate = false;
                    for (int j = 0; j < count; j++) {
                        if (strcmp(devices[j].ip_address, devices[count].ip_address) == 0) {
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
            }
            
            // If we've filled our device buffer, stop receiving
            if (count >= max_devices) {
                break;
            }
        }
        
        // Reset timeout for next attempt
        timeout.tv_sec = 1;
        timeout.tv_usec = 0;
    }
    
    // Close socket
    close(sock);
    
    log_info("Discovery response collection completed, found %d devices", count);
    
    return count;
}

// Extended version of receive_discovery_responses with configurable timeouts
int receive_extended_discovery_responses(onvif_device_info_t *devices, int max_devices,
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
    // Use a char buffer to avoid struct ip_mreq issues
    char mreq_buf[8];
    struct in_addr *imr_multiaddr = (struct in_addr *)mreq_buf;
    struct in_addr *imr_interface = (struct in_addr *)(mreq_buf + sizeof(struct in_addr));
    
    imr_multiaddr->s_addr = inet_addr("239.255.255.250");
    imr_interface->s_addr = htonl(INADDR_ANY);
    
    if (setsockopt(sock, IPPROTO_IP, IP_ADD_MEMBERSHIP, mreq_buf, sizeof(mreq_buf)) < 0) {
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
