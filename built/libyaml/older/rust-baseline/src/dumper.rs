//! Rust transliteration of libyaml's dumper.c.

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
use crate::{STACK_INIT, STACK_DEL, STACK_EMPTY, PUSH, POP, QUEUE_INIT, QUEUE_DEL, QUEUE_EMPTY, DEQUEUE};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};

extern "C" {
    pub fn yaml_emitter_emit(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> c_int;
}

use crate::api::yaml_document_delete;
use crate::writer::yaml_emitter_flush;

/*
 * Clean up functions.
 */

/*
 * Anchor functions.
 */

/*
 * Serialize functions.
 */

/*
 * Issue a STREAM-START event.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_open(emitter: *mut yaml_emitter_t) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    /* assert(emitter);            Non-NULL emitter object is required. */
    /* assert(!emitter->opened);   Emitter should not be opened yet. */

    // STREAM_START_EVENT_INIT(event, YAML_ANY_ENCODING, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_STREAM_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.stream_start.encoding = YAML_ANY_ENCODING;

    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    (*emitter).opened = 1;

    return 1;
}

/*
 * Issue a STREAM-END event.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_close(emitter: *mut yaml_emitter_t) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    /* assert(emitter);            Non-NULL emitter object is required. */
    /* assert(emitter->opened);    Emitter should be opened. */

    if (*emitter).closed != 0 {
        return 1;
    }

    // STREAM_END_EVENT_INIT(event, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_STREAM_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;

    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    (*emitter).closed = 1;

    return 1;
}

/*
 * Dump a YAML document.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_dump(
    emitter: *mut yaml_emitter_t,
    document: *mut yaml_document_t,
) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    /* assert(emitter);            Non-NULL emitter object is required. */
    /* assert(document);           Non-NULL emitter object is expected. */

    (*emitter).document = document;

    'error: loop {
        if (*emitter).opened == 0 {
            if yaml_emitter_open(emitter) == 0 {
                break 'error;
            }
        }

        if STACK_EMPTY!(emitter, (*document).nodes) {
            if yaml_emitter_close(emitter) == 0 {
                break 'error;
            }
            yaml_emitter_delete_document_and_anchors(emitter);
            return 1;
        }

        /* assert(emitter->opened);    Emitter should be opened. */

        (*emitter).anchors = yaml_malloc(
            (core::mem::size_of::<yaml_anchors_t>() as size_t)
                * ((*document).nodes.top.offset_from((*document).nodes.start) as size_t),
        ) as *mut yaml_anchors_t;
        if (*emitter).anchors.is_null() {
            break 'error;
        }
        libc::memset(
            (*emitter).anchors as *mut c_void,
            0,
            (core::mem::size_of::<yaml_anchors_t>() as size_t)
                * ((*document).nodes.top.offset_from((*document).nodes.start) as size_t),
        );

        // DOCUMENT_START_EVENT_INIT(event, document->version_directive,
        //         document->tag_directives.start, document->tag_directives.end,
        //         document->start_implicit, mark, mark);
        libc::memset(
            &mut event as *mut yaml_event_t as *mut c_void,
            0,
            core::mem::size_of::<yaml_event_t>(),
        );
        event.type_ = YAML_DOCUMENT_START_EVENT;
        event.start_mark = mark;
        event.end_mark = mark;
        event.data.document_start.version_directive = (*document).version_directive;
        event.data.document_start.tag_directives.start = (*document).tag_directives.start;
        event.data.document_start.tag_directives.end = (*document).tag_directives.end;
        event.data.document_start.implicit = (*document).start_implicit;
        if yaml_emitter_emit(emitter, &mut event) == 0 {
            break 'error;
        }

        yaml_emitter_anchor_node(emitter, 1);
        if yaml_emitter_dump_node(emitter, 1) == 0 {
            break 'error;
        }

        // DOCUMENT_END_EVENT_INIT(event, document->end_implicit, mark, mark);
        libc::memset(
            &mut event as *mut yaml_event_t as *mut c_void,
            0,
            core::mem::size_of::<yaml_event_t>(),
        );
        event.type_ = YAML_DOCUMENT_END_EVENT;
        event.start_mark = mark;
        event.end_mark = mark;
        event.data.document_end.implicit = (*document).end_implicit;
        if yaml_emitter_emit(emitter, &mut event) == 0 {
            break 'error;
        }

        yaml_emitter_delete_document_and_anchors(emitter);

        return 1;
    }

    // error:

    yaml_emitter_delete_document_and_anchors(emitter);

    return 0;
}

/*
 * Clean up the emitter object after a document is dumped.
 */

unsafe fn yaml_emitter_delete_document_and_anchors(emitter: *mut yaml_emitter_t) {
    let mut index: c_int;

    if (*emitter).anchors.is_null() {
        yaml_document_delete((*emitter).document);
        (*emitter).document = core::ptr::null_mut();
        return;
    }

    index = 0;
    while (*(*emitter).document).nodes.start.offset(index as isize)
        < (*(*emitter).document).nodes.top
    {
        let mut node: yaml_node_t = *(*(*emitter).document).nodes.start.offset(index as isize);
        if (*(*emitter).anchors.offset(index as isize)).serialized == 0 {
            yaml_free(node.tag as *mut c_void);
            if node.type_ == YAML_SCALAR_NODE {
                yaml_free(node.data.scalar.value as *mut c_void);
            }
        }
        if node.type_ == YAML_SEQUENCE_NODE {
            STACK_DEL!(emitter, node.data.sequence.items);
        }
        if node.type_ == YAML_MAPPING_NODE {
            STACK_DEL!(emitter, node.data.mapping.pairs);
        }
        index += 1;
    }

    STACK_DEL!(emitter, (*(*emitter).document).nodes);
    yaml_free((*emitter).anchors as *mut c_void);

    (*emitter).anchors = core::ptr::null_mut();
    (*emitter).last_anchor_id = 0;
    (*emitter).document = core::ptr::null_mut();
}

/*
 * Check the references of a node and assign the anchor id if needed.
 */

unsafe fn yaml_emitter_anchor_node(emitter: *mut yaml_emitter_t, index: c_int) {
    let node: *mut yaml_node_t =
        (*(*emitter).document).nodes.start.offset((index - 1) as isize);
    let mut item: *mut yaml_node_item_t;
    let mut pair: *mut yaml_node_pair_t;

    (*(*emitter).anchors.offset((index - 1) as isize)).references += 1;

    if (*(*emitter).anchors.offset((index - 1) as isize)).references == 1 {
        match (*node).type_ {
            YAML_SEQUENCE_NODE => {
                item = (*node).data.sequence.items.start;
                while item < (*node).data.sequence.items.top {
                    yaml_emitter_anchor_node(emitter, *item);
                    item = item.offset(1);
                }
            }
            YAML_MAPPING_NODE => {
                pair = (*node).data.mapping.pairs.start;
                while pair < (*node).data.mapping.pairs.top {
                    yaml_emitter_anchor_node(emitter, (*pair).key);
                    yaml_emitter_anchor_node(emitter, (*pair).value);
                    pair = pair.offset(1);
                }
            }
            _ => {}
        }
    } else if (*(*emitter).anchors.offset((index - 1) as isize)).references == 2 {
        (*emitter).last_anchor_id += 1;
        (*(*emitter).anchors.offset((index - 1) as isize)).anchor = (*emitter).last_anchor_id;
    }
}

/*
 * Generate a textual representation for an anchor.
 */

const ANCHOR_TEMPLATE: &[u8] = b"id%03d\0";
const ANCHOR_TEMPLATE_LENGTH: size_t = 16;

unsafe fn yaml_emitter_generate_anchor(
    _emitter: *mut yaml_emitter_t,
    anchor_id: c_int,
) -> *mut yaml_char_t {
    let anchor: *mut yaml_char_t = YAML_MALLOC(ANCHOR_TEMPLATE_LENGTH);

    if anchor.is_null() {
        return core::ptr::null_mut();
    }

    libc::sprintf(
        anchor as *mut c_char,
        ANCHOR_TEMPLATE.as_ptr() as *const c_char,
        anchor_id,
    );

    return anchor;
}

/*
 * Serialize a node.
 */

unsafe fn yaml_emitter_dump_node(emitter: *mut yaml_emitter_t, index: c_int) -> c_int {
    let node: *mut yaml_node_t =
        (*(*emitter).document).nodes.start.offset((index - 1) as isize);
    let anchor_id: c_int = (*(*emitter).anchors.offset((index - 1) as isize)).anchor;
    let mut anchor: *mut yaml_char_t = core::ptr::null_mut();

    if anchor_id != 0 {
        anchor = yaml_emitter_generate_anchor(emitter, anchor_id);
        if anchor.is_null() {
            return 0;
        }
    }

    if (*(*emitter).anchors.offset((index - 1) as isize)).serialized != 0 {
        return yaml_emitter_dump_alias(emitter, anchor);
    }

    (*(*emitter).anchors.offset((index - 1) as isize)).serialized = 1;

    match (*node).type_ {
        YAML_SCALAR_NODE => {
            return yaml_emitter_dump_scalar(emitter, node, anchor);
        }
        YAML_SEQUENCE_NODE => {
            return yaml_emitter_dump_sequence(emitter, node, anchor);
        }
        YAML_MAPPING_NODE => {
            return yaml_emitter_dump_mapping(emitter, node, anchor);
        }
        _ => {
            /* assert(0);      Could not happen. */
        }
    }

    return 0; /* Could not happen. */
}

/*
 * Serialize an alias.
 */

unsafe fn yaml_emitter_dump_alias(
    emitter: *mut yaml_emitter_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    // ALIAS_EVENT_INIT(event, anchor, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_ALIAS_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.alias.anchor = anchor;

    return yaml_emitter_emit(emitter, &mut event);
}

/*
 * Serialize a scalar.
 */

unsafe fn yaml_emitter_dump_scalar(
    emitter: *mut yaml_emitter_t,
    node: *mut yaml_node_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    let plain_implicit: c_int = (libc::strcmp(
        (*node).tag as *mut c_char,
        YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const c_char,
    ) == 0) as c_int;
    let quoted_implicit: c_int = (libc::strcmp(
        (*node).tag as *mut c_char,
        YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const c_char,
    ) == 0) as c_int;

    // SCALAR_EVENT_INIT(event, anchor, node->tag, node->data.scalar.value,
    //         node->data.scalar.length, plain_implicit, quoted_implicit,
    //         node->data.scalar.style, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_SCALAR_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.scalar.anchor = anchor;
    event.data.scalar.tag = (*node).tag;
    event.data.scalar.value = (*node).data.scalar.value;
    event.data.scalar.length = (*node).data.scalar.length;
    event.data.scalar.plain_implicit = plain_implicit;
    event.data.scalar.quoted_implicit = quoted_implicit;
    event.data.scalar.style = (*node).data.scalar.style;

    return yaml_emitter_emit(emitter, &mut event);
}

/*
 * Serialize a sequence.
 */

unsafe fn yaml_emitter_dump_sequence(
    emitter: *mut yaml_emitter_t,
    node: *mut yaml_node_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    let implicit: c_int = (libc::strcmp(
        (*node).tag as *mut c_char,
        YAML_DEFAULT_SEQUENCE_TAG.as_ptr() as *const c_char,
    ) == 0) as c_int;

    let mut item: *mut yaml_node_item_t;

    // SEQUENCE_START_EVENT_INIT(event, anchor, node->tag, implicit,
    //         node->data.sequence.style, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_SEQUENCE_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.sequence_start.anchor = anchor;
    event.data.sequence_start.tag = (*node).tag;
    event.data.sequence_start.implicit = implicit;
    event.data.sequence_start.style = (*node).data.sequence.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    item = (*node).data.sequence.items.start;
    while item < (*node).data.sequence.items.top {
        if yaml_emitter_dump_node(emitter, *item) == 0 {
            return 0;
        }
        item = item.offset(1);
    }

    // SEQUENCE_END_EVENT_INIT(event, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_SEQUENCE_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    return 1;
}

/*
 * Serialize a mapping.
 */

unsafe fn yaml_emitter_dump_mapping(
    emitter: *mut yaml_emitter_t,
    node: *mut yaml_node_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    let mut event: yaml_event_t = core::mem::zeroed();
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    let implicit: c_int = (libc::strcmp(
        (*node).tag as *mut c_char,
        YAML_DEFAULT_MAPPING_TAG.as_ptr() as *const c_char,
    ) == 0) as c_int;

    let mut pair: *mut yaml_node_pair_t;

    // MAPPING_START_EVENT_INIT(event, anchor, node->tag, implicit,
    //         node->data.mapping.style, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_MAPPING_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.mapping_start.anchor = anchor;
    event.data.mapping_start.tag = (*node).tag;
    event.data.mapping_start.implicit = implicit;
    event.data.mapping_start.style = (*node).data.mapping.style;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    pair = (*node).data.mapping.pairs.start;
    while pair < (*node).data.mapping.pairs.top {
        if yaml_emitter_dump_node(emitter, (*pair).key) == 0 {
            return 0;
        }
        if yaml_emitter_dump_node(emitter, (*pair).value) == 0 {
            return 0;
        }
        pair = pair.offset(1);
    }

    // MAPPING_END_EVENT_INIT(event, mark, mark);
    libc::memset(
        &mut event as *mut yaml_event_t as *mut c_void,
        0,
        core::mem::size_of::<yaml_event_t>(),
    );
    event.type_ = YAML_MAPPING_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    return 1;
}
