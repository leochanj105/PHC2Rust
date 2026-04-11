//! Rust transliteration of libyaml's `scanner.c`.
//!
//! The scanner converts a raw YAML byte stream into a queue of tokens that the
//! parser consumes. Every macro, helper, and static function from the C source
//! is preserved here line-for-line so output matches libyaml byte-for-byte.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::externs::*;
use crate::yaml::*;
use crate::yaml_private::{self, *};
use crate::{
    CLEAR, DEQUEUE, ENQUEUE, JOIN, POP, PUSH, QUEUE_DEL, QUEUE_EMPTY, QUEUE_INIT, QUEUE_INSERT,
    STACK_DEL, STACK_EMPTY, STACK_INIT, STRING_DEL, STRING_EXTEND, STRING_INIT,
};
use libc::{c_char, c_int, c_uchar, c_uint, c_void, size_t};

/* --------------------------------------------------------------------- */
/*  Buffer / mark helpers                                                 */
/* --------------------------------------------------------------------- */

/// Construct a `yaml_string_t` view of `parser->buffer` so that the
/// character-classification helpers (which take a `yaml_string_t` by value)
/// can be called on the parser buffer.
#[inline]
unsafe fn BUFFER_STRING(parser: *mut yaml_parser_t) -> yaml_string_t {
    yaml_string_t {
        start: (*parser).buffer.start,
        end: (*parser).buffer.last,
        pointer: (*parser).buffer.pointer,
    }
}

/// C macro `CACHE(parser, length)` — ensure the buffer contains at least
/// `length` characters by calling `yaml_parser_update_buffer` if needed.
#[inline]
unsafe fn CACHE(parser: *mut yaml_parser_t, length: size_t) -> c_int {
    if (*parser).unread >= length {
        1
    } else {
        yaml_parser_update_buffer(parser, length)
    }
}

/// C macro `SKIP(parser)` — advance the buffer pointer by one UTF-8 character.
#[inline]
unsafe fn SKIP(parser: *mut yaml_parser_t) {
    let w = WIDTH(BUFFER_STRING(parser)) as isize;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(w);
}

/// C macro `SKIP_LINE(parser)` — advance past a line break.
#[inline]
unsafe fn SKIP_LINE(parser: *mut yaml_parser_t) {
    if IS_CRLF(BUFFER_STRING(parser)) {
        (*parser).mark.index = (*parser).mark.index.wrapping_add(2);
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(2);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
    } else if IS_BREAK(BUFFER_STRING(parser)) {
        let w = WIDTH(BUFFER_STRING(parser)) as isize;
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(w);
    }
}

/// C macro `READ(parser,string)` — copy one UTF-8 character from the parser
/// buffer into `string` and advance both pointers.
#[inline]
unsafe fn READ(parser: *mut yaml_parser_t, string: &mut yaml_string_t) -> c_int {
    if STRING_EXTEND!(parser, (*string)) == 0 {
        return 0;
    }
    // Replicate COPY(string, parser->buffer):
    let p = (*parser).buffer.pointer;
    let c = *p;
    if (c & 0x80) == 0x00 {
        *string.pointer = *p;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
    } else if (c & 0xE0) == 0xC0 {
        *string.pointer = *p;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
    } else if (c & 0xF0) == 0xE0 {
        *string.pointer = *p;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
    } else if (c & 0xF8) == 0xF0 {
        *string.pointer = *p;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
    }
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    1
}

/// C macro `READ_LINE(parser,string)` — copy a line break from the parser
/// buffer into `string`, normalising it.
#[inline]
unsafe fn READ_LINE(parser: *mut yaml_parser_t, string: &mut yaml_string_t) -> c_int {
    if STRING_EXTEND!(parser, (*string)) == 0 {
        return 0;
    }
    let bs = BUFFER_STRING(parser);
    if CHECK_AT(bs, b'\r', 0) && CHECK_AT(bs, b'\n', 1) {
        /* CR LF -> LF */
        *string.pointer = b'\n' as yaml_char_t;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
        (*parser).mark.index = (*parser).mark.index.wrapping_add(2);
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(2);
    } else if CHECK_AT(bs, b'\r', 0) || CHECK_AT(bs, b'\n', 0) {
        /* CR|LF -> LF */
        *string.pointer = b'\n' as yaml_char_t;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
    } else if CHECK_AT(bs, 0xC2, 0) && CHECK_AT(bs, 0x85, 1) {
        /* NEL -> LF */
        *string.pointer = b'\n' as yaml_char_t;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
    } else if CHECK_AT(bs, 0xE2, 0)
        && CHECK_AT(bs, 0x80, 1)
        && (CHECK_AT(bs, 0xA8, 2) || CHECK_AT(bs, 0xA9, 2))
    {
        /* LS|PS -> LS|PS */
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *string.pointer = *(*parser).buffer.pointer;
        string.pointer = string.pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
    }
    1
}

/// Reach into the token queue to obtain the next unread token, lazily fetching
/// more tokens if needed. Mirrors `PEEK_TOKEN` in libyaml.
#[inline]
unsafe fn PEEK_TOKEN(parser: *mut yaml_parser_t) -> *mut yaml_token_t {
    if (*parser).token_available != 0 || yaml_parser_fetch_more_tokens(parser) != 0 {
        (*parser).tokens.head
    } else {
        core::ptr::null_mut()
    }
}

/* --------------------------------------------------------------------- */
/*  Public API: yaml_parser_scan                                          */
/* --------------------------------------------------------------------- */

/// Get the next token. Public API — mirrors `yaml_parser_scan`.
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_scan(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    assert!(!parser.is_null()); /* Non-NULL parser object is expected. */
    assert!(!token.is_null()); /* Non-NULL token object is expected. */

    /* Erase the token object. */

    libc::memset(token as *mut c_void, 0, core::mem::size_of::<yaml_token_t>());

    /* No tokens after STREAM-END or error. */

    if (*parser).stream_end_produced != 0 || (*parser).error != YAML_NO_ERROR {
        return 1;
    }

    /* Ensure that the tokens queue contains enough tokens. */

    if (*parser).token_available == 0 {
        if yaml_parser_fetch_more_tokens(parser) == 0 {
            return 0;
        }
    }

    /* Fetch the next token from the queue. */

    *token = DEQUEUE!(parser, (*parser).tokens);
    (*parser).token_available = 0;
    (*parser).tokens_parsed = (*parser).tokens_parsed.wrapping_add(1);

    if (*token).type_ == YAML_STREAM_END_TOKEN {
        (*parser).stream_end_produced = 1;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Error handling                                                        */
/* --------------------------------------------------------------------- */

/// Set the scanner error and return 0.
pub(crate) unsafe fn yaml_parser_set_scanner_error(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
) -> c_int {
    (*parser).error = YAML_SCANNER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = (*parser).mark;

    0
}

/* --------------------------------------------------------------------- */
/*  Public API: yaml_parser_fetch_more_tokens                             */
/* --------------------------------------------------------------------- */

/// Ensure that the tokens queue contains at least one token which can be
/// returned to the Parser.
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_fetch_more_tokens(parser: *mut yaml_parser_t) -> c_int {
    let mut need_more_tokens: c_int;

    /* While we need more tokens to fetch, do it. */

    loop {
        /*
         * Check if we really need to fetch more tokens.
         */

        need_more_tokens = 0;

        if (*parser).tokens.head == (*parser).tokens.tail {
            /* Queue is empty. */

            need_more_tokens = 1;
        } else {
            /* Check if any potential simple key may occupy the head position. */

            if yaml_parser_stale_simple_keys(parser) == 0 {
                return 0;
            }

            let mut simple_key: *mut yaml_simple_key_t = (*parser).simple_keys.start;
            while simple_key != (*parser).simple_keys.top {
                if (*simple_key).possible != 0
                    && (*simple_key).token_number == (*parser).tokens_parsed
                {
                    need_more_tokens = 1;
                    break;
                }
                simple_key = simple_key.add(1);
            }
        }

        /* We are finished. */

        if need_more_tokens == 0 {
            break;
        }

        /* Fetch the next token. */

        if yaml_parser_fetch_next_token(parser) == 0 {
            return 0;
        }
    }

    (*parser).token_available = 1;

    1
}

/* --------------------------------------------------------------------- */
/*  Token fetcher dispatch                                                */
/* --------------------------------------------------------------------- */

/// The dispatcher for token fetchers.
pub(crate) unsafe fn yaml_parser_fetch_next_token(parser: *mut yaml_parser_t) -> c_int {
    /* Ensure that the buffer is initialized. */

    if CACHE(parser, 1) == 0 {
        return 0;
    }

    /* Check if we just started scanning.  Fetch STREAM-START then. */

    if (*parser).stream_start_produced == 0 {
        return yaml_parser_fetch_stream_start(parser);
    }

    /* Eat whitespaces and comments until we reach the next token. */

    if yaml_parser_scan_to_next_token(parser) == 0 {
        return 0;
    }

    /* Remove obsolete potential simple keys. */

    if yaml_parser_stale_simple_keys(parser) == 0 {
        return 0;
    }

    /* Check the indentation level against the current column. */

    if yaml_parser_unroll_indent(parser, (*parser).mark.column as isize) == 0 {
        return 0;
    }

    /*
     * Ensure that the buffer contains at least 4 characters.  4 is the length
     * of the longest indicators ('--- ' and '... ').
     */

    if CACHE(parser, 4) == 0 {
        return 0;
    }

    /* Is it the end of the stream? */

    if IS_Z(BUFFER_STRING(parser)) {
        return yaml_parser_fetch_stream_end(parser);
    }

    /* Is it a directive? */

    if (*parser).mark.column == 0 && CHECK(BUFFER_STRING(parser), b'%') {
        return yaml_parser_fetch_directive(parser);
    }

    /* Is it the document start indicator? */

    if (*parser).mark.column == 0
        && CHECK_AT(BUFFER_STRING(parser), b'-', 0)
        && CHECK_AT(BUFFER_STRING(parser), b'-', 1)
        && CHECK_AT(BUFFER_STRING(parser), b'-', 2)
        && IS_BLANKZ_AT(BUFFER_STRING(parser), 3)
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }

    /* Is it the document end indicator? */

    if (*parser).mark.column == 0
        && CHECK_AT(BUFFER_STRING(parser), b'.', 0)
        && CHECK_AT(BUFFER_STRING(parser), b'.', 1)
        && CHECK_AT(BUFFER_STRING(parser), b'.', 2)
        && IS_BLANKZ_AT(BUFFER_STRING(parser), 3)
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }

    /* Is it the flow sequence start indicator? */

    if CHECK(BUFFER_STRING(parser), b'[') {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_SEQUENCE_START_TOKEN);
    }

    /* Is it the flow mapping start indicator? */

    if CHECK(BUFFER_STRING(parser), b'{') {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_MAPPING_START_TOKEN);
    }

    /* Is it the flow sequence end indicator? */

    if CHECK(BUFFER_STRING(parser), b']') {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_SEQUENCE_END_TOKEN);
    }

    /* Is it the flow mapping end indicator? */

    if CHECK(BUFFER_STRING(parser), b'}') {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_MAPPING_END_TOKEN);
    }

    /* Is it the flow entry indicator? */

    if CHECK(BUFFER_STRING(parser), b',') {
        return yaml_parser_fetch_flow_entry(parser);
    }

    /* Is it the block entry indicator? */

    if CHECK(BUFFER_STRING(parser), b'-') && IS_BLANKZ_AT(BUFFER_STRING(parser), 1) {
        return yaml_parser_fetch_block_entry(parser);
    }

    /* Is it the key indicator? */

    if CHECK(BUFFER_STRING(parser), b'?')
        && ((*parser).flow_level != 0 || IS_BLANKZ_AT(BUFFER_STRING(parser), 1))
    {
        return yaml_parser_fetch_key(parser);
    }

    /* Is it the value indicator? */

    if CHECK(BUFFER_STRING(parser), b':')
        && ((*parser).flow_level != 0 || IS_BLANKZ_AT(BUFFER_STRING(parser), 1))
    {
        return yaml_parser_fetch_value(parser);
    }

    /* Is it an alias? */

    if CHECK(BUFFER_STRING(parser), b'*') {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }

    /* Is it an anchor? */

    if CHECK(BUFFER_STRING(parser), b'&') {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }

    /* Is it a tag? */

    if CHECK(BUFFER_STRING(parser), b'!') {
        return yaml_parser_fetch_tag(parser);
    }

    /* Is it a literal scalar? */

    if CHECK(BUFFER_STRING(parser), b'|') && (*parser).flow_level == 0 {
        return yaml_parser_fetch_block_scalar(parser, 1);
    }

    /* Is it a folded scalar? */

    if CHECK(BUFFER_STRING(parser), b'>') && (*parser).flow_level == 0 {
        return yaml_parser_fetch_block_scalar(parser, 0);
    }

    /* Is it a single-quoted scalar? */

    if CHECK(BUFFER_STRING(parser), b'\'') {
        return yaml_parser_fetch_flow_scalar(parser, 1);
    }

    /* Is it a double-quoted scalar? */

    if CHECK(BUFFER_STRING(parser), b'"') {
        return yaml_parser_fetch_flow_scalar(parser, 0);
    }

    /*
     * Is it a plain scalar?
     *
     * A plain scalar may start with any non-blank characters except
     *
     *      '-', '?', ':', ',', '[', ']', '{', '}',
     *      '#', '&', '*', '!', '|', '>', '\'', '\"',
     *      '%', '@', '`'.
     *
     * In the block context (and, for the '-' indicator, in the flow context
     * too), it may also start with the characters
     *
     *      '-', '?', ':'
     *
     * if it is followed by a non-space character.
     *
     * The last rule is more restrictive than the specification requires.
     */

    if !(IS_BLANKZ(BUFFER_STRING(parser))
        || CHECK(BUFFER_STRING(parser), b'-')
        || CHECK(BUFFER_STRING(parser), b'?')
        || CHECK(BUFFER_STRING(parser), b':')
        || CHECK(BUFFER_STRING(parser), b',')
        || CHECK(BUFFER_STRING(parser), b'[')
        || CHECK(BUFFER_STRING(parser), b']')
        || CHECK(BUFFER_STRING(parser), b'{')
        || CHECK(BUFFER_STRING(parser), b'}')
        || CHECK(BUFFER_STRING(parser), b'#')
        || CHECK(BUFFER_STRING(parser), b'&')
        || CHECK(BUFFER_STRING(parser), b'*')
        || CHECK(BUFFER_STRING(parser), b'!')
        || CHECK(BUFFER_STRING(parser), b'|')
        || CHECK(BUFFER_STRING(parser), b'>')
        || CHECK(BUFFER_STRING(parser), b'\'')
        || CHECK(BUFFER_STRING(parser), b'"')
        || CHECK(BUFFER_STRING(parser), b'%')
        || CHECK(BUFFER_STRING(parser), b'@')
        || CHECK(BUFFER_STRING(parser), b'`'))
        || (CHECK(BUFFER_STRING(parser), b'-') && !IS_BLANK_AT(BUFFER_STRING(parser), 1))
        || ((*parser).flow_level == 0
            && (CHECK(BUFFER_STRING(parser), b'?') || CHECK(BUFFER_STRING(parser), b':'))
            && !IS_BLANKZ_AT(BUFFER_STRING(parser), 1))
    {
        return yaml_parser_fetch_plain_scalar(parser);
    }

    /*
     * If we don't determine the token type so far, it is an error.
     */

    yaml_parser_set_scanner_error(
        parser,
        b"while scanning for the next token\0".as_ptr() as *const c_char,
        (*parser).mark,
        b"found character that cannot start any token\0".as_ptr() as *const c_char,
    )
}

/* --------------------------------------------------------------------- */
/*  Simple keys                                                           */
/* --------------------------------------------------------------------- */

/// Check the list of potential simple keys and remove the positions that
/// cannot contain simple keys anymore.
pub(crate) unsafe fn yaml_parser_stale_simple_keys(parser: *mut yaml_parser_t) -> c_int {
    /* Check for a potential simple key for each flow level. */

    let mut simple_key: *mut yaml_simple_key_t = (*parser).simple_keys.start;
    while simple_key != (*parser).simple_keys.top {
        /*
         * The specification requires that a simple key
         *
         *  - is limited to a single line,
         *  - is shorter than 1024 characters.
         */

        if (*simple_key).possible != 0
            && ((*simple_key).mark.line < (*parser).mark.line
                || (*simple_key).mark.index.wrapping_add(1024) < (*parser).mark.index)
        {
            /* Check if the potential simple key to be removed is required. */

            if (*simple_key).required != 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a simple key\0".as_ptr() as *const c_char,
                    (*simple_key).mark,
                    b"could not find expected ':'\0".as_ptr() as *const c_char,
                );
            }

            (*simple_key).possible = 0;
        }
        simple_key = simple_key.add(1);
    }

    1
}

/// Check if a simple key may start at the current position and add it if
/// needed.
pub(crate) unsafe fn yaml_parser_save_simple_key(parser: *mut yaml_parser_t) -> c_int {
    /*
     * A simple key is required at the current position if the scanner is in
     * the block context and the current column coincides with the indentation
     * level.
     */

    let required: c_int = ((*parser).flow_level == 0
        && (*parser).indent as isize == (*parser).mark.column as isize) as c_int;

    /*
     * If the current position may start a simple key, save it.
     */

    if (*parser).simple_key_allowed != 0 {
        let mut simple_key: yaml_simple_key_t = yaml_simple_key_t {
            possible: 1,
            required,
            token_number: (*parser).tokens_parsed.wrapping_add(
                ((*parser).tokens.tail.offset_from((*parser).tokens.head)) as size_t,
            ),
            mark: (*parser).mark,
        };

        if yaml_parser_remove_simple_key(parser) == 0 {
            return 0;
        }

        *((*parser).simple_keys.top.offset(-1)) = simple_key;
    }

    1
}

/// Remove a potential simple key at the current flow level.
pub(crate) unsafe fn yaml_parser_remove_simple_key(parser: *mut yaml_parser_t) -> c_int {
    let simple_key: *mut yaml_simple_key_t = (*parser).simple_keys.top.offset(-1);

    if (*simple_key).possible != 0 {
        /* If the key is required, it is an error. */

        if (*simple_key).required != 0 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a simple key\0".as_ptr() as *const c_char,
                (*simple_key).mark,
                b"could not find expected ':'\0".as_ptr() as *const c_char,
            );
        }
    }

    /* Remove the key from the stack. */

    (*simple_key).possible = 0;

    1
}

/* --------------------------------------------------------------------- */
/*  Flow level                                                            */
/* --------------------------------------------------------------------- */

/// Increase the flow level and resize the simple key list if needed.
pub(crate) unsafe fn yaml_parser_increase_flow_level(parser: *mut yaml_parser_t) -> c_int {
    let empty_simple_key: yaml_simple_key_t = yaml_simple_key_t {
        possible: 0,
        required: 0,
        token_number: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };

    /* Reset the simple key on the next level. */

    if PUSH!(parser, (*parser).simple_keys, empty_simple_key) == 0 {
        return 0;
    }

    /* Increase the flow level. */

    if (*parser).flow_level == c_int::MAX {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0;
    }

    (*parser).flow_level += 1;

    1
}

/// Decrease the flow level.
pub(crate) unsafe fn yaml_parser_decrease_flow_level(parser: *mut yaml_parser_t) -> c_int {
    if (*parser).flow_level != 0 {
        (*parser).flow_level -= 1;
        let _ = POP!(parser, (*parser).simple_keys);
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Indentation                                                           */
/* --------------------------------------------------------------------- */

/// Push the current indentation level to the stack and set the new level if
/// the current column is greater than the indentation level. In this case,
/// append or insert the specified token into the token queue.
pub(crate) unsafe fn yaml_parser_roll_indent(
    parser: *mut yaml_parser_t,
    column: isize,
    number: isize,
    type_: yaml_token_type_t,
    mark: yaml_mark_t,
) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* In the flow context, do nothing. */

    if (*parser).flow_level != 0 {
        return 1;
    }

    if ((*parser).indent as isize) < column {
        /*
         * Push the current indentation level to the stack and set the new
         * indentation level.
         */

        if PUSH!(parser, (*parser).indents, (*parser).indent) == 0 {
            return 0;
        }

        if column > c_int::MAX as isize {
            (*parser).error = YAML_MEMORY_ERROR;
            return 0;
        }

        (*parser).indent = column as c_int;

        /* Create a token and insert it into the queue. */

        TOKEN_INIT(&mut token, type_, mark, mark);

        if number == -1 {
            if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
                return 0;
            }
        } else {
            if QUEUE_INSERT!(
                parser,
                (*parser).tokens,
                (number as size_t).wrapping_sub((*parser).tokens_parsed) as usize,
                token
            ) == 0
            {
                return 0;
            }
        }
    }

    1
}

/// Pop indentation levels from the indents stack until the current level
/// becomes less or equal to the column. For each indentation level, append
/// the BLOCK-END token.
pub(crate) unsafe fn yaml_parser_unroll_indent(parser: *mut yaml_parser_t, column: isize) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* In the flow context, do nothing. */

    if (*parser).flow_level != 0 {
        return 1;
    }

    /* Loop through the indentation levels in the stack. */

    while ((*parser).indent as isize) > column {
        /* Create a token and append it to the queue. */

        TOKEN_INIT(
            &mut token,
            YAML_BLOCK_END_TOKEN,
            (*parser).mark,
            (*parser).mark,
        );

        if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
            return 0;
        }

        /* Pop the indentation level. */

        (*parser).indent = POP!(parser, (*parser).indents);
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Stream start / end fetchers                                           */
/* --------------------------------------------------------------------- */

/// Initialize the scanner and produce the STREAM-START token.
pub(crate) unsafe fn yaml_parser_fetch_stream_start(parser: *mut yaml_parser_t) -> c_int {
    let simple_key: yaml_simple_key_t = yaml_simple_key_t {
        possible: 0,
        required: 0,
        token_number: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Set the initial indentation. */

    (*parser).indent = -1;

    /* Initialize the simple key stack. */

    if PUSH!(parser, (*parser).simple_keys, simple_key) == 0 {
        return 0;
    }

    /* A simple key is allowed at the beginning of the stream. */

    (*parser).simple_key_allowed = 1;

    /* We have started. */

    (*parser).stream_start_produced = 1;

    /* Create the STREAM-START token and append it to the queue. */

    TOKEN_INIT(
        &mut token,
        YAML_STREAM_START_TOKEN,
        (*parser).mark,
        (*parser).mark,
    );
    token.data.stream_start.encoding = (*parser).encoding;

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/// Produce the STREAM-END token and shut down the scanner.
pub(crate) unsafe fn yaml_parser_fetch_stream_end(parser: *mut yaml_parser_t) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Force new line. */

    if (*parser).mark.column != 0 {
        (*parser).mark.column = 0;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
    }

    /* Reset the indentation level. */

    if yaml_parser_unroll_indent(parser, -1) == 0 {
        return 0;
    }

    /* Reset simple keys. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    (*parser).simple_key_allowed = 0;

    /* Create the STREAM-END token and append it to the queue. */

    TOKEN_INIT(
        &mut token,
        YAML_STREAM_END_TOKEN,
        (*parser).mark,
        (*parser).mark,
    );

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Directive fetcher                                                     */
/* --------------------------------------------------------------------- */

/// Produce a VERSION-DIRECTIVE or TAG-DIRECTIVE token.
pub(crate) unsafe fn yaml_parser_fetch_directive(parser: *mut yaml_parser_t) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Reset the indentation level. */

    if yaml_parser_unroll_indent(parser, -1) == 0 {
        return 0;
    }

    /* Reset simple keys. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    (*parser).simple_key_allowed = 0;

    /* Create the YAML-DIRECTIVE or TAG-DIRECTIVE token. */

    if yaml_parser_scan_directive(parser, &mut token) == 0 {
        return 0;
    }

    /* Append the token to the queue. */

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        crate::api::yaml_token_delete(&mut token);
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Document indicator fetcher                                            */
/* --------------------------------------------------------------------- */

/// Produce the DOCUMENT-START or DOCUMENT-END token.
pub(crate) unsafe fn yaml_parser_fetch_document_indicator(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Reset the indentation level. */

    if yaml_parser_unroll_indent(parser, -1) == 0 {
        return 0;
    }

    /* Reset simple keys. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    (*parser).simple_key_allowed = 0;

    /* Consume the token. */

    start_mark = (*parser).mark;

    SKIP(parser);
    SKIP(parser);
    SKIP(parser);

    end_mark = (*parser).mark;

    /* Create the DOCUMENT-START or DOCUMENT-END token. */

    TOKEN_INIT(&mut token, type_, start_mark, end_mark);

    /* Append the token to the queue. */

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Flow collection fetchers                                              */
/* --------------------------------------------------------------------- */

/// Produce the FLOW-SEQUENCE-START or FLOW-MAPPING-START token.
pub(crate) unsafe fn yaml_parser_fetch_flow_collection_start(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();

    /* The indicators '[' and '{' may start a simple key. */

    if yaml_parser_save_simple_key(parser) == 0 {
        return 0;
    }

    /* Increase the flow level. */

    if yaml_parser_increase_flow_level(parser) == 0 {
        return 0;
    }

    /* A simple key may follow the indicators '[' and '{'. */

    (*parser).simple_key_allowed = 1;

    /* Consume the token. */

    start_mark = (*parser).mark;
    SKIP(parser);
    end_mark = (*parser).mark;

    /* Create the FLOW-SEQUENCE-START of FLOW-MAPPING-START token. */

    TOKEN_INIT(&mut token, type_, start_mark, end_mark);

    /* Append the token to the queue. */

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/// Produce the FLOW-SEQUENCE-END or FLOW-MAPPING-END token.
pub(crate) unsafe fn yaml_parser_fetch_flow_collection_end(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Reset any potential simple key on the current flow level. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    /* Decrease the flow level. */

    if yaml_parser_decrease_flow_level(parser) == 0 {
        return 0;
    }

    /* No simple keys after the indicators ']' and '}'. */

    (*parser).simple_key_allowed = 0;

    /* Consume the token. */

    start_mark = (*parser).mark;
    SKIP(parser);
    end_mark = (*parser).mark;

    /* Create the FLOW-SEQUENCE-END of FLOW-MAPPING-END token. */

    TOKEN_INIT(&mut token, type_, start_mark, end_mark);

    /* Append the token to the queue. */

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Flow entry / block entry fetchers                                     */
/* --------------------------------------------------------------------- */

/// Produce the FLOW-ENTRY token.
pub(crate) unsafe fn yaml_parser_fetch_flow_entry(parser: *mut yaml_parser_t) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Reset any potential simple keys on the current flow level. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    /* Simple keys are allowed after ','. */

    (*parser).simple_key_allowed = 1;

    /* Consume the token. */

    start_mark = (*parser).mark;
    SKIP(parser);
    end_mark = (*parser).mark;

    /* Create the FLOW-ENTRY token and append it to the queue. */

    TOKEN_INIT(&mut token, YAML_FLOW_ENTRY_TOKEN, start_mark, end_mark);

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/// Produce the BLOCK-ENTRY token.
pub(crate) unsafe fn yaml_parser_fetch_block_entry(parser: *mut yaml_parser_t) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Check if the scanner is in the block context. */

    if (*parser).flow_level == 0 {
        /* Check if we are allowed to start a new entry. */

        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                core::ptr::null(),
                (*parser).mark,
                b"block sequence entries are not allowed in this context\0".as_ptr() as *const c_char,
            );
        }

        /* Add the BLOCK-SEQUENCE-START token if needed. */

        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as isize,
            -1,
            YAML_BLOCK_SEQUENCE_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0;
        }
    } else {
        /*
         * It is an error for the '-' indicator to occur in the flow context,
         * but we let the Parser detect and report about it because the Parser
         * is able to point to the context.
         */
    }

    /* Reset any potential simple keys on the current flow level. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    /* Simple keys are allowed after '-'. */

    (*parser).simple_key_allowed = 1;

    /* Consume the token. */

    start_mark = (*parser).mark;
    SKIP(parser);
    end_mark = (*parser).mark;

    /* Create the BLOCK-ENTRY token and append it to the queue. */

    TOKEN_INIT(&mut token, YAML_BLOCK_ENTRY_TOKEN, start_mark, end_mark);

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Key / value fetchers                                                  */
/* --------------------------------------------------------------------- */

/// Produce the KEY token.
pub(crate) unsafe fn yaml_parser_fetch_key(parser: *mut yaml_parser_t) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();

    /* In the block context, additional checks are required. */

    if (*parser).flow_level == 0 {
        /* Check if we are allowed to start a new key (not necessary simple). */

        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                core::ptr::null(),
                (*parser).mark,
                b"mapping keys are not allowed in this context\0".as_ptr() as *const c_char,
            );
        }

        /* Add the BLOCK-MAPPING-START token if needed. */

        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as isize,
            -1,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0;
        }
    }

    /* Reset any potential simple keys on the current flow level. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    /* Simple keys are allowed after '?' in the block context. */

    (*parser).simple_key_allowed = ((*parser).flow_level == 0) as c_int;

    /* Consume the token. */

    start_mark = (*parser).mark;
    SKIP(parser);
    end_mark = (*parser).mark;

    /* Create the KEY token and append it to the queue. */

    TOKEN_INIT(&mut token, YAML_KEY_TOKEN, start_mark, end_mark);

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/// Produce the VALUE token.
pub(crate) unsafe fn yaml_parser_fetch_value(parser: *mut yaml_parser_t) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut token: yaml_token_t = core::mem::zeroed();
    let simple_key: *mut yaml_simple_key_t = (*parser).simple_keys.top.offset(-1);

    /* Have we found a simple key? */

    if (*simple_key).possible != 0 {
        /* Create the KEY token and insert it into the queue. */

        TOKEN_INIT(
            &mut token,
            YAML_KEY_TOKEN,
            (*simple_key).mark,
            (*simple_key).mark,
        );

        if QUEUE_INSERT!(
            parser,
            (*parser).tokens,
            (*simple_key)
                .token_number
                .wrapping_sub((*parser).tokens_parsed) as usize,
            token
        ) == 0
        {
            return 0;
        }

        /* In the block context, we may need to add the BLOCK-MAPPING-START token. */

        if yaml_parser_roll_indent(
            parser,
            (*simple_key).mark.column as isize,
            (*simple_key).token_number as isize,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*simple_key).mark,
        ) == 0
        {
            return 0;
        }

        /* Remove the simple key. */

        (*simple_key).possible = 0;

        /* A simple key cannot follow another simple key. */

        (*parser).simple_key_allowed = 0;
    } else {
        /* The ':' indicator follows a complex key. */

        /* In the block context, extra checks are required. */

        if (*parser).flow_level == 0 {
            /* Check if we are allowed to start a complex value. */

            if (*parser).simple_key_allowed == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    core::ptr::null(),
                    (*parser).mark,
                    b"mapping values are not allowed in this context\0".as_ptr() as *const c_char,
                );
            }

            /* Add the BLOCK-MAPPING-START token if needed. */

            if yaml_parser_roll_indent(
                parser,
                (*parser).mark.column as isize,
                -1,
                YAML_BLOCK_MAPPING_START_TOKEN,
                (*parser).mark,
            ) == 0
            {
                return 0;
            }
        }

        /* Simple keys after ':' are allowed in the block context. */

        (*parser).simple_key_allowed = ((*parser).flow_level == 0) as c_int;
    }

    /* Consume the token. */

    start_mark = (*parser).mark;
    SKIP(parser);
    end_mark = (*parser).mark;

    /* Create the VALUE token and append it to the queue. */

    TOKEN_INIT(&mut token, YAML_VALUE_TOKEN, start_mark, end_mark);

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Anchor / tag fetchers                                                 */
/* --------------------------------------------------------------------- */

/// Produce the ALIAS or ANCHOR token.
pub(crate) unsafe fn yaml_parser_fetch_anchor(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* An anchor or an alias could be a simple key. */

    if yaml_parser_save_simple_key(parser) == 0 {
        return 0;
    }

    /* A simple key cannot follow an anchor or an alias. */

    (*parser).simple_key_allowed = 0;

    /* Create the ALIAS or ANCHOR token and append it to the queue. */

    if yaml_parser_scan_anchor(parser, &mut token, type_) == 0 {
        return 0;
    }

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        crate::api::yaml_token_delete(&mut token);
        return 0;
    }
    1
}

/// Produce the TAG token.
pub(crate) unsafe fn yaml_parser_fetch_tag(parser: *mut yaml_parser_t) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* A tag could be a simple key. */

    if yaml_parser_save_simple_key(parser) == 0 {
        return 0;
    }

    /* A simple key cannot follow a tag. */

    (*parser).simple_key_allowed = 0;

    /* Create the TAG token and append it to the queue. */

    if yaml_parser_scan_tag(parser, &mut token) == 0 {
        return 0;
    }

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        crate::api::yaml_token_delete(&mut token);
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Scalar fetchers                                                       */
/* --------------------------------------------------------------------- */

/// Produce the SCALAR(...,literal) or SCALAR(...,folded) tokens.
pub(crate) unsafe fn yaml_parser_fetch_block_scalar(parser: *mut yaml_parser_t, literal: c_int) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* Remove any potential simple keys. */

    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0;
    }

    /* A simple key may follow a block scalar. */

    (*parser).simple_key_allowed = 1;

    /* Create the SCALAR token and append it to the queue. */

    if yaml_parser_scan_block_scalar(parser, &mut token, literal) == 0 {
        return 0;
    }

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        crate::api::yaml_token_delete(&mut token);
        return 0;
    }

    1
}

/// Produce the SCALAR(...,single-quoted) or SCALAR(...,double-quoted) tokens.
pub(crate) unsafe fn yaml_parser_fetch_flow_scalar(parser: *mut yaml_parser_t, single: c_int) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* A plain scalar could be a simple key. */

    if yaml_parser_save_simple_key(parser) == 0 {
        return 0;
    }

    /* A simple key cannot follow a flow scalar. */

    (*parser).simple_key_allowed = 0;

    /* Create the SCALAR token and append it to the queue. */

    if yaml_parser_scan_flow_scalar(parser, &mut token, single) == 0 {
        return 0;
    }

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        crate::api::yaml_token_delete(&mut token);
        return 0;
    }

    1
}

/// Produce the SCALAR(...,plain) token.
pub(crate) unsafe fn yaml_parser_fetch_plain_scalar(parser: *mut yaml_parser_t) -> c_int {
    let mut token: yaml_token_t = core::mem::zeroed();

    /* A plain scalar could be a simple key. */

    if yaml_parser_save_simple_key(parser) == 0 {
        return 0;
    }

    /* A simple key cannot follow a flow scalar. */

    (*parser).simple_key_allowed = 0;

    /* Create the SCALAR token and append it to the queue. */

    if yaml_parser_scan_plain_scalar(parser, &mut token) == 0 {
        return 0;
    }

    if ENQUEUE!(parser, (*parser).tokens, token) == 0 {
        crate::api::yaml_token_delete(&mut token);
        return 0;
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Scanners                                                              */
/* --------------------------------------------------------------------- */

/// Eat whitespaces and comments until the next token is found.
pub(crate) unsafe fn yaml_parser_scan_to_next_token(parser: *mut yaml_parser_t) -> c_int {
    /* Until the next token is not found. */

    loop {
        /* Allow the BOM mark to start a line. */

        if CACHE(parser, 1) == 0 {
            return 0;
        }

        if (*parser).mark.column == 0 && IS_BOM(BUFFER_STRING(parser)) {
            SKIP(parser);
        }

        /*
         * Eat whitespaces.
         *
         * Tabs are allowed:
         *
         *  - in the flow context;
         *  - in the block context, but not at the beginning of the line or
         *  after '-', '?', or ':' (complex value).
         */

        if CACHE(parser, 1) == 0 {
            return 0;
        }

        while CHECK(BUFFER_STRING(parser), b' ')
            || (((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && CHECK(BUFFER_STRING(parser), b'\t'))
        {
            SKIP(parser);
            if CACHE(parser, 1) == 0 {
                return 0;
            }
        }

        /* Eat a comment until a line break. */

        if CHECK(BUFFER_STRING(parser), b'#') {
            while !IS_BREAKZ(BUFFER_STRING(parser)) {
                SKIP(parser);
                if CACHE(parser, 1) == 0 {
                    return 0;
                }
            }
        }

        /* If it is a line break, eat it. */

        if IS_BREAK(BUFFER_STRING(parser)) {
            if CACHE(parser, 2) == 0 {
                return 0;
            }
            SKIP_LINE(parser);

            /* In the block context, a new line may start a simple key. */

            if (*parser).flow_level == 0 {
                (*parser).simple_key_allowed = 1;
            }
        } else {
            /* We have found a token. */

            break;
        }
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Directive scanner                                                     */
/* --------------------------------------------------------------------- */

/// Scan a YAML-DIRECTIVE or TAG-DIRECTIVE token.
///
/// Scope:
///      %YAML    1.1    # a comment \n
///      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///      %TAG    !yaml!  tag:yaml.org,2002:  \n
///      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
pub(crate) unsafe fn yaml_parser_scan_directive(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t = core::mem::zeroed();
    let mut name: *mut yaml_char_t = core::ptr::null_mut();
    let mut major: c_int = 0;
    let mut minor: c_int = 0;
    let mut handle: *mut yaml_char_t = core::ptr::null_mut();
    let mut prefix: *mut yaml_char_t = core::ptr::null_mut();

    'error: loop {
        /* Eat '%'. */

        start_mark = (*parser).mark;

        SKIP(parser);

        /* Scan the directive name. */

        if yaml_parser_scan_directive_name(parser, start_mark, &mut name) == 0 {
            break 'error;
        }

        /* Is it a YAML directive? */

        if libc::strcmp(
            name as *const c_char,
            b"YAML\0".as_ptr() as *const c_char,
        ) == 0
        {
            /* Scan the VERSION directive value. */

            if yaml_parser_scan_version_directive_value(parser, start_mark, &mut major, &mut minor)
                == 0
            {
                break 'error;
            }

            end_mark = (*parser).mark;

            /* Create a VERSION-DIRECTIVE token. */

            TOKEN_INIT(token, YAML_VERSION_DIRECTIVE_TOKEN, start_mark, end_mark);
            (*token).data.version_directive.major = major;
            (*token).data.version_directive.minor = minor;
        }
        /* Is it a TAG directive? */
        else if libc::strcmp(
            name as *const c_char,
            b"TAG\0".as_ptr() as *const c_char,
        ) == 0
        {
            /* Scan the TAG directive value. */

            if yaml_parser_scan_tag_directive_value(
                parser,
                start_mark,
                &mut handle,
                &mut prefix,
            ) == 0
            {
                break 'error;
            }

            end_mark = (*parser).mark;

            /* Create a TAG-DIRECTIVE token. */

            TOKEN_INIT(token, YAML_TAG_DIRECTIVE_TOKEN, start_mark, end_mark);
            (*token).data.tag_directive.handle = handle;
            (*token).data.tag_directive.prefix = prefix;
        }
        /* Unknown directive. */
        else {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0".as_ptr() as *const c_char,
                start_mark,
                b"found unknown directive name\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Eat the rest of the line including any comments. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while IS_BLANK(BUFFER_STRING(parser)) {
            SKIP(parser);
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        if CHECK(BUFFER_STRING(parser), b'#') {
            while !IS_BREAKZ(BUFFER_STRING(parser)) {
                SKIP(parser);
                if CACHE(parser, 1) == 0 {
                    break 'error;
                }
            }
        }

        /* Check if we are at the end of the line. */

        if !IS_BREAKZ(BUFFER_STRING(parser)) {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find expected comment or line break\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Eat a line break. */

        if IS_BREAK(BUFFER_STRING(parser)) {
            if CACHE(parser, 2) == 0 {
                break 'error;
            }
            SKIP_LINE(parser);
        }

        yaml_free(name as *mut c_void);

        return 1;
    }

    /* error: */
    yaml_free(prefix as *mut c_void);
    yaml_free(handle as *mut c_void);
    yaml_free(name as *mut c_void);
    0
}

/* --------------------------------------------------------------------- */
/*  Directive sub-scanners                                                */
/* --------------------------------------------------------------------- */

/// Scan the directive name.
///
/// Scope:
///      %YAML   1.1     # a comment \n
///       ^^^^
///      %TAG    !yaml!  tag:yaml.org,2002:  \n
///       ^^^
pub(crate) unsafe fn yaml_parser_scan_directive_name(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    name: *mut *mut yaml_char_t,
) -> c_int {
    let mut string: yaml_string_t = NULL_STRING;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        /* Consume the directive name. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while IS_ALPHA(BUFFER_STRING(parser)) {
            if READ(parser, &mut string) == 0 {
                break 'error;
            }
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        /* Check if the name is empty. */

        if string.start == string.pointer {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0".as_ptr() as *const c_char,
                start_mark,
                b"could not find expected directive name\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Check for an blank character after the name. */

        if !IS_BLANKZ(BUFFER_STRING(parser)) {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0".as_ptr() as *const c_char,
                start_mark,
                b"found unexpected non-alphabetical character\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        *name = string.start;

        return 1;
    }

    STRING_DEL!(parser, string);
    0
}

/// Scan the value of VERSION-DIRECTIVE.
///
/// Scope:
///      %YAML   1.1     # a comment \n
///           ^^^^^^
pub(crate) unsafe fn yaml_parser_scan_version_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    major: *mut c_int,
    minor: *mut c_int,
) -> c_int {
    /* Eat whitespaces. */

    if CACHE(parser, 1) == 0 {
        return 0;
    }

    while IS_BLANK(BUFFER_STRING(parser)) {
        SKIP(parser);
        if CACHE(parser, 1) == 0 {
            return 0;
        }
    }

    /* Consume the major version number. */

    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 {
        return 0;
    }

    /* Eat '.'. */

    if !CHECK(BUFFER_STRING(parser), b'.') {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected digit or '.' character\0".as_ptr() as *const c_char,
        );
    }

    SKIP(parser);

    /* Consume the minor version number. */

    if yaml_parser_scan_version_directive_number(parser, start_mark, minor) == 0 {
        return 0;
    }

    1
}

const MAX_NUMBER_LENGTH: size_t = 9;

/// Scan the version number of VERSION-DIRECTIVE.
///
/// Scope:
///      %YAML   1.1     # a comment \n
///              ^
///      %YAML   1.1     # a comment \n
///                ^
pub(crate) unsafe fn yaml_parser_scan_version_directive_number(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    number: *mut c_int,
) -> c_int {
    let mut value: c_int = 0;
    let mut length: size_t = 0;

    /* Repeat while the next character is digit. */

    if CACHE(parser, 1) == 0 {
        return 0;
    }

    while IS_DIGIT(BUFFER_STRING(parser)) {
        /* Check if the number is too long. */

        length = length.wrapping_add(1);
        if length > MAX_NUMBER_LENGTH {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0".as_ptr() as *const c_char,
                start_mark,
                b"found extremely long version number\0".as_ptr() as *const c_char,
            );
        }

        value = value * 10 + AS_DIGIT(BUFFER_STRING(parser));

        SKIP(parser);

        if CACHE(parser, 1) == 0 {
            return 0;
        }
    }

    /* Check if the number was present. */

    if length == 0 {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected version number\0".as_ptr() as *const c_char,
        );
    }

    *number = value;

    1
}

/// Scan the value of a TAG-DIRECTIVE token.
///
/// Scope:
///      %TAG    !yaml!  tag:yaml.org,2002:  \n
///          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
pub(crate) unsafe fn yaml_parser_scan_tag_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
    prefix: *mut *mut yaml_char_t,
) -> c_int {
    let mut handle_value: *mut yaml_char_t = core::ptr::null_mut();
    let mut prefix_value: *mut yaml_char_t = core::ptr::null_mut();

    'error: loop {
        /* Eat whitespaces. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while IS_BLANK(BUFFER_STRING(parser)) {
            SKIP(parser);
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        /* Scan a handle. */

        if yaml_parser_scan_tag_handle(parser, 1, start_mark, &mut handle_value) == 0 {
            break 'error;
        }

        /* Expect a whitespace. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        if !IS_BLANK(BUFFER_STRING(parser)) {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %TAG directive\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find expected whitespace\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Eat whitespaces. */

        while IS_BLANK(BUFFER_STRING(parser)) {
            SKIP(parser);
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        /* Scan a prefix. */

        if yaml_parser_scan_tag_uri(
            parser,
            1,
            1,
            core::ptr::null_mut(),
            start_mark,
            &mut prefix_value,
        ) == 0
        {
            break 'error;
        }

        /* Expect a whitespace or line break. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        if !IS_BLANKZ(BUFFER_STRING(parser)) {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %TAG directive\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find expected whitespace or line break\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        *handle = handle_value;
        *prefix = prefix_value;

        return 1;
    }

    yaml_free(handle_value as *mut c_void);
    yaml_free(prefix_value as *mut c_void);
    0
}

/* --------------------------------------------------------------------- */
/*  Anchor scanner                                                        */
/* --------------------------------------------------------------------- */

pub(crate) unsafe fn yaml_parser_scan_anchor(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    type_: yaml_token_type_t,
) -> c_int {
    let mut length: c_int = 0;
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut string: yaml_string_t = NULL_STRING;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        /* Eat the indicator character. */

        start_mark = (*parser).mark;

        SKIP(parser);

        /* Consume the value. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while IS_ALPHA(BUFFER_STRING(parser)) {
            if READ(parser, &mut string) == 0 {
                break 'error;
            }
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
            length += 1;
        }

        end_mark = (*parser).mark;

        /*
         * Check if length of the anchor is greater than 0 and it is followed by
         * a whitespace character or one of the indicators:
         *
         *      '?', ':', ',', ']', '}', '%', '@', '`'.
         */

        if length == 0
            || !(IS_BLANKZ(BUFFER_STRING(parser))
                || CHECK(BUFFER_STRING(parser), b'?')
                || CHECK(BUFFER_STRING(parser), b':')
                || CHECK(BUFFER_STRING(parser), b',')
                || CHECK(BUFFER_STRING(parser), b']')
                || CHECK(BUFFER_STRING(parser), b'}')
                || CHECK(BUFFER_STRING(parser), b'%')
                || CHECK(BUFFER_STRING(parser), b'@')
                || CHECK(BUFFER_STRING(parser), b'`'))
        {
            yaml_parser_set_scanner_error(
                parser,
                if type_ == YAML_ANCHOR_TOKEN {
                    b"while scanning an anchor\0".as_ptr() as *const c_char
                } else {
                    b"while scanning an alias\0".as_ptr() as *const c_char
                },
                start_mark,
                b"did not find expected alphabetic or numeric character\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Create a token. */

        if type_ == YAML_ANCHOR_TOKEN {
            TOKEN_INIT(token, YAML_ANCHOR_TOKEN, start_mark, end_mark);
            (*token).data.anchor.value = string.start;
        } else {
            TOKEN_INIT(token, YAML_ALIAS_TOKEN, start_mark, end_mark);
            (*token).data.alias.value = string.start;
        }

        return 1;
    }

    STRING_DEL!(parser, string);
    0
}

/* --------------------------------------------------------------------- */
/*  Tag scanner                                                           */
/* --------------------------------------------------------------------- */

/// Scan a TAG token.
pub(crate) unsafe fn yaml_parser_scan_tag(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    let mut handle: *mut yaml_char_t = core::ptr::null_mut();
    let mut suffix: *mut yaml_char_t = core::ptr::null_mut();
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;

    'error: loop {
        start_mark = (*parser).mark;

        /* Check if the tag is in the canonical form. */

        if CACHE(parser, 2) == 0 {
            break 'error;
        }

        if CHECK_AT(BUFFER_STRING(parser), b'<', 1) {
            /* Set the handle to '' */

            handle = YAML_MALLOC(1);
            if handle.is_null() {
                break 'error;
            }
            *handle = b'\0';

            /* Eat '!<' */

            SKIP(parser);
            SKIP(parser);

            /* Consume the tag value. */

            if yaml_parser_scan_tag_uri(
                parser,
                1,
                0,
                core::ptr::null_mut(),
                start_mark,
                &mut suffix,
            ) == 0
            {
                break 'error;
            }

            /* Check for '>' and eat it. */

            if !CHECK(BUFFER_STRING(parser), b'>') {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a tag\0".as_ptr() as *const c_char,
                    start_mark,
                    b"did not find the expected '>'\0".as_ptr() as *const c_char,
                );
                break 'error;
            }

            SKIP(parser);
        } else {
            /* The tag has either the '!suffix' or the '!handle!suffix' form. */

            /* First, try to scan a handle. */

            if yaml_parser_scan_tag_handle(parser, 0, start_mark, &mut handle) == 0 {
                break 'error;
            }

            /* Check if it is, indeed, handle. */

            if *handle == b'!'
                && *handle.add(1) != b'\0'
                && *handle.add(libc::strlen(handle as *const c_char).wrapping_sub(1)) == b'!'
            {
                /* Scan the suffix now. */

                if yaml_parser_scan_tag_uri(
                    parser,
                    0,
                    0,
                    core::ptr::null_mut(),
                    start_mark,
                    &mut suffix,
                ) == 0
                {
                    break 'error;
                }
            } else {
                /* It wasn't a handle after all.  Scan the rest of the tag. */

                if yaml_parser_scan_tag_uri(parser, 0, 0, handle, start_mark, &mut suffix) == 0 {
                    break 'error;
                }

                /* Set the handle to '!'. */

                yaml_free(handle as *mut c_void);
                handle = YAML_MALLOC(2);
                if handle.is_null() {
                    break 'error;
                }
                *handle = b'!';
                *handle.add(1) = b'\0';

                /*
                 * A special case: the '!' tag.  Set the handle to '' and the
                 * suffix to '!'.
                 */

                if *suffix == b'\0' {
                    let tmp = handle;
                    handle = suffix;
                    suffix = tmp;
                }
            }
        }

        /* Check the character which ends the tag. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        if !IS_BLANKZ(BUFFER_STRING(parser)) {
            if (*parser).flow_level == 0 || !CHECK(BUFFER_STRING(parser), b',') {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a tag\0".as_ptr() as *const c_char,
                    start_mark,
                    b"did not find expected whitespace or line break\0".as_ptr() as *const c_char,
                );
                break 'error;
            }
        }

        end_mark = (*parser).mark;

        /* Create a token. */

        TOKEN_INIT(token, YAML_TAG_TOKEN, start_mark, end_mark);
        (*token).data.tag.handle = handle;
        (*token).data.tag.suffix = suffix;

        return 1;
    }

    yaml_free(handle as *mut c_void);
    yaml_free(suffix as *mut c_void);
    0
}

/// Scan a tag handle.
pub(crate) unsafe fn yaml_parser_scan_tag_handle(
    parser: *mut yaml_parser_t,
    directive: c_int,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
) -> c_int {
    let mut string: yaml_string_t = NULL_STRING;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        /* Check the initial '!' character. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        if !CHECK(BUFFER_STRING(parser), b'!') {
            yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while scanning a tag directive\0".as_ptr() as *const c_char
                } else {
                    b"while scanning a tag\0".as_ptr() as *const c_char
                },
                start_mark,
                b"did not find expected '!'\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Copy the '!' character. */

        if READ(parser, &mut string) == 0 {
            break 'error;
        }

        /* Copy all subsequent alphabetical and numerical characters. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while IS_ALPHA(BUFFER_STRING(parser)) {
            if READ(parser, &mut string) == 0 {
                break 'error;
            }
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        /* Check if the trailing character is '!' and copy it. */

        if CHECK(BUFFER_STRING(parser), b'!') {
            if READ(parser, &mut string) == 0 {
                break 'error;
            }
        } else {
            /*
             * It's either the '!' tag or not really a tag handle.  If it's a %TAG
             * directive, it's an error.  If it's a tag token, it must be a part of
             * URI.
             */

            if directive != 0 && !(*string.start == b'!' && *string.start.add(1) == b'\0') {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while parsing a tag directive\0".as_ptr() as *const c_char,
                    start_mark,
                    b"did not find expected '!'\0".as_ptr() as *const c_char,
                );
                break 'error;
            }
        }

        *handle = string.start;

        return 1;
    }

    STRING_DEL!(parser, string);
    0
}

/// Scan a tag URI.
pub(crate) unsafe fn yaml_parser_scan_tag_uri(
    parser: *mut yaml_parser_t,
    uri_char: c_int,
    directive: c_int,
    head: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    uri: *mut *mut yaml_char_t,
) -> c_int {
    let mut length: size_t = if !head.is_null() {
        libc::strlen(head as *const c_char)
    } else {
        0
    };
    let mut string: yaml_string_t = NULL_STRING;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        /* Resize the string to include the head. */

        while (string.end.offset_from(string.start) as size_t) <= length {
            if yaml_string_extend(&mut string.start, &mut string.pointer, &mut string.end) == 0 {
                (*parser).error = YAML_MEMORY_ERROR;
                break 'error;
            }
        }

        /*
         * Copy the head if needed.
         *
         * Note that we don't copy the leading '!' character.
         */

        if length > 1 {
            libc::memcpy(
                string.start as *mut c_void,
                head.add(1) as *const c_void,
                length - 1,
            );
            string.pointer = string.pointer.add(length - 1);
        }

        /* Scan the tag. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        /*
         * The set of characters that may appear in URI is as follows:
         *
         *      '0'-'9', 'A'-'Z', 'a'-'z', '_', '-', ';', '/', '?', ':', '@', '&',
         *      '=', '+', '$', '.', '!', '~', '*', '\'', '(', ')', '%'.
         *
         * If we are inside a verbatim tag <...> (parameter uri_char is true)
         * then also the following flow indicators are allowed:
         *      ',', '[', ']'
         */

        while IS_ALPHA(BUFFER_STRING(parser))
            || CHECK(BUFFER_STRING(parser), b';')
            || CHECK(BUFFER_STRING(parser), b'/')
            || CHECK(BUFFER_STRING(parser), b'?')
            || CHECK(BUFFER_STRING(parser), b':')
            || CHECK(BUFFER_STRING(parser), b'@')
            || CHECK(BUFFER_STRING(parser), b'&')
            || CHECK(BUFFER_STRING(parser), b'=')
            || CHECK(BUFFER_STRING(parser), b'+')
            || CHECK(BUFFER_STRING(parser), b'$')
            || CHECK(BUFFER_STRING(parser), b'.')
            || CHECK(BUFFER_STRING(parser), b'%')
            || CHECK(BUFFER_STRING(parser), b'!')
            || CHECK(BUFFER_STRING(parser), b'~')
            || CHECK(BUFFER_STRING(parser), b'*')
            || CHECK(BUFFER_STRING(parser), b'\'')
            || CHECK(BUFFER_STRING(parser), b'(')
            || CHECK(BUFFER_STRING(parser), b')')
            || (uri_char != 0
                && (CHECK(BUFFER_STRING(parser), b',')
                    || CHECK(BUFFER_STRING(parser), b'[')
                    || CHECK(BUFFER_STRING(parser), b']')))
        {
            /* Check if it is a URI-escape sequence. */

            if CHECK(BUFFER_STRING(parser), b'%') {
                if STRING_EXTEND!(parser, string) == 0 {
                    break 'error;
                }

                if yaml_parser_scan_uri_escapes(parser, directive, start_mark, &mut string) == 0 {
                    break 'error;
                }
            } else {
                if READ(parser, &mut string) == 0 {
                    break 'error;
                }
            }

            length = length.wrapping_add(1);
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        /* Check if the tag is non-empty. */

        if length == 0 {
            if STRING_EXTEND!(parser, string) == 0 {
                break 'error;
            }

            yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0".as_ptr() as *const c_char
                } else {
                    b"while parsing a tag\0".as_ptr() as *const c_char
                },
                start_mark,
                b"did not find expected tag URI\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        *uri = string.start;

        return 1;
    }

    STRING_DEL!(parser, string);
    0
}

/// Decode a URI-escape sequence corresponding to a single UTF-8 character.
pub(crate) unsafe fn yaml_parser_scan_uri_escapes(
    parser: *mut yaml_parser_t,
    directive: c_int,
    start_mark: yaml_mark_t,
    string: *mut yaml_string_t,
) -> c_int {
    let mut width: c_int = 0;

    /* Decode the required number of characters. */

    loop {
        let mut octet: c_uchar = 0;

        /* Check for a URI-escaped octet. */

        if CACHE(parser, 3) == 0 {
            return 0;
        }

        if !(CHECK(BUFFER_STRING(parser), b'%')
            && IS_HEX_AT(BUFFER_STRING(parser), 1)
            && IS_HEX_AT(BUFFER_STRING(parser), 2))
        {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0".as_ptr() as *const c_char
                } else {
                    b"while parsing a tag\0".as_ptr() as *const c_char
                },
                start_mark,
                b"did not find URI escaped octet\0".as_ptr() as *const c_char,
            );
        }

        /* Get the octet. */

        octet = ((AS_HEX_AT(BUFFER_STRING(parser), 1) << 4)
            + AS_HEX_AT(BUFFER_STRING(parser), 2)) as c_uchar;

        /* If it is the leading octet, determine the length of the UTF-8 sequence. */

        if width == 0 {
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
            if width == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while parsing a %TAG directive\0".as_ptr() as *const c_char
                    } else {
                        b"while parsing a tag\0".as_ptr() as *const c_char
                    },
                    start_mark,
                    b"found an incorrect leading UTF-8 octet\0".as_ptr() as *const c_char,
                );
            }
        } else {
            /* Check if the trailing octet is correct. */

            if (octet & 0xC0) != 0x80 {
                return yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while parsing a %TAG directive\0".as_ptr() as *const c_char
                    } else {
                        b"while parsing a tag\0".as_ptr() as *const c_char
                    },
                    start_mark,
                    b"found an incorrect trailing UTF-8 octet\0".as_ptr() as *const c_char,
                );
            }
        }

        /* Copy the octet and move the pointers. */

        *(*string).pointer = octet;
        (*string).pointer = (*string).pointer.add(1);
        SKIP(parser);
        SKIP(parser);
        SKIP(parser);

        width -= 1;
        if width == 0 {
            break;
        }
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Block scalar scanner                                                  */
/* --------------------------------------------------------------------- */

/// Scan a block scalar.
pub(crate) unsafe fn yaml_parser_scan_block_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    literal: c_int,
) -> c_int {
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t = core::mem::zeroed();
    let mut string: yaml_string_t = NULL_STRING;
    let mut leading_break: yaml_string_t = NULL_STRING;
    let mut trailing_breaks: yaml_string_t = NULL_STRING;
    let mut chomping: c_int = 0;
    let mut increment: c_int = 0;
    let mut indent: c_int = 0;
    let mut leading_blank: c_int = 0;
    let mut trailing_blank: c_int;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, leading_break, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, trailing_breaks, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        /* Eat the indicator '|' or '>'. */

        start_mark = (*parser).mark;

        SKIP(parser);

        /* Scan the additional block scalar indicators. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        /* Check for a chomping indicator. */

        if CHECK(BUFFER_STRING(parser), b'+') || CHECK(BUFFER_STRING(parser), b'-') {
            /* Set the chomping method and eat the indicator. */

            chomping = if CHECK(BUFFER_STRING(parser), b'+') {
                1
            } else {
                -1
            };

            SKIP(parser);

            /* Check for an indentation indicator. */

            if CACHE(parser, 1) == 0 {
                break 'error;
            }

            if IS_DIGIT(BUFFER_STRING(parser)) {
                /* Check that the indentation is greater than 0. */

                if CHECK(BUFFER_STRING(parser), b'0') {
                    yaml_parser_set_scanner_error(
                        parser,
                        b"while scanning a block scalar\0".as_ptr() as *const c_char,
                        start_mark,
                        b"found an indentation indicator equal to 0\0".as_ptr() as *const c_char,
                    );
                    break 'error;
                }

                /* Get the indentation level and eat the indicator. */

                increment = AS_DIGIT(BUFFER_STRING(parser));

                SKIP(parser);
            }
        }
        /* Do the same as above, but in the opposite order. */
        else if IS_DIGIT(BUFFER_STRING(parser)) {
            if CHECK(BUFFER_STRING(parser), b'0') {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a block scalar\0".as_ptr() as *const c_char,
                    start_mark,
                    b"found an indentation indicator equal to 0\0".as_ptr() as *const c_char,
                );
                break 'error;
            }

            increment = AS_DIGIT(BUFFER_STRING(parser));

            SKIP(parser);

            if CACHE(parser, 1) == 0 {
                break 'error;
            }

            if CHECK(BUFFER_STRING(parser), b'+') || CHECK(BUFFER_STRING(parser), b'-') {
                chomping = if CHECK(BUFFER_STRING(parser), b'+') {
                    1
                } else {
                    -1
                };

                SKIP(parser);
            }
        }

        /* Eat whitespaces and comments to the end of the line. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while IS_BLANK(BUFFER_STRING(parser)) {
            SKIP(parser);
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
        }

        if CHECK(BUFFER_STRING(parser), b'#') {
            while !IS_BREAKZ(BUFFER_STRING(parser)) {
                SKIP(parser);
                if CACHE(parser, 1) == 0 {
                    break 'error;
                }
            }
        }

        /* Check if we are at the end of the line. */

        if !IS_BREAKZ(BUFFER_STRING(parser)) {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find expected comment or line break\0".as_ptr() as *const c_char,
            );
            break 'error;
        }

        /* Eat a line break. */

        if IS_BREAK(BUFFER_STRING(parser)) {
            if CACHE(parser, 2) == 0 {
                break 'error;
            }
            SKIP_LINE(parser);
        }

        end_mark = (*parser).mark;

        /* Set the indentation level if it was specified. */

        if increment != 0 {
            indent = if (*parser).indent >= 0 {
                (*parser).indent + increment
            } else {
                increment
            };
        }

        /* Scan the leading line breaks and determine the indentation level if needed. */

        if yaml_parser_scan_block_scalar_breaks(
            parser,
            &mut indent,
            &mut trailing_breaks,
            start_mark,
            &mut end_mark,
        ) == 0
        {
            break 'error;
        }

        /* Scan the block scalar content. */

        if CACHE(parser, 1) == 0 {
            break 'error;
        }

        while (*parser).mark.column as c_int == indent && !IS_Z(BUFFER_STRING(parser)) {
            /*
             * We are at the beginning of a non-empty line.
             */

            /* Is it a trailing whitespace? */

            trailing_blank = IS_BLANK(BUFFER_STRING(parser)) as c_int;

            /* Check if we need to fold the leading line break. */

            if literal == 0
                && *leading_break.start == b'\n'
                && leading_blank == 0
                && trailing_blank == 0
            {
                /* Do we need to join the lines by space? */

                if *trailing_breaks.start == b'\0' {
                    if STRING_EXTEND!(parser, string) == 0 {
                        break 'error;
                    }
                    *string.pointer = b' ';
                    string.pointer = string.pointer.add(1);
                }

                CLEAR!(parser, leading_break);
            } else {
                if JOIN!(parser, string, leading_break) == 0 {
                    break 'error;
                }
                CLEAR!(parser, leading_break);
            }

            /* Append the remaining line breaks. */

            if JOIN!(parser, string, trailing_breaks) == 0 {
                break 'error;
            }
            CLEAR!(parser, trailing_breaks);

            /* Is it a leading whitespace? */

            leading_blank = IS_BLANK(BUFFER_STRING(parser)) as c_int;

            /* Consume the current line. */

            while !IS_BREAKZ(BUFFER_STRING(parser)) {
                if READ(parser, &mut string) == 0 {
                    break 'error;
                }
                if CACHE(parser, 1) == 0 {
                    break 'error;
                }
            }

            /* Consume the line break. */

            if CACHE(parser, 2) == 0 {
                break 'error;
            }

            if READ_LINE(parser, &mut leading_break) == 0 {
                break 'error;
            }

            /* Eat the following indentation spaces and line breaks. */

            if yaml_parser_scan_block_scalar_breaks(
                parser,
                &mut indent,
                &mut trailing_breaks,
                start_mark,
                &mut end_mark,
            ) == 0
            {
                break 'error;
            }
        }

        /* Chomp the tail. */

        if chomping != -1 {
            if JOIN!(parser, string, leading_break) == 0 {
                break 'error;
            }
        }
        if chomping == 1 {
            if JOIN!(parser, string, trailing_breaks) == 0 {
                break 'error;
            }
        }

        /* Create a token. */

        TOKEN_INIT(token, YAML_SCALAR_TOKEN, start_mark, end_mark);
        (*token).data.scalar.value = string.start;
        (*token).data.scalar.length = string.pointer.offset_from(string.start) as size_t;
        (*token).data.scalar.style = if literal != 0 {
            YAML_LITERAL_SCALAR_STYLE
        } else {
            YAML_FOLDED_SCALAR_STYLE
        };

        STRING_DEL!(parser, leading_break);
        STRING_DEL!(parser, trailing_breaks);

        return 1;
    }

    STRING_DEL!(parser, string);
    STRING_DEL!(parser, leading_break);
    STRING_DEL!(parser, trailing_breaks);

    0
}

/// Scan indentation spaces and line breaks for a block scalar. Determine the
/// indentation level if needed.
pub(crate) unsafe fn yaml_parser_scan_block_scalar_breaks(
    parser: *mut yaml_parser_t,
    indent: *mut c_int,
    breaks: *mut yaml_string_t,
    start_mark: yaml_mark_t,
    end_mark: *mut yaml_mark_t,
) -> c_int {
    let mut max_indent: c_int = 0;

    *end_mark = (*parser).mark;

    /* Eat the indentation spaces and line breaks. */

    loop {
        /* Eat the indentation spaces. */

        if CACHE(parser, 1) == 0 {
            return 0;
        }

        while (*indent == 0 || ((*parser).mark.column as c_int) < *indent)
            && IS_SPACE(BUFFER_STRING(parser))
        {
            SKIP(parser);
            if CACHE(parser, 1) == 0 {
                return 0;
            }
        }

        if (*parser).mark.column as c_int > max_indent {
            max_indent = (*parser).mark.column as c_int;
        }

        /* Check for a tab character messing the indentation. */

        if (*indent == 0 || ((*parser).mark.column as c_int) < *indent)
            && IS_TAB(BUFFER_STRING(parser))
        {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0".as_ptr() as *const c_char,
                start_mark,
                b"found a tab character where an indentation space is expected\0".as_ptr()
                    as *const c_char,
            );
        }

        /* Have we found a non-empty line? */

        if !IS_BREAK(BUFFER_STRING(parser)) {
            break;
        }

        /* Consume the line break. */

        if CACHE(parser, 2) == 0 {
            return 0;
        }
        if READ_LINE(parser, &mut *breaks) == 0 {
            return 0;
        }
        *end_mark = (*parser).mark;
    }

    /* Determine the indentation level if needed. */

    if *indent == 0 {
        *indent = max_indent;
        if *indent < (*parser).indent + 1 {
            *indent = (*parser).indent + 1;
        }
        if *indent < 1 {
            *indent = 1;
        }
    }

    1
}

/* --------------------------------------------------------------------- */
/*  Flow (quoted) scalar scanner                                          */
/* --------------------------------------------------------------------- */

/// Scan a quoted scalar.
pub(crate) unsafe fn yaml_parser_scan_flow_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    single: c_int,
) -> c_int {
    let start_mark: yaml_mark_t;
    let end_mark: yaml_mark_t;
    let mut string: yaml_string_t = NULL_STRING;
    let mut leading_break: yaml_string_t = NULL_STRING;
    let mut trailing_breaks: yaml_string_t = NULL_STRING;
    let mut whitespaces: yaml_string_t = NULL_STRING;
    let mut leading_blanks: c_int;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, leading_break, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, trailing_breaks, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, whitespaces, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        /* Eat the left quote. */

        start_mark = (*parser).mark;

        SKIP(parser);

        /* Consume the content of the quoted scalar. */

        loop {
            /* Check that there are no document indicators at the beginning of the line. */

            if CACHE(parser, 4) == 0 {
                break 'error;
            }

            if (*parser).mark.column == 0
                && ((CHECK_AT(BUFFER_STRING(parser), b'-', 0)
                    && CHECK_AT(BUFFER_STRING(parser), b'-', 1)
                    && CHECK_AT(BUFFER_STRING(parser), b'-', 2))
                    || (CHECK_AT(BUFFER_STRING(parser), b'.', 0)
                        && CHECK_AT(BUFFER_STRING(parser), b'.', 1)
                        && CHECK_AT(BUFFER_STRING(parser), b'.', 2)))
                && IS_BLANKZ_AT(BUFFER_STRING(parser), 3)
            {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a quoted scalar\0".as_ptr() as *const c_char,
                    start_mark,
                    b"found unexpected document indicator\0".as_ptr() as *const c_char,
                );
                break 'error;
            }

            /* Check for EOF. */

            if IS_Z(BUFFER_STRING(parser)) {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a quoted scalar\0".as_ptr() as *const c_char,
                    start_mark,
                    b"found unexpected end of stream\0".as_ptr() as *const c_char,
                );
                break 'error;
            }

            /* Consume non-blank characters. */

            if CACHE(parser, 2) == 0 {
                break 'error;
            }

            leading_blanks = 0;

            while !IS_BLANKZ(BUFFER_STRING(parser)) {
                /* Check for an escaped single quote. */

                if single != 0
                    && CHECK_AT(BUFFER_STRING(parser), b'\'', 0)
                    && CHECK_AT(BUFFER_STRING(parser), b'\'', 1)
                {
                    if STRING_EXTEND!(parser, string) == 0 {
                        break 'error;
                    }
                    *string.pointer = b'\'';
                    string.pointer = string.pointer.add(1);
                    SKIP(parser);
                    SKIP(parser);
                }
                /* Check for the right quote. */
                else if CHECK(
                    BUFFER_STRING(parser),
                    if single != 0 { b'\'' } else { b'"' },
                ) {
                    break;
                }
                /* Check for an escaped line break. */
                else if single == 0
                    && CHECK(BUFFER_STRING(parser), b'\\')
                    && IS_BREAK_AT(BUFFER_STRING(parser), 1)
                {
                    if CACHE(parser, 3) == 0 {
                        break 'error;
                    }
                    SKIP(parser);
                    SKIP_LINE(parser);
                    leading_blanks = 1;
                    break;
                }
                /* Check for an escape sequence. */
                else if single == 0 && CHECK(BUFFER_STRING(parser), b'\\') {
                    let mut code_length: size_t = 0;

                    if STRING_EXTEND!(parser, string) == 0 {
                        break 'error;
                    }

                    /* Check the escape character. */

                    match *(*parser).buffer.pointer.add(1) {
                        b'0' => {
                            *string.pointer = b'\0';
                            string.pointer = string.pointer.add(1);
                        }
                        b'a' => {
                            *string.pointer = 0x07;
                            string.pointer = string.pointer.add(1);
                        }
                        b'b' => {
                            *string.pointer = 0x08;
                            string.pointer = string.pointer.add(1);
                        }
                        b't' | b'\t' => {
                            *string.pointer = 0x09;
                            string.pointer = string.pointer.add(1);
                        }
                        b'n' => {
                            *string.pointer = 0x0A;
                            string.pointer = string.pointer.add(1);
                        }
                        b'v' => {
                            *string.pointer = 0x0B;
                            string.pointer = string.pointer.add(1);
                        }
                        b'f' => {
                            *string.pointer = 0x0C;
                            string.pointer = string.pointer.add(1);
                        }
                        b'r' => {
                            *string.pointer = 0x0D;
                            string.pointer = string.pointer.add(1);
                        }
                        b'e' => {
                            *string.pointer = 0x1B;
                            string.pointer = string.pointer.add(1);
                        }
                        b' ' => {
                            *string.pointer = 0x20;
                            string.pointer = string.pointer.add(1);
                        }
                        b'"' => {
                            *string.pointer = b'"';
                            string.pointer = string.pointer.add(1);
                        }
                        b'/' => {
                            *string.pointer = b'/';
                            string.pointer = string.pointer.add(1);
                        }
                        b'\\' => {
                            *string.pointer = b'\\';
                            string.pointer = string.pointer.add(1);
                        }
                        b'N' => {
                            /* NEL (#x85) */
                            *string.pointer = 0xC2;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = 0x85;
                            string.pointer = string.pointer.add(1);
                        }
                        b'_' => {
                            /* #xA0 */
                            *string.pointer = 0xC2;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = 0xA0;
                            string.pointer = string.pointer.add(1);
                        }
                        b'L' => {
                            /* LS (#x2028) */
                            *string.pointer = 0xE2;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = 0x80;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = 0xA8;
                            string.pointer = string.pointer.add(1);
                        }
                        b'P' => {
                            /* PS (#x2029) */
                            *string.pointer = 0xE2;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = 0x80;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = 0xA9;
                            string.pointer = string.pointer.add(1);
                        }
                        b'x' => {
                            code_length = 2;
                        }
                        b'u' => {
                            code_length = 4;
                        }
                        b'U' => {
                            code_length = 8;
                        }
                        _ => {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while parsing a quoted scalar\0".as_ptr() as *const c_char,
                                start_mark,
                                b"found unknown escape character\0".as_ptr() as *const c_char,
                            );
                            break 'error;
                        }
                    }

                    SKIP(parser);
                    SKIP(parser);

                    /* Consume an arbitrary escape code. */

                    if code_length != 0 {
                        let mut value: c_uint = 0;
                        let mut k: size_t;

                        /* Scan the character value. */

                        if CACHE(parser, code_length) == 0 {
                            break 'error;
                        }

                        k = 0;
                        while k < code_length {
                            if !IS_HEX_AT(BUFFER_STRING(parser), k as isize) {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while parsing a quoted scalar\0".as_ptr() as *const c_char,
                                    start_mark,
                                    b"did not find expected hexdecimal number\0".as_ptr()
                                        as *const c_char,
                                );
                                break 'error;
                            }
                            value =
                                (value << 4) + AS_HEX_AT(BUFFER_STRING(parser), k as isize) as c_uint;
                            k = k.wrapping_add(1);
                        }

                        /* Check the value and write the character. */

                        if (value >= 0xD800 && value <= 0xDFFF) || value > 0x10FFFF {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while parsing a quoted scalar\0".as_ptr() as *const c_char,
                                start_mark,
                                b"found invalid Unicode character escape code\0".as_ptr()
                                    as *const c_char,
                            );
                            break 'error;
                        }

                        if value <= 0x7F {
                            *string.pointer = value as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                        } else if value <= 0x7FF {
                            *string.pointer = (0xC0 + (value >> 6)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = (0x80 + (value & 0x3F)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                        } else if value <= 0xFFFF {
                            *string.pointer = (0xE0 + (value >> 12)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = (0x80 + ((value >> 6) & 0x3F)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = (0x80 + (value & 0x3F)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                        } else {
                            *string.pointer = (0xF0 + (value >> 18)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = (0x80 + ((value >> 12) & 0x3F)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = (0x80 + ((value >> 6) & 0x3F)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                            *string.pointer = (0x80 + (value & 0x3F)) as yaml_char_t;
                            string.pointer = string.pointer.add(1);
                        }

                        /* Advance the pointer. */

                        k = 0;
                        while k < code_length {
                            SKIP(parser);
                            k = k.wrapping_add(1);
                        }
                    }
                } else {
                    /* It is a non-escaped non-blank character. */

                    if READ(parser, &mut string) == 0 {
                        break 'error;
                    }
                }

                if CACHE(parser, 2) == 0 {
                    break 'error;
                }
            }

            /* Check if we are at the end of the scalar. */

            /* Fix for crash uninitialized value crash
             * Credit for the bug and input is to OSS Fuzz
             * Credit for the fix to Alex Gaynor
             */
            if CACHE(parser, 1) == 0 {
                break 'error;
            }
            if CHECK(
                BUFFER_STRING(parser),
                if single != 0 { b'\'' } else { b'"' },
            ) {
                break;
            }

            /* Consume blank characters. */

            if CACHE(parser, 1) == 0 {
                break 'error;
            }

            while IS_BLANK(BUFFER_STRING(parser)) || IS_BREAK(BUFFER_STRING(parser)) {
                if IS_BLANK(BUFFER_STRING(parser)) {
                    /* Consume a space or a tab character. */

                    if leading_blanks == 0 {
                        if READ(parser, &mut whitespaces) == 0 {
                            break 'error;
                        }
                    } else {
                        SKIP(parser);
                    }
                } else {
                    if CACHE(parser, 2) == 0 {
                        break 'error;
                    }

                    /* Check if it is a first line break. */

                    if leading_blanks == 0 {
                        CLEAR!(parser, whitespaces);
                        if READ_LINE(parser, &mut leading_break) == 0 {
                            break 'error;
                        }
                        leading_blanks = 1;
                    } else {
                        if READ_LINE(parser, &mut trailing_breaks) == 0 {
                            break 'error;
                        }
                    }
                }
                if CACHE(parser, 1) == 0 {
                    break 'error;
                }
            }

            /* Join the whitespaces or fold line breaks. */

            if leading_blanks != 0 {
                /* Do we need to fold line breaks? */

                if *leading_break.start == b'\n' {
                    if *trailing_breaks.start == b'\0' {
                        if STRING_EXTEND!(parser, string) == 0 {
                            break 'error;
                        }
                        *string.pointer = b' ';
                        string.pointer = string.pointer.add(1);
                    } else {
                        if JOIN!(parser, string, trailing_breaks) == 0 {
                            break 'error;
                        }
                        CLEAR!(parser, trailing_breaks);
                    }
                    CLEAR!(parser, leading_break);
                } else {
                    if JOIN!(parser, string, leading_break) == 0 {
                        break 'error;
                    }
                    if JOIN!(parser, string, trailing_breaks) == 0 {
                        break 'error;
                    }
                    CLEAR!(parser, leading_break);
                    CLEAR!(parser, trailing_breaks);
                }
            } else {
                if JOIN!(parser, string, whitespaces) == 0 {
                    break 'error;
                }
                CLEAR!(parser, whitespaces);
            }
        }

        /* Eat the right quote. */

        SKIP(parser);

        end_mark = (*parser).mark;

        /* Create a token. */

        TOKEN_INIT(token, YAML_SCALAR_TOKEN, start_mark, end_mark);
        (*token).data.scalar.value = string.start;
        (*token).data.scalar.length = string.pointer.offset_from(string.start) as size_t;
        (*token).data.scalar.style = if single != 0 {
            YAML_SINGLE_QUOTED_SCALAR_STYLE
        } else {
            YAML_DOUBLE_QUOTED_SCALAR_STYLE
        };

        STRING_DEL!(parser, leading_break);
        STRING_DEL!(parser, trailing_breaks);
        STRING_DEL!(parser, whitespaces);

        return 1;
    }

    STRING_DEL!(parser, string);
    STRING_DEL!(parser, leading_break);
    STRING_DEL!(parser, trailing_breaks);
    STRING_DEL!(parser, whitespaces);

    0
}

/* --------------------------------------------------------------------- */
/*  Plain scalar scanner                                                  */
/* --------------------------------------------------------------------- */

/// Scan a plain scalar.
pub(crate) unsafe fn yaml_parser_scan_plain_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    let start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut string: yaml_string_t = NULL_STRING;
    let mut leading_break: yaml_string_t = NULL_STRING;
    let mut trailing_breaks: yaml_string_t = NULL_STRING;
    let mut whitespaces: yaml_string_t = NULL_STRING;
    let mut leading_blanks: c_int = 0;
    let indent: c_int = (*parser).indent + 1;

    'error: loop {
        if STRING_INIT!(parser, string, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, leading_break, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, trailing_breaks, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }
        if STRING_INIT!(parser, whitespaces, INITIAL_STRING_SIZE) == 0 {
            break 'error;
        }

        start_mark = (*parser).mark;
        end_mark = (*parser).mark;

        /* Consume the content of the plain scalar. */

        'outer: loop {
            /* Check for a document indicator. */

            if CACHE(parser, 4) == 0 {
                break 'error;
            }

            if (*parser).mark.column == 0
                && ((CHECK_AT(BUFFER_STRING(parser), b'-', 0)
                    && CHECK_AT(BUFFER_STRING(parser), b'-', 1)
                    && CHECK_AT(BUFFER_STRING(parser), b'-', 2))
                    || (CHECK_AT(BUFFER_STRING(parser), b'.', 0)
                        && CHECK_AT(BUFFER_STRING(parser), b'.', 1)
                        && CHECK_AT(BUFFER_STRING(parser), b'.', 2)))
                && IS_BLANKZ_AT(BUFFER_STRING(parser), 3)
            {
                break 'outer;
            }

            /* Check for a comment. */

            if CHECK(BUFFER_STRING(parser), b'#') {
                break 'outer;
            }

            /* Consume non-blank characters. */

            while !IS_BLANKZ(BUFFER_STRING(parser)) {
                /* Check for "x:" + one of ',?[]{}' in the flow context. TODO: Fix the test "spec-08-13".
                 * This is not completely according to the spec
                 * See http://yaml.org/spec/1.1/#id907281 9.1.3. Plain
                 */

                if (*parser).flow_level != 0
                    && CHECK(BUFFER_STRING(parser), b':')
                    && (CHECK_AT(BUFFER_STRING(parser), b',', 1)
                        || CHECK_AT(BUFFER_STRING(parser), b'?', 1)
                        || CHECK_AT(BUFFER_STRING(parser), b'[', 1)
                        || CHECK_AT(BUFFER_STRING(parser), b']', 1)
                        || CHECK_AT(BUFFER_STRING(parser), b'{', 1)
                        || CHECK_AT(BUFFER_STRING(parser), b'}', 1))
                {
                    yaml_parser_set_scanner_error(
                        parser,
                        b"while scanning a plain scalar\0".as_ptr() as *const c_char,
                        start_mark,
                        b"found unexpected ':'\0".as_ptr() as *const c_char,
                    );
                    break 'error;
                }

                /* Check for indicators that may end a plain scalar. */

                if (CHECK(BUFFER_STRING(parser), b':') && IS_BLANKZ_AT(BUFFER_STRING(parser), 1))
                    || ((*parser).flow_level != 0
                        && (CHECK(BUFFER_STRING(parser), b',')
                            || CHECK(BUFFER_STRING(parser), b'[')
                            || CHECK(BUFFER_STRING(parser), b']')
                            || CHECK(BUFFER_STRING(parser), b'{')
                            || CHECK(BUFFER_STRING(parser), b'}')))
                {
                    break;
                }

                /* Check if we need to join whitespaces and breaks. */

                if leading_blanks != 0 || whitespaces.start != whitespaces.pointer {
                    if leading_blanks != 0 {
                        /* Do we need to fold line breaks? */

                        if *leading_break.start == b'\n' {
                            if *trailing_breaks.start == b'\0' {
                                if STRING_EXTEND!(parser, string) == 0 {
                                    break 'error;
                                }
                                *string.pointer = b' ';
                                string.pointer = string.pointer.add(1);
                            } else {
                                if JOIN!(parser, string, trailing_breaks) == 0 {
                                    break 'error;
                                }
                                CLEAR!(parser, trailing_breaks);
                            }
                            CLEAR!(parser, leading_break);
                        } else {
                            if JOIN!(parser, string, leading_break) == 0 {
                                break 'error;
                            }
                            if JOIN!(parser, string, trailing_breaks) == 0 {
                                break 'error;
                            }
                            CLEAR!(parser, leading_break);
                            CLEAR!(parser, trailing_breaks);
                        }

                        leading_blanks = 0;
                    } else {
                        if JOIN!(parser, string, whitespaces) == 0 {
                            break 'error;
                        }
                        CLEAR!(parser, whitespaces);
                    }
                }

                /* Copy the character. */

                if READ(parser, &mut string) == 0 {
                    break 'error;
                }

                end_mark = (*parser).mark;

                if CACHE(parser, 2) == 0 {
                    break 'error;
                }
            }

            /* Is it the end? */

            if !(IS_BLANK(BUFFER_STRING(parser)) || IS_BREAK(BUFFER_STRING(parser))) {
                break 'outer;
            }

            /* Consume blank characters. */

            if CACHE(parser, 1) == 0 {
                break 'error;
            }

            while IS_BLANK(BUFFER_STRING(parser)) || IS_BREAK(BUFFER_STRING(parser)) {
                if IS_BLANK(BUFFER_STRING(parser)) {
                    /* Check for tab characters that abuse indentation. */

                    if leading_blanks != 0
                        && ((*parser).mark.column as c_int) < indent
                        && IS_TAB(BUFFER_STRING(parser))
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a plain scalar\0".as_ptr() as *const c_char,
                            start_mark,
                            b"found a tab character that violates indentation\0".as_ptr()
                                as *const c_char,
                        );
                        break 'error;
                    }

                    /* Consume a space or a tab character. */

                    if leading_blanks == 0 {
                        if READ(parser, &mut whitespaces) == 0 {
                            break 'error;
                        }
                    } else {
                        SKIP(parser);
                    }
                } else {
                    if CACHE(parser, 2) == 0 {
                        break 'error;
                    }

                    /* Check if it is a first line break. */

                    if leading_blanks == 0 {
                        CLEAR!(parser, whitespaces);
                        if READ_LINE(parser, &mut leading_break) == 0 {
                            break 'error;
                        }
                        leading_blanks = 1;
                    } else {
                        if READ_LINE(parser, &mut trailing_breaks) == 0 {
                            break 'error;
                        }
                    }
                }
                if CACHE(parser, 1) == 0 {
                    break 'error;
                }
            }

            /* Check indentation level. */

            if (*parser).flow_level == 0 && ((*parser).mark.column as c_int) < indent {
                break 'outer;
            }
        }

        /* Create a token. */

        TOKEN_INIT(token, YAML_SCALAR_TOKEN, start_mark, end_mark);
        (*token).data.scalar.value = string.start;
        (*token).data.scalar.length = string.pointer.offset_from(string.start) as size_t;
        (*token).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;

        /* Note that we change the 'simple_key_allowed' flag. */

        if leading_blanks != 0 {
            (*parser).simple_key_allowed = 1;
        }

        STRING_DEL!(parser, leading_break);
        STRING_DEL!(parser, trailing_breaks);
        STRING_DEL!(parser, whitespaces);

        return 1;
    }

    STRING_DEL!(parser, string);
    STRING_DEL!(parser, leading_break);
    STRING_DEL!(parser, trailing_breaks);
    STRING_DEL!(parser, whitespaces);

    0
}
