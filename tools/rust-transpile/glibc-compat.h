/* Translation-only adaptation: use declared libc functions instead of
glibc 2.43 C23 generic overload macros. The source remains unchanged. */
#ifndef _GNU_SOURCE
#define _GNU_SOURCE 1
#endif
#include <string.h>
#include <stdlib.h>
#include <wchar.h>
#undef memchr
#undef strchr
#undef strrchr
#undef strpbrk
#undef strstr
#undef bsearch
#undef wcschr
#undef wcsrchr
#undef wcspbrk
#undef wcsstr
#undef wmemchr
