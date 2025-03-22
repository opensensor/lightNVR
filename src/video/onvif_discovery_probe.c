#include "video/onvif_discovery_probe.h"
#include "video/onvif_discovery_messages.h"
#include "core/logger.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <errno.h>

// Send WS-Discovery probe message to a specific IP address
int send_discovery_probe(const char *ip_addr) {
    int sock;
    struct sockaddr_in addr;
    char uuid[64];
    char message[1024];
    int message_len;
    int ret;
    int success = 0;
    
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
    
    // Try with standard WS-Discovery port (3702)
    memset(&addr, 0, sizeof(addr));
    addr.sin_family = AF_INET;
    addr.sin_port = htons(3702); // Standard WS-Discovery port
    
    // Convert IP address
    if (inet_aton(ip_addr, &addr.sin_addr) == 0) {
        log_error("Invalid IP address: %s", ip_addr);
        close(sock);
        return -1;
    }
    
    // Generate UUID
    generate_uuid(uuid, sizeof(uuid));
    
    // Send standard ONVIF discovery message
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
        log_error("Failed to send standard discovery probe to %s:%d: %s", 
                 ip_addr, ntohs(addr.sin_port), strerror(errno));
    } else {
        log_debug("Sent standard discovery probe to %s:%d", ip_addr, ntohs(addr.sin_port));
        success = 1;
    }
    
    // Wait a bit before sending the next message
    usleep(50000); // 50ms
    
    // Send alternative ONVIF discovery message
    generate_uuid(uuid, sizeof(uuid)); // Generate a new UUID for this message
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG_ALT, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
        log_warn("Failed to send alternative discovery probe to %s:%d: %s", 
                ip_addr, ntohs(addr.sin_port), strerror(errno));
        // Continue anyway, the standard message might work
    } else {
        log_debug("Sent alternative discovery probe to %s:%d", ip_addr, ntohs(addr.sin_port));
        success = 1;
    }
    
    // Wait a bit before sending the next message
    usleep(50000); // 50ms
    
    // Send ONVIF discovery message with scope
    generate_uuid(uuid, sizeof(uuid)); // Generate a new UUID for this message
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG_WITH_SCOPE, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
        log_warn("Failed to send scoped discovery probe to %s:%d: %s", 
                ip_addr, ntohs(addr.sin_port), strerror(errno));
        // Continue anyway, the previous messages might work
    } else {
        log_debug("Sent scoped discovery probe to %s:%d", ip_addr, ntohs(addr.sin_port));
        success = 1;
    }
    
    // Now try with HTTP port (80) - many ONVIF cameras also respond on this port
    addr.sin_port = htons(80); // HTTP port
    
    // Generate UUID
    generate_uuid(uuid, sizeof(uuid));
    
    // Send standard ONVIF discovery message to HTTP port
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
        log_warn("Failed to send standard discovery probe to %s:%d: %s", 
                ip_addr, ntohs(addr.sin_port), strerror(errno));
    } else {
        log_debug("Sent standard discovery probe to %s:%d", ip_addr, ntohs(addr.sin_port));
        success = 1;
    }
    
    // Wait a bit before sending the next message
    usleep(50000); // 50ms
    
    // Send alternative ONVIF discovery message to HTTP port
    generate_uuid(uuid, sizeof(uuid));
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG_ALT, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)&addr, sizeof(addr));
    if (ret < 0) {
        log_warn("Failed to send alternative discovery probe to %s:%d: %s", 
                ip_addr, ntohs(addr.sin_port), strerror(errno));
    } else {
        log_debug("Sent alternative discovery probe to %s:%d", ip_addr, ntohs(addr.sin_port));
        success = 1;
    }
    
    // Close socket
    close(sock);
    
    return success ? 0 : -1;
}

// Send discovery probes to a specific IP address using all message templates
int send_all_discovery_probes(int sock, const char *ip_addr, struct sockaddr_in *dest_addr) {
    char uuid[64];
    char message[1024];
    int message_len;
    int ret;
    
    // Generate UUID for standard message
    generate_uuid(uuid, sizeof(uuid));
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)dest_addr, sizeof(*dest_addr));
    if (ret < 0) {
        log_warn("Failed to send standard discovery probe to %s: %s", ip_addr, strerror(errno));
    } else {
        log_debug("Sent standard discovery probe to %s", ip_addr);
    }
    
    // Wait a bit before sending the next message
    usleep(50000); // 50ms
    
    // Generate UUID for alternative message
    generate_uuid(uuid, sizeof(uuid));
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG_ALT, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)dest_addr, sizeof(*dest_addr));
    if (ret < 0) {
        log_warn("Failed to send alternative discovery probe to %s: %s", ip_addr, strerror(errno));
    } else {
        log_debug("Sent alternative discovery probe to %s", ip_addr);
    }
    
    // Wait a bit before sending the next message
    usleep(50000); // 50ms
    
    // Generate UUID for scoped message
    generate_uuid(uuid, sizeof(uuid));
    message_len = snprintf(message, sizeof(message), ONVIF_DISCOVERY_MSG_WITH_SCOPE, uuid);
    ret = sendto(sock, message, message_len, 0, (struct sockaddr *)dest_addr, sizeof(*dest_addr));
    if (ret < 0) {
        log_warn("Failed to send scoped discovery probe to %s: %s", ip_addr, strerror(errno));
        return -1; // Return error only if all three message types failed
    } else {
        log_debug("Sent scoped discovery probe to %s", ip_addr);
    }
    
    return 0;
}
