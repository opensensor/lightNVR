#ifndef ONVIF_DISCOVERY_MESSAGES_H
#define ONVIF_DISCOVERY_MESSAGES_H

#include <stddef.h>

// ONVIF WS-Discovery message templates
extern const char *ONVIF_DISCOVERY_MSG;
extern const char *ONVIF_DISCOVERY_MSG_ALT;
extern const char *ONVIF_DISCOVERY_MSG_WITH_SCOPE;

/**
 * Generate a random UUID for WS-Discovery message ID
 * 
 * @param uuid Buffer to store the generated UUID
 * @param size Size of the buffer
 */
void generate_uuid(char *uuid, size_t size);

#endif /* ONVIF_DISCOVERY_MESSAGES_H */
