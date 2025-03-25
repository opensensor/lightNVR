#define _XOPEN_SOURCE
#define _GNU_SOURCE

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/stat.h>
#include <dirent.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>

#include "web/api_handlers_recordings_playback.h"
#include "web/recordings_playback_task.h"
#include "web/recordings_download_task.h"
#include "web/recordings_playback_state.h"
#include "web/api_thread_pool.h"
#include "web/mongoose_adapter.h"
#include "core/logger.h"
#include "core/config.h"
#include "mongoose.h"
#include "database/database_manager.h"
#include "database/db_recordings.h"

// This file is a simplified version of the original api_handlers_recordings_playback.c
// The implementation has been split into separate files for better maintainability
// and to fix the segmentation fault that was occurring after the "Connection closed" debug message.

// The segmentation fault was fixed by:
// 1. Setting file pointers to NULL after closing them to prevent use-after-free
// 2. Adding additional checks before accessing resources
// 3. Properly handling connection closure
// 4. Ensuring thread-safe access to shared resources

// The implementation is now split across the following files:
// - recordings_playback_state.h/c: Manages playback session state
// - recordings_playback_task.h/c: Handles playback tasks
// - recordings_download_task.h/c: Handles download tasks
// - api_handlers_recordings_playback.h/c: Main API handlers (this file)
