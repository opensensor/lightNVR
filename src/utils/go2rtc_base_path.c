/* Shared upstream prefix, including builds with the go2rtc backend disabled. */
#include "video/go2rtc/go2rtc_api.h"
#include <pthread.h>
#include <string.h>
#ifdef LIGHTNVR_HAVE_LIBYAML
#include <yaml.h>
#endif

static pthread_mutex_t base_path_mutex = PTHREAD_MUTEX_INITIALIZER;
static char active_base_path[512] = GO2RTC_BASE_PATH;

const char *go2rtc_api_get_base_path(void) {
    static _Thread_local char copy[sizeof(active_base_path)];
    pthread_mutex_lock(&base_path_mutex);
    memcpy(copy, active_base_path, sizeof(copy));
    pthread_mutex_unlock(&base_path_mutex);
    return copy;
}

#ifdef LIGHTNVR_HAVE_LIBYAML
static yaml_node_t *mapping_value(yaml_document_t *doc, yaml_node_t *map,
                                  const char *key) {
    if (!map || map->type != YAML_MAPPING_NODE) return NULL;
    for (yaml_node_pair_t *pair = map->data.mapping.pairs.start;
         pair < map->data.mapping.pairs.top; ++pair) {
        yaml_node_t *name = yaml_document_get_node(doc, pair->key);
        if (name && name->type == YAML_SCALAR_NODE &&
            strcmp((const char *)name->data.scalar.value, key) == 0) {
            return yaml_document_get_node(doc, pair->value);
        }
    }
    return NULL;
}
#endif

bool go2rtc_api_apply_base_path_override(const char *yaml, size_t length) {
    char path[sizeof(active_base_path)] = GO2RTC_BASE_PATH;
#ifdef LIGHTNVR_HAVE_LIBYAML
    if (yaml && length) {
        yaml_parser_t parser;
        yaml_document_t doc;
        if (!yaml_parser_initialize(&parser)) return false;
        yaml_parser_set_input_string(&parser, (const unsigned char *)yaml, length);
        int loaded = yaml_parser_load(&parser, &doc);
        yaml_parser_delete(&parser);
        if (!loaded) return false;
        yaml_node_t *api = mapping_value(&doc, yaml_document_get_root_node(&doc), "api");
        yaml_node_t *prefix = mapping_value(&doc, api, "base_path");
        /* yaml.v3 leaves an existing string unchanged for YAML null. */
        if (prefix && prefix->type == YAML_SCALAR_NODE &&
            prefix->data.scalar.style == YAML_PLAIN_SCALAR_STYLE) {
            const char *value = (const char *)prefix->data.scalar.value;
            if (!*value || !strcmp(value, "~") || !strcmp(value, "null") ||
                !strcmp(value, "Null") || !strcmp(value, "NULL")) prefix = NULL;
        }
        bool valid = true;
        if (prefix) {
            valid = prefix->type == YAML_SCALAR_NODE &&
                    prefix->data.scalar.length < sizeof(path);
            if (valid) {
                size_t n = prefix->data.scalar.length;
                memcpy(path, prefix->data.scalar.value, n);
                path[n] = '\0';
                /* go2rtc concatenates base_path + "/api/..." verbatim.
                 * Require an absolute, canonical URL path or an empty prefix. */
                valid = !n || (path[0] == '/' && path[n - 1] != '/');
                for (size_t i = 0; valid && i < n; ++i) {
                    unsigned char c = (unsigned char)path[i];
                    valid = (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') ||
                            (c >= '0' && c <= '9') || strchr("/-._~", c) != NULL;
                    if (!c) valid = false;
                }
                if (strstr(path, "//") || strstr(path, "/../") ||
                    strstr(path, "/./") || (n >= 3 && !strcmp(path + n - 3, "/..")) ||
                    (n >= 2 && !strcmp(path + n - 2, "/."))) valid = false;
            }
        }
        yaml_document_delete(&doc);
        if (!valid) return false;
    }
#else
    (void)yaml;
    (void)length;
#endif
    pthread_mutex_lock(&base_path_mutex);
    memcpy(active_base_path, path, sizeof(path));
    pthread_mutex_unlock(&base_path_mutex);
    return true;
}
