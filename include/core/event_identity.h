#ifndef LIGHTNVR_CORE_EVENT_IDENTITY_H
#define LIGHTNVR_CORE_EVENT_IDENTITY_H

#include <stddef.h>

#include "core/event_envelope.h"

/* Load or create the installation identity persisted in system_settings. */
int event_identity_init(void);

/* Copy the stable urn:lightnvr:<uuid> source into caller-owned storage. */
int event_identity_get_source(char *output, size_t output_size);

/* Clear process-local identity state. The persisted UUID is not removed. */
void event_identity_shutdown(void);

#endif /* LIGHTNVR_CORE_EVENT_IDENTITY_H */
