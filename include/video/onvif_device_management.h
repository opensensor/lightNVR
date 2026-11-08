/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2025-2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

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
 * Get the ONVIF video source token associated with a media profile.
 *
 * Imaging service calls require a VideoSourceToken, which is often different
 * from the Media Profile token used by PTZ and stream URI calls.
 *
 * @param device_url Device service URL
 * @param username Username for authentication (can be NULL)
 * @param password Password for authentication (can be NULL)
 * @param profile_token Profile token to resolve; empty uses the first profile
 * @param video_source_token Buffer to fill with the video source token
 * @param token_size Size of the video_source_token buffer
 * @return 0 on success, non-zero on failure
 */
int get_onvif_video_source_token(const char *device_url, const char *username,
                                 const char *password, const char *profile_token,
                                 char *video_source_token, size_t token_size);

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

/** Results of test_onvif_connection_detail(). */
enum {
    ONVIF_TEST_OK = 0,
    ONVIF_TEST_DEVICE_ERROR = -1,   /* ONVIF answered but no usable profile */
    ONVIF_TEST_UNREACHABLE = -2,    /* ONVIF service could not be reached */
    ONVIF_TEST_AUTH_FAILED = -3,    /* the camera rejected the credentials */
    ONVIF_TEST_STREAM_FAILED = -4,  /* ONVIF works, the stream does not */
};

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
 * Like test_onvif_connection(), but returns an ONVIF_TEST_* code and writes a
 * user-facing explanation (credentials and query strings removed) to detail.
 * When the advertised stream is rtsps:// and cannot be opened, plain RTSP on
 * port 554 with the same path is tried before reporting a stream failure.
 */
int test_onvif_connection_detail(const char *url, const char *username, const char *password,
                                 char *detail, size_t detail_size);

/**
 * Rewrite an rtsps:// URI to rtsp:// on port 554, keeping userinfo, host and
 * path. Returns 0 on success, -1 when uri is not rtsps:// or out is too small.
 */
int onvif_plain_rtsp_fallback_url(const char *uri, char *out, size_t out_size);

#endif /* ONVIF_DEVICE_MANAGEMENT_H */
