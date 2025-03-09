#ifndef MEMORY_UTILS_H
#define MEMORY_UTILS_H

#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

/**
 * Safe memory allocation that checks for allocation failures
 *
 * @param size Size of memory to allocate
 * @return Pointer to allocated memory or NULL on failure
 */
void *safe_malloc(size_t size);

/**
 * Safe memory reallocation that checks for allocation failures
 *
 * @param ptr Pointer to memory to reallocate
 * @param size New size
 * @return Pointer to reallocated memory or NULL on failure
 */
void *safe_realloc(void *ptr, size_t size);

/**
 * Safe string duplication
 *
 * @param str String to duplicate
 * @return Pointer to duplicated string or NULL on failure
 */
char *safe_strdup(const char *str);

/**
 * Safe string copy with size checking
 *
 * @param dest Destination buffer
 * @param src Source string
 * @param size Size of destination buffer
 * @return 0 on success, -1 on failure
 */
int safe_strcpy(char *dest, const char *src, size_t size);

/**
 * Safe string concatenation with size checking
 *
 * @param dest Destination buffer
 * @param src Source string
 * @param size Size of destination buffer
 * @return 0 on success, -1 on failure
 */
int safe_strcat(char *dest, const char *src, size_t size);

/**
 * Secure memory clearing function that won't be optimized away
 *
 * @param ptr Pointer to memory to clear
 * @param size Size of memory to clear
 */
void secure_zero_memory(void *ptr, size_t size);

#endif /* MEMORY_UTILS_H */