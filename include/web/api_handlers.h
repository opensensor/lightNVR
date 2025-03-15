#ifndef API_HANDLERS_H
#define API_HANDLERS_H

#include "web/web_server.h"

// Include all API handler modules
#include "web/api_handlers_common.h"
#include "web/api_handlers_settings.h"
#include "web/api_handlers_streams.h"
#include "web/api_handlers_system.h"
#include "web/api_handlers_recordings.h"
#include "web/api_handlers_streaming.h"
#include "web/api_handlers_detection.h"
#include "web/api_handlers_detection_results.h"

/**
 * Register all API handlers
 */
void register_api_handlers(void);

#endif /* API_HANDLERS_H */
