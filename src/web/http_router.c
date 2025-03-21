#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <ctype.h>
#include <regex.h>

#include "web/http_router.h"
#include "core/logger.h"
#include "utils/memory.h"

// Maximum number of route parameters
#define MAX_ROUTE_PARAMS 16

// Route structure
typedef struct {
    char method[16];
    char pattern[256];
    regex_t regex;
    char param_names[MAX_ROUTE_PARAMS][64];
    int param_count;
    route_handler_t handler;
    void *user_data;
} route_t;

// Router structure
struct http_router_t {
    route_t *routes;
    int route_count;
    int route_capacity;
};

/**
 * @brief Create a new HTTP router
 */
http_router_handle_t http_router_create(void) {
    http_router_handle_t router = calloc(1, sizeof(struct http_router_t));
    if (!router) {
        log_error("Failed to allocate memory for router");
        return NULL;
    }

    // Allocate initial routes array
    router->route_capacity = 16;
    router->routes = calloc(router->route_capacity, sizeof(route_t));
    if (!router->routes) {
        log_error("Failed to allocate memory for routes");
        free(router);
        return NULL;
    }

    router->route_count = 0;
    return router;
}

/**
 * @brief Destroy HTTP router
 */
void http_router_destroy(http_router_handle_t router) {
    if (!router) {
        return;
    }

    // Free regex patterns
    for (int i = 0; i < router->route_count; i++) {
        regfree(&router->routes[i].regex);
    }

    // Free routes array
    free(router->routes);

    // Free router
    free(router);
}

/**
 * @brief Convert URL pattern to regex pattern
 * 
 * Converts patterns like "/api/streams/:id" to regex patterns like "^/api/streams/([^/]+)$"
 */
static int pattern_to_regex(const char *pattern, char *regex_pattern, size_t regex_size, 
                           char param_names[MAX_ROUTE_PARAMS][64], int *param_count) {
    if (!pattern || !regex_pattern || !param_names || !param_count) {
        return -1;
    }

    *param_count = 0;
    size_t pattern_len = strlen(pattern);
    size_t regex_len = 0;

    // Start with ^ to match from beginning
    if (regex_len < regex_size - 1) {
        regex_pattern[regex_len++] = '^';
    }

    // Process pattern character by character
    for (size_t i = 0; i < pattern_len; i++) {
        if (pattern[i] == ':') {
            // Parameter found
            if (*param_count >= MAX_ROUTE_PARAMS) {
                log_error("Too many parameters in route pattern: %s", pattern);
                return -1;
            }

            // Extract parameter name
            size_t param_start = i + 1;
            size_t param_len = 0;
            while (param_start + param_len < pattern_len && 
                  (isalnum(pattern[param_start + param_len]) || pattern[param_start + param_len] == '_')) {
                param_len++;
            }

            if (param_len == 0) {
                log_error("Invalid parameter name in route pattern: %s", pattern);
                return -1;
            }

            // Copy parameter name
            if (param_len >= sizeof(param_names[0])) {
                param_len = sizeof(param_names[0]) - 1;
            }
            strncpy(param_names[*param_count], pattern + param_start, param_len);
            param_names[*param_count][param_len] = '\0';
            (*param_count)++;

            // Add regex capture group for parameter
            const char *capture = "([^/]+)";
            size_t capture_len = strlen(capture);
            if (regex_len + capture_len < regex_size - 1) {
                strcpy(regex_pattern + regex_len, capture);
                regex_len += capture_len;
            }

            // Skip parameter name in pattern
            i = param_start + param_len - 1;
        } else if (pattern[i] == '*') {
            // Wildcard
            const char *wildcard = "(.*)";
            size_t wildcard_len = strlen(wildcard);
            if (regex_len + wildcard_len < regex_size - 1) {
                strcpy(regex_pattern + regex_len, wildcard);
                regex_len += wildcard_len;
            }
        } else {
            // Regular character
            if (regex_len < regex_size - 1) {
                regex_pattern[regex_len++] = pattern[i];
            }
        }
    }

    // End with $ to match until end
    if (regex_len < regex_size - 1) {
        regex_pattern[regex_len++] = '$';
    }

    // Null-terminate
    regex_pattern[regex_len] = '\0';
    return 0;
}

/**
 * @brief Add a route to the router
 */
int http_router_add_route(http_router_handle_t router, const char *method, const char *pattern, 
                         route_handler_t handler, void *user_data) {
    if (!router || !pattern || !handler) {
        log_error("Invalid parameters for add_route");
        return -1;
    }

    // Check if we need to resize the routes array
    if (router->route_count >= router->route_capacity) {
        int new_capacity = router->route_capacity * 2;
        route_t *new_routes = realloc(router->routes, new_capacity * sizeof(route_t));
        if (!new_routes) {
            log_error("Failed to resize routes array");
            return -1;
        }

        router->routes = new_routes;
        router->route_capacity = new_capacity;
    }

    // Initialize new route
    route_t *route = &router->routes[router->route_count];
    memset(route, 0, sizeof(route_t));

    // Copy method
    if (method) {
        strncpy(route->method, method, sizeof(route->method) - 1);
        route->method[sizeof(route->method) - 1] = '\0';
    }

    // Copy pattern
    strncpy(route->pattern, pattern, sizeof(route->pattern) - 1);
    route->pattern[sizeof(route->pattern) - 1] = '\0';

    // Convert pattern to regex
    char regex_pattern[512];
    if (pattern_to_regex(pattern, regex_pattern, sizeof(regex_pattern), 
                        route->param_names, &route->param_count) != 0) {
        log_error("Failed to convert pattern to regex: %s", pattern);
        return -1;
    }

    // Compile regex
    int regex_flags = REG_EXTENDED;
    int regex_result = regcomp(&route->regex, regex_pattern, regex_flags);
    if (regex_result != 0) {
        char error_buffer[256];
        regerror(regex_result, &route->regex, error_buffer, sizeof(error_buffer));
        log_error("Failed to compile regex for pattern %s: %s", pattern, error_buffer);
        return -1;
    }

    // Set handler and user data
    route->handler = handler;
    route->user_data = user_data;

    // Increment route count
    router->route_count++;

    log_debug("Added route: method=%s, pattern=%s, regex=%s, params=%d", 
             method ? method : "ANY", pattern, regex_pattern, route->param_count);
    return 0;
}

/**
 * @brief Match a request to a route
 */
route_handler_t http_router_match(http_router_handle_t router, const http_request_t *request, 
                                 route_match_t *match) {
    if (!router || !request || !match) {
        return NULL;
    }

    // Initialize match
    memset(match, 0, sizeof(route_match_t));

    // Get request method as string
    const char *method_str = NULL;
    switch (request->method) {
        case HTTP_GET:
            method_str = "GET";
            break;
        case HTTP_POST:
            method_str = "POST";
            break;
        case HTTP_PUT:
            method_str = "PUT";
            break;
        case HTTP_DELETE:
            method_str = "DELETE";
            break;
        case HTTP_OPTIONS:
            method_str = "OPTIONS";
            break;
        case HTTP_HEAD:
            method_str = "HEAD";
            break;
        default:
            log_error("Unknown HTTP method: %d", request->method);
            return NULL;
    }

    // Try to match each route
    for (int i = 0; i < router->route_count; i++) {
        route_t *route = &router->routes[i];

        // Check method
        if (route->method[0] != '\0' && strcmp(route->method, method_str) != 0) {
            continue;
        }

        // Try to match regex
        regmatch_t regex_matches[MAX_ROUTE_PARAMS + 1];
        int regex_result = regexec(&route->regex, request->path, MAX_ROUTE_PARAMS + 1, regex_matches, 0);
        if (regex_result == 0) {
            // Route matched
            log_debug("Route matched: method=%s, pattern=%s, path=%s", 
                     route->method[0] ? route->method : "ANY", route->pattern, request->path);

            // Allocate parameters array
            if (route->param_count > 0) {
                match->params = calloc(route->param_count, sizeof(route_param_t));
                if (!match->params) {
                    log_error("Failed to allocate memory for route parameters");
                    return NULL;
                }
                match->param_count = route->param_count;

                // Extract parameter values
                for (int j = 0; j < route->param_count; j++) {
                    // Copy parameter name
                    strncpy(match->params[j].name, route->param_names[j], sizeof(match->params[j].name) - 1);
                    match->params[j].name[sizeof(match->params[j].name) - 1] = '\0';

                    // Extract parameter value from regex match
                    regmatch_t *rm = &regex_matches[j + 1];
                    if (rm->rm_so != -1 && rm->rm_eo != -1) {
                        size_t value_len = rm->rm_eo - rm->rm_so;
                        if (value_len >= sizeof(match->params[j].value)) {
                            value_len = sizeof(match->params[j].value) - 1;
                        }
                        strncpy(match->params[j].value, request->path + rm->rm_so, value_len);
                        match->params[j].value[value_len] = '\0';
                    }

                    log_debug("Route parameter: %s = %s", match->params[j].name, match->params[j].value);
                }
            }

            // Set user data
            match->user_data = route->user_data;

            // Return handler
            return route->handler;
        }
    }

    // No route matched
    log_debug("No route matched for path: %s", request->path);
    return NULL;
}

/**
 * @brief Dispatch a request to the appropriate handler
 */
int http_router_dispatch(http_router_handle_t router, const http_request_t *request, 
                        http_response_t *response) {
    if (!router || !request || !response) {
        return -1;
    }

    // Match request to route
    route_match_t match;
    route_handler_t handler = http_router_match(router, request, &match);
    if (!handler) {
        return -1;
    }

    // Call handler
    handler(request, response, &match);

    // Free match resources
    http_router_free_match(&match);

    return 0;
}

/**
 * @brief Free route match resources
 */
void http_router_free_match(route_match_t *match) {
    if (!match) {
        return;
    }

    // Free parameters array
    if (match->params) {
        free(match->params);
        match->params = NULL;
    }

    match->param_count = 0;
    match->user_data = NULL;
}

/**
 * @brief Get a parameter value from a route match
 */
const char* http_router_get_param(const route_match_t *match, const char *name) {
    if (!match || !name || !match->params) {
        return NULL;
    }

    // Search for parameter by name
    for (int i = 0; i < match->param_count; i++) {
        if (strcmp(match->params[i].name, name) == 0) {
            return match->params[i].value;
        }
    }

    return NULL;
}
