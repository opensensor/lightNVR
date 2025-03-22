#ifndef ONVIF_DISCOVERY_RESPONSE_H
#define ONVIF_DISCOVERY_RESPONSE_H

#include "video/onvif_discovery.h"

/**
 * Parse ONVIF device information from discovery response
 * 
 * @param response Response string
 * @param device_info Pointer to store device information
 * @return 0 on success, non-zero on failure
 */
int parse_device_info(const char *response, onvif_device_info_t *device_info);

/**
 * Receive and process discovery responses
 * 
 * @param devices Array to store device information
 * @param max_devices Maximum number of devices to store
 * @return Number of devices found, or -1 on error
 */
int receive_discovery_responses(onvif_device_info_t *devices, int max_devices);

/**
 * Extended version of receive_discovery_responses with configurable timeouts
 * 
 * @param devices Array to store device information
 * @param max_devices Maximum number of devices to store
 * @param timeout_sec Timeout in seconds for each attempt
 * @param max_attempts Maximum number of attempts
 * @return Number of devices found, or -1 on error
 */
int receive_extended_discovery_responses(onvif_device_info_t *devices, int max_devices,
                                        int timeout_sec, int max_attempts);

#endif /* ONVIF_DISCOVERY_RESPONSE_H */
