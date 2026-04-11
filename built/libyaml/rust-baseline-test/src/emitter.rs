//! Rust transliteration of libyaml's emitter.c.

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
    CLEAR, DEQUEUE, ENQUEUE, JOIN, POP, PUSH, QUEUE_DEL, QUEUE_EMPTY, QUEUE_INIT, QUEUE_INSERT,
    STACK_DEL, STACK_EMPTY, STACK_INIT, STRING_DEL, STRING_EXTEND, STRING_INIT,
};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};

extern "C" {
    pub fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> c_int;
    pub fn yaml_event_delete(event: *mut yaml_event_t);
}

/* --------------------------------------------------------------------- */
/*  Local output helpers corresponding to C macros FLUSH / PUT /         */
/*  PUT_BREAK / WRITE / WRITE_BREAK in emitter.c.                         */
/* --------------------------------------------------------------------- */

#[inline]
unsafe fn FLUSH(emitter: *mut yaml_emitter_t) -> c_int {
    if (*emitter).buffer.pointer.add(5) < (*emitter).buffer.end {
        1
    } else {
        yaml_emitter_flush(emitter)
    }
}

#[inline]
unsafe fn PUT(emitter: *mut yaml_emitter_t, value: c_int) -> c_int {
    if FLUSH(emitter) != 0 {
        *(*emitter).buffer.pointer = value as yaml_char_t;
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
        (*emitter).column += 1;
        1
    } else {
        0
    }
}

#[inline]
unsafe fn PUT_BREAK(emitter: *mut yaml_emitter_t) -> c_int {
    if FLUSH(emitter) != 0 {
        if (*emitter).line_break == YAML_CR_BREAK {
            *(*emitter).buffer.pointer = b'\r' as yaml_char_t;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
        } else if (*emitter).line_break == YAML_LN_BREAK {
            *(*emitter).buffer.pointer = b'\n' as yaml_char_t;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
        } else if (*emitter).line_break == YAML_CRLN_BREAK {
            *(*emitter).buffer.pointer = b'\r' as yaml_char_t;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            *(*emitter).buffer.pointer = b'\n' as yaml_char_t;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
        }
        (*emitter).column = 0;
        (*emitter).line += 1;
        1
    } else {
        0
    }
}

/* Copy a single UTF-8 character from `string` into the emitter buffer. */
#[inline]
unsafe fn WRITE(emitter: *mut yaml_emitter_t, string: &mut yaml_string_t) -> c_int {
    if FLUSH(emitter) != 0 {
        let c0 = *string.pointer;
        if (c0 & 0x80) == 0x00 {
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
        } else if (c0 & 0xE0) == 0xC0 {
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
        } else if (c0 & 0xF0) == 0xE0 {
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
        } else if (c0 & 0xF8) == 0xF0 {
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
            *(*emitter).buffer.pointer = *string.pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            string.pointer = string.pointer.add(1);
        }
        (*emitter).column += 1;
        1
    } else {
        0
    }
}

#[inline]
unsafe fn WRITE_BREAK(emitter: *mut yaml_emitter_t, string: &mut yaml_string_t) -> c_int {
    if FLUSH(emitter) != 0 {
        if CHECK(*string, b'\n') {
            if PUT_BREAK(emitter) == 0 {
                return 0;
            }
            string.pointer = string.pointer.add(1);
            1
        } else {
            if WRITE(emitter, string) == 0 {
                return 0;
            }
            (*emitter).column = 0;
            (*emitter).line += 1;
            1
        }
    } else {
        0
    }
}

/* --------------------------------------------------------------------- */
/*  Forward declarations (static helpers).                                */
/* --------------------------------------------------------------------- */

/*
 * Set an emitter error and return 0.
 */

pub(crate) unsafe fn yaml_emitter_set_emitter_error(
    emitter: *mut yaml_emitter_t,
    problem: *const c_char,
) -> c_int {
    (*emitter).error = YAML_EMITTER_ERROR;
    (*emitter).problem = problem;
    0
}

/*
 * Emit an event.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_emit(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    if ENQUEUE!(emitter, (*emitter).events, *event) == 0 {
        yaml_event_delete(event);
        return 0;
    }

    while yaml_emitter_need_more_events(emitter) == 0 {
        if yaml_emitter_analyze_event(emitter, (*emitter).events.head) == 0 {
            return 0;
        }
        if yaml_emitter_state_machine(emitter, (*emitter).events.head) == 0 {
            return 0;
        }
        let mut __dq = DEQUEUE!(emitter, (*emitter).events);
        yaml_event_delete(&mut __dq);
    }

    1
}

/*
 * Check if we need to accumulate more events before emitting.
 *
 * We accumulate extra
 *  - 1 event for DOCUMENT-START
 *  - 2 events for SEQUENCE-START
 *  - 3 events for MAPPING-START
 */

pub(crate) unsafe fn yaml_emitter_need_more_events(emitter: *mut yaml_emitter_t) -> c_int {
    let mut level: c_int = 0;
    let mut accumulate: c_int = 0;
    let mut event: *mut yaml_event_t;

    if QUEUE_EMPTY!(emitter, (*emitter).events) {
        return 1;
    }

    match (*(*emitter).events.head).type_ {
        YAML_DOCUMENT_START_EVENT => {
            accumulate = 1;
        }
        YAML_SEQUENCE_START_EVENT => {
            accumulate = 2;
        }
        YAML_MAPPING_START_EVENT => {
            accumulate = 3;
        }
        _ => {
            return 0;
        }
    }

    if (*emitter).events.tail.offset_from((*emitter).events.head) as c_int > accumulate {
        return 0;
    }

    event = (*emitter).events.head;
    while event != (*emitter).events.tail {
        match (*event).type_ {
            YAML_STREAM_START_EVENT
            | YAML_DOCUMENT_START_EVENT
            | YAML_SEQUENCE_START_EVENT
            | YAML_MAPPING_START_EVENT => {
                level += 1;
            }
            YAML_STREAM_END_EVENT
            | YAML_DOCUMENT_END_EVENT
            | YAML_SEQUENCE_END_EVENT
            | YAML_MAPPING_END_EVENT => {
                level -= 1;
            }
            _ => {}
        }
        if level == 0 {
            return 0;
        }
        event = event.add(1);
    }

    1
}

/*
 * Append a directive to the directives stack.
 */

pub(crate) unsafe fn yaml_emitter_append_tag_directive(
    emitter: *mut yaml_emitter_t,
    value: yaml_tag_directive_t,
    allow_duplicates: c_int,
) -> c_int {
    let mut tag_directive: *mut yaml_tag_directive_t;
    let mut copy = yaml_tag_directive_t {
        handle: core::ptr::null_mut(),
        prefix: core::ptr::null_mut(),
    };

    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        if libc::strcmp(
            value.handle as *const c_char,
            (*tag_directive).handle as *const c_char,
        ) == 0
        {
            if allow_duplicates != 0 {
                return 1;
            }
            return yaml_emitter_set_emitter_error(
                emitter,
                b"duplicate %TAG directive\0".as_ptr() as *const c_char,
            );
        }
        tag_directive = tag_directive.add(1);
    }

    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    'error: loop {
        if copy.handle.is_null() || copy.prefix.is_null() {
            (*emitter).error = YAML_MEMORY_ERROR;
            break 'error;
        }

        if PUSH!(emitter, (*emitter).tag_directives, copy) == 0 {
            break 'error;
        }

        return 1;
    }

    yaml_free(copy.handle as *mut c_void);
    yaml_free(copy.prefix as *mut c_void);
    0
}

/*
 * Increase the indentation level.
 */

pub(crate) unsafe fn yaml_emitter_increase_indent(
    emitter: *mut yaml_emitter_t,
    flow: c_int,
    indentless: c_int,
) -> c_int {
    if PUSH!(emitter, (*emitter).indents, (*emitter).indent) == 0 {
        return 0;
    }

    if (*emitter).indent < 0 {
        (*emitter).indent = if flow != 0 { (*emitter).best_indent } else { 0 };
    } else if indentless == 0 {
        (*emitter).indent += (*emitter).best_indent;
    }

    1
}

/*
 * State dispatcher.
 */

pub(crate) unsafe fn yaml_emitter_state_machine(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    match (*emitter).state {
        YAML_EMIT_STREAM_START_STATE => {
            return yaml_emitter_emit_stream_start(emitter, event);
        }
        YAML_EMIT_FIRST_DOCUMENT_START_STATE => {
            return yaml_emitter_emit_document_start(emitter, event, 1);
        }
        YAML_EMIT_DOCUMENT_START_STATE => {
            return yaml_emitter_emit_document_start(emitter, event, 0);
        }
        YAML_EMIT_DOCUMENT_CONTENT_STATE => {
            return yaml_emitter_emit_document_content(emitter, event);
        }
        YAML_EMIT_DOCUMENT_END_STATE => {
            return yaml_emitter_emit_document_end(emitter, event);
        }
        YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 1);
        }
        YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 0);
        }
        YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE => {
            return yaml_emitter_emit_flow_mapping_key(emitter, event, 1);
        }
        YAML_EMIT_FLOW_MAPPING_KEY_STATE => {
            return yaml_emitter_emit_flow_mapping_key(emitter, event, 0);
        }
        YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 1);
        }
        YAML_EMIT_FLOW_MAPPING_VALUE_STATE => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 0);
        }
        YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 1);
        }
        YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 0);
        }
        YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 1);
        }
        YAML_EMIT_BLOCK_MAPPING_KEY_STATE => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 0);
        }
        YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 1);
        }
        YAML_EMIT_BLOCK_MAPPING_VALUE_STATE => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 0);
        }
        YAML_EMIT_END_STATE => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected nothing after STREAM-END\0".as_ptr() as *const c_char,
            );
        }
        _ => {
            /* assert(1);      /* Invalid state. */ */
        }
    }

    0
}

/*
 * Expect STREAM-START.
 */

pub(crate) unsafe fn yaml_emitter_emit_stream_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    (*emitter).open_ended = 0;
    if (*event).type_ == YAML_STREAM_START_EVENT {
        if (*emitter).encoding == 0 {
            (*emitter).encoding = (*event).data.stream_start.encoding;
        }

        if (*emitter).encoding == 0 {
            (*emitter).encoding = YAML_UTF8_ENCODING;
        }

        if (*emitter).best_indent < 2 || (*emitter).best_indent > 9 {
            (*emitter).best_indent = 2;
        }

        if (*emitter).best_width >= 0
            && (*emitter).best_width <= (*emitter).best_indent * 2
        {
            (*emitter).best_width = 80;
        }

        if (*emitter).best_width < 0 {
            (*emitter).best_width = c_int::MAX;
        }

        if (*emitter).line_break == 0 {
            (*emitter).line_break = YAML_LN_BREAK;
        }

        (*emitter).indent = -1;

        (*emitter).line = 0;
        (*emitter).column = 0;
        (*emitter).whitespace = 1;
        (*emitter).indention = 1;

        if (*emitter).encoding != YAML_UTF8_ENCODING {
            if yaml_emitter_write_bom(emitter) == 0 {
                return 0;
            }
        }

        (*emitter).state = YAML_EMIT_FIRST_DOCUMENT_START_STATE;

        return 1;
    }

    yaml_emitter_set_emitter_error(
        emitter,
        b"expected STREAM-START\0".as_ptr() as *const c_char,
    )
}

/*
 * Expect DOCUMENT-START or STREAM-END.
 */

pub(crate) unsafe fn yaml_emitter_emit_document_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    if (*event).type_ == YAML_DOCUMENT_START_EVENT {
        let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
            yaml_tag_directive_t {
                handle: b"!\0".as_ptr() as *mut yaml_char_t,
                prefix: b"!\0".as_ptr() as *mut yaml_char_t,
            },
            yaml_tag_directive_t {
                handle: b"!!\0".as_ptr() as *mut yaml_char_t,
                prefix: b"tag:yaml.org,2002:\0".as_ptr() as *mut yaml_char_t,
            },
            yaml_tag_directive_t {
                handle: core::ptr::null_mut(),
                prefix: core::ptr::null_mut(),
            },
        ];
        let mut tag_directive: *mut yaml_tag_directive_t;
        let mut implicit: c_int;

        if !(*event).data.document_start.version_directive.is_null() {
            if yaml_emitter_analyze_version_directive(
                emitter,
                *(*event).data.document_start.version_directive,
            ) == 0
            {
                return 0;
            }
        }

        tag_directive = (*event).data.document_start.tag_directives.start;
        while tag_directive != (*event).data.document_start.tag_directives.end {
            if yaml_emitter_analyze_tag_directive(emitter, *tag_directive) == 0 {
                return 0;
            }
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 0) == 0 {
                return 0;
            }
            tag_directive = tag_directive.add(1);
        }

        tag_directive = default_tag_directives.as_mut_ptr();
        while !(*tag_directive).handle.is_null() {
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 1) == 0 {
                return 0;
            }
            tag_directive = tag_directive.add(1);
        }

        implicit = (*event).data.document_start.implicit;
        if first == 0 || (*emitter).canonical != 0 {
            implicit = 0;
        }

        if ((!(*event).data.document_start.version_directive.is_null())
            || ((*event).data.document_start.tag_directives.start
                != (*event).data.document_start.tag_directives.end))
            && (*emitter).open_ended != 0
        {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0".as_ptr() as *const c_char,
                1,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        }
        (*emitter).open_ended = 0;

        if !(*event).data.document_start.version_directive.is_null() {
            implicit = 0;
            if yaml_emitter_write_indicator(
                emitter,
                b"%YAML\0".as_ptr() as *const c_char,
                1,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            if (*(*event).data.document_start.version_directive).minor == 1 {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"1.1\0".as_ptr() as *const c_char,
                    1,
                    0,
                    0,
                ) == 0
                {
                    return 0;
                }
            } else {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"1.2\0".as_ptr() as *const c_char,
                    1,
                    0,
                    0,
                ) == 0
                {
                    return 0;
                }
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        }

        if (*event).data.document_start.tag_directives.start
            != (*event).data.document_start.tag_directives.end
        {
            implicit = 0;
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"%TAG\0".as_ptr() as *const c_char,
                    1,
                    0,
                    0,
                ) == 0
                {
                    return 0;
                }
                if yaml_emitter_write_tag_handle(
                    emitter,
                    (*tag_directive).handle,
                    libc::strlen((*tag_directive).handle as *const c_char),
                ) == 0
                {
                    return 0;
                }
                if yaml_emitter_write_tag_content(
                    emitter,
                    (*tag_directive).prefix,
                    libc::strlen((*tag_directive).prefix as *const c_char),
                    1,
                ) == 0
                {
                    return 0;
                }
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
                tag_directive = tag_directive.add(1);
            }
        }

        if yaml_emitter_check_empty_document(emitter) != 0 {
            implicit = 0;
        }

        if implicit == 0 {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
            if yaml_emitter_write_indicator(
                emitter,
                b"---\0".as_ptr() as *const c_char,
                1,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            if (*emitter).canonical != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
            }
        }

        (*emitter).state = YAML_EMIT_DOCUMENT_CONTENT_STATE;

        (*emitter).open_ended = 0;
        return 1;
    } else if (*event).type_ == YAML_STREAM_END_EVENT {
        /*
         * This can happen if a block scalar with trailing empty lines
         * is at the end of the stream
         */
        if (*emitter).open_ended == 2 {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0".as_ptr() as *const c_char,
                1,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            (*emitter).open_ended = 0;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0;
        }

        (*emitter).state = YAML_EMIT_END_STATE;

        return 1;
    }

    yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-START or STREAM-END\0".as_ptr() as *const c_char,
    )
}

/*
 * Expect the root node.
 */

pub(crate) unsafe fn yaml_emitter_emit_document_content(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    if PUSH!(emitter, (*emitter).states, YAML_EMIT_DOCUMENT_END_STATE) == 0 {
        return 0;
    }

    yaml_emitter_emit_node(emitter, event, 1, 0, 0, 0)
}

/*
 * Expect DOCUMENT-END.
 */

pub(crate) unsafe fn yaml_emitter_emit_document_end(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    if (*event).type_ == YAML_DOCUMENT_END_EVENT {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0;
        }
        if (*event).data.document_end.implicit == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0".as_ptr() as *const c_char,
                1,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            (*emitter).open_ended = 0;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        } else if (*emitter).open_ended == 0 {
            (*emitter).open_ended = 1;
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0;
        }

        (*emitter).state = YAML_EMIT_DOCUMENT_START_STATE;

        while !STACK_EMPTY!(emitter, (*emitter).tag_directives) {
            let tag_directive: yaml_tag_directive_t =
                POP!(emitter, (*emitter).tag_directives);
            yaml_free(tag_directive.handle as *mut c_void);
            yaml_free(tag_directive.prefix as *mut c_void);
        }

        return 1;
    }

    yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-END\0".as_ptr() as *const c_char,
    )
}

/*
 *
 * Expect a flow item node.
 */

pub(crate) unsafe fn yaml_emitter_emit_flow_sequence_item(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"[\0".as_ptr() as *const c_char,
            1,
            1,
            0,
        ) == 0
        {
            return 0;
        }
        if yaml_emitter_increase_indent(emitter, 1, 0) == 0 {
            return 0;
        }
        (*emitter).flow_level += 1;
    }

    if (*event).type_ == YAML_SEQUENCE_END_EVENT {
        (*emitter).flow_level -= 1;
        (*emitter).indent = POP!(emitter, (*emitter).indents);
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0".as_ptr() as *const c_char,
                0,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"]\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
        (*emitter).state = POP!(emitter, (*emitter).states);

        return 1;
    }

    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
    }

    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0;
        }
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE
    ) == 0
    {
        return 0;
    }

    yaml_emitter_emit_node(emitter, event, 0, 1, 0, 0)
}

/*
 * Expect a flow key node.
 */

pub(crate) unsafe fn yaml_emitter_emit_flow_mapping_key(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"{\0".as_ptr() as *const c_char,
            1,
            1,
            0,
        ) == 0
        {
            return 0;
        }
        if yaml_emitter_increase_indent(emitter, 1, 0) == 0 {
            return 0;
        }
        (*emitter).flow_level += 1;
    }

    if (*event).type_ == YAML_MAPPING_END_EVENT {
        (*emitter).flow_level -= 1;
        (*emitter).indent = POP!(emitter, (*emitter).indents);
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0".as_ptr() as *const c_char,
                0,
                0,
                0,
            ) == 0
            {
                return 0;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"}\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
        (*emitter).state = POP!(emitter, (*emitter).states);

        return 1;
    }

    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0;
        }
    }

    if (*emitter).canonical == 0 && yaml_emitter_check_simple_key(emitter) != 0 {
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE
        ) == 0
        {
            return 0;
        }

        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 1);
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0".as_ptr() as *const c_char,
            1,
            0,
            0,
        ) == 0
        {
            return 0;
        }
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_FLOW_MAPPING_VALUE_STATE
        ) == 0
        {
            return 0;
        }

        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0);
    }
}

/*
 * Expect a flow value node.
 */

pub(crate) unsafe fn yaml_emitter_emit_flow_mapping_value(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    simple: c_int,
) -> c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
    } else {
        if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0".as_ptr() as *const c_char,
            1,
            0,
            0,
        ) == 0
        {
            return 0;
        }
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_FLOW_MAPPING_KEY_STATE
    ) == 0
    {
        return 0;
    }
    yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0)
}

/*
 * Expect a block item node.
 */

pub(crate) unsafe fn yaml_emitter_emit_block_sequence_item(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(
            emitter,
            0,
            if (*emitter).mapping_context != 0 && (*emitter).indention == 0 {
                1
            } else {
                0
            },
        ) == 0
        {
            return 0;
        }
    }

    if (*event).type_ == YAML_SEQUENCE_END_EVENT {
        (*emitter).indent = POP!(emitter, (*emitter).indents);
        (*emitter).state = POP!(emitter, (*emitter).states);

        return 1;
    }

    if yaml_emitter_write_indent(emitter) == 0 {
        return 0;
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"-\0".as_ptr() as *const c_char,
        1,
        0,
        1,
    ) == 0
    {
        return 0;
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE
    ) == 0
    {
        return 0;
    }

    yaml_emitter_emit_node(emitter, event, 0, 1, 0, 0)
}

/*
 * Expect a block key node.
 */

pub(crate) unsafe fn yaml_emitter_emit_block_mapping_key(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0, 0) == 0 {
            return 0;
        }
    }

    if (*event).type_ == YAML_MAPPING_END_EVENT {
        (*emitter).indent = POP!(emitter, (*emitter).indents);
        (*emitter).state = POP!(emitter, (*emitter).states);

        return 1;
    }

    if yaml_emitter_write_indent(emitter) == 0 {
        return 0;
    }

    if yaml_emitter_check_simple_key(emitter) != 0 {
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE
        ) == 0
        {
            return 0;
        }

        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 1);
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0".as_ptr() as *const c_char,
            1,
            0,
            1,
        ) == 0
        {
            return 0;
        }
        if PUSH!(
            emitter,
            (*emitter).states,
            YAML_EMIT_BLOCK_MAPPING_VALUE_STATE
        ) == 0
        {
            return 0;
        }

        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0);
    }
}

/*
 * Expect a block value node.
 */

pub(crate) unsafe fn yaml_emitter_emit_block_mapping_value(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    simple: c_int,
) -> c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
    } else {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0".as_ptr() as *const c_char,
            1,
            0,
            1,
        ) == 0
        {
            return 0;
        }
    }
    if PUSH!(
        emitter,
        (*emitter).states,
        YAML_EMIT_BLOCK_MAPPING_KEY_STATE
    ) == 0
    {
        return 0;
    }

    yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0)
}

/*
 * Expect a node.
 */

pub(crate) unsafe fn yaml_emitter_emit_node(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    root: c_int,
    sequence: c_int,
    mapping: c_int,
    simple_key: c_int,
) -> c_int {
    (*emitter).root_context = root;
    (*emitter).sequence_context = sequence;
    (*emitter).mapping_context = mapping;
    (*emitter).simple_key_context = simple_key;

    match (*event).type_ {
        YAML_ALIAS_EVENT => {
            return yaml_emitter_emit_alias(emitter, event);
        }
        YAML_SCALAR_EVENT => {
            return yaml_emitter_emit_scalar(emitter, event);
        }
        YAML_SEQUENCE_START_EVENT => {
            return yaml_emitter_emit_sequence_start(emitter, event);
        }
        YAML_MAPPING_START_EVENT => {
            return yaml_emitter_emit_mapping_start(emitter, event);
        }
        _ => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected SCALAR, SEQUENCE-START, MAPPING-START, or ALIAS\0".as_ptr()
                    as *const c_char,
            );
        }
    }
}

/*
 * Expect ALIAS.
 */

pub(crate) unsafe fn yaml_emitter_emit_alias(
    emitter: *mut yaml_emitter_t,
    _event: *mut yaml_event_t,
) -> c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0;
    }
    if (*emitter).simple_key_context != 0 {
        if PUT(emitter, b' ' as c_int) == 0 {
            return 0;
        }
    }
    (*emitter).state = POP!(emitter, (*emitter).states);

    1
}

/*
 * Expect SCALAR.
 */

pub(crate) unsafe fn yaml_emitter_emit_scalar(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    if yaml_emitter_select_scalar_style(emitter, event) == 0 {
        return 0;
    }
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0;
    }
    if yaml_emitter_increase_indent(emitter, 1, 0) == 0 {
        return 0;
    }
    if yaml_emitter_process_scalar(emitter) == 0 {
        return 0;
    }
    (*emitter).indent = POP!(emitter, (*emitter).indents);
    (*emitter).state = POP!(emitter, (*emitter).states);

    1
}

/*
 * Expect SEQUENCE-START.
 */

pub(crate) unsafe fn yaml_emitter_emit_sequence_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0;
    }

    if (*emitter).flow_level != 0
        || (*emitter).canonical != 0
        || (*event).data.sequence_start.style == YAML_FLOW_SEQUENCE_STYLE
        || yaml_emitter_check_empty_sequence(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE;
    }

    1
}

/*
 * Expect MAPPING-START.
 */

pub(crate) unsafe fn yaml_emitter_emit_mapping_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0;
    }

    if (*emitter).flow_level != 0
        || (*emitter).canonical != 0
        || (*event).data.mapping_start.style == YAML_FLOW_MAPPING_STYLE
        || yaml_emitter_check_empty_mapping(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE;
    }

    1
}

/*
 * Check if the document content is an empty scalar.
 */

pub(crate) unsafe fn yaml_emitter_check_empty_document(_emitter: *mut yaml_emitter_t) -> c_int {
    0
}

/*
 * Check if the next events represent an empty sequence.
 */

pub(crate) unsafe fn yaml_emitter_check_empty_sequence(emitter: *mut yaml_emitter_t) -> c_int {
    if ((*emitter).events.tail.offset_from((*emitter).events.head) as isize) < 2 {
        return 0;
    }

    if (*(*emitter).events.head.offset(0)).type_ == YAML_SEQUENCE_START_EVENT
        && (*(*emitter).events.head.offset(1)).type_ == YAML_SEQUENCE_END_EVENT
    {
        1
    } else {
        0
    }
}

/*
 * Check if the next events represent an empty mapping.
 */

pub(crate) unsafe fn yaml_emitter_check_empty_mapping(emitter: *mut yaml_emitter_t) -> c_int {
    if ((*emitter).events.tail.offset_from((*emitter).events.head) as isize) < 2 {
        return 0;
    }

    if (*(*emitter).events.head.offset(0)).type_ == YAML_MAPPING_START_EVENT
        && (*(*emitter).events.head.offset(1)).type_ == YAML_MAPPING_END_EVENT
    {
        1
    } else {
        0
    }
}

/*
 * Check if the next node can be expressed as a simple key.
 */

pub(crate) unsafe fn yaml_emitter_check_simple_key(emitter: *mut yaml_emitter_t) -> c_int {
    let event: *mut yaml_event_t = (*emitter).events.head;
    let mut length: size_t = 0;

    match (*event).type_ {
        YAML_ALIAS_EVENT => {
            length += (*emitter).anchor_data.anchor_length;
        }
        YAML_SCALAR_EVENT => {
            if (*emitter).scalar_data.multiline != 0 {
                return 0;
            }
            length += (*emitter).anchor_data.anchor_length
                + (*emitter).tag_data.handle_length
                + (*emitter).tag_data.suffix_length
                + (*emitter).scalar_data.length;
        }
        YAML_SEQUENCE_START_EVENT => {
            if yaml_emitter_check_empty_sequence(emitter) == 0 {
                return 0;
            }
            length += (*emitter).anchor_data.anchor_length
                + (*emitter).tag_data.handle_length
                + (*emitter).tag_data.suffix_length;
        }
        YAML_MAPPING_START_EVENT => {
            if yaml_emitter_check_empty_mapping(emitter) == 0 {
                return 0;
            }
            length += (*emitter).anchor_data.anchor_length
                + (*emitter).tag_data.handle_length
                + (*emitter).tag_data.suffix_length;
        }
        _ => {
            return 0;
        }
    }

    if length > 128 {
        return 0;
    }

    1
}

/*
 * Determine an acceptable scalar style.
 */

pub(crate) unsafe fn yaml_emitter_select_scalar_style(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut style: yaml_scalar_style_t = (*event).data.scalar.style;
    let no_tag: c_int = if (*emitter).tag_data.handle.is_null()
        && (*emitter).tag_data.suffix.is_null()
    {
        1
    } else {
        0
    };

    if no_tag != 0
        && (*event).data.scalar.plain_implicit == 0
        && (*event).data.scalar.quoted_implicit == 0
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"neither tag nor implicit flags are specified\0".as_ptr() as *const c_char,
        );
    }

    if style == YAML_ANY_SCALAR_STYLE {
        style = YAML_PLAIN_SCALAR_STYLE;
    }

    if (*emitter).canonical != 0 {
        style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
    }

    if (*emitter).simple_key_context != 0 && (*emitter).scalar_data.multiline != 0 {
        style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
    }

    if style == YAML_PLAIN_SCALAR_STYLE {
        if ((*emitter).flow_level != 0 && (*emitter).scalar_data.flow_plain_allowed == 0)
            || ((*emitter).flow_level == 0
                && (*emitter).scalar_data.block_plain_allowed == 0)
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if (*emitter).scalar_data.length == 0
            && ((*emitter).flow_level != 0 || (*emitter).simple_key_context != 0)
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if no_tag != 0 && (*event).data.scalar.plain_implicit == 0 {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
    }

    if style == YAML_SINGLE_QUOTED_SCALAR_STYLE {
        if (*emitter).scalar_data.single_quoted_allowed == 0 {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }

    if style == YAML_LITERAL_SCALAR_STYLE || style == YAML_FOLDED_SCALAR_STYLE {
        if (*emitter).scalar_data.block_allowed == 0
            || (*emitter).flow_level != 0
            || (*emitter).simple_key_context != 0
        {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }

    if no_tag != 0
        && (*event).data.scalar.quoted_implicit == 0
        && style != YAML_PLAIN_SCALAR_STYLE
    {
        (*emitter).tag_data.handle = b"!\0".as_ptr() as *mut yaml_char_t;
        (*emitter).tag_data.handle_length = 1;
    }

    (*emitter).scalar_data.style = style;

    1
}

/*
 * Write an anchor.
 */

pub(crate) unsafe fn yaml_emitter_process_anchor(emitter: *mut yaml_emitter_t) -> c_int {
    if (*emitter).anchor_data.anchor.is_null() {
        return 1;
    }

    if yaml_emitter_write_indicator(
        emitter,
        if (*emitter).anchor_data.alias != 0 {
            b"*\0".as_ptr() as *const c_char
        } else {
            b"&\0".as_ptr() as *const c_char
        },
        1,
        0,
        0,
    ) == 0
    {
        return 0;
    }

    yaml_emitter_write_anchor(
        emitter,
        (*emitter).anchor_data.anchor,
        (*emitter).anchor_data.anchor_length,
    )
}

/*
 * Write a tag.
 */

pub(crate) unsafe fn yaml_emitter_process_tag(emitter: *mut yaml_emitter_t) -> c_int {
    if (*emitter).tag_data.handle.is_null() && (*emitter).tag_data.suffix.is_null() {
        return 1;
    }

    if !(*emitter).tag_data.handle.is_null() {
        if yaml_emitter_write_tag_handle(
            emitter,
            (*emitter).tag_data.handle,
            (*emitter).tag_data.handle_length,
        ) == 0
        {
            return 0;
        }
        if !(*emitter).tag_data.suffix.is_null() {
            if yaml_emitter_write_tag_content(
                emitter,
                (*emitter).tag_data.suffix,
                (*emitter).tag_data.suffix_length,
                0,
            ) == 0
            {
                return 0;
            }
        }
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"!<\0".as_ptr() as *const c_char,
            1,
            0,
            0,
        ) == 0
        {
            return 0;
        }
        if yaml_emitter_write_tag_content(
            emitter,
            (*emitter).tag_data.suffix,
            (*emitter).tag_data.suffix_length,
            0,
        ) == 0
        {
            return 0;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b">\0".as_ptr() as *const c_char,
            0,
            0,
            0,
        ) == 0
        {
            return 0;
        }
    }

    1
}

/*
 * Write a scalar.
 */

pub(crate) unsafe fn yaml_emitter_process_scalar(emitter: *mut yaml_emitter_t) -> c_int {
    match (*emitter).scalar_data.style {
        YAML_PLAIN_SCALAR_STYLE => {
            return yaml_emitter_write_plain_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                if (*emitter).simple_key_context != 0 { 0 } else { 1 },
            );
        }
        YAML_SINGLE_QUOTED_SCALAR_STYLE => {
            return yaml_emitter_write_single_quoted_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                if (*emitter).simple_key_context != 0 { 0 } else { 1 },
            );
        }
        YAML_DOUBLE_QUOTED_SCALAR_STYLE => {
            return yaml_emitter_write_double_quoted_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                if (*emitter).simple_key_context != 0 { 0 } else { 1 },
            );
        }
        YAML_LITERAL_SCALAR_STYLE => {
            return yaml_emitter_write_literal_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
            );
        }
        YAML_FOLDED_SCALAR_STYLE => {
            return yaml_emitter_write_folded_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
            );
        }
        _ => {
            /* assert(1);      /* Impossible. */ */
        }
    }

    0
}

/*
 * Check if a %YAML directive is valid.
 */

pub(crate) unsafe fn yaml_emitter_analyze_version_directive(
    emitter: *mut yaml_emitter_t,
    version_directive: yaml_version_directive_t,
) -> c_int {
    if version_directive.major != 1
        || (version_directive.minor != 1 && version_directive.minor != 2)
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"incompatible %YAML directive\0".as_ptr() as *const c_char,
        );
    }

    1
}

/*
 * Check if a %TAG directive is valid.
 */

pub(crate) unsafe fn yaml_emitter_analyze_tag_directive(
    emitter: *mut yaml_emitter_t,
    tag_directive: yaml_tag_directive_t,
) -> c_int {
    let mut handle: yaml_string_t;
    let prefix: yaml_string_t;
    let handle_length: size_t;
    let prefix_length: size_t;

    handle_length = libc::strlen(tag_directive.handle as *const c_char);
    prefix_length = libc::strlen(tag_directive.prefix as *const c_char);
    handle = STRING_ASSIGN(tag_directive.handle, handle_length);
    prefix = STRING_ASSIGN(tag_directive.prefix, prefix_length);

    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must not be empty\0".as_ptr() as *const c_char,
        );
    }

    if *handle.start.offset(0) != b'!' {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must start with '!'\0".as_ptr() as *const c_char,
        );
    }

    if *handle.end.offset(-1) != b'!' {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must end with '!'\0".as_ptr() as *const c_char,
        );
    }

    handle.pointer = handle.pointer.add(1);

    while handle.pointer < handle.end.offset(-1) {
        if !IS_ALPHA(handle) {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"tag handle must contain alphanumerical characters only\0".as_ptr()
                    as *const c_char,
            );
        }
        handle.pointer = handle.pointer.offset(WIDTH(handle) as isize);
    }

    if prefix.start == prefix.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag prefix must not be empty\0".as_ptr() as *const c_char,
        );
    }

    1
}

/*
 * Check if an anchor is valid.
 */

pub(crate) unsafe fn yaml_emitter_analyze_anchor(
    emitter: *mut yaml_emitter_t,
    anchor: *mut yaml_char_t,
    alias: c_int,
) -> c_int {
    let anchor_length: size_t;
    let mut string: yaml_string_t;

    anchor_length = libc::strlen(anchor as *const c_char);
    string = STRING_ASSIGN(anchor, anchor_length);

    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            if alias != 0 {
                b"alias value must not be empty\0".as_ptr() as *const c_char
            } else {
                b"anchor value must not be empty\0".as_ptr() as *const c_char
            },
        );
    }

    while string.pointer != string.end {
        if !IS_ALPHA(string) {
            return yaml_emitter_set_emitter_error(
                emitter,
                if alias != 0 {
                    b"alias value must contain alphanumerical characters only\0".as_ptr()
                        as *const c_char
                } else {
                    b"anchor value must contain alphanumerical characters only\0".as_ptr()
                        as *const c_char
                },
            );
        }
        string.pointer = string.pointer.offset(WIDTH(string) as isize);
    }

    (*emitter).anchor_data.anchor = string.start;
    (*emitter).anchor_data.anchor_length = string.end.offset_from(string.start) as size_t;
    (*emitter).anchor_data.alias = alias;

    1
}

/*
 * Check if a tag is valid.
 */

pub(crate) unsafe fn yaml_emitter_analyze_tag(
    emitter: *mut yaml_emitter_t,
    tag: *mut yaml_char_t,
) -> c_int {
    let tag_length: size_t;
    let string: yaml_string_t;
    let mut tag_directive: *mut yaml_tag_directive_t;

    tag_length = libc::strlen(tag as *const c_char);
    string = STRING_ASSIGN(tag, tag_length);

    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag value must not be empty\0".as_ptr() as *const c_char,
        );
    }

    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        let prefix_length: size_t =
            libc::strlen((*tag_directive).prefix as *const c_char);
        if prefix_length < (string.end.offset_from(string.start) as size_t)
            && libc::strncmp(
                (*tag_directive).prefix as *const c_char,
                string.start as *const c_char,
                prefix_length,
            ) == 0
        {
            (*emitter).tag_data.handle = (*tag_directive).handle;
            (*emitter).tag_data.handle_length =
                libc::strlen((*tag_directive).handle as *const c_char);
            (*emitter).tag_data.suffix = string.start.add(prefix_length);
            (*emitter).tag_data.suffix_length =
                (string.end.offset_from(string.start) as size_t) - prefix_length;
            return 1;
        }
        tag_directive = tag_directive.add(1);
    }

    (*emitter).tag_data.suffix = string.start;
    (*emitter).tag_data.suffix_length = string.end.offset_from(string.start) as size_t;

    1
}

/*
 * Check if a scalar is valid.
 */

pub(crate) unsafe fn yaml_emitter_analyze_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    let mut string: yaml_string_t;

    let mut block_indicators: c_int = 0;
    let mut flow_indicators: c_int = 0;
    let mut line_breaks: c_int = 0;
    let mut special_characters: c_int = 0;

    let mut leading_space: c_int = 0;
    let mut leading_break: c_int = 0;
    let mut trailing_space: c_int = 0;
    let mut trailing_break: c_int = 0;
    let mut break_space: c_int = 0;
    let mut space_break: c_int = 0;

    let mut preceded_by_whitespace: c_int = 0;
    let mut followed_by_whitespace: c_int = 0;
    let mut previous_space: c_int = 0;
    let mut previous_break: c_int = 0;

    string = STRING_ASSIGN(value, length);

    (*emitter).scalar_data.value = value;
    (*emitter).scalar_data.length = length;

    if string.start == string.end {
        (*emitter).scalar_data.multiline = 0;
        (*emitter).scalar_data.flow_plain_allowed = 0;
        (*emitter).scalar_data.block_plain_allowed = 1;
        (*emitter).scalar_data.single_quoted_allowed = 1;
        (*emitter).scalar_data.block_allowed = 0;

        return 1;
    }

    if (CHECK_AT(string, b'-', 0)
        && CHECK_AT(string, b'-', 1)
        && CHECK_AT(string, b'-', 2))
        || (CHECK_AT(string, b'.', 0)
            && CHECK_AT(string, b'.', 1)
            && CHECK_AT(string, b'.', 2))
    {
        block_indicators = 1;
        flow_indicators = 1;
    }

    preceded_by_whitespace = 1;
    followed_by_whitespace = if IS_BLANKZ_AT(string, WIDTH(string) as isize) {
        1
    } else {
        0
    };

    while string.pointer != string.end {
        if string.start == string.pointer {
            if CHECK(string, b'#')
                || CHECK(string, b',')
                || CHECK(string, b'[')
                || CHECK(string, b']')
                || CHECK(string, b'{')
                || CHECK(string, b'}')
                || CHECK(string, b'&')
                || CHECK(string, b'*')
                || CHECK(string, b'!')
                || CHECK(string, b'|')
                || CHECK(string, b'>')
                || CHECK(string, b'\'')
                || CHECK(string, b'"')
                || CHECK(string, b'%')
                || CHECK(string, b'@')
                || CHECK(string, b'`')
            {
                flow_indicators = 1;
                block_indicators = 1;
            }

            if CHECK(string, b'?') || CHECK(string, b':') {
                flow_indicators = 1;
                if followed_by_whitespace != 0 {
                    block_indicators = 1;
                }
            }

            if CHECK(string, b'-') && followed_by_whitespace != 0 {
                flow_indicators = 1;
                block_indicators = 1;
            }
        } else {
            if CHECK(string, b',')
                || CHECK(string, b'?')
                || CHECK(string, b'[')
                || CHECK(string, b']')
                || CHECK(string, b'{')
                || CHECK(string, b'}')
            {
                flow_indicators = 1;
            }

            if CHECK(string, b':') {
                flow_indicators = 1;
                if followed_by_whitespace != 0 {
                    block_indicators = 1;
                }
            }

            if CHECK(string, b'#') && preceded_by_whitespace != 0 {
                flow_indicators = 1;
                block_indicators = 1;
            }
        }

        if !IS_PRINTABLE(string) || (!IS_ASCII(string) && (*emitter).unicode == 0) {
            special_characters = 1;
        }

        if IS_BREAK(string) {
            line_breaks = 1;
        }

        if IS_SPACE(string) {
            if string.start == string.pointer {
                leading_space = 1;
            }
            if string.pointer.offset(WIDTH(string) as isize) == string.end {
                trailing_space = 1;
            }
            if previous_break != 0 {
                break_space = 1;
            }
            previous_space = 1;
            previous_break = 0;
        } else if IS_BREAK(string) {
            if string.start == string.pointer {
                leading_break = 1;
            }
            if string.pointer.offset(WIDTH(string) as isize) == string.end {
                trailing_break = 1;
            }
            if previous_space != 0 {
                space_break = 1;
            }
            previous_space = 0;
            previous_break = 1;
        } else {
            previous_space = 0;
            previous_break = 0;
        }

        preceded_by_whitespace = if IS_BLANKZ(string) { 1 } else { 0 };
        string.pointer = string.pointer.offset(WIDTH(string) as isize);
        if string.pointer != string.end {
            followed_by_whitespace = if IS_BLANKZ_AT(string, WIDTH(string) as isize) {
                1
            } else {
                0
            };
        }
    }

    (*emitter).scalar_data.multiline = line_breaks;

    (*emitter).scalar_data.flow_plain_allowed = 1;
    (*emitter).scalar_data.block_plain_allowed = 1;
    (*emitter).scalar_data.single_quoted_allowed = 1;
    (*emitter).scalar_data.block_allowed = 1;

    if leading_space != 0 || leading_break != 0 || trailing_space != 0 || trailing_break != 0
    {
        (*emitter).scalar_data.flow_plain_allowed = 0;
        (*emitter).scalar_data.block_plain_allowed = 0;
    }

    if trailing_space != 0 {
        (*emitter).scalar_data.block_allowed = 0;
    }

    if break_space != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0;
        (*emitter).scalar_data.block_plain_allowed = 0;
        (*emitter).scalar_data.single_quoted_allowed = 0;
    }

    if space_break != 0 || special_characters != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0;
        (*emitter).scalar_data.block_plain_allowed = 0;
        (*emitter).scalar_data.single_quoted_allowed = 0;
        (*emitter).scalar_data.block_allowed = 0;
    }

    if line_breaks != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0;
        (*emitter).scalar_data.block_plain_allowed = 0;
    }

    if flow_indicators != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0;
    }

    if block_indicators != 0 {
        (*emitter).scalar_data.block_plain_allowed = 0;
    }

    1
}

/*
 * Check if the event data is valid.
 */

pub(crate) unsafe fn yaml_emitter_analyze_event(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    (*emitter).anchor_data.anchor = core::ptr::null_mut();
    (*emitter).anchor_data.anchor_length = 0;
    (*emitter).tag_data.handle = core::ptr::null_mut();
    (*emitter).tag_data.handle_length = 0;
    (*emitter).tag_data.suffix = core::ptr::null_mut();
    (*emitter).tag_data.suffix_length = 0;
    (*emitter).scalar_data.value = core::ptr::null_mut();
    (*emitter).scalar_data.length = 0;

    match (*event).type_ {
        YAML_ALIAS_EVENT => {
            if yaml_emitter_analyze_anchor(emitter, (*event).data.alias.anchor, 1) == 0 {
                return 0;
            }
            return 1;
        }
        YAML_SCALAR_EVENT => {
            if !(*event).data.scalar.anchor.is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.scalar.anchor, 0) == 0 {
                    return 0;
                }
            }
            if !(*event).data.scalar.tag.is_null()
                && ((*emitter).canonical != 0
                    || ((*event).data.scalar.plain_implicit == 0
                        && (*event).data.scalar.quoted_implicit == 0))
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.scalar.tag) == 0 {
                    return 0;
                }
            }
            if yaml_emitter_analyze_scalar(
                emitter,
                (*event).data.scalar.value,
                (*event).data.scalar.length,
            ) == 0
            {
                return 0;
            }
            return 1;
        }
        YAML_SEQUENCE_START_EVENT => {
            if !(*event).data.sequence_start.anchor.is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.sequence_start.anchor,
                    0,
                ) == 0
                {
                    return 0;
                }
            }
            if !(*event).data.sequence_start.tag.is_null()
                && ((*emitter).canonical != 0
                    || (*event).data.sequence_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.sequence_start.tag) == 0 {
                    return 0;
                }
            }
            return 1;
        }
        YAML_MAPPING_START_EVENT => {
            if !(*event).data.mapping_start.anchor.is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.mapping_start.anchor,
                    0,
                ) == 0
                {
                    return 0;
                }
            }
            if !(*event).data.mapping_start.tag.is_null()
                && ((*emitter).canonical != 0 || (*event).data.mapping_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.mapping_start.tag) == 0 {
                    return 0;
                }
            }
            return 1;
        }
        _ => {
            return 1;
        }
    }
}

/*
 * Write the BOM character.
 */

pub(crate) unsafe fn yaml_emitter_write_bom(emitter: *mut yaml_emitter_t) -> c_int {
    if FLUSH(emitter) == 0 {
        return 0;
    }

    *(*emitter).buffer.pointer = 0xEF as yaml_char_t;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    *(*emitter).buffer.pointer = 0xBB as yaml_char_t;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    *(*emitter).buffer.pointer = 0xBF as yaml_char_t;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);

    1
}

pub(crate) unsafe fn yaml_emitter_write_indent(emitter: *mut yaml_emitter_t) -> c_int {
    let indent: c_int = if (*emitter).indent >= 0 { (*emitter).indent } else { 0 };

    if (*emitter).indention == 0
        || (*emitter).column > indent
        || ((*emitter).column == indent && (*emitter).whitespace == 0)
    {
        if PUT_BREAK(emitter) == 0 {
            return 0;
        }
    }

    while (*emitter).column < indent {
        if PUT(emitter, b' ' as c_int) == 0 {
            return 0;
        }
    }

    (*emitter).whitespace = 1;
    (*emitter).indention = 1;

    1
}

pub(crate) unsafe fn yaml_emitter_write_indicator(
    emitter: *mut yaml_emitter_t,
    indicator: *const c_char,
    need_whitespace: c_int,
    is_whitespace: c_int,
    is_indention: c_int,
) -> c_int {
    let indicator_length: size_t;
    let mut string: yaml_string_t;

    indicator_length = libc::strlen(indicator);
    string = STRING_ASSIGN(indicator as *mut yaml_char_t, indicator_length);

    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if PUT(emitter, b' ' as c_int) == 0 {
            return 0;
        }
    }

    while string.pointer != string.end {
        if WRITE(emitter, &mut string) == 0 {
            return 0;
        }
    }

    (*emitter).whitespace = is_whitespace;
    (*emitter).indention = if (*emitter).indention != 0 && is_indention != 0 {
        1
    } else {
        0
    };

    1
}

pub(crate) unsafe fn yaml_emitter_write_anchor(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    let mut string: yaml_string_t;
    string = STRING_ASSIGN(value, length);

    while string.pointer != string.end {
        if WRITE(emitter, &mut string) == 0 {
            return 0;
        }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;

    1
}

pub(crate) unsafe fn yaml_emitter_write_tag_handle(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    let mut string: yaml_string_t;
    string = STRING_ASSIGN(value, length);

    if (*emitter).whitespace == 0 {
        if PUT(emitter, b' ' as c_int) == 0 {
            return 0;
        }
    }

    while string.pointer != string.end {
        if WRITE(emitter, &mut string) == 0 {
            return 0;
        }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;

    1
}

pub(crate) unsafe fn yaml_emitter_write_tag_content(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    need_whitespace: c_int,
) -> c_int {
    let mut string: yaml_string_t;
    string = STRING_ASSIGN(value, length);

    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if PUT(emitter, b' ' as c_int) == 0 {
            return 0;
        }
    }

    while string.pointer != string.end {
        if IS_ALPHA(string)
            || CHECK(string, b';')
            || CHECK(string, b'/')
            || CHECK(string, b'?')
            || CHECK(string, b':')
            || CHECK(string, b'@')
            || CHECK(string, b'&')
            || CHECK(string, b'=')
            || CHECK(string, b'+')
            || CHECK(string, b'$')
            || CHECK(string, b',')
            || CHECK(string, b'_')
            || CHECK(string, b'.')
            || CHECK(string, b'~')
            || CHECK(string, b'*')
            || CHECK(string, b'\'')
            || CHECK(string, b'(')
            || CHECK(string, b')')
            || CHECK(string, b'[')
            || CHECK(string, b']')
        {
            if WRITE(emitter, &mut string) == 0 {
                return 0;
            }
        } else {
            let mut width: c_int = WIDTH(string);
            let mut value: c_uint;
            while width > 0 {
                width -= 1;
                value = *string.pointer as c_uint;
                string.pointer = string.pointer.add(1);
                if PUT(emitter, b'%' as c_int) == 0 {
                    return 0;
                }
                if PUT(
                    emitter,
                    ((value >> 4)
                        + (if (value >> 4) < 10 {
                            b'0' as c_uint
                        } else {
                            (b'A' as c_uint) - 10
                        })) as c_int,
                ) == 0
                {
                    return 0;
                }
                if PUT(
                    emitter,
                    ((value & 0x0F)
                        + (if (value & 0x0F) < 10 {
                            b'0' as c_uint
                        } else {
                            (b'A' as c_uint) - 10
                        })) as c_int,
                ) == 0
                {
                    return 0;
                }
            }
        }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;

    1
}

pub(crate) unsafe fn yaml_emitter_write_plain_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: c_int,
) -> c_int {
    let mut string: yaml_string_t;
    let mut spaces: c_int = 0;
    let mut breaks: c_int = 0;

    string = STRING_ASSIGN(value, length);

    /*
     * Avoid trailing spaces for empty values in block mode.
     * In flow mode, we still want the space to prevent ambiguous things
     * like {a:}.
     * Currently, the emitter forbids any plain empty scalar in flow mode
     * (e.g. it outputs {a: ''} instead), so emitter->flow_level will
     * never be true here.
     * But if the emitter is ever changed to allow emitting empty values,
     * the check for flow_level is already here.
     */
    if (*emitter).whitespace == 0 && (length != 0 || (*emitter).flow_level != 0) {
        if PUT(emitter, b' ' as c_int) == 0 {
            return 0;
        }
    }

    while string.pointer != string.end {
        if IS_SPACE(string) {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !IS_SPACE_AT(string, 1)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
                string.pointer = string.pointer.offset(WIDTH(string) as isize);
            } else {
                if WRITE(emitter, &mut string) == 0 {
                    return 0;
                }
            }
            spaces = 1;
        } else if IS_BREAK(string) {
            if breaks == 0 && CHECK(string, b'\n') {
                if PUT_BREAK(emitter) == 0 {
                    return 0;
                }
            }
            if WRITE_BREAK(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
            }
            if WRITE(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 0;
            spaces = 0;
            breaks = 0;
        }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;

    1
}

pub(crate) unsafe fn yaml_emitter_write_single_quoted_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: c_int,
) -> c_int {
    let mut string: yaml_string_t;
    let mut spaces: c_int = 0;
    let mut breaks: c_int = 0;

    string = STRING_ASSIGN(value, length);

    if yaml_emitter_write_indicator(emitter, b"'\0".as_ptr() as *const c_char, 1, 0, 0) == 0 {
        return 0;
    }

    while string.pointer != string.end {
        if IS_SPACE(string) {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.offset(-1)
                && !IS_SPACE_AT(string, 1)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
                string.pointer = string.pointer.offset(WIDTH(string) as isize);
            } else {
                if WRITE(emitter, &mut string) == 0 {
                    return 0;
                }
            }
            spaces = 1;
        } else if IS_BREAK(string) {
            if breaks == 0 && CHECK(string, b'\n') {
                if PUT_BREAK(emitter) == 0 {
                    return 0;
                }
            }
            if WRITE_BREAK(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
            }
            if CHECK(string, b'\'') {
                if PUT(emitter, b'\'' as c_int) == 0 {
                    return 0;
                }
            }
            if WRITE(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 0;
            spaces = 0;
            breaks = 0;
        }
    }

    if breaks != 0 {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0;
        }
    }

    if yaml_emitter_write_indicator(emitter, b"'\0".as_ptr() as *const c_char, 0, 0, 0) == 0 {
        return 0;
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;

    1
}

pub(crate) unsafe fn yaml_emitter_write_double_quoted_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: c_int,
) -> c_int {
    let mut string: yaml_string_t;
    let mut spaces: c_int = 0;

    string = STRING_ASSIGN(value, length);

    if yaml_emitter_write_indicator(emitter, b"\"\0".as_ptr() as *const c_char, 1, 0, 0) == 0 {
        return 0;
    }

    while string.pointer != string.end {
        if !IS_PRINTABLE(string)
            || ((*emitter).unicode == 0 && !IS_ASCII(string))
            || IS_BOM(string)
            || IS_BREAK(string)
            || CHECK(string, b'"')
            || CHECK(string, b'\\')
        {
            let mut octet: c_uchar;
            let mut width: c_uint;
            let mut value: c_uint;
            let mut k: c_int;

            octet = *string.pointer.offset(0);
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
            k = 1;
            while k < width as c_int {
                octet = *string.pointer.offset(k as isize);
                value = (value << 6) + ((octet & 0x3F) as c_uint);
                k += 1;
            }
            string.pointer = string.pointer.add(width as usize);

            if PUT(emitter, b'\\' as c_int) == 0 {
                return 0;
            }

            match value {
                0x00 => {
                    if PUT(emitter, b'0' as c_int) == 0 {
                        return 0;
                    }
                }
                0x07 => {
                    if PUT(emitter, b'a' as c_int) == 0 {
                        return 0;
                    }
                }
                0x08 => {
                    if PUT(emitter, b'b' as c_int) == 0 {
                        return 0;
                    }
                }
                0x09 => {
                    if PUT(emitter, b't' as c_int) == 0 {
                        return 0;
                    }
                }
                0x0A => {
                    if PUT(emitter, b'n' as c_int) == 0 {
                        return 0;
                    }
                }
                0x0B => {
                    if PUT(emitter, b'v' as c_int) == 0 {
                        return 0;
                    }
                }
                0x0C => {
                    if PUT(emitter, b'f' as c_int) == 0 {
                        return 0;
                    }
                }
                0x0D => {
                    if PUT(emitter, b'r' as c_int) == 0 {
                        return 0;
                    }
                }
                0x1B => {
                    if PUT(emitter, b'e' as c_int) == 0 {
                        return 0;
                    }
                }
                0x22 => {
                    if PUT(emitter, b'"' as c_int) == 0 {
                        return 0;
                    }
                }
                0x5C => {
                    if PUT(emitter, b'\\' as c_int) == 0 {
                        return 0;
                    }
                }
                0x85 => {
                    if PUT(emitter, b'N' as c_int) == 0 {
                        return 0;
                    }
                }
                0xA0 => {
                    if PUT(emitter, b'_' as c_int) == 0 {
                        return 0;
                    }
                }
                0x2028 => {
                    if PUT(emitter, b'L' as c_int) == 0 {
                        return 0;
                    }
                }
                0x2029 => {
                    if PUT(emitter, b'P' as c_int) == 0 {
                        return 0;
                    }
                }
                _ => {
                    if value <= 0xFF {
                        if PUT(emitter, b'x' as c_int) == 0 {
                            return 0;
                        }
                        width = 2;
                    } else if value <= 0xFFFF {
                        if PUT(emitter, b'u' as c_int) == 0 {
                            return 0;
                        }
                        width = 4;
                    } else {
                        if PUT(emitter, b'U' as c_int) == 0 {
                            return 0;
                        }
                        width = 8;
                    }
                    k = ((width - 1) * 4) as c_int;
                    while k >= 0 {
                        let digit: c_int = ((value >> k) & 0x0F) as c_int;
                        if PUT(
                            emitter,
                            digit
                                + if digit < 10 {
                                    b'0' as c_int
                                } else {
                                    (b'A' as c_int) - 10
                                },
                        ) == 0
                        {
                            return 0;
                        }
                        k -= 4;
                    }
                }
            }
            spaces = 0;
        } else if IS_SPACE(string) {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.offset(-1)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
                if IS_SPACE_AT(string, 1) {
                    if PUT(emitter, b'\\' as c_int) == 0 {
                        return 0;
                    }
                }
                string.pointer = string.pointer.offset(WIDTH(string) as isize);
            } else {
                if WRITE(emitter, &mut string) == 0 {
                    return 0;
                }
            }
            spaces = 1;
        } else {
            if WRITE(emitter, &mut string) == 0 {
                return 0;
            }
            spaces = 0;
        }
    }

    if yaml_emitter_write_indicator(emitter, b"\"\0".as_ptr() as *const c_char, 0, 0, 0) == 0 {
        return 0;
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;

    1
}

pub(crate) unsafe fn yaml_emitter_write_block_scalar_hints(
    emitter: *mut yaml_emitter_t,
    mut string: yaml_string_t,
) -> c_int {
    let mut indent_hint: [c_char; 2] = [0; 2];
    let mut chomp_hint: *const c_char = core::ptr::null();

    if IS_SPACE(string) || IS_BREAK(string) {
        indent_hint[0] = (b'0' as c_char) + ((*emitter).best_indent as c_char);
        indent_hint[1] = b'\0' as c_char;
        if yaml_emitter_write_indicator(emitter, indent_hint.as_ptr(), 0, 0, 0) == 0 {
            return 0;
        }
    }

    (*emitter).open_ended = 0;

    string.pointer = string.end;
    if string.start == string.pointer {
        chomp_hint = b"-\0".as_ptr() as *const c_char;
    } else {
        loop {
            string.pointer = string.pointer.offset(-1);
            if (*string.pointer & 0xC0) != 0x80 {
                break;
            }
        }
        if !IS_BREAK(string) {
            chomp_hint = b"-\0".as_ptr() as *const c_char;
        } else if string.start == string.pointer {
            chomp_hint = b"+\0".as_ptr() as *const c_char;
            (*emitter).open_ended = 2;
        } else {
            loop {
                string.pointer = string.pointer.offset(-1);
                if (*string.pointer & 0xC0) != 0x80 {
                    break;
                }
            }
            if IS_BREAK(string) {
                chomp_hint = b"+\0".as_ptr() as *const c_char;
                (*emitter).open_ended = 2;
            }
        }
    }

    if !chomp_hint.is_null() {
        if yaml_emitter_write_indicator(emitter, chomp_hint, 0, 0, 0) == 0 {
            return 0;
        }
    }

    1
}

pub(crate) unsafe fn yaml_emitter_write_literal_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    let mut string: yaml_string_t;
    let mut breaks: c_int = 1;

    string = STRING_ASSIGN(value, length);

    if yaml_emitter_write_indicator(emitter, b"|\0".as_ptr() as *const c_char, 1, 0, 0) == 0 {
        return 0;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0;
    }
    if PUT_BREAK(emitter) == 0 {
        return 0;
    }
    (*emitter).indention = 1;
    (*emitter).whitespace = 1;

    while string.pointer != string.end {
        if IS_BREAK(string) {
            if WRITE_BREAK(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
            }
            if WRITE(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 0;
            breaks = 0;
        }
    }

    1
}

pub(crate) unsafe fn yaml_emitter_write_folded_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    let mut string: yaml_string_t;
    let mut breaks: c_int = 1;
    let mut leading_spaces: c_int = 1;

    string = STRING_ASSIGN(value, length);

    if yaml_emitter_write_indicator(emitter, b">\0".as_ptr() as *const c_char, 1, 0, 0) == 0 {
        return 0;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0;
    }
    if PUT_BREAK(emitter) == 0 {
        return 0;
    }
    (*emitter).indention = 1;
    (*emitter).whitespace = 1;

    while string.pointer != string.end {
        if IS_BREAK(string) {
            if breaks == 0 && leading_spaces == 0 && CHECK(string, b'\n') {
                let mut k: c_int = 0;
                while IS_BREAK_AT(string, k as isize) {
                    k += WIDTH_AT(string, k as isize);
                }
                if !IS_BLANKZ_AT(string, k as isize) {
                    if PUT_BREAK(emitter) == 0 {
                        return 0;
                    }
                }
            }
            if WRITE_BREAK(emitter, &mut string) == 0 {
                return 0;
            }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
                leading_spaces = if IS_BLANK(string) { 1 } else { 0 };
            }
            if breaks == 0
                && IS_SPACE(string)
                && !IS_SPACE_AT(string, 1)
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0;
                }
                string.pointer = string.pointer.offset(WIDTH(string) as isize);
            } else {
                if WRITE(emitter, &mut string) == 0 {
                    return 0;
                }
            }
            (*emitter).indention = 0;
            breaks = 0;
        }
    }

    1
}
