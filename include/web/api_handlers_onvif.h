#ifndef API_HANDLERS_ONVIF_H
#define API_HANDLERS_ONVIF_H

#include "../../external/mongoose/mongoose.h"

/**
 * @brief Handle GET request for ONVIF discovery status
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_onvif_discovery_status(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle GET request for discovered ONVIF devices
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_discovered_onvif_devices(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request to manually discover ONVIF devices
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_discover_onvif_devices(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle GET request for ONVIF device profiles
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_get_onvif_device_profiles(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request to add ONVIF device as stream
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_add_onvif_device_as_stream(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Handle POST request to test ONVIF connection
 * 
 * @param c Mongoose connection
 * @param hm Mongoose HTTP message
 */
void mg_handle_post_test_onvif_connection(struct mg_connection *c, struct mg_http_message *hm);

/**
 * @brief Register all ONVIF API handlers
 */
void register_onvif_api_handlers(void);

#endif /* API_HANDLERS_ONVIF_H */
