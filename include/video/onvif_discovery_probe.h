#ifndef ONVIF_DISCOVERY_PROBE_H
#define ONVIF_DISCOVERY_PROBE_H

#include <netinet/in.h>

/**
 * Send WS-Discovery probe message to a specific IP address
 * 
 * @param ip_addr IP address to send the probe to
 * @return 0 on success, non-zero on failure
 */
int send_discovery_probe(const char *ip_addr);

/**
 * Send discovery probes to a specific IP address using all message templates
 * 
 * @param sock Socket to use for sending
 * @param ip_addr IP address to send the probe to (for logging)
 * @param dest_addr Destination address structure
 * @return 0 on success, non-zero on failure
 */
int send_all_discovery_probes(int sock, const char *ip_addr, struct sockaddr_in *dest_addr);

#endif /* ONVIF_DISCOVERY_PROBE_H */
