#ifndef ONVIF_DISCOVERY_NETWORK_H
#define ONVIF_DISCOVERY_NETWORK_H

#include <stdint.h>

/**
 * Parse network string (e.g., "192.168.1.0/24") into base address and subnet mask
 * 
 * @param network Network string in CIDR notation
 * @param base_addr Pointer to store the base address
 * @param subnet_mask Pointer to store the subnet mask
 * @return 0 on success, non-zero on failure
 */
int parse_network(const char *network, uint32_t *base_addr, uint32_t *subnet_mask);

/**
 * Detect local networks
 * 
 * @param networks Array to store detected networks
 * @param max_networks Maximum number of networks to detect
 * @return Number of networks detected, or -1 on error
 */
int detect_local_networks(char networks[][64], int max_networks);

#endif /* ONVIF_DISCOVERY_NETWORK_H */
