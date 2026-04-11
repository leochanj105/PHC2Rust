#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_parens,
)]

use crate::externs::*;
use crate::yaml::*;
use crate::yaml_private::*;
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};

/*
 * Set the writer error and return 0.
 */

pub(crate) unsafe fn yaml_emitter_set_writer_error(
    emitter: *mut yaml_emitter_t,
    problem: *const c_char,
) -> c_int {
    (*emitter).error = YAML_WRITER_ERROR;
    (*emitter).problem = problem;

    return 0;
}

/*
 * Flush the output buffer.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> c_int {
    let low: c_int;
    let high: c_int;

    /* assert(emitter);    Non-NULL emitter object is expected. */
    /* assert(emitter->write_handler); Write handler must be set. */
    /* assert(emitter->encoding);  Output encoding must be set. */

    (*emitter).buffer.last = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.start;

    /* Check if the buffer is empty. */

    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1;
    }

    /* If the output encoding is UTF-8, we don't need to recode the buffer. */

    if (*emitter).encoding == YAML_UTF8_ENCODING {
        if ((*emitter).write_handler.unwrap())(
            (*emitter).write_handler_data,
            (*emitter).buffer.start,
            (*emitter).buffer.last.offset_from((*emitter).buffer.start) as size_t,
        ) != 0
        {
            (*emitter).buffer.last = (*emitter).buffer.start;
            (*emitter).buffer.pointer = (*emitter).buffer.start;
            return 1;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const c_char,
            );
        }
    }

    /* Recode the buffer into the raw buffer. */

    low = if (*emitter).encoding == YAML_UTF16LE_ENCODING {
        0
    } else {
        1
    };
    high = if (*emitter).encoding == YAML_UTF16LE_ENCODING {
        1
    } else {
        0
    };

    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: c_uchar;
        let width: c_uint;
        let mut value: c_uint;
        let mut k: size_t;

        /*
         * See the "reader.c" code for more details on UTF-8 encoding.  Note
         * that we assume that the buffer contains a valid UTF-8 sequence.
         */

        /* Read the next UTF-8 character. */

        octet = *(*emitter).buffer.pointer.offset(0);

        width = if (octet as c_int & 0x80) == 0x00 {
            1
        } else if (octet as c_int & 0xE0) == 0xC0 {
            2
        } else if (octet as c_int & 0xF0) == 0xE0 {
            3
        } else if (octet as c_int & 0xF8) == 0xF0 {
            4
        } else {
            0
        };

        value = if (octet as c_int & 0x80) == 0x00 {
            (octet as c_int & 0x7F) as c_uint
        } else if (octet as c_int & 0xE0) == 0xC0 {
            (octet as c_int & 0x1F) as c_uint
        } else if (octet as c_int & 0xF0) == 0xE0 {
            (octet as c_int & 0x0F) as c_uint
        } else if (octet as c_int & 0xF8) == 0xF0 {
            (octet as c_int & 0x07) as c_uint
        } else {
            0
        };

        k = 1;
        while k < width as size_t {
            octet = *(*emitter).buffer.pointer.offset(k as isize);
            value = (value << 6) + (octet as c_uint & 0x3F);
            k = k.wrapping_add(1);
        }

        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(width as isize);

        /* Write the character. */

        if value < 0x10000 {
            *(*emitter).raw_buffer.last.offset(high as isize) = (value >> 8) as c_uchar;
            *(*emitter).raw_buffer.last.offset(low as isize) = (value & 0xFF) as c_uchar;

            (*emitter).raw_buffer.last = (*emitter).raw_buffer.last.offset(2);
        } else {
            /* Write the character using a surrogate pair (check "reader.c"). */

            value -= 0x10000;
            *(*emitter).raw_buffer.last.offset(high as isize) =
                (0xD8 + (value >> 18)) as c_uchar;
            *(*emitter).raw_buffer.last.offset(low as isize) =
                ((value >> 10) & 0xFF) as c_uchar;
            *(*emitter).raw_buffer.last.offset((high + 2) as isize) =
                (0xDC + ((value >> 8) & 0xFF)) as c_uchar;
            *(*emitter).raw_buffer.last.offset((low + 2) as isize) = (value & 0xFF) as c_uchar;

            (*emitter).raw_buffer.last = (*emitter).raw_buffer.last.offset(4);
        }
    }

    /* Write the raw buffer. */

    if ((*emitter).write_handler.unwrap())(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        (*emitter).raw_buffer.last.offset_from((*emitter).raw_buffer.start) as size_t,
    ) != 0
    {
        (*emitter).buffer.last = (*emitter).buffer.start;
        (*emitter).buffer.pointer = (*emitter).buffer.start;
        (*emitter).raw_buffer.last = (*emitter).raw_buffer.start;
        (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.start;
        return 1;
    } else {
        return yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const c_char,
        );
    }
}
