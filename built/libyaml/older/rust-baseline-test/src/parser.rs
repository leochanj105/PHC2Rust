//! Rust transliteration of libyaml's parser.c.
//!
//! Implements the YAML event parser: the state machine that turns a stream
//! of tokens produced by the scanner into a stream of events. Every
//! non-static C function is exposed as `#[no_mangle] pub unsafe extern "C"`
//! and every static function is kept module-private.

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
use crate::{PUSH, POP, STACK_INIT, STACK_DEL, STACK_EMPTY, STACK_LIMIT};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};

/*
 * The parser implements the following grammar:
 *
 * stream               ::= STREAM-START implicit_document? explicit_document* STREAM-END
 * implicit_document    ::= block_node DOCUMENT-END*
 * explicit_document    ::= DIRECTIVE* DOCUMENT-START block_node? DOCUMENT-END*
 * block_node_or_indentless_sequence    ::=
 *                          ALIAS
 *                          | properties (block_content | indentless_block_sequence)?
 *                          | block_content
 *                          | indentless_block_sequence
 * block_node           ::= ALIAS
 *                          | properties block_content?
 *                          | block_content
 * flow_node            ::= ALIAS
 *                          | properties flow_content?
 *                          | flow_content
 * properties           ::= TAG ANCHOR? | ANCHOR TAG?
 * block_content        ::= block_collection | flow_collection | SCALAR
 * flow_content         ::= flow_collection | SCALAR
 * block_collection     ::= block_sequence | block_mapping
 * flow_collection      ::= flow_sequence | flow_mapping
 * block_sequence       ::= BLOCK-SEQUENCE-START (BLOCK-ENTRY block_node?)* BLOCK-END
 * indentless_sequence  ::= (BLOCK-ENTRY block_node?)+
 * block_mapping        ::= BLOCK-MAPPING_START
 *                          ((KEY block_node_or_indentless_sequence?)?
 *                          (VALUE block_node_or_indentless_sequence?)?)*
 *                          BLOCK-END
 * flow_sequence        ::= FLOW-SEQUENCE-START
 *                          (flow_sequence_entry FLOW-ENTRY)*
 *                          flow_sequence_entry?
 *                          FLOW-SEQUENCE-END
 * flow_sequence_entry  ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 * flow_mapping         ::= FLOW-MAPPING-START
 *                          (flow_mapping_entry FLOW-ENTRY)*
 *                          flow_mapping_entry?
 *                          FLOW-MAPPING-END
 * flow_mapping_entry   ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 */

/*
 * Peek the next token in the token queue.
 */

#[inline]
unsafe fn PEEK_TOKEN(parser: *mut yaml_parser_t) -> *mut yaml_token_t {
    if (*parser).token_available != 0 || yaml_parser_fetch_more_tokens(parser) != 0 {
        (*parser).tokens.head
    } else {
        core::ptr::null_mut()
    }
}

/*
 * Remove the next token from the queue (must be called after PEEK_TOKEN).
 */

#[inline]
unsafe fn SKIP_TOKEN(parser: *mut yaml_parser_t) {
    (*parser).token_available = 0;
    (*parser).tokens_parsed = (*parser).tokens_parsed.wrapping_add(1);
    (*parser).stream_end_produced =
        ((*(*parser).tokens.head).type_ == YAML_STREAM_END_TOKEN) as c_int;
    (*parser).tokens.head = (*parser).tokens.head.add(1);
}

/*
 * Public API declarations.
 */

static mut max_nest_level: c_int = 1000;

#[no_mangle]
pub unsafe extern "C" fn yaml_set_max_nest_level(max: c_int) {
    max_nest_level = max;
}

/*
 * Get the next event.
 */

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_parse(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    assert!(!parser.is_null()); /* Non-NULL parser object is expected. */
    assert!(!event.is_null()); /* Non-NULL event object is expected. */

    /* Erase the event object. */

    memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());

    /* No events after the end of the stream or error. */

    if (*parser).stream_end_produced != 0
        || (*parser).error != YAML_NO_ERROR
        || (*parser).state == YAML_PARSE_END_STATE
    {
        return 1;
    }

    /* Generate the next event. */

    yaml_parser_state_machine(parser, event)
}

/*
 * Set parser error.
 */

pub(crate) unsafe fn yaml_parser_set_parser_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    (*parser).error = YAML_PARSER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;

    0
}

pub(crate) unsafe fn yaml_parser_set_parser_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    (*parser).error = YAML_PARSER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;

    0
}

pub(crate) unsafe fn yaml_maximum_level_reached(
    parser: *mut yaml_parser_t,
    context_mark: yaml_mark_t,
    problem_mark: yaml_mark_t,
) -> c_int {
    yaml_parser_set_parser_error_context(
        parser,
        b"while parsing\0".as_ptr() as *const c_char,
        context_mark,
        b"Maximum nesting level reached, set with yaml_set_max_nest_level())\0".as_ptr()
            as *const c_char,
        problem_mark,
    );
    0
}

/*
 * State dispatcher.
 */

pub(crate) unsafe fn yaml_parser_state_machine(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    match (*parser).state {
        x if x == YAML_PARSE_STREAM_START_STATE => {
            return yaml_parser_parse_stream_start(parser, event);
        }

        x if x == YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE => {
            return yaml_parser_parse_document_start(parser, event, 1);
        }

        x if x == YAML_PARSE_DOCUMENT_START_STATE => {
            return yaml_parser_parse_document_start(parser, event, 0);
        }

        x if x == YAML_PARSE_DOCUMENT_CONTENT_STATE => {
            return yaml_parser_parse_document_content(parser, event);
        }

        x if x == YAML_PARSE_DOCUMENT_END_STATE => {
            return yaml_parser_parse_document_end(parser, event);
        }

        x if x == YAML_PARSE_BLOCK_NODE_STATE => {
            return yaml_parser_parse_node(parser, event, 1, 0);
        }

        x if x == YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE => {
            return yaml_parser_parse_node(parser, event, 1, 1);
        }

        x if x == YAML_PARSE_FLOW_NODE_STATE => {
            return yaml_parser_parse_node(parser, event, 0, 0);
        }

        x if x == YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE => {
            return yaml_parser_parse_block_sequence_entry(parser, event, 1);
        }

        x if x == YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE => {
            return yaml_parser_parse_block_sequence_entry(parser, event, 0);
        }

        x if x == YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE => {
            return yaml_parser_parse_indentless_sequence_entry(parser, event);
        }

        x if x == YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE => {
            return yaml_parser_parse_block_mapping_key(parser, event, 1);
        }

        x if x == YAML_PARSE_BLOCK_MAPPING_KEY_STATE => {
            return yaml_parser_parse_block_mapping_key(parser, event, 0);
        }

        x if x == YAML_PARSE_BLOCK_MAPPING_VALUE_STATE => {
            return yaml_parser_parse_block_mapping_value(parser, event);
        }

        x if x == YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 1);
        }

        x if x == YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE => {
            return yaml_parser_parse_flow_sequence_entry(parser, event, 0);
        }

        x if x == YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE => {
            return yaml_parser_parse_flow_sequence_entry_mapping_key(parser, event);
        }

        x if x == YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE => {
            return yaml_parser_parse_flow_sequence_entry_mapping_value(parser, event);
        }

        x if x == YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE => {
            return yaml_parser_parse_flow_sequence_entry_mapping_end(parser, event);
        }

        x if x == YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE => {
            return yaml_parser_parse_flow_mapping_key(parser, event, 1);
        }

        x if x == YAML_PARSE_FLOW_MAPPING_KEY_STATE => {
            return yaml_parser_parse_flow_mapping_key(parser, event, 0);
        }

        x if x == YAML_PARSE_FLOW_MAPPING_VALUE_STATE => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 0);
        }

        x if x == YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE => {
            return yaml_parser_parse_flow_mapping_value(parser, event, 1);
        }

        _ => {
            assert!(true); /* Invalid state. */
        }
    }

    0
}

/*
 * Parse the production:
 * stream   ::= STREAM-START implicit_document? explicit_document* STREAM-END
 *              ************
 */

pub(crate) unsafe fn yaml_parser_parse_stream_start(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ != YAML_STREAM_START_TOKEN {
        return yaml_parser_set_parser_error(
            parser,
            b"did not find expected <stream-start>\0".as_ptr() as *const c_char,
            (*token).start_mark,
        );
    }

    (*parser).state = YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE;
    EVENT_INIT(
        event,
        YAML_STREAM_START_EVENT,
        (*token).start_mark,
        (*token).start_mark,
    );
    (*event).data.stream_start.encoding = (*token).data.stream_start.encoding;
    SKIP_TOKEN(parser);

    1
}

/*
 * Parse the productions:
 * implicit_document    ::= block_node DOCUMENT-END*
 *                          *
 * explicit_document    ::= DIRECTIVE* DOCUMENT-START block_node? DOCUMENT-END*
 *                          *************************
 */

pub(crate) unsafe fn yaml_parser_parse_document_start(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    implicit: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;
    let mut version_directive: *mut yaml_version_directive_t = core::ptr::null_mut();
    #[repr(C)]
    struct TagDirectives {
        start: *mut yaml_tag_directive_t,
        end: *mut yaml_tag_directive_t,
    }
    let mut tag_directives = TagDirectives {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
    };

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    /* Parse extra document end indicators. */

    if implicit == 0 {
        while (*token).type_ == YAML_DOCUMENT_END_TOKEN {
            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                return 0;
            }
        }
    }

    /* Parse an implicit document. */

    if implicit != 0
        && (*token).type_ != YAML_VERSION_DIRECTIVE_TOKEN
        && (*token).type_ != YAML_TAG_DIRECTIVE_TOKEN
        && (*token).type_ != YAML_DOCUMENT_START_TOKEN
        && (*token).type_ != YAML_STREAM_END_TOKEN
    {
        if yaml_parser_process_directives(
            parser,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        ) == 0
        {
            return 0;
        }
        if PUSH!(parser, (*parser).states, YAML_PARSE_DOCUMENT_END_STATE) == 0 {
            return 0;
        }
        (*parser).state = YAML_PARSE_BLOCK_NODE_STATE;
        EVENT_INIT(
            event,
            YAML_DOCUMENT_START_EVENT,
            (*token).start_mark,
            (*token).start_mark,
        );
        (*event).data.document_start.version_directive = core::ptr::null_mut();
        (*event).data.document_start.tag_directives.start = core::ptr::null_mut();
        (*event).data.document_start.tag_directives.end = core::ptr::null_mut();
        (*event).data.document_start.implicit = 1;
        return 1;
    }
    /* Parse an explicit document. */
    else if (*token).type_ != YAML_STREAM_END_TOKEN {
        let mut start_mark: yaml_mark_t;
        let mut end_mark: yaml_mark_t;
        start_mark = (*token).start_mark;
        'error: loop {
            if yaml_parser_process_directives(
                parser,
                &mut version_directive,
                &mut tag_directives.start,
                &mut tag_directives.end,
            ) == 0
            {
                return 0;
            }
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                break 'error;
            }
            if (*token).type_ != YAML_DOCUMENT_START_TOKEN {
                yaml_parser_set_parser_error(
                    parser,
                    b"did not find expected <document start>\0".as_ptr() as *const c_char,
                    (*token).start_mark,
                );
                break 'error;
            }
            if PUSH!(parser, (*parser).states, YAML_PARSE_DOCUMENT_END_STATE) == 0 {
                break 'error;
            }
            (*parser).state = YAML_PARSE_DOCUMENT_CONTENT_STATE;
            end_mark = (*token).end_mark;
            EVENT_INIT(event, YAML_DOCUMENT_START_EVENT, start_mark, end_mark);
            (*event).data.document_start.version_directive = version_directive;
            (*event).data.document_start.tag_directives.start = tag_directives.start;
            (*event).data.document_start.tag_directives.end = tag_directives.end;
            (*event).data.document_start.implicit = 0;
            SKIP_TOKEN(parser);
            version_directive = core::ptr::null_mut();
            tag_directives.start = core::ptr::null_mut();
            tag_directives.end = core::ptr::null_mut();
            return 1;
        }
        // fall through to error handling
    }
    /* Parse the stream end. */
    else {
        (*parser).state = YAML_PARSE_END_STATE;
        EVENT_INIT(
            event,
            YAML_STREAM_END_EVENT,
            (*token).start_mark,
            (*token).end_mark,
        );
        SKIP_TOKEN(parser);
        return 1;
    }

    // error:
    yaml_free(version_directive as *mut c_void);
    while tag_directives.start != tag_directives.end {
        yaml_free((*tag_directives.end.offset(-1)).handle as *mut c_void);
        yaml_free((*tag_directives.end.offset(-1)).prefix as *mut c_void);
        tag_directives.end = tag_directives.end.offset(-1);
    }
    yaml_free(tag_directives.start as *mut c_void);
    0
}

/*
 * Parse the productions:
 * explicit_document    ::= DIRECTIVE* DOCUMENT-START block_node? DOCUMENT-END*
 *                                                    ***********
 */

pub(crate) unsafe fn yaml_parser_parse_document_content(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_VERSION_DIRECTIVE_TOKEN
        || (*token).type_ == YAML_TAG_DIRECTIVE_TOKEN
        || (*token).type_ == YAML_DOCUMENT_START_TOKEN
        || (*token).type_ == YAML_DOCUMENT_END_TOKEN
        || (*token).type_ == YAML_STREAM_END_TOKEN
    {
        (*parser).state = POP!(parser, (*parser).states);
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    } else {
        return yaml_parser_parse_node(parser, event, 1, 0);
    }
}

/*
 * Parse the productions:
 * implicit_document    ::= block_node DOCUMENT-END*
 *                                     *************
 * explicit_document    ::= DIRECTIVE* DOCUMENT-START block_node? DOCUMENT-END*
 *                                                                *************
 */

pub(crate) unsafe fn yaml_parser_parse_document_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;
    let mut start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut implicit: c_int = 1;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    start_mark = (*token).start_mark;
    end_mark = (*token).start_mark;

    if (*token).type_ == YAML_DOCUMENT_END_TOKEN {
        end_mark = (*token).end_mark;
        SKIP_TOKEN(parser);
        implicit = 0;
    }

    while !STACK_EMPTY!(parser, (*parser).tag_directives) {
        let tag_directive: yaml_tag_directive_t = POP!(parser, (*parser).tag_directives);
        yaml_free(tag_directive.handle as *mut c_void);
        yaml_free(tag_directive.prefix as *mut c_void);
    }

    (*parser).state = YAML_PARSE_DOCUMENT_START_STATE;
    EVENT_INIT(event, YAML_DOCUMENT_END_EVENT, start_mark, end_mark);
    (*event).data.document_end.implicit = implicit;

    1
}

/*
 * Parse the productions:
 * block_node_or_indentless_sequence    ::=
 *                          ALIAS
 *                          *****
 *                          | properties (block_content | indentless_block_sequence)?
 *                            **********  *
 *                          | block_content | indentless_block_sequence
 *                            *
 * block_node           ::= ALIAS
 *                          *****
 *                          | properties block_content?
 *                            ********** *
 *                          | block_content
 *                            *
 * flow_node            ::= ALIAS
 *                          *****
 *                          | properties flow_content?
 *                            ********** *
 *                          | flow_content
 *                            *
 * properties           ::= TAG ANCHOR? | ANCHOR TAG?
 *                          *************************
 * block_content        ::= block_collection | flow_collection | SCALAR
 *                                                               ******
 * flow_content         ::= flow_collection | SCALAR
 *                                            ******
 */

pub(crate) unsafe fn yaml_parser_parse_node(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    block: c_int,
    indentless_sequence: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;
    let mut anchor: *mut yaml_char_t = core::ptr::null_mut();
    let mut tag_handle: *mut yaml_char_t = core::ptr::null_mut();
    let mut tag_suffix: *mut yaml_char_t = core::ptr::null_mut();
    let mut tag: *mut yaml_char_t = core::ptr::null_mut();
    let mut start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut tag_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut implicit: c_int;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_ALIAS_TOKEN {
        (*parser).state = POP!(parser, (*parser).states);
        EVENT_INIT(
            event,
            YAML_ALIAS_EVENT,
            (*token).start_mark,
            (*token).end_mark,
        );
        (*event).data.alias.anchor = (*token).data.alias.value;
        SKIP_TOKEN(parser);
        return 1;
    } else {
        start_mark = (*token).start_mark;
        end_mark = (*token).start_mark;

        'error: loop {
            if (*token).type_ == YAML_ANCHOR_TOKEN {
                anchor = (*token).data.anchor.value;
                start_mark = (*token).start_mark;
                end_mark = (*token).end_mark;
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    break 'error;
                }
                if (*token).type_ == YAML_TAG_TOKEN {
                    tag_handle = (*token).data.tag.handle;
                    tag_suffix = (*token).data.tag.suffix;
                    tag_mark = (*token).start_mark;
                    end_mark = (*token).end_mark;
                    SKIP_TOKEN(parser);
                    token = PEEK_TOKEN(parser);
                    if token.is_null() {
                        break 'error;
                    }
                }
            } else if (*token).type_ == YAML_TAG_TOKEN {
                tag_handle = (*token).data.tag.handle;
                tag_suffix = (*token).data.tag.suffix;
                start_mark = (*token).start_mark;
                tag_mark = (*token).start_mark;
                end_mark = (*token).end_mark;
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    break 'error;
                }
                if (*token).type_ == YAML_ANCHOR_TOKEN {
                    anchor = (*token).data.anchor.value;
                    end_mark = (*token).end_mark;
                    SKIP_TOKEN(parser);
                    token = PEEK_TOKEN(parser);
                    if token.is_null() {
                        break 'error;
                    }
                }
            }

            if !tag_handle.is_null() {
                if *tag_handle == 0 {
                    tag = tag_suffix;
                    yaml_free(tag_handle as *mut c_void);
                    tag_handle = core::ptr::null_mut();
                    tag_suffix = core::ptr::null_mut();
                } else {
                    let mut tag_directive: *mut yaml_tag_directive_t;
                    tag_directive = (*parser).tag_directives.start;
                    while tag_directive != (*parser).tag_directives.top {
                        if libc::strcmp(
                            (*tag_directive).handle as *const c_char,
                            tag_handle as *const c_char,
                        ) == 0
                        {
                            let prefix_len: size_t =
                                libc::strlen((*tag_directive).prefix as *const c_char);
                            let suffix_len: size_t = libc::strlen(tag_suffix as *const c_char);
                            tag = YAML_MALLOC(prefix_len + suffix_len + 1);
                            if tag.is_null() {
                                (*parser).error = YAML_MEMORY_ERROR;
                                break 'error;
                            }
                            memcpy(
                                tag as *mut c_void,
                                (*tag_directive).prefix as *const c_void,
                                prefix_len,
                            );
                            memcpy(
                                tag.add(prefix_len) as *mut c_void,
                                tag_suffix as *const c_void,
                                suffix_len,
                            );
                            *tag.add(prefix_len + suffix_len) = b'\0';
                            yaml_free(tag_handle as *mut c_void);
                            yaml_free(tag_suffix as *mut c_void);
                            tag_handle = core::ptr::null_mut();
                            tag_suffix = core::ptr::null_mut();
                            break;
                        }
                        tag_directive = tag_directive.add(1);
                    }
                    if tag.is_null() {
                        yaml_parser_set_parser_error_context(
                            parser,
                            b"while parsing a node\0".as_ptr() as *const c_char,
                            start_mark,
                            b"found undefined tag handle\0".as_ptr() as *const c_char,
                            tag_mark,
                        );
                        break 'error;
                    }
                }
            }

            implicit = (tag.is_null() || *tag == 0) as c_int;
            if indentless_sequence != 0 && (*token).type_ == YAML_BLOCK_ENTRY_TOKEN {
                end_mark = (*token).end_mark;
                (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
                EVENT_INIT(event, YAML_SEQUENCE_START_EVENT, start_mark, end_mark);
                (*event).data.sequence_start.anchor = anchor;
                (*event).data.sequence_start.tag = tag;
                (*event).data.sequence_start.implicit = implicit;
                (*event).data.sequence_start.style = YAML_BLOCK_SEQUENCE_STYLE;
                return 1;
            } else {
                if (*token).type_ == YAML_SCALAR_TOKEN {
                    let mut plain_implicit: c_int = 0;
                    let mut quoted_implicit: c_int = 0;
                    end_mark = (*token).end_mark;
                    if ((*token).data.scalar.style == YAML_PLAIN_SCALAR_STYLE && tag.is_null())
                        || (!tag.is_null()
                            && libc::strcmp(
                                tag as *const c_char,
                                b"!\0".as_ptr() as *const c_char,
                            ) == 0)
                    {
                        plain_implicit = 1;
                    } else if tag.is_null() {
                        quoted_implicit = 1;
                    }
                    (*parser).state = POP!(parser, (*parser).states);
                    EVENT_INIT(event, YAML_SCALAR_EVENT, start_mark, end_mark);
                    (*event).data.scalar.anchor = anchor;
                    (*event).data.scalar.tag = tag;
                    (*event).data.scalar.value = (*token).data.scalar.value;
                    (*event).data.scalar.length = (*token).data.scalar.length;
                    (*event).data.scalar.plain_implicit = plain_implicit;
                    (*event).data.scalar.quoted_implicit = quoted_implicit;
                    (*event).data.scalar.style = (*token).data.scalar.style;
                    SKIP_TOKEN(parser);
                    return 1;
                } else if (*token).type_ == YAML_FLOW_SEQUENCE_START_TOKEN {
                    if STACK_LIMIT!(
                        parser,
                        (*parser).indents,
                        max_nest_level - (*parser).flow_level
                    ) == 0
                    {
                        yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
                        break 'error;
                    }
                    end_mark = (*token).end_mark;
                    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE;
                    EVENT_INIT(event, YAML_SEQUENCE_START_EVENT, start_mark, end_mark);
                    (*event).data.sequence_start.anchor = anchor;
                    (*event).data.sequence_start.tag = tag;
                    (*event).data.sequence_start.implicit = implicit;
                    (*event).data.sequence_start.style = YAML_FLOW_SEQUENCE_STYLE;
                    return 1;
                } else if (*token).type_ == YAML_FLOW_MAPPING_START_TOKEN {
                    if STACK_LIMIT!(
                        parser,
                        (*parser).indents,
                        max_nest_level - (*parser).flow_level
                    ) == 0
                    {
                        yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
                        break 'error;
                    }
                    end_mark = (*token).end_mark;
                    (*parser).state = YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE;
                    EVENT_INIT(event, YAML_MAPPING_START_EVENT, start_mark, end_mark);
                    (*event).data.mapping_start.anchor = anchor;
                    (*event).data.mapping_start.tag = tag;
                    (*event).data.mapping_start.implicit = implicit;
                    (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
                    return 1;
                } else if block != 0 && (*token).type_ == YAML_BLOCK_SEQUENCE_START_TOKEN {
                    if STACK_LIMIT!(
                        parser,
                        (*parser).indents,
                        max_nest_level - (*parser).flow_level
                    ) == 0
                    {
                        yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
                        break 'error;
                    }
                    end_mark = (*token).end_mark;
                    (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE;
                    EVENT_INIT(event, YAML_SEQUENCE_START_EVENT, start_mark, end_mark);
                    (*event).data.sequence_start.anchor = anchor;
                    (*event).data.sequence_start.tag = tag;
                    (*event).data.sequence_start.implicit = implicit;
                    (*event).data.sequence_start.style = YAML_BLOCK_SEQUENCE_STYLE;
                    return 1;
                } else if block != 0 && (*token).type_ == YAML_BLOCK_MAPPING_START_TOKEN {
                    if STACK_LIMIT!(
                        parser,
                        (*parser).indents,
                        max_nest_level - (*parser).flow_level
                    ) == 0
                    {
                        yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
                        break 'error;
                    }
                    end_mark = (*token).end_mark;
                    (*parser).state = YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE;
                    EVENT_INIT(event, YAML_MAPPING_START_EVENT, start_mark, end_mark);
                    (*event).data.mapping_start.anchor = anchor;
                    (*event).data.mapping_start.tag = tag;
                    (*event).data.mapping_start.implicit = implicit;
                    (*event).data.mapping_start.style = YAML_BLOCK_MAPPING_STYLE;
                    return 1;
                } else if !anchor.is_null() || !tag.is_null() {
                    let value: *mut yaml_char_t = YAML_MALLOC(1);
                    if value.is_null() {
                        (*parser).error = YAML_MEMORY_ERROR;
                        break 'error;
                    }
                    *value.add(0) = b'\0';
                    (*parser).state = POP!(parser, (*parser).states);
                    EVENT_INIT(event, YAML_SCALAR_EVENT, start_mark, end_mark);
                    (*event).data.scalar.anchor = anchor;
                    (*event).data.scalar.tag = tag;
                    (*event).data.scalar.value = value;
                    (*event).data.scalar.length = 0;
                    (*event).data.scalar.plain_implicit = implicit;
                    (*event).data.scalar.quoted_implicit = 0;
                    (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                    return 1;
                } else {
                    yaml_parser_set_parser_error_context(
                        parser,
                        if block != 0 {
                            b"while parsing a block node\0".as_ptr() as *const c_char
                        } else {
                            b"while parsing a flow node\0".as_ptr() as *const c_char
                        },
                        start_mark,
                        b"did not find expected node content\0".as_ptr() as *const c_char,
                        (*token).start_mark,
                    );
                    break 'error;
                }
            }
        }
    }

    // error:
    yaml_free(anchor as *mut c_void);
    yaml_free(tag_handle as *mut c_void);
    yaml_free(tag_suffix as *mut c_void);
    yaml_free(tag as *mut c_void);

    0
}

/*
 * Parse the productions:
 * block_sequence ::= BLOCK-SEQUENCE-START (BLOCK-ENTRY block_node?)* BLOCK-END
 *                    ********************  *********** *             *********
 */

pub(crate) unsafe fn yaml_parser_parse_block_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;

    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return 0;
        }
        SKIP_TOKEN(parser);
    }

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_BLOCK_ENTRY_TOKEN {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return 0;
        }
        if (*token).type_ != YAML_BLOCK_ENTRY_TOKEN && (*token).type_ != YAML_BLOCK_END_TOKEN {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 1, 0);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_ == YAML_BLOCK_END_TOKEN {
        (*parser).state = POP!(parser, (*parser).states);
        let _ = POP!(parser, (*parser).marks);
        EVENT_INIT(
            event,
            YAML_SEQUENCE_END_EVENT,
            (*token).start_mark,
            (*token).end_mark,
        );
        SKIP_TOKEN(parser);
        return 1;
    } else {
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block collection\0".as_ptr() as *const c_char,
            POP!(parser, (*parser).marks),
            b"did not find expected '-' indicator\0".as_ptr() as *const c_char,
            (*token).start_mark,
        );
    }
}

/*
 * Parse the productions:
 * indentless_sequence  ::= (BLOCK-ENTRY block_node?)+
 *                           *********** *
 */

pub(crate) unsafe fn yaml_parser_parse_indentless_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_BLOCK_ENTRY_TOKEN {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return 0;
        }
        if (*token).type_ != YAML_BLOCK_ENTRY_TOKEN
            && (*token).type_ != YAML_KEY_TOKEN
            && (*token).type_ != YAML_VALUE_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 1, 0);
        } else {
            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        (*parser).state = POP!(parser, (*parser).states);
        EVENT_INIT(
            event,
            YAML_SEQUENCE_END_EVENT,
            (*token).start_mark,
            (*token).start_mark,
        );
        return 1;
    }
}

/*
 * Parse the productions:
 * block_mapping        ::= BLOCK-MAPPING_START
 *                          *******************
 *                          ((KEY block_node_or_indentless_sequence?)?
 *                            *** *
 *                          (VALUE block_node_or_indentless_sequence?)?)*
 *
 *                          BLOCK-END
 *                          *********
 */

pub(crate) unsafe fn yaml_parser_parse_block_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;

    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return 0;
        }
        SKIP_TOKEN(parser);
    }

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_KEY_TOKEN {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return 0;
        }
        if (*token).type_ != YAML_KEY_TOKEN
            && (*token).type_ != YAML_VALUE_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_BLOCK_MAPPING_VALUE_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 1, 1);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_ == YAML_BLOCK_END_TOKEN {
        (*parser).state = POP!(parser, (*parser).states);
        let _ = POP!(parser, (*parser).marks);
        EVENT_INIT(
            event,
            YAML_MAPPING_END_EVENT,
            (*token).start_mark,
            (*token).end_mark,
        );
        SKIP_TOKEN(parser);
        return 1;
    } else {
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block mapping\0".as_ptr() as *const c_char,
            POP!(parser, (*parser).marks),
            b"did not find expected key\0".as_ptr() as *const c_char,
            (*token).start_mark,
        );
    }
}

/*
 * Parse the productions:
 * block_mapping        ::= BLOCK-MAPPING_START
 *
 *                          ((KEY block_node_or_indentless_sequence?)?
 *
 *                          (VALUE block_node_or_indentless_sequence?)?)*
 *                           ***** *
 *                          BLOCK-END
 *
 */

pub(crate) unsafe fn yaml_parser_parse_block_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_VALUE_TOKEN {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return 0;
        }
        if (*token).type_ != YAML_KEY_TOKEN
            && (*token).type_ != YAML_VALUE_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_BLOCK_MAPPING_KEY_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 1, 1);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        (*parser).state = YAML_PARSE_BLOCK_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }
}

/*
 * Parse the productions:
 * flow_sequence        ::= FLOW-SEQUENCE-START
 *                          *******************
 *                          (flow_sequence_entry FLOW-ENTRY)*
 *                           *                   **********
 *                          flow_sequence_entry?
 *                          *
 *                          FLOW-SEQUENCE-END
 *                          *****************
 * flow_sequence_entry  ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 *                          *
 */

pub(crate) unsafe fn yaml_parser_parse_flow_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;

    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return 0;
        }
        SKIP_TOKEN(parser);
    }

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN {
        if first == 0 {
            if (*token).type_ == YAML_FLOW_ENTRY_TOKEN {
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    return 0;
                }
            } else {
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow sequence\0".as_ptr() as *const c_char,
                    POP!(parser, (*parser).marks),
                    b"did not find expected ',' or ']'\0".as_ptr() as *const c_char,
                    (*token).start_mark,
                );
            }
        }

        if (*token).type_ == YAML_KEY_TOKEN {
            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE;
            EVENT_INIT(
                event,
                YAML_MAPPING_START_EVENT,
                (*token).start_mark,
                (*token).end_mark,
            );
            (*event).data.mapping_start.anchor = core::ptr::null_mut();
            (*event).data.mapping_start.tag = core::ptr::null_mut();
            (*event).data.mapping_start.implicit = 1;
            (*event).data.mapping_start.style = YAML_FLOW_MAPPING_STYLE;
            SKIP_TOKEN(parser);
            return 1;
        } else if (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = POP!(parser, (*parser).states);
    let _ = POP!(parser, (*parser).marks);
    EVENT_INIT(
        event,
        YAML_SEQUENCE_END_EVENT,
        (*token).start_mark,
        (*token).end_mark,
    );
    SKIP_TOKEN(parser);
    1
}

/*
 * Parse the productions:
 * flow_sequence_entry  ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 *                                      *** *
 */

pub(crate) unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ != YAML_VALUE_TOKEN
        && (*token).type_ != YAML_FLOW_ENTRY_TOKEN
        && (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN
    {
        if PUSH!(
            parser,
            (*parser).states,
            YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE
        ) == 0
        {
            return 0;
        }
        return yaml_parser_parse_node(parser, event, 0, 0);
    } else if (*token).type_ == YAML_FLOW_SEQUENCE_END_TOKEN {
        let mark: yaml_mark_t = (*token).start_mark;
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        return yaml_parser_process_empty_scalar(parser, event, mark);
    } else {
        let mark: yaml_mark_t = (*token).end_mark;
        SKIP_TOKEN(parser);
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        return yaml_parser_process_empty_scalar(parser, event, mark);
    }
}

/*
 * Parse the productions:
 * flow_sequence_entry  ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 *                                                      ***** *
 */

pub(crate) unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ == YAML_VALUE_TOKEN {
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return 0;
        }
        if (*token).type_ != YAML_FLOW_ENTRY_TOKEN
            && (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }
    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
    yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
}

/*
 * Parse the productions:
 * flow_sequence_entry  ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 *                                                                      *
 */

pub(crate) unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    let token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;

    EVENT_INIT(
        event,
        YAML_MAPPING_END_EVENT,
        (*token).start_mark,
        (*token).start_mark,
    );
    1
}

/*
 * Parse the productions:
 * flow_mapping         ::= FLOW-MAPPING-START
 *                          ******************
 *                          (flow_mapping_entry FLOW-ENTRY)*
 *                           *                  **********
 *                          flow_mapping_entry?
 *                          ******************
 *                          FLOW-MAPPING-END
 *                          ****************
 * flow_mapping_entry   ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 *                          *           *** *
 */

pub(crate) unsafe fn yaml_parser_parse_flow_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;

    if first != 0 {
        token = PEEK_TOKEN(parser);
        if PUSH!(parser, (*parser).marks, (*token).start_mark) == 0 {
            return 0;
        }
        SKIP_TOKEN(parser);
    }

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN {
        if first == 0 {
            if (*token).type_ == YAML_FLOW_ENTRY_TOKEN {
                SKIP_TOKEN(parser);
                token = PEEK_TOKEN(parser);
                if token.is_null() {
                    return 0;
                }
            } else {
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow mapping\0".as_ptr() as *const c_char,
                    POP!(parser, (*parser).marks),
                    b"did not find expected ',' or '}'\0".as_ptr() as *const c_char,
                    (*token).start_mark,
                );
            }
        }

        if (*token).type_ == YAML_KEY_TOKEN {
            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                return 0;
            }
            if (*token).type_ != YAML_VALUE_TOKEN
                && (*token).type_ != YAML_FLOW_ENTRY_TOKEN
                && (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN
            {
                if PUSH!(
                    parser,
                    (*parser).states,
                    YAML_PARSE_FLOW_MAPPING_VALUE_STATE
                ) == 0
                {
                    return 0;
                }
                return yaml_parser_parse_node(parser, event, 0, 0);
            } else {
                (*parser).state = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
            }
        } else if (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = POP!(parser, (*parser).states);
    let _ = POP!(parser, (*parser).marks);
    EVENT_INIT(
        event,
        YAML_MAPPING_END_EVENT,
        (*token).start_mark,
        (*token).end_mark,
    );
    SKIP_TOKEN(parser);
    1
}

/*
 * Parse the productions:
 * flow_mapping_entry   ::= flow_node | KEY flow_node? (VALUE flow_node?)?
 *                                   *                  ***** *
 */

pub(crate) unsafe fn yaml_parser_parse_flow_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    empty: c_int,
) -> c_int {
    let mut token: *mut yaml_token_t;

    token = PEEK_TOKEN(parser);
    if token.is_null() {
        return 0;
    }

    if empty != 0 {
        (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }

    if (*token).type_ == YAML_VALUE_TOKEN {
        SKIP_TOKEN(parser);
        token = PEEK_TOKEN(parser);
        if token.is_null() {
            return 0;
        }
        if (*token).type_ != YAML_FLOW_ENTRY_TOKEN && (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN
        {
            if PUSH!(
                parser,
                (*parser).states,
                YAML_PARSE_FLOW_MAPPING_KEY_STATE
            ) == 0
            {
                return 0;
            }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
    yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
}

/*
 * Generate an empty scalar event.
 */

pub(crate) unsafe fn yaml_parser_process_empty_scalar(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    mark: yaml_mark_t,
) -> c_int {
    let value: *mut yaml_char_t;

    value = YAML_MALLOC(1);
    if value.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0;
    }
    *value.add(0) = b'\0';

    EVENT_INIT(event, YAML_SCALAR_EVENT, mark, mark);
    (*event).data.scalar.anchor = core::ptr::null_mut();
    (*event).data.scalar.tag = core::ptr::null_mut();
    (*event).data.scalar.value = value;
    (*event).data.scalar.length = 0;
    (*event).data.scalar.plain_implicit = 1;
    (*event).data.scalar.quoted_implicit = 0;
    (*event).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;

    1
}

/*
 * Parse directives.
 */

pub(crate) unsafe fn yaml_parser_process_directives(
    parser: *mut yaml_parser_t,
    version_directive_ref: *mut *mut yaml_version_directive_t,
    tag_directives_start_ref: *mut *mut yaml_tag_directive_t,
    tag_directives_end_ref: *mut *mut yaml_tag_directive_t,
) -> c_int {
    let default_tag_directives: [yaml_tag_directive_t; 3] = [
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
    let mut default_tag_directive: *const yaml_tag_directive_t;
    let mut version_directive: *mut yaml_version_directive_t = core::ptr::null_mut();
    let mut tag_directives = yaml_stack_tag_directive_t {
        start: core::ptr::null_mut(),
        end: core::ptr::null_mut(),
        top: core::ptr::null_mut(),
    };
    let mut token: *mut yaml_token_t;

    'error: loop {
        if STACK_INIT!(parser, tag_directives, yaml_tag_directive_t) == 0 {
            break 'error;
        }

        token = PEEK_TOKEN(parser);
        if token.is_null() {
            break 'error;
        }

        while (*token).type_ == YAML_VERSION_DIRECTIVE_TOKEN
            || (*token).type_ == YAML_TAG_DIRECTIVE_TOKEN
        {
            if (*token).type_ == YAML_VERSION_DIRECTIVE_TOKEN {
                if !version_directive.is_null() {
                    yaml_parser_set_parser_error(
                        parser,
                        b"found duplicate %YAML directive\0".as_ptr() as *const c_char,
                        (*token).start_mark,
                    );
                    break 'error;
                }
                if (*token).data.version_directive.major != 1
                    || ((*token).data.version_directive.minor != 1
                        && (*token).data.version_directive.minor != 2)
                {
                    yaml_parser_set_parser_error(
                        parser,
                        b"found incompatible YAML document\0".as_ptr() as *const c_char,
                        (*token).start_mark,
                    );
                    break 'error;
                }
                version_directive = YAML_MALLOC_STATIC::<yaml_version_directive_t>();
                if version_directive.is_null() {
                    (*parser).error = YAML_MEMORY_ERROR;
                    break 'error;
                }
                (*version_directive).major = (*token).data.version_directive.major;
                (*version_directive).minor = (*token).data.version_directive.minor;
            } else if (*token).type_ == YAML_TAG_DIRECTIVE_TOKEN {
                let value = yaml_tag_directive_t {
                    handle: (*token).data.tag_directive.handle,
                    prefix: (*token).data.tag_directive.prefix,
                };

                if yaml_parser_append_tag_directive(parser, value, 0, (*token).start_mark) == 0 {
                    break 'error;
                }
                if PUSH!(parser, tag_directives, value) == 0 {
                    break 'error;
                }
            }

            SKIP_TOKEN(parser);
            token = PEEK_TOKEN(parser);
            if token.is_null() {
                break 'error;
            }
        }

        default_tag_directive = default_tag_directives.as_ptr();
        while !(*default_tag_directive).handle.is_null() {
            if yaml_parser_append_tag_directive(
                parser,
                *default_tag_directive,
                1,
                (*token).start_mark,
            ) == 0
            {
                break 'error;
            }
            default_tag_directive = default_tag_directive.add(1);
        }

        if !version_directive_ref.is_null() {
            *version_directive_ref = version_directive;
        }
        if !tag_directives_start_ref.is_null() {
            if STACK_EMPTY!(parser, tag_directives) {
                *tag_directives_start_ref = core::ptr::null_mut();
                *tag_directives_end_ref = core::ptr::null_mut();
                STACK_DEL!(parser, tag_directives);
            } else {
                *tag_directives_start_ref = tag_directives.start;
                *tag_directives_end_ref = tag_directives.top;
            }
        } else {
            STACK_DEL!(parser, tag_directives);
        }

        if version_directive_ref.is_null() {
            yaml_free(version_directive as *mut c_void);
        }
        return 1;
    }

    // error:
    yaml_free(version_directive as *mut c_void);
    while !STACK_EMPTY!(parser, tag_directives) {
        let tag_directive: yaml_tag_directive_t = POP!(parser, tag_directives);
        yaml_free(tag_directive.handle as *mut c_void);
        yaml_free(tag_directive.prefix as *mut c_void);
    }
    STACK_DEL!(parser, tag_directives);
    0
}

/*
 * Append a tag directive to the directives stack.
 */

pub(crate) unsafe fn yaml_parser_append_tag_directive(
    parser: *mut yaml_parser_t,
    value: yaml_tag_directive_t,
    allow_duplicates: c_int,
    mark: yaml_mark_t,
) -> c_int {
    let mut tag_directive: *mut yaml_tag_directive_t;
    let mut copy = yaml_tag_directive_t {
        handle: core::ptr::null_mut(),
        prefix: core::ptr::null_mut(),
    };

    tag_directive = (*parser).tag_directives.start;
    while tag_directive != (*parser).tag_directives.top {
        if libc::strcmp(
            value.handle as *const c_char,
            (*tag_directive).handle as *const c_char,
        ) == 0
        {
            if allow_duplicates != 0 {
                return 1;
            }
            return yaml_parser_set_parser_error(
                parser,
                b"found duplicate %TAG directive\0".as_ptr() as *const c_char,
                mark,
            );
        }
        tag_directive = tag_directive.add(1);
    }

    'error: loop {
        copy.handle = yaml_strdup(value.handle);
        copy.prefix = yaml_strdup(value.prefix);
        if copy.handle.is_null() || copy.prefix.is_null() {
            (*parser).error = YAML_MEMORY_ERROR;
            break 'error;
        }

        if PUSH!(parser, (*parser).tag_directives, copy) == 0 {
            break 'error;
        }

        return 1;
    }

    // error:
    yaml_free(copy.handle as *mut c_void);
    yaml_free(copy.prefix as *mut c_void);
    0
}
