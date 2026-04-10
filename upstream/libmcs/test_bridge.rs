//! Rust-side test bridge.
//! Exports C-ABI wrappers so the Rust test binary provides the same symbols
//! that the C test_bridge.c provides for the C binary.
//! Test infrastructure only — not part of the library.
#![allow(warnings)]

use crate::complexd::DoubleComplex;
use crate::complexf::FloatComplex;

// ── Constants (defined in libm/common/tools.c) ──

#[no_mangle] pub static __infd: f64 = f64::INFINITY;
#[no_mangle] pub static __inff: f32 = f32::INFINITY;

// ── Naming fixes ──
// C exports __fpclassifyd/__signbitd, Rust exports __fpclassify/__signbit

#[no_mangle] pub extern "C" fn __fpclassifyd(x: f64) -> i32 { crate::mathd::__fpclassifyd(x) }
#[no_mangle] pub extern "C" fn __signbitd(x: f64) -> i32 { crate::mathd::__signbitd(x) }

// ── Trig kernels ──
// C: __cos(x,y), __sin(x,y,iy) — these are the kernel functions, not cos(x)/sin(x)

#[no_mangle] pub extern "C" fn __cos(x: f64, y: f64) -> f64 { crate::mathd::cos_kern(x, y) }
#[no_mangle] pub extern "C" fn __sin(x: f64, y: f64, iy: i32) -> f64 { crate::mathd::sin_kern(x, y, iy) }
#[no_mangle] pub extern "C" fn __cosf(x: f32, y: f32) -> f32 { crate::mathf::cosf_kern(x, y) }
#[no_mangle] pub extern "C" fn __sinf(x: f32, y: f32, iy: i32) -> f32 { crate::mathf::sinf_kern(x, y, iy) }

// ── Argument reduction ──

#[no_mangle]
pub unsafe extern "C" fn __rem_pio2(x: f64, y: *mut f64) -> i32 {
    let mut yarr = [0.0f64; 2];
    let r = crate::mathd::rem_pio2(x, &mut yarr);
    *y = yarr[0];
    *y.add(1) = yarr[1];
    r
}

#[no_mangle]
pub unsafe extern "C" fn __rem_pio2f(x: f32, y: *mut f32) -> i32 {
    let mut yarr = [0.0f32; 2];
    let r = crate::mathf::rem_pio2f_fn(x, &mut yarr);
    *y = yarr[0];
    *y.add(1) = yarr[1];
    r
}

// ── Gamma internals ──
// C: __lgamma(x, &signgam) — returns result, writes sign via pointer

#[no_mangle]
pub unsafe extern "C" fn __lgamma(x: f64, signgamp: *mut i32) -> f64 {
    let mut sg: i32 = 0;
    let r = crate::mathd::lgamma_r(x, &mut sg);
    *signgamp = sg;
    r
}

#[no_mangle]
pub unsafe extern "C" fn __lgammaf(x: f32, signgamp: *mut i32) -> f32 {
    let mut sg: i32 = 0;
    let r = crate::mathf::lgammaf_r(x, &mut sg);
    *signgamp = sg;
    r
}

// ── Complex helpers ──

#[no_mangle] pub extern "C" fn __redupi(x: f64) -> f64 { crate::complexd::redupi(x) }
#[no_mangle] pub extern "C" fn __redupif(x: f32) -> f32 { crate::complexf::redupif(x) }

#[no_mangle]
pub unsafe extern "C" fn __ccoshsinh(x: f64, c: *mut f64, s: *mut f64) {
    let (cv, sv) = crate::complexd::ccoshsinh(x);
    *c = cv;
    *s = sv;
}

#[no_mangle]
pub unsafe extern "C" fn __ccoshsinhf(x: f32, c: *mut f32, s: *mut f32) {
    let (cv, sv) = crate::complexf::ccoshsinhf(x);
    *c = cv;
    *s = sv;
}

// ── Bridge wrappers for static functions ──
// C has these as static; bridge_xxx is the test-callable wrapper

#[no_mangle] pub extern "C" fn bridge___tan(x: f64, y: f64, iy: i32) -> f64 { crate::mathd::tan_kern(x, y, iy) }
#[no_mangle] pub extern "C" fn bridge___tanf(x: f32, y: f32, iy: i32) -> f32 { crate::mathf::tanf_kern(x, y, iy) }
#[no_mangle] pub extern "C" fn bridge___sin_pi(x: f64) -> f64 { crate::mathd::sin_pi(x) }
#[no_mangle] pub extern "C" fn bridge___sin_pif(x: f32) -> f32 { crate::mathf::sin_pif(x) }
#[no_mangle] pub extern "C" fn bridge___ctans(z: DoubleComplex) -> f64 { crate::complexd::ctans(z) }
#[no_mangle] pub extern "C" fn bridge___ctansf(z: FloatComplex) -> f32 { crate::complexf::ctansf(z) }

#[no_mangle]
pub unsafe extern "C" fn bridge___rem_pio2_internal(x: *const f64, y: *mut f64, e0: i32, nx: i32) -> i32 {
    let x_slice = std::slice::from_raw_parts(x, nx as usize);
    let y_slice = std::slice::from_raw_parts_mut(y, 2);
    crate::mathd::rem_pio2_internal(x_slice, y_slice, e0, nx as usize)
}

#[no_mangle]
pub unsafe extern "C" fn bridge___rem_pio2f_internal(x: *const f32, y: *mut f32, e0: i32, nx: i32) -> i32 {
    let x_arr: [f32; 3] = [*x, *x.add(1), *x.add(2)];
    let mut y_arr: [f32; 2] = [0.0; 2];
    let r = crate::mathf::rem_pio2f_internal(&x_arr, &mut y_arr, e0, nx);
    *y = y_arr[0];
    *y.add(1) = y_arr[1];
    r
}
