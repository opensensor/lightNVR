/**
 * @file http_router.h
 * @brief HTTP routing system for mapping URLs to handlers
 */

#ifndef HTTP_ROUTER_H
#define HTTP_ROUTER_H

#include <stddef.h>
#include "request_response.h"

/**
 * @brief Route parameter structure
 */
typedef struct {
    char name[64];
    char value[256];
} route_param_t;

/**
 * @brief Route match structure
 */
typedef struct {
    route_param_t *params;
    int param_count;
    void *user_data;
} route_match_t;

/**
 * @brief Route handler function type
 */
typedef void (*route_handler_t)(const http_request_t *request, 
                               http_response_t *response, 
                               const route_match_t *match);

/**
 * @brief HTTP router handle
 */
typedef struct http_router_t* http_router_handle_t;

/**
 * @brief Create a new HTTP router
 * 
 * @return http_router_handle_t Router handle or NULL on error
 */
http_router_handle_t http_router_create(void);

/**
 * @brief Destroy HTTP router
 * 
 * @param router Router handle
 */
void http_router_destroy(http_router_handle_t router);

/**
 * @brief Add a route to the router
 * 
 * @param router Router handle
 * @param method HTTP method (GET, POST, etc.) or NULL for any method
 * @param pattern URL pattern (e.g., "/api/streams/:id")
 * @param handler Route handler function
 * @param user_data User data to pass to the handler
 * @return int 0 on success, non-zero on error
 */
int http_router_add_route(http_router_handle_t router, 
                         const char *method, 
                         const char *pattern, 
                         route_handler_t handler, 
                         void *user_data);

/**
 * @brief Match a request to a route
 * 
 * @param router Router handle
 * @param request HTTP request
 * @param match Route match structure to fill
 * @return route_handler_t Handler function or NULL if no match
 */
route_handler_t http_router_match(http_router_handle_t router, 
                                 const http_request_t *request, 
                                 route_match_t *match);

/**
 * @brief Dispatch a request to the appropriate handler
 * 
 * @param router Router handle
 * @param request HTTP request
 * @param response HTTP response
 * @return int 0 if a route was matched and handled, non-zero otherwise
 */
int http_router_dispatch(http_router_handle_t router, 
                        const http_request_t *request, 
                        http_response_t *response);

/**
 * @brief Free route match resources
 * 
 * @param match Route match structure
 */
void http_router_free_match(route_match_t *match);

/**
 * @brief Get a parameter value from a route match
 * 
 * @param match Route match structure
 * @param name Parameter name
 * @return const char* Parameter value or NULL if not found
 */
const char* http_router_get_param(const route_match_t *match, const char *name);

#endif /* HTTP_ROUTER_H */
