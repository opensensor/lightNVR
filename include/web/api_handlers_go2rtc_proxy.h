/**
 * @file api_handlers_go2rtc_proxy.h
 * @brief API handlers for proxying requests to go2rtc
 */

#ifndef API_HANDLERS_GO2RTC_PROXY_H
#define API_HANDLERS_GO2RTC_PROXY_H

#include "mongoose.h"

/**
 * @brief Direct handler for POST /api/webrtc
 * 
 * This handler proxies WebRTC offer requests to the go2rtc API.
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_offer(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for POST /api/webrtc/ice
 * 
 * This handler proxies WebRTC ICE candidate requests to the go2rtc API.
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_ice(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for OPTIONS /api/webrtc
 * 
 * This handler responds to CORS preflight requests for the WebRTC API.
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_options(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Direct handler for OPTIONS /api/webrtc/ice
 * 
 * This handler responds to CORS preflight requests for the WebRTC ICE API.
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_go2rtc_webrtc_ice_options(struct mg_connection *c, struct mg_http_message *hm);

#endif /* API_HANDLERS_GO2RTC_PROXY_H */
