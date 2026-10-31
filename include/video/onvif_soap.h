/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef ONVIF_SOAP_H
#define ONVIF_SOAP_H

#include <stddef.h>

/**
 * Create a WS-Security SOAP header element for ONVIF digest authentication.
 *
 * Generates a random 16-byte nonce, computes the PasswordDigest as
 * Base64(SHA-1(nonce_raw || created || password)) per the WS-UsernameToken
 * Profile 1.0 specification, and returns the complete
 * <wsse:Security> ... </wsse:Security> XML fragment ready to embed
 * inside a SOAP <s:Header> element.
 *
 * @param username  Camera username.  Must be non-NULL and non-empty.
 * @param password  Camera password.  Must be non-NULL and non-empty.
 * @return          Heap-allocated XML string for the WS-Security element,
 *                  or NULL on allocation / crypto failure.
 *                  The caller is responsible for free()ing the returned string.
 */
char *onvif_create_security_header(const char *username, const char *password);

/**
 * Parse a SOAP Fault from an ONVIF error response and log the details.
 *
 * Attempts to extract the fault Code, Subcode, and Reason text from a
 * SOAP 1.2 Fault element in the response body.  Logs whatever information
 * can be parsed at the error level.  If the XML cannot be parsed at all,
 * logs the raw response (truncated) as a fallback.
 *
 * @param response      The raw XML response body.
 * @param response_len  Length of the response in bytes.
 * @param context       A short description of the request that failed
 *                      (e.g. "PullMessages", "CreatePullPointSubscription"),
 *                      used to prefix the log message.  May be NULL.
 */
void onvif_log_soap_fault(const char *response, size_t response_len, const char *context);

#endif /* ONVIF_SOAP_H */

