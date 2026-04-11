//! Thin C stdlib shim: libyaml relies on `malloc`/`realloc`/`free`,
//! `memcpy`/`memmove`/`memset`/`memcmp`, `strlen`/`strdup`, plus `FILE`/`fread`/`fwrite`/`ferror`.
//!
//! We forward all of these to `libc` so the translated Rust code can be written
//! as a direct transliteration using raw pointers.

use libc::{c_char, c_int, c_uchar, c_void, size_t, FILE};

pub use libc::{fread, fwrite, ferror, strlen, strdup};
pub use libc::{malloc, realloc, free};
pub use libc::{memcpy, memmove, memset, memcmp};

pub type Libc_FILE = FILE;

#[inline]
pub unsafe fn ptr_offset<T>(p: *mut T, n: isize) -> *mut T {
    p.offset(n)
}

#[inline]
pub unsafe fn ptr_offset_from<T>(a: *const T, b: *const T) -> isize {
    (a as isize - b as isize) / core::mem::size_of::<T>() as isize
}
