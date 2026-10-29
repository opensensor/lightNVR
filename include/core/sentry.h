#ifndef LIGHTNVR_SENTRY_H
#define LIGHTNVR_SENTRY_H

/* Disabled unless built with ENABLE_SENTRY and configured with SENTRY_DSN. */
void lightnvr_sentry_init(void);
void lightnvr_sentry_shutdown(void);

#endif
