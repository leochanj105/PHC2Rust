/* test_bridge.h — declarations for bridge wrappers of static functions.
 * Pre-generated. Include this in test_suite.c to call static functions.
 * Do NOT regenerate — this is shared infrastructure across all experiments.
 */
#ifndef TEST_BRIDGE_H
#define TEST_BRIDGE_H

#include <complex.h>

/* mathd/internal/trigd.c */
extern int bridge___rem_pio2_internal(double *x, double *y, int e0, int nx);

/* mathd/tand.c */
extern double bridge___tan(double x, double y, int iy);

/* mathd/internal/gammad.c */
extern double bridge___sin_pi(double x);

/* mathf/internal/trigf.c */
extern int bridge___rem_pio2f_internal(float *x, float *y, int e0, int nx);

/* mathf/tanf.c */
extern float bridge___tanf(float x, float y, int iy);

/* complexd/ctand.c */
extern double bridge___ctans(double _Complex z);

/* complexf/ctanf.c */
extern float bridge___ctansf(float _Complex z);

/* mathf/internal/gammaf.c */
extern float bridge___sin_pif(float x);

#endif
