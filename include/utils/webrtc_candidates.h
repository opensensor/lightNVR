/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 * Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
 *
 * This file is part of LightNVR <https://github.com/opensensor/lightNVR>.
 */

#ifndef LIGHTNVR_UTILS_WEBRTC_CANDIDATES_H
#define LIGHTNVR_UTILS_WEBRTC_CANDIDATES_H

#define GO2RTC_MAX_EXTERNAL_CANDIDATES 8
#define GO2RTC_CANDIDATE_SIZE 128

/**
 * Parse the go2rtc "External IP" setting into WebRTC candidates.
 *
 * Entries are separated by commas, semicolons or whitespace. Each entry is:
 *   "*" or "auto"            -> "*:<port>" (auto-detect)
 *   IPv4 or hostname[:port]  -> "<host>:<port>"
 *   IPv6 or [IPv6][:port]    -> "[<ipv6>]:<port>"
 *   stun:host[:port]         -> passed through
 * Duplicates collapse. Output strings are safe inside a double-quoted YAML scalar.
 *
 * @return number of candidates written (0 when the list is empty or NULL),
 *         or -1 when any entry is invalid or there are more than max entries.
 */
int go2rtc_parse_external_candidates(const char *list, int port,
                                     char out[][GO2RTC_CANDIDATE_SIZE], int max);

#endif /* LIGHTNVR_UTILS_WEBRTC_CANDIDATES_H */
