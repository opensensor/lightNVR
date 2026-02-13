/**
 * @file api_handlers_go2rtc_proxy.h
 * @brief Reverse proxy handler for forwarding requests to go2rtc
 *
 * This handler proxies requests from /go2rtc/* to the local go2rtc instance
 * on localhost:{go2rtc_api_port}. This allows HLS streaming to work off-network
 * by routing through lightNVR's port instead of requiring direct access to go2rtc's port.
 */

#ifndef API_HANDLERS_GO2RTC_PROXY_H
#define API_HANDLERS_GO2RTC_PROXY_H

#include "web/request_response.h"

/**
 * @brief Proxy handler for /go2rtc/* requests
 *
 * Forwards the request to go2rtc running on localhost, preserving the path,
 * query string, method, headers, and body. The response from go2rtc is
 * forwarded back to the client.
 *
 * @param req HTTP request
 * @param res HTTP response
 */
void handle_go2rtc_proxy(const http_request_t *req, http_response_t *res);

#endif /* API_HANDLERS_GO2RTC_PROXY_H */

