/**
 * @file db_query_builder.c
 * @brief Dynamic SQL query builder implementation
 */

#define _POSIX_C_SOURCE 200809L
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "database/db_query_builder.h"
#include "database/db_schema_cache.h"
#include "core/logger.h"

int qb_init(query_builder_t *qb, const char *table_name) {
    if (!qb || !table_name) {
        return -1;
    }
    
    memset(qb, 0, sizeof(query_builder_t));
    qb->table_name = table_name;
    qb->column_count = 0;
    qb->result_column_count = 0;
    
    return 0;
}

int qb_add_column(query_builder_t *qb, const char *column_name, bool is_required) {
    if (!qb || !column_name) {
        return -1;
    }
    
    if (qb->column_count >= MAX_TRACKED_COLUMNS) {
        log_error("Too many columns tracked (max %d)", MAX_TRACKED_COLUMNS);
        return -1;
    }
    
    column_info_t *col = &qb->columns[qb->column_count];
    strncpy(col->name, column_name, sizeof(col->name) - 1);
    col->name[sizeof(col->name) - 1] = '\0';
    
    // Check if column exists using the cached schema lookup
    col->present = cached_column_exists(qb->table_name, column_name);
    col->index = -1;  // Will be set during build
    
    if (is_required && !col->present) {
        log_error("Required column '%s.%s' does not exist", qb->table_name, column_name);
        return -1;
    }
    
    qb->column_count++;
    return 0;
}

const char *qb_build_select(query_builder_t *qb, const char *where_clause, const char *order_by) {
    if (!qb) {
        return NULL;
    }
    
    // Start building the query
    char *ptr = qb->query;
    size_t remaining = sizeof(qb->query);
    int written;
    
    written = snprintf(ptr, remaining, "SELECT ");
    ptr += written;
    remaining -= written;
    
    // Add columns that exist
    int result_idx = 0;
    bool first = true;
    
    for (int i = 0; i < qb->column_count; i++) {
        column_info_t *col = &qb->columns[i];
        
        if (!col->present) {
            continue;
        }
        
        if (!first) {
            written = snprintf(ptr, remaining, ", ");
            ptr += written;
            remaining -= written;
        }
        
        written = snprintf(ptr, remaining, "%s", col->name);
        ptr += written;
        remaining -= written;
        
        col->index = result_idx++;
        first = true;
        first = false;
    }
    
    qb->result_column_count = result_idx;
    
    if (result_idx == 0) {
        log_error("No columns available for query on table %s", qb->table_name);
        return NULL;
    }
    
    // Add FROM clause
    written = snprintf(ptr, remaining, " FROM %s", qb->table_name);
    ptr += written;
    remaining -= written;
    
    // Add WHERE clause if provided
    if (where_clause && strlen(where_clause) > 0) {
        written = snprintf(ptr, remaining, " WHERE %s", where_clause);
        ptr += written;
        remaining -= written;
    }
    
    // Add ORDER BY clause if provided
    if (order_by && strlen(order_by) > 0) {
        written = snprintf(ptr, remaining, " ORDER BY %s", order_by);
        ptr += written;
        remaining -= written;
    }
    
    // Add semicolon
    written = snprintf(ptr, remaining, ";");
    ptr += written;
    remaining -= written;
    
    return qb->query;
}

bool qb_has_column(const query_builder_t *qb, const char *column_name) {
    if (!qb || !column_name) {
        return false;
    }
    
    for (int i = 0; i < qb->column_count; i++) {
        if (strcmp(qb->columns[i].name, column_name) == 0) {
            return qb->columns[i].present && qb->columns[i].index >= 0;
        }
    }
    
    return false;
}

int qb_get_column_index(const query_builder_t *qb, const char *column_name) {
    if (!qb || !column_name) {
        return -1;
    }

    for (int i = 0; i < qb->column_count; i++) {
        if (strcmp(qb->columns[i].name, column_name) == 0) {
            return qb->columns[i].index;
        }
    }

    return -1;
}

int qb_get_int(sqlite3_stmt *stmt, const query_builder_t *qb,
               const char *column_name, int default_value) {
    int idx = qb_get_column_index(qb, column_name);
    if (idx < 0) {
        return default_value;
    }

    if (sqlite3_column_type(stmt, idx) == SQLITE_NULL) {
        return default_value;
    }

    return sqlite3_column_int(stmt, idx);
}

const char *qb_get_text(sqlite3_stmt *stmt, const query_builder_t *qb,
                        const char *column_name, char *buffer, size_t buffer_len,
                        const char *default_value) {
    int idx = qb_get_column_index(qb, column_name);
    if (idx < 0) {
        if (default_value && buffer && buffer_len > 0) {
            strncpy(buffer, default_value, buffer_len - 1);
            buffer[buffer_len - 1] = '\0';
        }
        return default_value;
    }

    const char *text = (const char *)sqlite3_column_text(stmt, idx);
    if (!text) {
        if (default_value && buffer && buffer_len > 0) {
            strncpy(buffer, default_value, buffer_len - 1);
            buffer[buffer_len - 1] = '\0';
        }
        return default_value;
    }

    if (buffer && buffer_len > 0) {
        strncpy(buffer, text, buffer_len - 1);
        buffer[buffer_len - 1] = '\0';
    }

    return text;
}

double qb_get_double(sqlite3_stmt *stmt, const query_builder_t *qb,
                     const char *column_name, double default_value) {
    int idx = qb_get_column_index(qb, column_name);
    if (idx < 0) {
        return default_value;
    }

    if (sqlite3_column_type(stmt, idx) == SQLITE_NULL) {
        return default_value;
    }

    return sqlite3_column_double(stmt, idx);
}

bool qb_get_bool(sqlite3_stmt *stmt, const query_builder_t *qb,
                 const char *column_name, bool default_value) {
    int idx = qb_get_column_index(qb, column_name);
    if (idx < 0) {
        return default_value;
    }

    if (sqlite3_column_type(stmt, idx) == SQLITE_NULL) {
        return default_value;
    }

    return sqlite3_column_int(stmt, idx) != 0;
}

