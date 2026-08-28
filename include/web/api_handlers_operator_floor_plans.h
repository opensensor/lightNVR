#ifndef LIGHTNVR_API_HANDLERS_OPERATOR_FLOOR_PLANS_H
#define LIGHTNVR_API_HANDLERS_OPERATOR_FLOOR_PLANS_H

#include "web/request_response.h"

void handle_get_operator_floor_plans(const http_request_t *req,
                                     http_response_t *res);
void handle_post_operator_floor_plan(const http_request_t *req,
                                     http_response_t *res);
void handle_get_operator_floor_plan(const http_request_t *req,
                                    http_response_t *res);
void handle_put_operator_floor_plan(const http_request_t *req,
                                    http_response_t *res);
void handle_delete_operator_floor_plan(const http_request_t *req,
                                       http_response_t *res);

#endif /* LIGHTNVR_API_HANDLERS_OPERATOR_FLOOR_PLANS_H */
