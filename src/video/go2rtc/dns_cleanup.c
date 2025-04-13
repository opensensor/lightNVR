#include "video/go2rtc/dns_cleanup.h"
#include "core/logger.h"

#include <netdb.h>
#include <resolv.h>
#include <stdlib.h>

/**
 * @brief Clean up DNS resolver resources to prevent memory leaks
 * 
 * This function forces the DNS resolver to release any cached memory
 * that might be leaked during DNS resolution operations.
 */
void cleanup_dns_resolver(void) {
    log_debug("Cleaning up DNS resolver resources");
    
    // Force the resolver to reinitialize, which should clean up any leaked memory
    res_init();
    
    // Additional cleanup for glibc's resolver
#ifdef _GNU_SOURCE
    // Reset the resolver state
    __res_init();
#endif

    log_debug("DNS resolver cleanup completed");
}
