//! Rust translation of yaml_private.h — memory helpers, stack/queue/buffer
//! primitives, and the character-classification macros that permeate scanner.c
//! and emitter.c.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_macros)]

use crate::externs::*;
use crate::yaml::*;
use libc::{c_char, c_int, c_uchar, c_void, size_t};

pub const INPUT_RAW_BUFFER_SIZE: size_t = 16384;
pub const INPUT_BUFFER_SIZE: size_t = INPUT_RAW_BUFFER_SIZE * 3;
pub const OUTPUT_BUFFER_SIZE: size_t = 16384;
pub const OUTPUT_RAW_BUFFER_SIZE: size_t = OUTPUT_BUFFER_SIZE * 2 + 2;
pub const MAX_FILE_SIZE: size_t = (!0usize) / 2;

pub const INITIAL_STACK_SIZE: size_t = 16;
pub const INITIAL_QUEUE_SIZE: size_t = 16;
pub const INITIAL_STRING_SIZE: size_t = 16;

/* --------------------------------------------------------------------- */
/*  Memory helpers (defined in api.rs, declared here so the rest of the
 *  crate can call them).                                                 */
/* --------------------------------------------------------------------- */

extern "C" {
    pub fn yaml_malloc(size: size_t) -> *mut c_void;
    pub fn yaml_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn yaml_free(ptr: *mut c_void);
    pub fn yaml_strdup(str_: *const yaml_char_t) -> *mut yaml_char_t;
    pub fn yaml_string_extend(
        start: *mut *mut yaml_char_t,
        pointer: *mut *mut yaml_char_t,
        end: *mut *mut yaml_char_t,
    ) -> c_int;
    pub fn yaml_string_join(
        a_start: *mut *mut yaml_char_t,
        a_pointer: *mut *mut yaml_char_t,
        a_end: *mut *mut yaml_char_t,
        b_start: *mut *mut yaml_char_t,
        b_pointer: *mut *mut yaml_char_t,
        b_end: *mut *mut yaml_char_t,
    ) -> c_int;
    pub fn yaml_stack_extend(
        start: *mut *mut c_void,
        top: *mut *mut c_void,
        end: *mut *mut c_void,
    ) -> c_int;
    pub fn yaml_queue_extend(
        start: *mut *mut c_void,
        head: *mut *mut c_void,
        tail: *mut *mut c_void,
        end: *mut *mut c_void,
    ) -> c_int;
    pub fn yaml_parser_update_buffer(parser: *mut yaml_parser_t, length: size_t) -> c_int;
    pub fn yaml_parser_fetch_more_tokens(parser: *mut yaml_parser_t) -> c_int;
}

/* --------------------------------------------------------------------- */
/*  A yaml_string_t is a tiny (start,end,pointer) triple used throughout
 *  scanner.c / emitter.c for scratch strings.                            */
/* --------------------------------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}

pub const NULL_STRING: yaml_string_t = yaml_string_t {
    start: core::ptr::null_mut(),
    end: core::ptr::null_mut(),
    pointer: core::ptr::null_mut(),
};

#[inline]
pub unsafe fn STRING_ASSIGN(string: *mut yaml_char_t, length: size_t) -> yaml_string_t {
    yaml_string_t {
        start: string,
        end: string.add(length),
        pointer: string,
    }
}

/* --------------------------------------------------------------------- */
/*  Character classification helpers.
 *
 *  These mirror the C macros in yaml_private.h.  They take a
 *  `yaml_string_t` (or the corresponding buffer inside parser/emitter)
 *  and an offset and return a bool / u8.                                 */
/* --------------------------------------------------------------------- */

#[inline]
pub unsafe fn CHECK_AT(string: yaml_string_t, octet: u8, offset: isize) -> bool {
    *string.pointer.offset(offset) == octet
}

#[inline]
pub unsafe fn CHECK(string: yaml_string_t, octet: u8) -> bool {
    CHECK_AT(string, octet, 0)
}

#[inline]
pub unsafe fn IS_ALPHA_AT(string: yaml_string_t, offset: isize) -> bool {
    let c = *string.pointer.offset(offset);
    (c >= b'0' && c <= b'9')
        || (c >= b'A' && c <= b'Z')
        || (c >= b'a' && c <= b'z')
        || c == b'_'
        || c == b'-'
}
#[inline]
pub unsafe fn IS_ALPHA(string: yaml_string_t) -> bool {
    IS_ALPHA_AT(string, 0)
}

#[inline]
pub unsafe fn IS_DIGIT_AT(string: yaml_string_t, offset: isize) -> bool {
    let c = *string.pointer.offset(offset);
    c >= b'0' && c <= b'9'
}
#[inline]
pub unsafe fn IS_DIGIT(string: yaml_string_t) -> bool {
    IS_DIGIT_AT(string, 0)
}

#[inline]
pub unsafe fn AS_DIGIT_AT(string: yaml_string_t, offset: isize) -> c_int {
    (*string.pointer.offset(offset) as c_int) - (b'0' as c_int)
}
#[inline]
pub unsafe fn AS_DIGIT(string: yaml_string_t) -> c_int {
    AS_DIGIT_AT(string, 0)
}

#[inline]
pub unsafe fn IS_HEX_AT(string: yaml_string_t, offset: isize) -> bool {
    let c = *string.pointer.offset(offset);
    (c >= b'0' && c <= b'9') || (c >= b'A' && c <= b'F') || (c >= b'a' && c <= b'f')
}
#[inline]
pub unsafe fn IS_HEX(string: yaml_string_t) -> bool {
    IS_HEX_AT(string, 0)
}

#[inline]
pub unsafe fn AS_HEX_AT(string: yaml_string_t, offset: isize) -> c_int {
    let c = *string.pointer.offset(offset);
    if c >= b'A' && c <= b'F' {
        (c as c_int) - (b'A' as c_int) + 10
    } else if c >= b'a' && c <= b'f' {
        (c as c_int) - (b'a' as c_int) + 10
    } else {
        (c as c_int) - (b'0' as c_int)
    }
}
#[inline]
pub unsafe fn AS_HEX(string: yaml_string_t) -> c_int {
    AS_HEX_AT(string, 0)
}

#[inline]
pub unsafe fn IS_ASCII_AT(string: yaml_string_t, offset: isize) -> bool {
    *string.pointer.offset(offset) <= 0x7F
}
#[inline]
pub unsafe fn IS_ASCII(string: yaml_string_t) -> bool {
    IS_ASCII_AT(string, 0)
}

#[inline]
pub unsafe fn IS_PRINTABLE_AT(string: yaml_string_t, offset: isize) -> bool {
    let p = string.pointer.offset(offset);
    let c0 = *p;
    (c0 == 0x0A)
        || (c0 >= 0x20 && c0 <= 0x7E)
        || (c0 == 0xC2 && *p.add(1) >= 0xA0)
        || (c0 > 0xC2 && c0 < 0xED)
        || (c0 == 0xED && *p.add(1) < 0xA0)
        || (c0 == 0xEE)
        || (c0 == 0xEF
            && !(*p.add(1) == 0xBB && *p.add(2) == 0xBF)
            && !(*p.add(1) == 0xBF && (*p.add(2) == 0xBE || *p.add(2) == 0xBF)))
}
#[inline]
pub unsafe fn IS_PRINTABLE(string: yaml_string_t) -> bool {
    IS_PRINTABLE_AT(string, 0)
}

#[inline]
pub unsafe fn IS_Z_AT(string: yaml_string_t, offset: isize) -> bool {
    CHECK_AT(string, b'\0', offset)
}
#[inline]
pub unsafe fn IS_Z(string: yaml_string_t) -> bool {
    IS_Z_AT(string, 0)
}

#[inline]
pub unsafe fn IS_BOM_AT(string: yaml_string_t, offset: isize) -> bool {
    CHECK_AT(string, 0xEF, offset)
        && CHECK_AT(string, 0xBB, offset + 1)
        && CHECK_AT(string, 0xBF, offset + 2)
}
#[inline]
pub unsafe fn IS_BOM(string: yaml_string_t) -> bool {
    IS_BOM_AT(string, 0)
}

#[inline]
pub unsafe fn IS_SPACE_AT(string: yaml_string_t, offset: isize) -> bool {
    CHECK_AT(string, b' ', offset)
}
#[inline]
pub unsafe fn IS_SPACE(string: yaml_string_t) -> bool {
    IS_SPACE_AT(string, 0)
}

#[inline]
pub unsafe fn IS_TAB_AT(string: yaml_string_t, offset: isize) -> bool {
    CHECK_AT(string, b'\t', offset)
}
#[inline]
pub unsafe fn IS_TAB(string: yaml_string_t) -> bool {
    IS_TAB_AT(string, 0)
}

#[inline]
pub unsafe fn IS_BLANK_AT(string: yaml_string_t, offset: isize) -> bool {
    IS_SPACE_AT(string, offset) || IS_TAB_AT(string, offset)
}
#[inline]
pub unsafe fn IS_BLANK(string: yaml_string_t) -> bool {
    IS_BLANK_AT(string, 0)
}

#[inline]
pub unsafe fn IS_BREAK_AT(string: yaml_string_t, offset: isize) -> bool {
    CHECK_AT(string, b'\r', offset)
        || CHECK_AT(string, b'\n', offset)
        || (CHECK_AT(string, 0xC2, offset) && CHECK_AT(string, 0x85, offset + 1))
        || (CHECK_AT(string, 0xE2, offset)
            && CHECK_AT(string, 0x80, offset + 1)
            && CHECK_AT(string, 0xA8, offset + 2))
        || (CHECK_AT(string, 0xE2, offset)
            && CHECK_AT(string, 0x80, offset + 1)
            && CHECK_AT(string, 0xA9, offset + 2))
}
#[inline]
pub unsafe fn IS_BREAK(string: yaml_string_t) -> bool {
    IS_BREAK_AT(string, 0)
}

#[inline]
pub unsafe fn IS_CRLF_AT(string: yaml_string_t, offset: isize) -> bool {
    CHECK_AT(string, b'\r', offset) && CHECK_AT(string, b'\n', offset + 1)
}
#[inline]
pub unsafe fn IS_CRLF(string: yaml_string_t) -> bool {
    IS_CRLF_AT(string, 0)
}

#[inline]
pub unsafe fn IS_BREAKZ_AT(string: yaml_string_t, offset: isize) -> bool {
    IS_BREAK_AT(string, offset) || IS_Z_AT(string, offset)
}
#[inline]
pub unsafe fn IS_BREAKZ(string: yaml_string_t) -> bool {
    IS_BREAKZ_AT(string, 0)
}

#[inline]
pub unsafe fn IS_SPACEZ_AT(string: yaml_string_t, offset: isize) -> bool {
    IS_SPACE_AT(string, offset) || IS_BREAKZ_AT(string, offset)
}
#[inline]
pub unsafe fn IS_SPACEZ(string: yaml_string_t) -> bool {
    IS_SPACEZ_AT(string, 0)
}

#[inline]
pub unsafe fn IS_BLANKZ_AT(string: yaml_string_t, offset: isize) -> bool {
    IS_BLANK_AT(string, offset) || IS_BREAKZ_AT(string, offset)
}
#[inline]
pub unsafe fn IS_BLANKZ(string: yaml_string_t) -> bool {
    IS_BLANKZ_AT(string, 0)
}

#[inline]
pub unsafe fn WIDTH_AT(string: yaml_string_t, offset: isize) -> c_int {
    let c = *string.pointer.offset(offset);
    if (c & 0x80) == 0x00 {
        1
    } else if (c & 0xE0) == 0xC0 {
        2
    } else if (c & 0xF0) == 0xE0 {
        3
    } else if (c & 0xF8) == 0xF0 {
        4
    } else {
        0
    }
}
#[inline]
pub unsafe fn WIDTH(string: yaml_string_t) -> c_int {
    WIDTH_AT(string, 0)
}

/* --------------------------------------------------------------------- */
/*  Buffer/stack/queue macros.
 *
 *  Each of these mirrors a `#define FOO(context,...)` macro in
 *  yaml_private.h.  We expose them as `macro_rules!` so translated source
 *  can keep the same call style, and so the implicit `context->error`
 *  assignment is preserved.                                              */
/* --------------------------------------------------------------------- */

#[macro_export]
macro_rules! BUFFER_INIT {
    ($context:expr, $buffer:expr, $size:expr) => {{
        let __sz: libc::size_t = $size;
        let __p = $crate::yaml_private::yaml_malloc(__sz) as *mut $crate::yaml::yaml_char_t;
        if !__p.is_null() {
            $buffer.start = __p;
            $buffer.pointer = __p;
            $buffer.last = __p;
            $buffer.end = __p.add(__sz);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! BUFFER_DEL {
    ($context:expr, $buffer:expr) => {{
        $crate::yaml_private::yaml_free($buffer.start as *mut libc::c_void);
        $buffer.start = core::ptr::null_mut();
        $buffer.pointer = core::ptr::null_mut();
        $buffer.end = core::ptr::null_mut();
    }};
}

#[macro_export]
macro_rules! STRING_INIT {
    ($context:expr, $string:expr, $size:expr) => {{
        let __sz: libc::size_t = $size;
        let __p = $crate::yaml_private::yaml_malloc(__sz) as *mut $crate::yaml::yaml_char_t;
        if !__p.is_null() {
            $string.start = __p;
            $string.pointer = __p;
            $string.end = __p.add(__sz);
            libc::memset(__p as *mut libc::c_void, 0, __sz);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! STRING_DEL {
    ($context:expr, $string:expr) => {{
        $crate::yaml_private::yaml_free($string.start as *mut libc::c_void);
        $string.start = core::ptr::null_mut();
        $string.pointer = core::ptr::null_mut();
        $string.end = core::ptr::null_mut();
    }};
}

#[macro_export]
macro_rules! STRING_EXTEND {
    ($context:expr, $string:expr) => {{
        if $string.pointer.add(5) < $string.end
            || $crate::yaml_private::yaml_string_extend(
                &mut $string.start,
                &mut $string.pointer,
                &mut $string.end,
            ) != 0
        {
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! CLEAR {
    ($context:expr, $string:expr) => {{
        $string.pointer = $string.start;
        libc::memset(
            $string.start as *mut libc::c_void,
            0,
            ($string.end as usize - $string.start as usize),
        );
    }};
}

#[macro_export]
macro_rules! JOIN {
    ($context:expr, $string_a:expr, $string_b:expr) => {{
        if $crate::yaml_private::yaml_string_join(
            &mut $string_a.start,
            &mut $string_a.pointer,
            &mut $string_a.end,
            &mut $string_b.start,
            &mut $string_b.pointer,
            &mut $string_b.end,
        ) != 0
        {
            $string_b.pointer = $string_b.start;
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! STACK_INIT {
    ($context:expr, $stack:expr, $type:ty) => {{
        let __elem_size = core::mem::size_of::<$type>() as libc::size_t;
        let __p = $crate::yaml_private::yaml_malloc(
            ($crate::yaml_private::INITIAL_STACK_SIZE * __elem_size),
        ) as *mut $type;
        if !__p.is_null() {
            $stack.start = __p;
            $stack.top = __p;
            $stack.end = __p.add($crate::yaml_private::INITIAL_STACK_SIZE);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! STACK_DEL {
    ($context:expr, $stack:expr) => {{
        $crate::yaml_private::yaml_free($stack.start as *mut libc::c_void);
        $stack.start = core::ptr::null_mut();
        $stack.top = core::ptr::null_mut();
        $stack.end = core::ptr::null_mut();
    }};
}

#[macro_export]
macro_rules! STACK_EMPTY {
    ($context:expr, $stack:expr) => {
        $stack.start == $stack.top
    };
}

#[macro_export]
macro_rules! STACK_LIMIT {
    ($context:expr, $stack:expr, $size:expr) => {{
        if ($stack.top.offset_from($stack.start) as libc::c_int) < $size {
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! PUSH {
    ($context:expr, $stack:expr, $value:expr) => {{
        let __ok = if $stack.top != $stack.end {
            true
        } else {
            $crate::yaml_private::yaml_stack_extend(
                &mut $stack.start as *mut _ as *mut *mut libc::c_void,
                &mut $stack.top as *mut _ as *mut *mut libc::c_void,
                &mut $stack.end as *mut _ as *mut *mut libc::c_void,
            ) != 0
        };
        if __ok {
            *$stack.top = $value;
            $stack.top = $stack.top.add(1);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! POP {
    ($context:expr, $stack:expr) => {{
        $stack.top = $stack.top.offset(-1);
        *$stack.top
    }};
}

#[macro_export]
macro_rules! QUEUE_INIT {
    ($context:expr, $queue:expr, $size:expr, $type:ty) => {{
        let __sz: libc::size_t = $size;
        let __elem_size = core::mem::size_of::<$type>() as libc::size_t;
        let __p = $crate::yaml_private::yaml_malloc(__sz * __elem_size) as *mut $type;
        if !__p.is_null() {
            $queue.start = __p;
            $queue.head = __p;
            $queue.tail = __p;
            $queue.end = __p.add(__sz);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! QUEUE_DEL {
    ($context:expr, $queue:expr) => {{
        $crate::yaml_private::yaml_free($queue.start as *mut libc::c_void);
        $queue.start = core::ptr::null_mut();
        $queue.head = core::ptr::null_mut();
        $queue.tail = core::ptr::null_mut();
        $queue.end = core::ptr::null_mut();
    }};
}

#[macro_export]
macro_rules! QUEUE_EMPTY {
    ($context:expr, $queue:expr) => {
        $queue.head == $queue.tail
    };
}

#[macro_export]
macro_rules! ENQUEUE {
    ($context:expr, $queue:expr, $value:expr) => {{
        let __ok = if $queue.tail != $queue.end {
            true
        } else {
            $crate::yaml_private::yaml_queue_extend(
                &mut $queue.start as *mut _ as *mut *mut libc::c_void,
                &mut $queue.head as *mut _ as *mut *mut libc::c_void,
                &mut $queue.tail as *mut _ as *mut *mut libc::c_void,
                &mut $queue.end as *mut _ as *mut *mut libc::c_void,
            ) != 0
        };
        if __ok {
            *$queue.tail = $value;
            $queue.tail = $queue.tail.add(1);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

#[macro_export]
macro_rules! DEQUEUE {
    ($context:expr, $queue:expr) => {{
        let __v = *$queue.head;
        $queue.head = $queue.head.add(1);
        __v
    }};
}

#[macro_export]
macro_rules! QUEUE_INSERT {
    ($context:expr, $queue:expr, $index:expr, $value:expr) => {{
        let __ok = if $queue.tail != $queue.end {
            true
        } else {
            $crate::yaml_private::yaml_queue_extend(
                &mut $queue.start as *mut _ as *mut *mut libc::c_void,
                &mut $queue.head as *mut _ as *mut *mut libc::c_void,
                &mut $queue.tail as *mut _ as *mut *mut libc::c_void,
                &mut $queue.end as *mut _ as *mut *mut libc::c_void,
            ) != 0
        };
        if __ok {
            libc::memmove(
                $queue.head.add($index + 1) as *mut libc::c_void,
                $queue.head.add($index) as *const libc::c_void,
                ($queue.tail.offset_from($queue.head) as libc::size_t - ($index as libc::size_t))
                    * core::mem::size_of_val(&*$queue.start),
            );
            *$queue.head.add($index) = $value;
            $queue.tail = $queue.tail.add(1);
            1
        } else {
            (*$context).error = $crate::yaml::YAML_MEMORY_ERROR;
            0
        }
    }};
}

/* --------------------------------------------------------------------- */
/*  Token / event / node init helpers.                                    */
/* --------------------------------------------------------------------- */

#[inline]
pub unsafe fn TOKEN_INIT(
    token: *mut yaml_token_t,
    token_type: yaml_token_type_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    libc::memset(token as *mut c_void, 0, core::mem::size_of::<yaml_token_t>());
    (*token).type_ = token_type;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
}

#[inline]
pub unsafe fn EVENT_INIT(
    event: *mut yaml_event_t,
    event_type: yaml_event_type_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = event_type;
    (*event).start_mark = start_mark;
    (*event).end_mark = end_mark;
}

#[inline]
pub unsafe fn NODE_INIT(
    node: *mut yaml_node_t,
    node_type: yaml_node_type_t,
    tag: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    libc::memset(node as *mut c_void, 0, core::mem::size_of::<yaml_node_t>());
    (*node).type_ = node_type;
    (*node).tag = tag;
    (*node).start_mark = start_mark;
    (*node).end_mark = end_mark;
}

/* A tiny helper matching the C `YAML_MALLOC_STATIC(type)` macro. */
#[inline]
pub unsafe fn YAML_MALLOC_STATIC<T>() -> *mut T {
    yaml_malloc(core::mem::size_of::<T>() as size_t) as *mut T
}

#[inline]
pub unsafe fn YAML_MALLOC(size: size_t) -> *mut yaml_char_t {
    yaml_malloc(size) as *mut yaml_char_t
}
