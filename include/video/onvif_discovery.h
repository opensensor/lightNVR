#ifndef ONVIF_DISCOVERY_H
#define ONVIF_DISCOVERY_H

#include <stdbool.h>
#include <stdint.h>
#include <time.h>
#include <pthread.h>
#include "core/config.h"

// ONVIF device information structure
typedef struct {
    char endpoint[MAX_URL_LENGTH];
    char device_service[MAX_URL_LENGTH];
    char media_service[MAX_URL_LENGTH];
    char ptz_service[MAX_URL_LENGTH];
    char imaging_service[MAX_URL_LENGTH];
    char manufacturer[64];
    char model[64];
    char firmware_version[32];
    char serial_number[64];
    char hardware_id[64];
    char ip_address[64];
    char mac_address[32];
    time_t discovery_time;
    bool online;
} onvif_device_info_t;

// ONVIF profile information structure
typedef struct {
    char token[64];
    char name[64];
    char snapshot_uri[MAX_URL_LENGTH];
    char stream_uri[MAX_URL_LENGTH];
    int width;
    int height;
    char encoding[16];
    int fps;
    int bitrate;
} onvif_profile_t;

/**
 * Initialize ONVIF discovery module
 * 
 * @return 0 on success, non-zero on failure
 */
int init_onvif_discovery(void);

/**
 * Shutdown ONVIF discovery module
 */
void shutdown_onvif_discovery(void);

/**
 * Start ONVIF discovery process
 * This will start a background thread that periodically scans for ONVIF devices
 * 
 * @param network Network to scan (e.g., "192.168.1.0/24")
 * @param interval Interval in seconds between discovery attempts
 * @return 0 on success, non-zero on failure
 */
int start_onvif_discovery(const char *network, int interval);

/**
 * Get discovery mutex
 * 
 * @return Pointer to discovery mutex
 */
pthread_mutex_t* get_discovery_mutex(void);

/**
 * Get current discovery network
 * 
 * @return Current discovery network, or NULL if not running
 */
const char* get_current_discovery_network(void);

/**
 * Stop ONVIF discovery process
 * 
 * @return 0 on success, non-zero on failure
 */
int stop_onvif_discovery(void);

/**
 * Get discovered ONVIF devices
 * 
 * @param devices Array to fill with device information
 * @param max_devices Maximum number of devices to return
 * @return Number of devices found, or -1 on error
 */
int get_discovered_onvif_devices(onvif_device_info_t *devices, int max_devices);

/**
 * Get ONVIF device profiles
 * 
 * @param device_url Device service URL
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param profiles Array to fill with profile information
 * @param max_profiles Maximum number of profiles to return
 * @return Number of profiles found, or -1 on error
 */
int get_onvif_device_profiles(const char *device_url, const char *username, 
                             const char *password, onvif_profile_t *profiles, 
                             int max_profiles);

/**
 * Get ONVIF stream URL for a specific profile
 * 
 * @param device_url Device service URL
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param profile_token Profile token
 * @param stream_url Buffer to fill with stream URL
 * @param url_size Size of the stream_url buffer
 * @return 0 on success, non-zero on failure
 */
int get_onvif_stream_url(const char *device_url, const char *username, 
                        const char *password, const char *profile_token, 
                        char *stream_url, size_t url_size);

/**
 * Add discovered ONVIF device as a stream
 * 
 * @param device_info Device information
 * @param profile Profile information
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param stream_name Name for the new stream
 * @return 0 on success, non-zero on failure
 */
int add_onvif_device_as_stream(const onvif_device_info_t *device_info, 
                              const onvif_profile_t *profile, 
                              const char *username, const char *password, 
                              const char *stream_name);

/**
 * Manually discover ONVIF devices on a specific network
 * 
 * @param network Network to scan (e.g., "192.168.1.0/24")
 * @param devices Array to fill with device information
 * @param max_devices Maximum number of devices to return
 * @return Number of devices found, or -1 on error
 */
int discover_onvif_devices(const char *network, onvif_device_info_t *devices,
                          int max_devices);

/**
 * Test connection to an ONVIF device
 * 
 * @param url Device service URL
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @return 0 on success, non-zero on failure
 */
int test_onvif_connection(const char *url, const char *username, const char *password);

/**
 * Try direct HTTP probing for ONVIF devices
 * This is used as a fallback when WS-Discovery fails
 * 
 * @param candidate_ips Array of IP addresses to probe
 * @param candidate_count Number of IP addresses in the array
 * @param devices Array to fill with device information
 * @param max_devices Maximum number of devices to return
 * @return Number of devices found
 */
int try_direct_http_discovery(char candidate_ips[][16], int candidate_count, 
                             onvif_device_info_t *devices, int max_devices);

#endif /* ONVIF_DISCOVERY_H */
