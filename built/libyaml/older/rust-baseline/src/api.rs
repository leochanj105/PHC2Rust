//! Rust transliteration of libyaml's api.c.
//!
//! This file implements the memory helpers, parser/emitter lifecycle
//! routines, event/document/node initializers, and the various setters
//! that together form the libyaml public API.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::externs::*;
use crate::yaml::*;
use crate::yaml_private::{self, *};
use crate::{
    BUFFER_DEL, BUFFER_INIT, DEQUEUE, POP, PUSH, QUEUE_DEL, QUEUE_EMPTY, QUEUE_INIT, STACK_DEL,
    STACK_EMPTY, STACK_INIT,
};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t, FILE};

/* --------------------------------------------------------------------- */
/*  A tiny "context" struct matching the anonymous `struct { yaml_error_type_t error; }`
 *  local variables used in the C sources.                                */
/* --------------------------------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
struct yaml_api_context_t {
    error: yaml_error_type_t,
}

/*
 * Get the library version.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_get_version_string() -> *const c_char {
    YAML_VERSION_STRING.as_ptr() as *const c_char
}

/*
 * Get the library version numbers.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_get_version(
    major: *mut c_int,
    minor: *mut c_int,
    patch: *mut c_int,
) {
    *major = YAML_VERSION_MAJOR;
    *minor = YAML_VERSION_MINOR;
    *patch = YAML_VERSION_PATCH;
}

/*
 * Allocate a dynamic memory block.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_malloc(size: size_t) -> *mut c_void {
    libc::malloc(if size != 0 { size } else { 1 })
}

/*
 * Reallocate a dynamic memory block.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    if !ptr.is_null() {
        libc::realloc(ptr, if size != 0 { size } else { 1 })
    } else {
        libc::malloc(if size != 0 { size } else { 1 })
    }
}

/*
 * Free a dynamic memory block.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        libc::free(ptr);
    }
}

/*
 * Duplicate a string.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_strdup(str_: *const yaml_char_t) -> *mut yaml_char_t {
    if str_.is_null() {
        return core::ptr::null_mut();
    }
    libc::strdup(str_ as *const c_char) as *mut yaml_char_t
}

/*
 * Extend a string.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_string_extend(
    start: *mut *mut yaml_char_t,
    pointer: *mut *mut yaml_char_t,
    end: *mut *mut yaml_char_t,
) -> c_int {
    let old_len = (*end).offset_from(*start) as size_t;
    let new_start =
        yaml_realloc(*start as *mut c_void, old_len.wrapping_mul(2)) as *mut yaml_char_t;

    if new_start.is_null() {
        return 0;
    }

    libc::memset(
        new_start.add(old_len) as *mut c_void,
        0,
        old_len,
    );

    *pointer = new_start.offset((*pointer).offset_from(*start));
    *end = new_start.add(old_len.wrapping_mul(2));
    *start = new_start;

    1
}

/*
 * Append a string B to a string A.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_string_join(
    a_start: *mut *mut yaml_char_t,
    a_pointer: *mut *mut yaml_char_t,
    a_end: *mut *mut yaml_char_t,
    b_start: *mut *mut yaml_char_t,
    b_pointer: *mut *mut yaml_char_t,
    b_end: *mut *mut yaml_char_t,
) -> c_int {
    let _ = b_end;
    if *b_start == *b_pointer {
        return 1;
    }

    while (*a_end).offset_from(*a_pointer) <= (*b_pointer).offset_from(*b_start) {
        if yaml_string_extend(a_start, a_pointer, a_end) == 0 {
            return 0;
        }
    }

    let copy_len = (*b_pointer).offset_from(*b_start) as size_t;
    libc::memcpy(
        *a_pointer as *mut c_void,
        *b_start as *const c_void,
        copy_len,
    );
    *a_pointer = (*a_pointer).add(copy_len);

    1
}

/*
 * Extend a stack.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_stack_extend(
    start: *mut *mut c_void,
    top: *mut *mut c_void,
    end: *mut *mut c_void,
) -> c_int {
    let cur_bytes = (*end as *mut c_char).offset_from(*start as *mut c_char);
    if cur_bytes >= (c_int::MAX / 2) as isize {
        return 0;
    }

    let new_start = yaml_realloc(*start, (cur_bytes as size_t).wrapping_mul(2));

    if new_start.is_null() {
        return 0;
    }

    let top_off = (*top as *mut c_char).offset_from(*start as *mut c_char);
    let end_off = (*end as *mut c_char).offset_from(*start as *mut c_char);
    *top = (new_start as *mut c_char).offset(top_off) as *mut c_void;
    *end = (new_start as *mut c_char).offset(end_off * 2) as *mut c_void;
    *start = new_start;

    1
}

/*
 * Extend or move a queue.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_queue_extend(
    start: *mut *mut c_void,
    head: *mut *mut c_void,
    tail: *mut *mut c_void,
    end: *mut *mut c_void,
) -> c_int {
    /* Check if we need to resize the queue. */

    if *start == *head && *tail == *end {
        let cur_bytes = (*end as *mut c_char).offset_from(*start as *mut c_char);
        let new_start = yaml_realloc(*start, (cur_bytes as size_t).wrapping_mul(2));

        if new_start.is_null() {
            return 0;
        }

        let head_off = (*head as *mut c_char).offset_from(*start as *mut c_char);
        let tail_off = (*tail as *mut c_char).offset_from(*start as *mut c_char);
        let end_off = (*end as *mut c_char).offset_from(*start as *mut c_char);
        *head = (new_start as *mut c_char).offset(head_off) as *mut c_void;
        *tail = (new_start as *mut c_char).offset(tail_off) as *mut c_void;
        *end = (new_start as *mut c_char).offset(end_off * 2) as *mut c_void;
        *start = new_start;
    }

    /* Check if we need to move the queue at the beginning of the buffer. */

    if *tail == *end {
        if *head != *tail {
            let move_len = (*tail as *mut c_char).offset_from(*head as *mut c_char);
            libc::memmove(*start, *head as *const c_void, move_len as size_t);
        }
        let tail_minus_head =
            (*tail as *mut c_char).offset_from(*head as *mut c_char);
        *tail = (*start as *mut c_char).offset(tail_minus_head) as *mut c_void;
        *head = *start;
    }

    1
}

/*
 * Create a new parser object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> c_int {
    assert!(!parser.is_null()); /* Non-NULL parser object expected. */

    libc::memset(parser as *mut c_void, 0, core::mem::size_of::<yaml_parser_t>());
    'error: loop {
        if BUFFER_INIT!(parser, (*parser).raw_buffer, INPUT_RAW_BUFFER_SIZE) == 0 {
            break 'error;
        }
        if BUFFER_INIT!(parser, (*parser).buffer, INPUT_BUFFER_SIZE) == 0 {
            break 'error;
        }
        if QUEUE_INIT!(parser, (*parser).tokens, INITIAL_QUEUE_SIZE, yaml_token_t) == 0 {
            break 'error;
        }
        if STACK_INIT!(parser, (*parser).indents, c_int) == 0 {
            break 'error;
        }
        if STACK_INIT!(parser, (*parser).simple_keys, yaml_simple_key_t) == 0 {
            break 'error;
        }
        if STACK_INIT!(parser, (*parser).states, yaml_parser_state_t) == 0 {
            break 'error;
        }
        if STACK_INIT!(parser, (*parser).marks, yaml_mark_t) == 0 {
            break 'error;
        }
        if STACK_INIT!(parser, (*parser).tag_directives, yaml_tag_directive_t) == 0 {
            break 'error;
        }

        return 1;
    }

    BUFFER_DEL!(parser, (*parser).raw_buffer);
    BUFFER_DEL!(parser, (*parser).buffer);
    QUEUE_DEL!(parser, (*parser).tokens);
    STACK_DEL!(parser, (*parser).indents);
    STACK_DEL!(parser, (*parser).simple_keys);
    STACK_DEL!(parser, (*parser).states);
    STACK_DEL!(parser, (*parser).marks);
    STACK_DEL!(parser, (*parser).tag_directives);

    0
}

/*
 * Destroy a parser object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_delete(parser: *mut yaml_parser_t) {
    assert!(!parser.is_null()); /* Non-NULL parser object expected. */

    BUFFER_DEL!(parser, (*parser).raw_buffer);
    BUFFER_DEL!(parser, (*parser).buffer);
    while !QUEUE_EMPTY!(parser, (*parser).tokens) {
        let mut __tok: yaml_token_t = DEQUEUE!(parser, (*parser).tokens);
        yaml_token_delete(&mut __tok);
    }
    QUEUE_DEL!(parser, (*parser).tokens);
    STACK_DEL!(parser, (*parser).indents);
    STACK_DEL!(parser, (*parser).simple_keys);
    STACK_DEL!(parser, (*parser).states);
    STACK_DEL!(parser, (*parser).marks);
    while !STACK_EMPTY!(parser, (*parser).tag_directives) {
        let tag_directive: yaml_tag_directive_t = POP!(parser, (*parser).tag_directives);
        yaml_free(tag_directive.handle as *mut c_void);
        yaml_free(tag_directive.prefix as *mut c_void);
    }
    STACK_DEL!(parser, (*parser).tag_directives);

    libc::memset(parser as *mut c_void, 0, core::mem::size_of::<yaml_parser_t>());
}

/*
 * String read handler.
 */

unsafe extern "C" fn yaml_string_read_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    mut size: size_t,
    size_read: *mut size_t,
) -> c_int {
    let parser = data as *mut yaml_parser_t;

    if (*parser).input.string.current == (*parser).input.string.end {
        *size_read = 0;
        return 1;
    }

    let remaining = (*parser).input.string.end.offset_from((*parser).input.string.current) as size_t;
    if size > remaining {
        size = remaining;
    }

    libc::memcpy(
        buffer as *mut c_void,
        (*parser).input.string.current as *const c_void,
        size,
    );
    (*parser).input.string.current = (*parser).input.string.current.add(size);
    *size_read = size;
    1
}

/*
 * File read handler.
 */

unsafe extern "C" fn yaml_file_read_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
    size_read: *mut size_t,
) -> c_int {
    let parser = data as *mut yaml_parser_t;

    *size_read = libc::fread(buffer as *mut c_void, 1, size, (*parser).input.file);
    (libc::ferror((*parser).input.file) == 0) as c_int
}

/*
 * Set a string input.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input_string(
    parser: *mut yaml_parser_t,
    input: *const c_uchar,
    size: size_t,
) {
    assert!(!parser.is_null()); /* Non-NULL parser object expected. */
    assert!((*parser).read_handler.is_none()); /* You can set the source only once. */
    assert!(!input.is_null()); /* Non-NULL input string expected. */

    (*parser).read_handler = Some(yaml_string_read_handler);
    (*parser).read_handler_data = parser as *mut c_void;

    (*parser).input.string.start = input;
    (*parser).input.string.current = input;
    (*parser).input.string.end = input.add(size);
}

/*
 * Set a file input.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input_file(
    parser: *mut yaml_parser_t,
    file: *mut FILE,
) {
    assert!(!parser.is_null()); /* Non-NULL parser object expected. */
    assert!((*parser).read_handler.is_none()); /* You can set the source only once. */
    assert!(!file.is_null()); /* Non-NULL file object expected. */

    (*parser).read_handler = Some(yaml_file_read_handler);
    (*parser).read_handler_data = parser as *mut c_void;

    (*parser).input.file = file;
}

/*
 * Set a generic input.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input(
    parser: *mut yaml_parser_t,
    handler: yaml_read_handler_t,
    data: *mut c_void,
) {
    assert!(!parser.is_null()); /* Non-NULL parser object expected. */
    assert!((*parser).read_handler.is_none()); /* You can set the source only once. */
    /* Non-NULL read handler expected. */

    (*parser).read_handler = Some(handler);
    (*parser).read_handler_data = data;
}

/*
 * Set the source encoding.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_encoding(
    parser: *mut yaml_parser_t,
    encoding: yaml_encoding_t,
) {
    assert!(!parser.is_null()); /* Non-NULL parser object expected. */
    assert!((*parser).encoding == YAML_ANY_ENCODING); /* Encoding is already set or detected. */

    (*parser).encoding = encoding;
}

/*
 * Create a new emitter object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> c_int {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    libc::memset(
        emitter as *mut c_void,
        0,
        core::mem::size_of::<yaml_emitter_t>(),
    );
    'error: loop {
        if BUFFER_INIT!(emitter, (*emitter).buffer, OUTPUT_BUFFER_SIZE) == 0 {
            break 'error;
        }
        if BUFFER_INIT!(emitter, (*emitter).raw_buffer, OUTPUT_RAW_BUFFER_SIZE) == 0 {
            break 'error;
        }
        if STACK_INIT!(emitter, (*emitter).states, yaml_emitter_state_t) == 0 {
            break 'error;
        }
        if QUEUE_INIT!(emitter, (*emitter).events, INITIAL_QUEUE_SIZE, yaml_event_t) == 0 {
            break 'error;
        }
        if STACK_INIT!(emitter, (*emitter).indents, c_int) == 0 {
            break 'error;
        }
        if STACK_INIT!(emitter, (*emitter).tag_directives, yaml_tag_directive_t) == 0 {
            break 'error;
        }

        return 1;
    }

    BUFFER_DEL!(emitter, (*emitter).buffer);
    BUFFER_DEL!(emitter, (*emitter).raw_buffer);
    STACK_DEL!(emitter, (*emitter).states);
    QUEUE_DEL!(emitter, (*emitter).events);
    STACK_DEL!(emitter, (*emitter).indents);
    STACK_DEL!(emitter, (*emitter).tag_directives);

    0
}

/*
 * Destroy an emitter object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_delete(emitter: *mut yaml_emitter_t) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    BUFFER_DEL!(emitter, (*emitter).buffer);
    BUFFER_DEL!(emitter, (*emitter).raw_buffer);
    STACK_DEL!(emitter, (*emitter).states);
    while !QUEUE_EMPTY!(emitter, (*emitter).events) {
        let mut __ev: yaml_event_t = DEQUEUE!(emitter, (*emitter).events);
        yaml_event_delete(&mut __ev);
    }
    QUEUE_DEL!(emitter, (*emitter).events);
    STACK_DEL!(emitter, (*emitter).indents);
    while !STACK_EMPTY!(empty, (*emitter).tag_directives) {
        let tag_directive: yaml_tag_directive_t = POP!(emitter, (*emitter).tag_directives);
        yaml_free(tag_directive.handle as *mut c_void);
        yaml_free(tag_directive.prefix as *mut c_void);
    }
    STACK_DEL!(emitter, (*emitter).tag_directives);
    yaml_free((*emitter).anchors as *mut c_void);

    libc::memset(
        emitter as *mut c_void,
        0,
        core::mem::size_of::<yaml_emitter_t>(),
    );
}

/*
 * String write handler.
 */

unsafe extern "C" fn yaml_string_write_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
) -> c_int {
    let emitter = data as *mut yaml_emitter_t;

    if (*emitter).output.string.size - *(*emitter).output.string.size_written < size {
        libc::memcpy(
            (*emitter)
                .output
                .string
                .buffer
                .add(*(*emitter).output.string.size_written)
                as *mut c_void,
            buffer as *const c_void,
            (*emitter).output.string.size - *(*emitter).output.string.size_written,
        );
        *(*emitter).output.string.size_written = (*emitter).output.string.size;
        return 0;
    }

    libc::memcpy(
        (*emitter)
            .output
            .string
            .buffer
            .add(*(*emitter).output.string.size_written) as *mut c_void,
        buffer as *const c_void,
        size,
    );
    *(*emitter).output.string.size_written += size;
    1
}

/*
 * File write handler.
 */

unsafe extern "C" fn yaml_file_write_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
) -> c_int {
    let emitter = data as *mut yaml_emitter_t;

    (libc::fwrite(
        buffer as *const c_void,
        1,
        size,
        (*emitter).output.file,
    ) == size) as c_int
}

/*
 * Set a string output.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output_string(
    emitter: *mut yaml_emitter_t,
    output: *mut c_uchar,
    size: size_t,
    size_written: *mut size_t,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */
    assert!((*emitter).write_handler.is_none()); /* You can set the output only once. */
    assert!(!output.is_null()); /* Non-NULL output string expected. */

    (*emitter).write_handler = Some(yaml_string_write_handler);
    (*emitter).write_handler_data = emitter as *mut c_void;

    (*emitter).output.string.buffer = output;
    (*emitter).output.string.size = size;
    (*emitter).output.string.size_written = size_written;
    *size_written = 0;
}

/*
 * Set a file output.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output_file(
    emitter: *mut yaml_emitter_t,
    file: *mut FILE,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */
    assert!((*emitter).write_handler.is_none()); /* You can set the output only once. */
    assert!(!file.is_null()); /* Non-NULL file object expected. */

    (*emitter).write_handler = Some(yaml_file_write_handler);
    (*emitter).write_handler_data = emitter as *mut c_void;

    (*emitter).output.file = file;
}

/*
 * Set a generic output handler.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output(
    emitter: *mut yaml_emitter_t,
    handler: yaml_write_handler_t,
    data: *mut c_void,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */
    assert!((*emitter).write_handler.is_none()); /* You can set the output only once. */
    /* Non-NULL handler object expected. */

    (*emitter).write_handler = Some(handler);
    (*emitter).write_handler_data = data;
}

/*
 * Set the output encoding.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_encoding(
    emitter: *mut yaml_emitter_t,
    encoding: yaml_encoding_t,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */
    assert!((*emitter).encoding == YAML_ANY_ENCODING); /* You can set encoding only once. */

    (*emitter).encoding = encoding;
}

/*
 * Set the canonical output style.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_canonical(
    emitter: *mut yaml_emitter_t,
    canonical: c_int,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    (*emitter).canonical = (canonical != 0) as c_int;
}

/*
 * Set the indentation increment.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_indent(emitter: *mut yaml_emitter_t, indent: c_int) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    (*emitter).best_indent = if 1 < indent && indent < 10 { indent } else { 2 };
}

/*
 * Set the preferred line width.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_width(emitter: *mut yaml_emitter_t, width: c_int) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    (*emitter).best_width = if width >= 0 { width } else { -1 };
}

/*
 * Set if unescaped non-ASCII characters are allowed.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_unicode(
    emitter: *mut yaml_emitter_t,
    unicode: c_int,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    (*emitter).unicode = (unicode != 0) as c_int;
}

/*
 * Set the preferred line break character.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_break(
    emitter: *mut yaml_emitter_t,
    line_break: yaml_break_t,
) {
    assert!(!emitter.is_null()); /* Non-NULL emitter object expected. */

    (*emitter).line_break = line_break;
}

/*
 * Destroy a token object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_token_delete(token: *mut yaml_token_t) {
    assert!(!token.is_null()); /* Non-NULL token object expected. */

    match (*token).type_ {
        YAML_TAG_DIRECTIVE_TOKEN => {
            yaml_free((*token).data.tag_directive.handle as *mut c_void);
            yaml_free((*token).data.tag_directive.prefix as *mut c_void);
        }
        YAML_ALIAS_TOKEN => {
            yaml_free((*token).data.alias.value as *mut c_void);
        }
        YAML_ANCHOR_TOKEN => {
            yaml_free((*token).data.anchor.value as *mut c_void);
        }
        YAML_TAG_TOKEN => {
            yaml_free((*token).data.tag.handle as *mut c_void);
            yaml_free((*token).data.tag.suffix as *mut c_void);
        }
        YAML_SCALAR_TOKEN => {
            yaml_free((*token).data.scalar.value as *mut c_void);
        }
        _ => {}
    }

    libc::memset(token as *mut c_void, 0, core::mem::size_of::<yaml_token_t>());
}

/*
 * Check if a string is a valid UTF-8 sequence.
 *
 * Check 'reader.c' for more details on UTF-8 encoding.
 */

unsafe fn yaml_check_utf8(start: *const yaml_char_t, length: size_t) -> c_int {
    let end = start.add(length);
    let mut pointer = start;

    while pointer < end {
        let mut octet: c_uchar;
        let width: c_uint;
        let mut value: c_uint;
        let mut k: size_t;

        octet = *pointer.add(0);
        width = if (octet & 0x80) == 0x00 {
            1
        } else if (octet & 0xE0) == 0xC0 {
            2
        } else if (octet & 0xF0) == 0xE0 {
            3
        } else if (octet & 0xF8) == 0xF0 {
            4
        } else {
            0
        };
        value = if (octet & 0x80) == 0x00 {
            (octet & 0x7F) as c_uint
        } else if (octet & 0xE0) == 0xC0 {
            (octet & 0x1F) as c_uint
        } else if (octet & 0xF0) == 0xE0 {
            (octet & 0x0F) as c_uint
        } else if (octet & 0xF8) == 0xF0 {
            (octet & 0x07) as c_uint
        } else {
            0
        };
        if width == 0 {
            return 0;
        }
        if pointer.add(width as usize) > end {
            return 0;
        }
        k = 1;
        while k < width as size_t {
            octet = *pointer.add(k as usize);
            if (octet & 0xC0) != 0x80 {
                return 0;
            }
            value = (value << 6) + (octet & 0x3F) as c_uint;
            k += 1;
        }
        if !((width == 1)
            || (width == 2 && value >= 0x80)
            || (width == 3 && value >= 0x800)
            || (width == 4 && value >= 0x10000))
        {
            return 0;
        }

        pointer = pointer.add(width as usize);
    }

    1
}

/*
 * Create STREAM-START.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_stream_start_event_initialize(
    event: *mut yaml_event_t,
    encoding: yaml_encoding_t,
) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };

    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    EVENT_INIT(event, YAML_STREAM_START_EVENT, mark, mark);
    (*event).data.stream_start.encoding = encoding;

    1
}

/*
 * Create STREAM-END.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };

    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    EVENT_INIT(event, YAML_STREAM_END_EVENT, mark, mark);

    1
}

/*
 * Create DOCUMENT-START.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_start_event_initialize(
    event: *mut yaml_event_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    implicit: c_int,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut version_directive_copy: *mut yaml_version_directive_t = core::ptr::null_mut();
    let mut tag_directives_copy = yaml_stack_tag_directive_t {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
        top: core::ptr::null_mut(),
    };
    let mut value = yaml_tag_directive_t {
        handle: core::ptr::null_mut(),
        prefix: core::ptr::null_mut(),
    };

    assert!(!event.is_null()); /* Non-NULL event object is expected. */
    assert!(
        (!tag_directives_start.is_null() && !tag_directives_end.is_null())
            || (tag_directives_start == tag_directives_end)
    );
    /* Valid tag directives are expected. */

    'error: loop {
        if !version_directive.is_null() {
            version_directive_copy = YAML_MALLOC_STATIC::<yaml_version_directive_t>();
            if version_directive_copy.is_null() {
                break 'error;
            }
            (*version_directive_copy).major = (*version_directive).major;
            (*version_directive_copy).minor = (*version_directive).minor;
        }

        if tag_directives_start != tag_directives_end {
            let mut tag_directive: *mut yaml_tag_directive_t;
            if STACK_INIT!(&mut context, tag_directives_copy, yaml_tag_directive_t) == 0 {
                break 'error;
            }
            tag_directive = tag_directives_start;
            while tag_directive != tag_directives_end {
                assert!(!(*tag_directive).handle.is_null());
                assert!(!(*tag_directive).prefix.is_null());
                if yaml_check_utf8(
                    (*tag_directive).handle,
                    libc::strlen((*tag_directive).handle as *const c_char),
                ) == 0
                {
                    break 'error;
                }
                if yaml_check_utf8(
                    (*tag_directive).prefix,
                    libc::strlen((*tag_directive).prefix as *const c_char),
                ) == 0
                {
                    break 'error;
                }
                value.handle = yaml_strdup((*tag_directive).handle);
                value.prefix = yaml_strdup((*tag_directive).prefix);
                if value.handle.is_null() || value.prefix.is_null() {
                    break 'error;
                }
                if PUSH!(&mut context, tag_directives_copy, value) == 0 {
                    break 'error;
                }
                value.handle = core::ptr::null_mut();
                value.prefix = core::ptr::null_mut();
                tag_directive = tag_directive.add(1);
            }
        }

        EVENT_INIT(event, YAML_DOCUMENT_START_EVENT, mark, mark);
        (*event).data.document_start.version_directive = version_directive_copy;
        (*event).data.document_start.tag_directives.start = tag_directives_copy.start;
        (*event).data.document_start.tag_directives.end = tag_directives_copy.top;
        (*event).data.document_start.implicit = implicit;

        return 1;
    }

    yaml_free(version_directive_copy as *mut c_void);
    while !STACK_EMPTY!(&mut context, tag_directives_copy) {
        let value2: yaml_tag_directive_t = POP!(&mut context, tag_directives_copy);
        yaml_free(value2.handle as *mut c_void);
        yaml_free(value2.prefix as *mut c_void);
    }
    STACK_DEL!(&mut context, tag_directives_copy);
    yaml_free(value.handle as *mut c_void);
    yaml_free(value.prefix as *mut c_void);

    0
}

/*
 * Create DOCUMENT-END.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_end_event_initialize(
    event: *mut yaml_event_t,
    implicit: c_int,
) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };

    assert!(!event.is_null()); /* Non-NULL emitter object is expected. */

    EVENT_INIT(event, YAML_DOCUMENT_END_EVENT, mark, mark);
    (*event).data.document_end.implicit = implicit;

    1
}

/*
 * Create ALIAS.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_alias_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let anchor_copy: *mut yaml_char_t;

    assert!(!event.is_null()); /* Non-NULL event object is expected. */
    assert!(!anchor.is_null()); /* Non-NULL anchor is expected. */

    if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 {
        return 0;
    }

    anchor_copy = yaml_strdup(anchor);
    if anchor_copy.is_null() {
        return 0;
    }

    EVENT_INIT(event, YAML_ALIAS_EVENT, mark, mark);
    (*event).data.alias.anchor = anchor_copy;

    1
}

/*
 * Create SCALAR.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_scalar_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    value: *const yaml_char_t,
    mut length: c_int,
    plain_implicit: c_int,
    quoted_implicit: c_int,
    style: yaml_scalar_style_t,
) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut anchor_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut tag_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut value_copy: *mut yaml_char_t = core::ptr::null_mut();

    assert!(!event.is_null()); /* Non-NULL event object is expected. */
    assert!(!value.is_null()); /* Non-NULL anchor is expected. */

    'error: loop {
        if !anchor.is_null() {
            if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 {
                break 'error;
            }
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                break 'error;
            }
        }

        if !tag.is_null() {
            if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
                break 'error;
            }
            tag_copy = yaml_strdup(tag);
            if tag_copy.is_null() {
                break 'error;
            }
        }

        if length < 0 {
            length = libc::strlen(value as *const c_char) as c_int;
        }

        if yaml_check_utf8(value, length as size_t) == 0 {
            break 'error;
        }
        value_copy = YAML_MALLOC((length + 1) as size_t);
        if value_copy.is_null() {
            break 'error;
        }
        libc::memcpy(
            value_copy as *mut c_void,
            value as *const c_void,
            length as size_t,
        );
        *value_copy.add(length as usize) = b'\0';

        EVENT_INIT(event, YAML_SCALAR_EVENT, mark, mark);
        (*event).data.scalar.anchor = anchor_copy;
        (*event).data.scalar.tag = tag_copy;
        (*event).data.scalar.value = value_copy;
        (*event).data.scalar.length = length as size_t;
        (*event).data.scalar.plain_implicit = plain_implicit;
        (*event).data.scalar.quoted_implicit = quoted_implicit;
        (*event).data.scalar.style = style;

        return 1;
    }

    yaml_free(anchor_copy as *mut c_void);
    yaml_free(tag_copy as *mut c_void);
    yaml_free(value_copy as *mut c_void);

    0
}

/*
 * Create SEQUENCE-START.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_sequence_start_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: c_int,
    style: yaml_sequence_style_t,
) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut anchor_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut tag_copy: *mut yaml_char_t = core::ptr::null_mut();

    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    'error: loop {
        if !anchor.is_null() {
            if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 {
                break 'error;
            }
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                break 'error;
            }
        }

        if !tag.is_null() {
            if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
                break 'error;
            }
            tag_copy = yaml_strdup(tag);
            if tag_copy.is_null() {
                break 'error;
            }
        }

        EVENT_INIT(event, YAML_SEQUENCE_START_EVENT, mark, mark);
        (*event).data.sequence_start.anchor = anchor_copy;
        (*event).data.sequence_start.tag = tag_copy;
        (*event).data.sequence_start.implicit = implicit;
        (*event).data.sequence_start.style = style;

        return 1;
    }

    yaml_free(anchor_copy as *mut c_void);
    yaml_free(tag_copy as *mut c_void);

    0
}

/*
 * Create SEQUENCE-END.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };

    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    EVENT_INIT(event, YAML_SEQUENCE_END_EVENT, mark, mark);

    1
}

/*
 * Create MAPPING-START.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_mapping_start_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: c_int,
    style: yaml_mapping_style_t,
) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut anchor_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut tag_copy: *mut yaml_char_t = core::ptr::null_mut();

    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    'error: loop {
        if !anchor.is_null() {
            if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 {
                break 'error;
            }
            anchor_copy = yaml_strdup(anchor);
            if anchor_copy.is_null() {
                break 'error;
            }
        }

        if !tag.is_null() {
            if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
                break 'error;
            }
            tag_copy = yaml_strdup(tag);
            if tag_copy.is_null() {
                break 'error;
            }
        }

        EVENT_INIT(event, YAML_MAPPING_START_EVENT, mark, mark);
        (*event).data.mapping_start.anchor = anchor_copy;
        (*event).data.mapping_start.tag = tag_copy;
        (*event).data.mapping_start.implicit = implicit;
        (*event).data.mapping_start.style = style;

        return 1;
    }

    yaml_free(anchor_copy as *mut c_void);
    yaml_free(tag_copy as *mut c_void);

    0
}

/*
 * Create MAPPING-END.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> c_int {
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };

    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    EVENT_INIT(event, YAML_MAPPING_END_EVENT, mark, mark);

    1
}

/*
 * Destroy an event object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_event_delete(event: *mut yaml_event_t) {
    let mut tag_directive: *mut yaml_tag_directive_t;

    assert!(!event.is_null()); /* Non-NULL event object expected. */

    match (*event).type_ {
        YAML_DOCUMENT_START_EVENT => {
            yaml_free((*event).data.document_start.version_directive as *mut c_void);
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                yaml_free((*tag_directive).handle as *mut c_void);
                yaml_free((*tag_directive).prefix as *mut c_void);
                tag_directive = tag_directive.add(1);
            }
            yaml_free((*event).data.document_start.tag_directives.start as *mut c_void);
        }
        YAML_ALIAS_EVENT => {
            yaml_free((*event).data.alias.anchor as *mut c_void);
        }
        YAML_SCALAR_EVENT => {
            yaml_free((*event).data.scalar.anchor as *mut c_void);
            yaml_free((*event).data.scalar.tag as *mut c_void);
            yaml_free((*event).data.scalar.value as *mut c_void);
        }
        YAML_SEQUENCE_START_EVENT => {
            yaml_free((*event).data.sequence_start.anchor as *mut c_void);
            yaml_free((*event).data.sequence_start.tag as *mut c_void);
        }
        YAML_MAPPING_START_EVENT => {
            yaml_free((*event).data.mapping_start.anchor as *mut c_void);
            yaml_free((*event).data.mapping_start.tag as *mut c_void);
        }
        _ => {}
    }

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
}

/*
 * Create a document object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_initialize(
    document: *mut yaml_document_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    start_implicit: c_int,
    end_implicit: c_int,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };
    let mut nodes = yaml_document_nodes_s {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
        top: core::ptr::null_mut(),
    };
    let mut version_directive_copy: *mut yaml_version_directive_t = core::ptr::null_mut();
    let mut tag_directives_copy = yaml_stack_tag_directive_t {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
        top: core::ptr::null_mut(),
    };
    let mut value = yaml_tag_directive_t {
        handle: core::ptr::null_mut(),
        prefix: core::ptr::null_mut(),
    };
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };

    assert!(!document.is_null()); /* Non-NULL document object is expected. */
    assert!(
        (!tag_directives_start.is_null() && !tag_directives_end.is_null())
            || (tag_directives_start == tag_directives_end)
    );
    /* Valid tag directives are expected. */

    'error: loop {
        if STACK_INIT!(&mut context, nodes, yaml_node_t) == 0 {
            break 'error;
        }

        if !version_directive.is_null() {
            version_directive_copy = YAML_MALLOC_STATIC::<yaml_version_directive_t>();
            if version_directive_copy.is_null() {
                break 'error;
            }
            (*version_directive_copy).major = (*version_directive).major;
            (*version_directive_copy).minor = (*version_directive).minor;
        }

        if tag_directives_start != tag_directives_end {
            let mut tag_directive: *mut yaml_tag_directive_t;
            if STACK_INIT!(&mut context, tag_directives_copy, yaml_tag_directive_t) == 0 {
                break 'error;
            }
            tag_directive = tag_directives_start;
            while tag_directive != tag_directives_end {
                assert!(!(*tag_directive).handle.is_null());
                assert!(!(*tag_directive).prefix.is_null());
                if yaml_check_utf8(
                    (*tag_directive).handle,
                    libc::strlen((*tag_directive).handle as *const c_char),
                ) == 0
                {
                    break 'error;
                }
                if yaml_check_utf8(
                    (*tag_directive).prefix,
                    libc::strlen((*tag_directive).prefix as *const c_char),
                ) == 0
                {
                    break 'error;
                }
                value.handle = yaml_strdup((*tag_directive).handle);
                value.prefix = yaml_strdup((*tag_directive).prefix);
                if value.handle.is_null() || value.prefix.is_null() {
                    break 'error;
                }
                if PUSH!(&mut context, tag_directives_copy, value) == 0 {
                    break 'error;
                }
                value.handle = core::ptr::null_mut();
                value.prefix = core::ptr::null_mut();
                tag_directive = tag_directive.add(1);
            }
        }

        libc::memset(
            document as *mut c_void,
            0,
            core::mem::size_of::<yaml_document_t>(),
        );
        (*document).nodes.start = nodes.start;
        (*document).nodes.end = nodes.end;
        (*document).nodes.top = nodes.start;
        (*document).version_directive = version_directive_copy;
        (*document).tag_directives.start = tag_directives_copy.start;
        (*document).tag_directives.end = tag_directives_copy.top;
        (*document).start_implicit = start_implicit;
        (*document).end_implicit = end_implicit;
        (*document).start_mark = mark;
        (*document).end_mark = mark;

        return 1;
    }

    STACK_DEL!(&mut context, nodes);
    yaml_free(version_directive_copy as *mut c_void);
    while !STACK_EMPTY!(&mut context, tag_directives_copy) {
        let value2: yaml_tag_directive_t = POP!(&mut context, tag_directives_copy);
        yaml_free(value2.handle as *mut c_void);
        yaml_free(value2.prefix as *mut c_void);
    }
    STACK_DEL!(&mut context, tag_directives_copy);
    yaml_free(value.handle as *mut c_void);
    yaml_free(value.prefix as *mut c_void);

    0
}

/*
 * Destroy a document object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_delete(document: *mut yaml_document_t) {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };
    let mut tag_directive: *mut yaml_tag_directive_t;

    assert!(!document.is_null()); /* Non-NULL document object is expected. */

    while !STACK_EMPTY!(&mut context, (*document).nodes) {
        let node: yaml_node_t = POP!(&mut context, (*document).nodes);
        yaml_free(node.tag as *mut c_void);
        match node.type_ {
            YAML_SCALAR_NODE => {
                yaml_free(node.data.scalar.value as *mut c_void);
            }
            YAML_SEQUENCE_NODE => {
                let mut items = node.data.sequence.items;
                STACK_DEL!(&mut context, items);
            }
            YAML_MAPPING_NODE => {
                let mut pairs = node.data.mapping.pairs;
                STACK_DEL!(&mut context, pairs);
            }
            _ => {
                assert!(false); /* Should not happen. */
            }
        }
    }
    STACK_DEL!(&mut context, (*document).nodes);

    yaml_free((*document).version_directive as *mut c_void);
    tag_directive = (*document).tag_directives.start;
    while tag_directive != (*document).tag_directives.end {
        yaml_free((*tag_directive).handle as *mut c_void);
        yaml_free((*tag_directive).prefix as *mut c_void);
        tag_directive = tag_directive.add(1);
    }
    yaml_free((*document).tag_directives.start as *mut c_void);

    libc::memset(
        document as *mut c_void,
        0,
        core::mem::size_of::<yaml_document_t>(),
    );
}

/**
 * Get a document node.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_get_node(
    document: *mut yaml_document_t,
    index: c_int,
) -> *mut yaml_node_t {
    assert!(!document.is_null()); /* Non-NULL document object is expected. */

    if index > 0 && (*document).nodes.start.offset(index as isize) <= (*document).nodes.top {
        return (*document).nodes.start.offset((index - 1) as isize);
    }
    core::ptr::null_mut()
}

/**
 * Get the root object.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_get_root_node(
    document: *mut yaml_document_t,
) -> *mut yaml_node_t {
    assert!(!document.is_null()); /* Non-NULL document object is expected. */

    if (*document).nodes.top != (*document).nodes.start {
        return (*document).nodes.start;
    }
    core::ptr::null_mut()
}

/*
 * Add a scalar node to a document.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_scalar(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    value: *const yaml_char_t,
    mut length: c_int,
    style: yaml_scalar_style_t,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut tag_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut value_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut node: yaml_node_t = core::mem::zeroed();

    assert!(!document.is_null()); /* Non-NULL document object is expected. */
    assert!(!value.is_null()); /* Non-NULL value is expected. */

    'error: loop {
        if tag.is_null() {
            tag = YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const yaml_char_t;
        }

        if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
            break 'error;
        }
        tag_copy = yaml_strdup(tag);
        if tag_copy.is_null() {
            break 'error;
        }

        if length < 0 {
            length = libc::strlen(value as *const c_char) as c_int;
        }

        if yaml_check_utf8(value, length as size_t) == 0 {
            break 'error;
        }
        value_copy = YAML_MALLOC((length + 1) as size_t);
        if value_copy.is_null() {
            break 'error;
        }
        libc::memcpy(
            value_copy as *mut c_void,
            value as *const c_void,
            length as size_t,
        );
        *value_copy.add(length as usize) = b'\0';

        NODE_INIT(&mut node, YAML_SCALAR_NODE, tag_copy, mark, mark);
        node.data.scalar.value = value_copy;
        node.data.scalar.length = length as size_t;
        node.data.scalar.style = style;
        if PUSH!(&mut context, (*document).nodes, node) == 0 {
            break 'error;
        }

        return ((*document).nodes.top.offset_from((*document).nodes.start)) as c_int;
    }

    yaml_free(tag_copy as *mut c_void);
    yaml_free(value_copy as *mut c_void);

    0
}

/*
 * Add a sequence node to a document.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_sequence(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    style: yaml_sequence_style_t,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut tag_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut items = yaml_node_sequence_items_s {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
        top: core::ptr::null_mut(),
    };
    let mut node: yaml_node_t = core::mem::zeroed();

    assert!(!document.is_null()); /* Non-NULL document object is expected. */

    'error: loop {
        if tag.is_null() {
            tag = YAML_DEFAULT_SEQUENCE_TAG.as_ptr() as *const yaml_char_t;
        }

        if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
            break 'error;
        }
        tag_copy = yaml_strdup(tag);
        if tag_copy.is_null() {
            break 'error;
        }

        if STACK_INIT!(&mut context, items, yaml_node_item_t) == 0 {
            break 'error;
        }

        NODE_INIT(&mut node, YAML_SEQUENCE_NODE, tag_copy, mark, mark);
        node.data.sequence.items.start = items.start;
        node.data.sequence.items.end = items.end;
        node.data.sequence.items.top = items.start;
        node.data.sequence.style = style;
        if PUSH!(&mut context, (*document).nodes, node) == 0 {
            break 'error;
        }

        return ((*document).nodes.top.offset_from((*document).nodes.start)) as c_int;
    }

    STACK_DEL!(&mut context, items);
    yaml_free(tag_copy as *mut c_void);

    0
}

/*
 * Add a mapping node to a document.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_mapping(
    document: *mut yaml_document_t,
    mut tag: *const yaml_char_t,
    style: yaml_mapping_style_t,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };
    let mark = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut tag_copy: *mut yaml_char_t = core::ptr::null_mut();
    let mut pairs = yaml_node_mapping_pairs_s {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
        top: core::ptr::null_mut(),
    };
    let mut node: yaml_node_t = core::mem::zeroed();

    assert!(!document.is_null()); /* Non-NULL document object is expected. */

    'error: loop {
        if tag.is_null() {
            tag = YAML_DEFAULT_MAPPING_TAG.as_ptr() as *const yaml_char_t;
        }

        if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
            break 'error;
        }
        tag_copy = yaml_strdup(tag);
        if tag_copy.is_null() {
            break 'error;
        }

        if STACK_INIT!(&mut context, pairs, yaml_node_pair_t) == 0 {
            break 'error;
        }

        NODE_INIT(&mut node, YAML_MAPPING_NODE, tag_copy, mark, mark);
        node.data.mapping.pairs.start = pairs.start;
        node.data.mapping.pairs.end = pairs.end;
        node.data.mapping.pairs.top = pairs.start;
        node.data.mapping.style = style;
        if PUSH!(&mut context, (*document).nodes, node) == 0 {
            break 'error;
        }

        return ((*document).nodes.top.offset_from((*document).nodes.start)) as c_int;
    }

    STACK_DEL!(&mut context, pairs);
    yaml_free(tag_copy as *mut c_void);

    0
}

/*
 * Append an item to a sequence node.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_append_sequence_item(
    document: *mut yaml_document_t,
    sequence: c_int,
    item: c_int,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };

    assert!(!document.is_null()); /* Non-NULL document is required. */
    assert!(
        sequence > 0
            && (*document).nodes.start.offset(sequence as isize) <= (*document).nodes.top
    );
    /* Valid sequence id is required. */
    assert!(
        (*(*document).nodes.start.offset((sequence - 1) as isize)).type_ == YAML_SEQUENCE_NODE
    );
    /* A sequence node is required. */
    assert!(
        item > 0 && (*document).nodes.start.offset(item as isize) <= (*document).nodes.top
    );
    /* Valid item id is required. */

    if PUSH!(
        &mut context,
        (*(*document).nodes.start.offset((sequence - 1) as isize))
            .data
            .sequence
            .items,
        item
    ) == 0
    {
        return 0;
    }

    1
}

/*
 * Append a pair of a key and a value to a mapping node.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_document_append_mapping_pair(
    document: *mut yaml_document_t,
    mapping: c_int,
    key: c_int,
    value: c_int,
) -> c_int {
    let mut context = yaml_api_context_t { error: YAML_NO_ERROR };

    let mut pair: yaml_node_pair_t = yaml_node_pair_t { key: 0, value: 0 };

    assert!(!document.is_null()); /* Non-NULL document is required. */
    assert!(
        mapping > 0
            && (*document).nodes.start.offset(mapping as isize) <= (*document).nodes.top
    );
    /* Valid mapping id is required. */
    assert!(
        (*(*document).nodes.start.offset((mapping - 1) as isize)).type_ == YAML_MAPPING_NODE
    );
    /* A mapping node is required. */
    assert!(key > 0 && (*document).nodes.start.offset(key as isize) <= (*document).nodes.top);
    /* Valid key id is required. */
    assert!(
        value > 0 && (*document).nodes.start.offset(value as isize) <= (*document).nodes.top
    );
    /* Valid value id is required. */

    pair.key = key;
    pair.value = value;

    if PUSH!(
        &mut context,
        (*(*document).nodes.start.offset((mapping - 1) as isize))
            .data
            .mapping
            .pairs,
        pair
    ) == 0
    {
        return 0;
    }

    1
}
