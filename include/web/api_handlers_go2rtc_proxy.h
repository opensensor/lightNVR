/**
 * @file api_handlers_go2rtc_proxy.h
 * @brief API handlers for proxying requests to go2rtc
 */

#ifndef API_HANDLERS_GO2RTC_PROXY_H
#define API_HANDLERS_GO2RTC_PROXY_H

#include "mongoose.h"

/**
 * @brief Handler for POST /api/webrtc
 *
 * This handler proxies WebRTC offer requests to the go2rtc API.
 * It uses the multithreading pattern to handle the request in a worker thread.
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Worker function for POST /api/webrtc
 *
 * This function is called by the multithreading system to handle WebRTC offer requests.
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_offer_worker(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for POST /api/webrtc/ice
 *
 * This handler proxies WebRTC ICE candidate requests to the go2rtc API.
 * It uses the multithreading pattern to handle the request in a worker thread.
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Worker function for POST /api/webrtc/ice
 *
 * This function is called by the multithreading system to handle WebRTC ICE candidate requests.
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_ice_worker(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for OPTIONS /api/webrtc
 *
 * This handler responds to CORS preflight requests for the WebRTC API.
 * This is a simple handler that doesn't use the multithreading pattern.
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_options(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for OPTIONS /api/webrtc/ice
 *
 * This handler responds to CORS preflight requests for the WebRTC ICE API.
 * This is a simple handler that doesn't use the multithreading pattern.
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_ice_options(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handler for GET /api/webrtc/config
 *
 * Proxies a request to go2rtc to fetch WebRTC configuration (including ICE servers).
 *
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_config(struct mg_connection *c, struct mg_http_message *hm);


#endif /* API_HANDLERS_GO2RTC_PROXY_H */
