/* test_bridge.c — expose static functions for testing */

/*
 * Strategy: include source .c files to access their static functions.
 * Use #define to rename conflicting static const variables between files.
 */

/* ===== GROUP 1: Double-precision trig internals (trigd.c) ===== */
#include "/home/leochanj/Desktop/libmcs/libm/mathd/internal/trigd.c"

int bridge___rem_pio2_internal(double *x, double *y, int e0, int nx) {
    return __rem_pio2_internal(x, y, e0, nx);
}

/* ===== GROUP 2: Double-precision tan (tand.c) ===== */
/* No const name conflicts with trigd.c (pio4, pio4lo, T are unique) */
#include "/home/leochanj/Desktop/libmcs/libm/mathd/tand.c"

double bridge___tan(double x, double y, int iy) {
    return __tan(x, y, iy);
}

/* ===== GROUP 3: Double-precision gamma (gammad.c) ===== */
/* Conflicts with trigd.c: half, one, zero */
#define half _gd_half
#define one  _gd_one
#define zero _gd_zero
#include "/home/leochanj/Desktop/libmcs/libm/mathd/internal/gammad.c"
#undef half
#undef one
#undef zero

double bridge___sin_pi(double x) {
    return __sin_pi(x);
}

/* ===== GROUP 4: Float-precision trig internals (trigf.c) ===== */
/* Conflicts with trigd.c: PIo2, ipio2, zero, one, half, invpio2,
   pio2_1, pio2_1t, pio2_2, pio2_2t, pio2_3, pio2_3t, C1-C3, S1-S4 */
#define PIo2    _tf_PIo2
#define ipio2   _tf_ipio2
#define zero    _tf_zero
#define one     _tf_one
#define half    _tf_half
#define invpio2 _tf_invpio2
#define pio2_1  _tf_pio2_1
#define pio2_1t _tf_pio2_1t
#define pio2_2  _tf_pio2_2
#define pio2_2t _tf_pio2_2t
#define pio2_3  _tf_pio2_3
#define pio2_3t _tf_pio2_3t
#define C1      _tf_C1
#define C2      _tf_C2
#define C3      _tf_C3
#define S1      _tf_S1
#define S2      _tf_S2
#define S3      _tf_S3
#define S4      _tf_S4
#include "/home/leochanj/Desktop/libmcs/libm/mathf/internal/trigf.c"
#undef PIo2
#undef ipio2
#undef zero
#undef one
#undef half
#undef invpio2
#undef pio2_1
#undef pio2_1t
#undef pio2_2
#undef pio2_2t
#undef pio2_3
#undef pio2_3t
#undef C1
#undef C2
#undef C3
#undef S1
#undef S2
#undef S3
#undef S4

int bridge___rem_pio2f_internal(float *x, float *y, int e0, int nx) {
    return __rem_pio2f_internal(x, y, e0, nx);
}

/* ===== GROUP 5: Float-precision tan (tanf.c) ===== */
/* Conflicts with tand.c: pio4, pio4lo, T */
#define pio4   _tanf_pio4
#define pio4lo _tanf_pio4lo
#define T      _tanf_T
#include "/home/leochanj/Desktop/libmcs/libm/mathf/tanf.c"
#undef pio4
#undef pio4lo
#undef T

float bridge___tanf(float x, float y, int iy) {
    return __tanf(x, y, iy);
}

/* ===== GROUP 6: Complex double tan (ctand.c) ===== */
/* No static consts, no conflicts */
#include "/home/leochanj/Desktop/libmcs/libm/complexd/ctand.c"

double bridge___ctans(double _Complex z) {
    return __ctans(z);
}

/* ===== GROUP 7: Complex float tan (ctanf.c) ===== */
/* No static consts, no conflicts */
#include "/home/leochanj/Desktop/libmcs/libm/complexf/ctanf.c"

float bridge___ctansf(float _Complex z) {
    return __ctansf(z);
}

/* ===== GROUP 8: Float-precision gamma (gammaf.c) ===== */
/* Conflicts with gammad.c: two23/two52, half, one, pi, a0-a11, tc, tf, tt,
   t0-t14, u0-u5, v1-v5, s0-s6, r1-r6, w0-w6, zero */
#define two23 _gf_two23
#define half  _gf_half
#define one   _gf_one
#define pi    _gf_pi
#define a0    _gf_a0
#define a1    _gf_a1
#define a2    _gf_a2
#define a3    _gf_a3
#define a4    _gf_a4
#define a5    _gf_a5
#define a6    _gf_a6
#define a7    _gf_a7
#define a8    _gf_a8
#define a9    _gf_a9
#define a10   _gf_a10
#define a11   _gf_a11
#define tc    _gf_tc
#define tf    _gf_tf
#define tt    _gf_tt
#define t0    _gf_t0
#define t1    _gf_t1
#define t2    _gf_t2
#define t3    _gf_t3
#define t4    _gf_t4
#define t5    _gf_t5
#define t6    _gf_t6
#define t7    _gf_t7
#define t8    _gf_t8
#define t9    _gf_t9
#define t10   _gf_t10
#define t11   _gf_t11
#define t12   _gf_t12
#define t13   _gf_t13
#define t14   _gf_t14
#define u0    _gf_u0
#define u1    _gf_u1
#define u2    _gf_u2
#define u3    _gf_u3
#define u4    _gf_u4
#define u5    _gf_u5
#define v1    _gf_v1
#define v2    _gf_v2
#define v3    _gf_v3
#define v4    _gf_v4
#define v5    _gf_v5
#define s0    _gf_s0
#define s1    _gf_s1
#define s2    _gf_s2
#define s3    _gf_s3
#define s4    _gf_s4
#define s5    _gf_s5
#define s6    _gf_s6
#define r1    _gf_r1
#define r2    _gf_r2
#define r3    _gf_r3
#define r4    _gf_r4
#define r5    _gf_r5
#define r6    _gf_r6
#define w0    _gf_w0
#define w1    _gf_w1
#define w2    _gf_w2
#define w3    _gf_w3
#define w4    _gf_w4
#define w5    _gf_w5
#define w6    _gf_w6
#define zero  _gf_zero
/* Rename __lgammaf to avoid duplicate definition with library symbol */
#define __lgammaf _gf___lgammaf
#include "/home/leochanj/Desktop/libmcs/libm/mathf/internal/gammaf.c"
#undef __lgammaf
#undef two23
#undef half
#undef one
#undef pi
#undef a0
#undef a1
#undef a2
#undef a3
#undef a4
#undef a5
#undef a6
#undef a7
#undef a8
#undef a9
#undef a10
#undef a11
#undef tc
#undef tf
#undef tt
#undef t0
#undef t1
#undef t2
#undef t3
#undef t4
#undef t5
#undef t6
#undef t7
#undef t8
#undef t9
#undef t10
#undef t11
#undef t12
#undef t13
#undef t14
#undef u0
#undef u1
#undef u2
#undef u3
#undef u4
#undef u5
#undef v1
#undef v2
#undef v3
#undef v4
#undef v5
#undef s0
#undef s1
#undef s2
#undef s3
#undef s4
#undef s5
#undef s6
#undef r1
#undef r2
#undef r3
#undef r4
#undef r5
#undef r6
#undef w0
#undef w1
#undef w2
#undef w3
#undef w4
#undef w5
#undef w6
#undef zero

float bridge___sin_pif(float x) {
    return __sin_pif(x);
}
