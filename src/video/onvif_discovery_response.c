#include "video/onvif_discovery_response.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/time.h>
#include <sys/select.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>
#include <ctype.h>
#include <time.h>
#include <stdbool.h>
#include <netinet/ip.h>

// Helper function to extract content between XML tags
static char* extract_xml_content(const char *xml, const char *tag_start, const char *tag_end, char *buffer, size_t buffer_size) {
    const char *start = strstr(xml, tag_start);
    if (!start) {
        return NULL;
    }
    
    start += strlen(tag_start);
    const char *end = strstr(start, tag_end);
    if (!end) {
        return NULL;
    }
    
    size_t len = end - start;
    if (len >= buffer_size) {
        len = buffer_size - 1;
    }
    
    strncpy(buffer, start, len);
    buffer[len] = '\0';
    
    // Trim leading/trailing whitespace
    char *trim_start = buffer;
    char *trim_end = buffer + len - 1;
    
    while (*trim_start && isspace(*trim_start)) trim_start++;
    while (trim_end > trim_start && isspace(*trim_end)) *trim_end-- = '\0';
    
    if (trim_start != buffer) {
        memmove(buffer, trim_start, strlen(trim_start) + 1);
    }
    
    return buffer;
}

// Parse ONVIF device information from discovery response
int parse_device_info(const char *response, onvif_device_info_t *device_info) {
    // This is a more robust implementation for parsing ONVIF discovery responses
    // without using a full XML parser
    
    // Initialize device info
    memset(device_info, 0, sizeof(onvif_device_info_t));
    
    // Log the first 500 characters of the response for debugging
    char debug_buffer[501];
    strncpy(debug_buffer, response, 500);
    debug_buffer[500] = '\0';
    log_info("Parsing response: %s...", debug_buffer);
    
    // Check if this is a valid ONVIF response and not a probe message
    if (strstr(response, "Probe") && !strstr(response, "ProbeMatch")) {
        log_info("Ignoring probe message (not a device response)");
        return -1;
    }
    
    if (!strstr(response, "NetworkVideoTransmitter") && 
        !strstr(response, "Device") && 
        !strstr(response, "ONVIF")) {
        log_info("Not an ONVIF response (missing required keywords)");
        return -1;
    }
    
    // Try to extract XAddrs using different tag formats
    char xaddrs[MAX_URL_LENGTH] = {0};
    
    // Try with d:XAddrs format
    if (!extract_xml_content(response, "<d:XAddrs>", "</d:XAddrs>", xaddrs, sizeof(xaddrs))) {
        // Try with XAddrs format
        if (!extract_xml_content(response, "<XAddrs>", "</XAddrs>", xaddrs, sizeof(xaddrs))) {
            // Try with any namespace prefix
            const char *xaddr_tag = strstr(response, "XAddrs>");
            if (xaddr_tag) {
                const char *start = strchr(xaddr_tag, '>');
                if (start) {
                    start++; // Skip '>'
                    const char *end = strstr(start, "</");
                    if (end) {
                        size_t len = end - start;
                        if (len < sizeof(xaddrs)) {
                            strncpy(xaddrs, start, len);
                            xaddrs[len] = '\0';
                            
                            // Trim whitespace
                            char *trim_start = xaddrs;
                            char *trim_end = xaddrs + len - 1;
                            
                            while (*trim_start && isspace(*trim_start)) trim_start++;
                            while (trim_end > trim_start && isspace(*trim_end)) *trim_end-- = '\0';
                            
                            if (trim_start != xaddrs) {
                                memmove(xaddrs, trim_start, strlen(trim_start) + 1);
                            }
                        }
                    }
                }
            }
        }
    }
    
    if (strlen(xaddrs) == 0) {
        log_debug("Failed to find XAddrs in response");
        return -1;
    }
    
    log_debug("Found XAddrs: %s", xaddrs);
    
    // Split multiple URLs if present (some devices return multiple space-separated URLs)
    char *url = strtok(xaddrs, " \t\n\r");
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
            size_t len = ip_end - ip_start;
            if (len >= sizeof(device_info->ip_address)) {
                len = sizeof(device_info->ip_address) - 1;
            }
            
            strncpy(device_info->ip_address, ip_start, len);
            device_info->ip_address[len] = '\0';
            log_debug("Extracted IP address: %s", device_info->ip_address);
        }
    }
    
    // Try to extract device type/model information
    char types[128] = {0};
    
    // Try with d:Types format
    if (!extract_xml_content(response, "<d:Types>", "</d:Types>", types, sizeof(types))) {
        // Try with Types format
        extract_xml_content(response, "<Types>", "</Types>", types, sizeof(types));
    }
    
    if (strlen(types) > 0) {
        // Extract model information if available
        if (strstr(types, "NetworkVideoTransmitter")) {
            strncpy(device_info->model, "NetworkVideoTransmitter", sizeof(device_info->model) - 1);
            device_info->model[sizeof(device_info->model) - 1] = '\0';
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
    char *buffer = NULL;
    int buffer_size = 8192; // Buffer size for responses
    int ret;
    int count = 0;
    
    // Allocate buffer dynamically to avoid stack overflow on embedded devices
    buffer = (char *)malloc(buffer_size);
    if (!buffer) {
        log_error("Failed to allocate memory for receive buffer");
        return -1;
    }
    fd_set readfds;
    struct timeval timeout;
    
    log_info("Setting up socket to receive discovery responses");
    
    // Create socket
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }
    
    // Set socket options for address reuse
    int reuse = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &reuse, sizeof(reuse)) < 0) {
        log_error("Failed to set socket options (SO_REUSEADDR): %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    // Set SO_REUSEPORT if available (not available on all systems)
    #ifdef SO_REUSEPORT
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEPORT, &reuse, sizeof(reuse)) < 0) {
        log_warn("Failed to set SO_REUSEPORT option: %s", strerror(errno));
        // Continue anyway, SO_REUSEADDR might be enough
    }
    #endif
    
    // Set broadcast option
    int broadcast = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_BROADCAST, &broadcast, sizeof(broadcast)) < 0) {
        log_error("Failed to set socket options (SO_BROADCAST): %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    // Increase socket buffer size (reduced for embedded devices)
    int rcvbuf = 256 * 1024; // 256KB buffer
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
    // This is a workaround for systems where ip_mreq might not be properly defined
    char mreq_buf[8]; // Size of two struct in_addr
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
    
    // Try binding with a few retries if it fails with "Address in use"
    int bind_attempts = 0;
    int max_bind_attempts = 5;
    int bind_result = -1;
    int original_port = ntohs(addr.sin_port); // Save original port
    
    // Add a longer delay before first binding attempt to allow TIME_WAIT sockets to clear
    log_info("Waiting 2 seconds before binding to allow TIME_WAIT sockets to clear");
    sleep(2);
    
    while (bind_attempts < max_bind_attempts) {
        bind_result = bind(sock, (struct sockaddr *)&addr, sizeof(addr));
        if (bind_result == 0) {
            // Binding successful
            log_info("Successfully bound to port %d", ntohs(addr.sin_port));
            break;
        }
        
        log_warn("Failed to bind to port %d: %s (attempt %d/%d)", 
                ntohs(addr.sin_port), strerror(errno), bind_attempts + 1, max_bind_attempts);
        
        if (errno != EADDRINUSE) {
            // If error is not "Address in use", try a different port
            log_warn("Trying a different port...");
            addr.sin_port = htons(original_port + bind_attempts + 1);
        } else {
            // If "Address in use", wait a bit and retry
            log_warn("Address in use, waiting 1 second before retry");
            sleep(1);
            
            // After a couple of attempts, try a different port
            if (bind_attempts >= 2) {
                addr.sin_port = htons(original_port + 10000 + bind_attempts);
                log_warn("Trying alternative port %d", ntohs(addr.sin_port));
            }
        }
        
        bind_attempts++;
    }
    
    if (bind_result < 0) {
        log_warn("Failed to bind socket after %d attempts: %s", 
                max_bind_attempts, strerror(errno));
        log_warn("Continuing with discovery process, but may not receive responses");
        // Continue anyway, we can still send probes to the broadcast address
    }
    
    // Add a small delay after binding to ensure the socket is fully ready
    usleep(100000); // 100ms
    
    log_info("Waiting for discovery responses (timeout: 10 seconds, attempts: 5)");
    
    // Set timeout for select - significantly increased timeout
    timeout.tv_sec = 10;
    timeout.tv_usec = 0;
    
    // Wait for responses - increased number of attempts
    for (int i = 0; i < 5; i++) {
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
                extern const char *ONVIF_DISCOVERY_MSG_ALT;
                extern void generate_uuid(char *uuid, size_t size);
                
                // Send both standard and alternative message formats
                generate_uuid(uuid, sizeof(uuid));
                int message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
                sendto(sock, message, message_len, 0, (struct sockaddr *)&broadcast_addr, sizeof(broadcast_addr));
                
                usleep(50000); // 50ms delay between messages
                
                generate_uuid(uuid, sizeof(uuid));
                message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG_ALT, uuid);
                sendto(sock, message, message_len, 0, (struct sockaddr *)&broadcast_addr, sizeof(broadcast_addr));
            }
            
            continue;
        }
        
        // Process all available responses without blocking
        while (1) {
            // Receive data with MSG_DONTWAIT to avoid blocking
            ret = recvfrom(sock, buffer, buffer_size - 1, MSG_DONTWAIT, 
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
            
            // Dump the first 500 characters of response for debugging
            char debug_buffer[501];
            strncpy(debug_buffer, buffer, 500);
            debug_buffer[500] = '\0';
            log_info("Response (first 500 chars): %s", debug_buffer);
            
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
        timeout.tv_sec = 2;
        timeout.tv_usec = 0;
    }
    
    // Close socket
    close(sock);
    
    // Free dynamically allocated buffer
    free(buffer);
    
    log_info("Discovery response collection completed, found %d devices", count);
    
    return count;
}

// Extended version of receive_discovery_responses with configurable timeouts
int receive_extended_discovery_responses(onvif_device_info_t *devices, int max_devices,
                                        int timeout_sec, int max_attempts) {
    int sock;
    struct sockaddr_in addr;
    socklen_t addr_len = sizeof(addr);
    char *buffer = NULL;
    int buffer_size = 8192; // Buffer size for responses
    int ret;
    int count = 0;
    
    // Allocate buffer dynamically to avoid stack overflow on embedded devices
    buffer = (char *)malloc(buffer_size);
    if (!buffer) {
        log_error("Failed to allocate memory for receive buffer");
        return -1;
    }
    fd_set readfds;
    struct timeval timeout;

    log_info("Setting up socket to receive discovery responses (extended timeouts)");

    // Create socket
    sock = socket(AF_INET, SOCK_DGRAM, 0);
    if (sock < 0) {
        log_error("Failed to create socket: %s", strerror(errno));
        return -1;
    }

    // Set socket options for address reuse
    int reuse = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &reuse, sizeof(reuse)) < 0) {
        log_error("Failed to set socket options (SO_REUSEADDR): %s", strerror(errno));
        close(sock);
        return -1;
    }
    
    // Set SO_REUSEPORT if available (not available on all systems)
    #ifdef SO_REUSEPORT
    if (setsockopt(sock, SOL_SOCKET, SO_REUSEPORT, &reuse, sizeof(reuse)) < 0) {
        log_warn("Failed to set SO_REUSEPORT option: %s", strerror(errno));
        // Continue anyway, SO_REUSEADDR might be enough
    }
    #endif

    // Set broadcast option
    int broadcast = 1;
    if (setsockopt(sock, SOL_SOCKET, SO_BROADCAST, &broadcast, sizeof(broadcast)) < 0) {
        log_error("Failed to set socket options (SO_BROADCAST): %s", strerror(errno));
        close(sock);
        return -1;
    }

    // Join multicast group for ONVIF discovery
    // Use a char buffer to avoid struct ip_mreq issues
    // This is a workaround for systems where ip_mreq might not be properly defined
    char mreq_buf[8]; // Size of two struct in_addr
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

    // Try binding with a few retries if it fails with "Address in use"
    int bind_attempts = 0;
    int max_bind_attempts = 5;
    int bind_result = -1;
    int original_port = ntohs(addr.sin_port); // Save original port
    
    // Add a longer delay before first binding attempt to allow TIME_WAIT sockets to clear
    log_info("Waiting 2 seconds before binding to allow TIME_WAIT sockets to clear");
    sleep(2);
    
    while (bind_attempts < max_bind_attempts) {
        bind_result = bind(sock, (struct sockaddr *)&addr, sizeof(addr));
        if (bind_result == 0) {
            // Binding successful
            log_info("Successfully bound to port %d", ntohs(addr.sin_port));
            break;
        }
        
        log_warn("Failed to bind to port %d: %s (attempt %d/%d)", 
                ntohs(addr.sin_port), strerror(errno), bind_attempts + 1, max_bind_attempts);
        
        if (errno != EADDRINUSE) {
            // If error is not "Address in use", try a different port
            log_warn("Trying a different port...");
            addr.sin_port = htons(original_port + bind_attempts + 1);
        } else {
            // If "Address in use", wait a bit and retry
            log_warn("Address in use, waiting 1 second before retry");
            sleep(1);
            
            // After a couple of attempts, try a different port
            if (bind_attempts >= 2) {
                addr.sin_port = htons(original_port + 10000 + bind_attempts);
                log_warn("Trying alternative port %d", ntohs(addr.sin_port));
            }
        }
        
        bind_attempts++;
    }
    
    if (bind_result < 0) {
        log_warn("Failed to bind socket after %d attempts: %s", 
                max_bind_attempts, strerror(errno));
        log_warn("Continuing with discovery process, but may not receive responses");
        // Continue anyway, we can still send probes to the broadcast address
    }
    
    // Add a small delay after binding to ensure the socket is fully ready
    usleep(100000); // 100ms

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
            ret = recvfrom(sock, buffer, buffer_size - 1, MSG_DONTWAIT,
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

            // Dump the full response for debugging
            log_info("Full response: %s", buffer);

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
    
    // Free dynamically allocated buffer
    free(buffer);
    
    log_info("Discovery response collection completed, found %d devices", count);
    
    return count;
}
