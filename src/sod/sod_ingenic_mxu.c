/*
 * Ingenic XBurst MXU2/MXU3 acceleration for SOD.
 *
 * Copyright (C) 2026 LightNVR contributors
 *
 * This file is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * MXU2 and MXU3 are incompatible COP2 instruction sets.  The short raw
 * instruction sequences below implement only the float32 operations needed
 * by GEMM.  Raw words keep the build compatible with ordinary MIPS32
 * toolchains, which do not normally know Ingenic's vector mnemonics.
 */

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif

#include "sod_ingenic_mxu.h"

#include <pthread.h>
#include <sched.h>
#include <setjmp.h>
#include <signal.h>
#include <stddef.h>
#include <string.h>

#if !defined(__mips__)
#error "SOD_ENABLE_INGENIC_MXU requires a MIPS target"
#endif

#define SOD_MXU_WORD_INNER(word) ".word " #word "\n\t"
#define SOD_MXU_WORD(word) SOD_MXU_WORD_INNER(word)

enum sod_mxu_backend {
	SOD_MXU_NONE = 0,
	SOD_MXU2,
	SOD_MXU3
};

static enum sod_mxu_backend sod_mxu_selected = SOD_MXU_NONE;
static pthread_once_t sod_mxu_once = PTHREAD_ONCE_INIT;

/*
 * Probe state is thread-local because signal dispositions are process-wide:
 * an unrelated fault in another thread must not jump into this thread's
 * probe frame.  Unrelated signals are forwarded to the previous disposition.
 */
static _Thread_local sigjmp_buf sod_mxu_probe_jmp;
static _Thread_local volatile sig_atomic_t sod_mxu_probe_active;

struct sod_mxu_probe_signal {
	int number;
	struct sigaction previous;
	int installed;
};

static struct sod_mxu_probe_signal sod_mxu_probe_signals[] = {
	{ .number = SIGILL },
	{ .number = SIGBUS },
	{ .number = SIGSEGV }
};

static struct sigaction *sod_mxu_previous_action(int signal_number)
{
	size_t i;

	for (i = 0; i < sizeof(sod_mxu_probe_signals) /
		sizeof(sod_mxu_probe_signals[0]); ++i) {
		if (sod_mxu_probe_signals[i].number == signal_number) {
			return &sod_mxu_probe_signals[i].previous;
		}
	}
	return NULL;
}

static void sod_mxu_probe_signal_handler(int signal_number, siginfo_t *info,
	void *context)
{
	struct sigaction *previous;

	if (sod_mxu_probe_active) {
		sod_mxu_probe_active = 0;
		siglongjmp(sod_mxu_probe_jmp, 1);
	}

	previous = sod_mxu_previous_action(signal_number);
	if (previous == NULL) {
		return;
	}
	if (previous->sa_handler == SIG_IGN) {
		return;
	}
	if (previous->sa_handler != SIG_DFL) {
		if ((previous->sa_flags & SA_SIGINFO) != 0) {
			previous->sa_sigaction(signal_number, info, context);
		}
		else {
			previous->sa_handler(signal_number);
		}
		return;
	}

	/* Preserve the default action for a fault unrelated to the probe. */
	sigaction(signal_number, previous, NULL);
	raise(signal_number);
}

static int sod_mxu_install_probe_handlers(void)
{
	struct sigaction action;
	size_t i;

	memset(&action, 0, sizeof(action));
	action.sa_sigaction = sod_mxu_probe_signal_handler;
	action.sa_flags = SA_SIGINFO;
	sigemptyset(&action.sa_mask);

	for (i = 0; i < sizeof(sod_mxu_probe_signals) /
		sizeof(sod_mxu_probe_signals[0]); ++i) {
		if (sigaction(sod_mxu_probe_signals[i].number, &action,
			&sod_mxu_probe_signals[i].previous) != 0) {
			return 0;
		}
		sod_mxu_probe_signals[i].installed = 1;
	}
	return 1;
}

static void sod_mxu_restore_probe_handlers(void)
{
	size_t i;

	for (i = 0; i < sizeof(sod_mxu_probe_signals) /
		sizeof(sod_mxu_probe_signals[0]); ++i) {
		if (sod_mxu_probe_signals[i].installed) {
			sigaction(sod_mxu_probe_signals[i].number,
				&sod_mxu_probe_signals[i].previous, NULL);
			sod_mxu_probe_signals[i].installed = 0;
		}
	}
}

/* Load a four-lane float broadcast into MXU2 VPR1. */
static inline void sod_mxu2_load_scale(const float *scale)
{
	__asm__ __volatile__(
		".set push\n\t"
		".set noreorder\n\t"
		".set noat\n\t"
		"move $t0, %[scale]\n\t"
		SOD_MXU_WORD(0x71000054) /* lu1q VPR1, 0($t0) */
		".set pop\n\t"
		:
		: [scale] "r"(scale)
		: "$t0", "memory");
}

/* C[0:4] = B[0:4] * VPR1 + C[0:4]. */
static inline void sod_mxu2_axpy4(const float *B, float *C)
{
	__asm__ __volatile__(
		".set push\n\t"
		".set noreorder\n\t"
		".set noat\n\t"
		"move $t0, %[b]\n\t"
		SOD_MXU_WORD(0x71000014) /* lu1q VPR0, 0($t0) */
		"move $t0, %[c]\n\t"
		SOD_MXU_WORD(0x71000094) /* lu1q VPR2, 0($t0) */
		SOD_MXU_WORD(0x4b010088) /* fmadd.w VPR2, VPR0, VPR1 */
		SOD_MXU_WORD(0x7100009c) /* su1q VPR2, 0($t0) */
		".set pop\n\t"
		:
		: [b] "r"(B), [c] "r"(C)
		: "$t0", "memory");
}

/* Load a sixteen-lane float broadcast into MXU3 VPR1, four quarters. */
static inline void sod_mxu3_load_scale(const float *scale)
{
	__asm__ __volatile__(
		".set push\n\t"
		".set noreorder\n\t"
		".set noat\n\t"
		"move $t0, %[scale]\n\t"
		SOD_MXU_WORD(0x71000913) /* luq VPR1.q0, $t0 */
		SOD_MXU_WORD(0x71000953) /* luq VPR1.q1, $t0 */
		SOD_MXU_WORD(0x71000993) /* luq VPR1.q2, $t0 */
		SOD_MXU_WORD(0x710009d3) /* luq VPR1.q3, $t0 */
		".set pop\n\t"
		:
		: [scale] "r"(scale)
		: "$t0", "memory");
}

/* C[0:16] += B[0:16] * VPR1.  LUQ/SUQ tolerate unaligned rows. */
static inline void sod_mxu3_axpy16(const float *B, float *C)
{
	__asm__ __volatile__(
		".set push\n\t"
		".set noreorder\n\t"
		".set noat\n\t"
		"move $t0, %[b]\n\t"
		SOD_MXU_WORD(0x71000813) /* luq VPR0.q0, $t0 */
		SOD_MXU_WORD(0x71000853) /* luq VPR0.q1, $t0 */
		SOD_MXU_WORD(0x71000893) /* luq VPR0.q2, $t0 */
		SOD_MXU_WORD(0x710008d3) /* luq VPR0.q3, $t0 */
		"move $t0, %[c]\n\t"
		SOD_MXU_WORD(0x71000a13) /* luq VPR2.q0, $t0 */
		SOD_MXU_WORD(0x71000a53) /* luq VPR2.q1, $t0 */
		SOD_MXU_WORD(0x71000a93) /* luq VPR2.q2, $t0 */
		SOD_MXU_WORD(0x71000ad3) /* luq VPR2.q3, $t0 */
		SOD_MXU_WORD(0x4a602323) /* fmul.w VPR3.q0, VPR0.q0, VPR1.q0 */
		SOD_MXU_WORD(0x4a612b63) /* fmul.w VPR3.q1, VPR0.q1, VPR1.q1 */
		SOD_MXU_WORD(0x4a6233a3) /* fmul.w VPR3.q2, VPR0.q2, VPR1.q2 */
		SOD_MXU_WORD(0x4a633be3) /* fmul.w VPR3.q3, VPR0.q3, VPR1.q3 */
		SOD_MXU_WORD(0x4a8c4203) /* fadd.w VPR2.q0, VPR3.q0, VPR2.q0 */
		SOD_MXU_WORD(0x4a8d4a43) /* fadd.w VPR2.q1, VPR3.q1, VPR2.q1 */
		SOD_MXU_WORD(0x4a8e5283) /* fadd.w VPR2.q2, VPR3.q2, VPR2.q2 */
		SOD_MXU_WORD(0x4a8f5ac3) /* fadd.w VPR2.q3, VPR3.q3, VPR2.q3 */
		"move $t0, %[c]\n\t"
		SOD_MXU_WORD(0x71004057) /* suq VPR2.q0, $t0 */
		SOD_MXU_WORD(0x71004857) /* suq VPR2.q1, $t0 */
		SOD_MXU_WORD(0x71005057) /* suq VPR2.q2, $t0 */
		SOD_MXU_WORD(0x71005857) /* suq VPR2.q3, $t0 */
		".set pop\n\t"
		:
		: [b] "r"(B), [c] "r"(C)
		: "$t0", "memory");
}

static int sod_mxu2_probe_operation(void)
{
	float scale[4] __attribute__((aligned(16))) = { 2, 2, 2, 2 };
	float b_storage[5] __attribute__((aligned(16))) = { 0, 1, 2, 3, 4 };
	float c_storage[5] __attribute__((aligned(16))) = { 0, 4, 4, 4, 4 };
	float *C = c_storage + 1;
	int i;

	sod_mxu2_load_scale(scale);
	sod_mxu2_axpy4(b_storage + 1, C);
	for (i = 0; i < 4; ++i) {
		if (C[i] != (float)(6 + 2 * i)) {
			return 0;
		}
	}
	return 1;
}

static int sod_mxu3_probe_operation(void)
{
	float scale[16] __attribute__((aligned(64)));
	float b_storage[17] __attribute__((aligned(64)));
	float c_storage[17] __attribute__((aligned(64)));
	float *B = b_storage + 1;
	float *C = c_storage + 1;
	int i;

	for (i = 0; i < 16; ++i) {
		scale[i] = 2;
		B[i] = (float)(i + 1);
		C[i] = 4;
	}
	sod_mxu3_load_scale(scale);
	sod_mxu3_axpy16(B, C);
	for (i = 0; i < 16; ++i) {
		if (C[i] != (float)(6 + 2 * i)) {
			return 0;
		}
	}
	return 1;
}

static int sod_mxu_try_probe(enum sod_mxu_backend backend)
{
	volatile int result = 0;

	sod_mxu_probe_active = 1;
	if (sigsetjmp(sod_mxu_probe_jmp, 1) == 0) {
		if (backend == SOD_MXU3) {
			result = sod_mxu3_probe_operation();
		}
		else if (backend == SOD_MXU2) {
			result = sod_mxu2_probe_operation();
		}
	}
	sod_mxu_probe_active = 0;
	return result;
}

static enum sod_mxu_backend sod_mxu_probe_current_cpu(void)
{
	if (sod_mxu_try_probe(SOD_MXU3)) {
		return SOD_MXU3;
	}
	if (sod_mxu_try_probe(SOD_MXU2)) {
		return SOD_MXU2;
	}
	return SOD_MXU_NONE;
}

static void sod_mxu_detect(void)
{
	cpu_set_t original_affinity;
	cpu_set_t one_cpu;
	enum sod_mxu_backend detected = SOD_MXU_NONE;
	int have_result = 0;
	int affinity_ok = 1;
	int cpu;

	if (sched_getaffinity(0, sizeof(original_affinity),
		&original_affinity) != 0) {
		return;
	}
	if (!sod_mxu_install_probe_handlers()) {
		sod_mxu_restore_probe_handlers();
		return;
	}

	for (cpu = 0; cpu < CPU_SETSIZE; ++cpu) {
		enum sod_mxu_backend current;

		if (!CPU_ISSET(cpu, &original_affinity)) {
			continue;
		}
		CPU_ZERO(&one_cpu);
		CPU_SET(cpu, &one_cpu);
		if (sched_setaffinity(0, sizeof(one_cpu), &one_cpu) != 0) {
			affinity_ok = 0;
			break;
		}
		current = sod_mxu_probe_current_cpu();
		if (!have_result) {
			detected = current;
			have_result = 1;
		}
		else if (current != detected) {
			detected = SOD_MXU_NONE;
			affinity_ok = 0;
			break;
		}
	}

	if (sched_setaffinity(0, sizeof(original_affinity),
		&original_affinity) != 0) {
		affinity_ok = 0;
	}
	sod_mxu_restore_probe_handlers();

	if (affinity_ok && have_result) {
		sod_mxu_selected = detected;
	}
}

static void sod_mxu2_gemm_nn(int M, int N, int K, float alpha,
	const float *A, int lda, const float *B, int ldb, float *C, int ldc)
{
	int i, j, k;

	for (i = 0; i < M; ++i) {
		for (k = 0; k < K; ++k) {
			float a_part = alpha * A[i * lda + k];
			float scale[4] __attribute__((aligned(16))) = {
				a_part, a_part, a_part, a_part
			};
			const float *b_row = B + k * ldb;
			float *c_row = C + i * ldc;

			sod_mxu2_load_scale(scale);
			for (j = 0; j + 4 <= N; j += 4) {
				sod_mxu2_axpy4(b_row + j, c_row + j);
			}
			for (; j < N; ++j) {
				c_row[j] += a_part * b_row[j];
			}
		}
	}
}

static void sod_mxu3_gemm_nn(int M, int N, int K, float alpha,
	const float *A, int lda, const float *B, int ldb, float *C, int ldc)
{
	int i, j, k;

	for (i = 0; i < M; ++i) {
		for (k = 0; k < K; ++k) {
			float a_part = alpha * A[i * lda + k];
			float scale[16] __attribute__((aligned(64)));
			const float *b_row = B + k * ldb;
			float *c_row = C + i * ldc;

			for (j = 0; j < 16; ++j) {
				scale[j] = a_part;
			}
			sod_mxu3_load_scale(scale);
			for (j = 0; j + 16 <= N; j += 16) {
				sod_mxu3_axpy16(b_row + j, c_row + j);
			}
			for (; j < N; ++j) {
				c_row[j] += a_part * b_row[j];
			}
		}
	}
}

int sod_ingenic_gemm_nn(int M, int N, int K, float alpha,
	const float *A, int lda, const float *B, int ldb, float *C, int ldc)
{
	pthread_once(&sod_mxu_once, sod_mxu_detect);

	if (sod_mxu_selected == SOD_MXU3 && N >= 16) {
		sod_mxu3_gemm_nn(M, N, K, alpha, A, lda, B, ldb, C, ldc);
		return 1;
	}
	if (sod_mxu_selected == SOD_MXU2 && N >= 4) {
		sod_mxu2_gemm_nn(M, N, K, alpha, A, lda, B, ldb, C, ldc);
		return 1;
	}
	return 0;
}
