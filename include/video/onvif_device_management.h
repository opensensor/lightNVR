#ifndef ONVIF_DEVICE_MANAGEMENT_H
#define ONVIF_DEVICE_MANAGEMENT_H

#include "video/onvif_discovery.h"
#include <stddef.h>

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
 * Test connection to an ONVIF device
 * 
 * @param url Device service URL
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @return 0 on success, non-zero on failure
 */
int test_onvif_connection(const char *url, const char *username, const char *password);

#endif /* ONVIF_DEVICE_MANAGEMENT_H */
