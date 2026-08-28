/*
 * Ingenic MXU acceleration for SOD's non-transposed GEMM hot path.
 *
 * This file is part of Symisc SOD - Open Source Release (GPLv3).
 */
#ifndef SOD_INGENIC_MXU_H
#define SOD_INGENIC_MXU_H

#if defined(__GNUC__) || defined(__clang__)
#define SOD_MXU_INTERNAL __attribute__((visibility("hidden")))
#else
#define SOD_MXU_INTERNAL
#endif

SOD_MXU_INTERNAL int sod_ingenic_gemm_nn(int M, int N, int K, float alpha,
	const float *A, int lda,
	const float *B, int ldb,
	float *C, int ldc);

#undef SOD_MXU_INTERNAL

#endif /* SOD_INGENIC_MXU_H */
