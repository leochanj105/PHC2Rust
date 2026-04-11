//! Rust translation of libyaml's public header (yaml.h).
//!
//! Every type is `#[repr(C)]` so it is ABI-compatible with the original C
//! layout, and every union is a `#[repr(C)]` Rust union. Field names match
//! the C code verbatim so translated source can keep the C naming.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use libc::{c_char, c_int, c_uchar, c_void, size_t, FILE};

pub type yaml_char_t = c_uchar;

/* --------------------------------------------------------------------- */
/*  Tag constants                                                         */
/* --------------------------------------------------------------------- */

pub const YAML_NULL_TAG: &[u8] = b"tag:yaml.org,2002:null\0";
pub const YAML_BOOL_TAG: &[u8] = b"tag:yaml.org,2002:bool\0";
pub const YAML_STR_TAG: &[u8] = b"tag:yaml.org,2002:str\0";
pub const YAML_INT_TAG: &[u8] = b"tag:yaml.org,2002:int\0";
pub const YAML_FLOAT_TAG: &[u8] = b"tag:yaml.org,2002:float\0";
pub const YAML_TIMESTAMP_TAG: &[u8] = b"tag:yaml.org,2002:timestamp\0";
pub const YAML_SEQ_TAG: &[u8] = b"tag:yaml.org,2002:seq\0";
pub const YAML_MAP_TAG: &[u8] = b"tag:yaml.org,2002:map\0";

pub const YAML_DEFAULT_SCALAR_TAG: &[u8] = YAML_STR_TAG;
pub const YAML_DEFAULT_SEQUENCE_TAG: &[u8] = YAML_SEQ_TAG;
pub const YAML_DEFAULT_MAPPING_TAG: &[u8] = YAML_MAP_TAG;

pub const YAML_VERSION_MAJOR: c_int = 0;
pub const YAML_VERSION_MINOR: c_int = 2;
pub const YAML_VERSION_PATCH: c_int = 5;
pub const YAML_VERSION_STRING: &[u8] = b"0.2.5\0";

/* --------------------------------------------------------------------- */
/*  Version / tag directive                                               */
/* --------------------------------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_version_directive_t {
    pub major: c_int,
    pub minor: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_tag_directive_t {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}

/* --------------------------------------------------------------------- */
/*  Enumerations                                                          */
/* --------------------------------------------------------------------- */

pub type yaml_encoding_t = c_uint;
pub const YAML_ANY_ENCODING: yaml_encoding_t = 0;
pub const YAML_UTF8_ENCODING: yaml_encoding_t = 1;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_t = 2;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_t = 3;

pub type yaml_break_t = c_uint;
pub const YAML_ANY_BREAK: yaml_break_t = 0;
pub const YAML_CR_BREAK: yaml_break_t = 1;
pub const YAML_LN_BREAK: yaml_break_t = 2;
pub const YAML_CRLN_BREAK: yaml_break_t = 3;

pub type yaml_error_type_t = c_uint;
pub const YAML_NO_ERROR: yaml_error_type_t = 0;
pub const YAML_MEMORY_ERROR: yaml_error_type_t = 1;
pub const YAML_READER_ERROR: yaml_error_type_t = 2;
pub const YAML_SCANNER_ERROR: yaml_error_type_t = 3;
pub const YAML_PARSER_ERROR: yaml_error_type_t = 4;
pub const YAML_COMPOSER_ERROR: yaml_error_type_t = 5;
pub const YAML_WRITER_ERROR: yaml_error_type_t = 6;
pub const YAML_EMITTER_ERROR: yaml_error_type_t = 7;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_mark_t {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}

use libc::c_uint;

pub type yaml_scalar_style_t = c_uint;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_t = 0;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_t = 1;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_t = 2;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_t = 3;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_t = 4;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_t = 5;

pub type yaml_sequence_style_t = c_uint;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_t = 0;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_t = 1;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_t = 2;

pub type yaml_mapping_style_t = c_uint;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_t = 0;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_t = 1;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_t = 2;

pub type yaml_token_type_t = c_uint;
pub const YAML_NO_TOKEN: yaml_token_type_t = 0;
pub const YAML_STREAM_START_TOKEN: yaml_token_type_t = 1;
pub const YAML_STREAM_END_TOKEN: yaml_token_type_t = 2;
pub const YAML_VERSION_DIRECTIVE_TOKEN: yaml_token_type_t = 3;
pub const YAML_TAG_DIRECTIVE_TOKEN: yaml_token_type_t = 4;
pub const YAML_DOCUMENT_START_TOKEN: yaml_token_type_t = 5;
pub const YAML_DOCUMENT_END_TOKEN: yaml_token_type_t = 6;
pub const YAML_BLOCK_SEQUENCE_START_TOKEN: yaml_token_type_t = 7;
pub const YAML_BLOCK_MAPPING_START_TOKEN: yaml_token_type_t = 8;
pub const YAML_BLOCK_END_TOKEN: yaml_token_type_t = 9;
pub const YAML_FLOW_SEQUENCE_START_TOKEN: yaml_token_type_t = 10;
pub const YAML_FLOW_SEQUENCE_END_TOKEN: yaml_token_type_t = 11;
pub const YAML_FLOW_MAPPING_START_TOKEN: yaml_token_type_t = 12;
pub const YAML_FLOW_MAPPING_END_TOKEN: yaml_token_type_t = 13;
pub const YAML_BLOCK_ENTRY_TOKEN: yaml_token_type_t = 14;
pub const YAML_FLOW_ENTRY_TOKEN: yaml_token_type_t = 15;
pub const YAML_KEY_TOKEN: yaml_token_type_t = 16;
pub const YAML_VALUE_TOKEN: yaml_token_type_t = 17;
pub const YAML_ALIAS_TOKEN: yaml_token_type_t = 18;
pub const YAML_ANCHOR_TOKEN: yaml_token_type_t = 19;
pub const YAML_TAG_TOKEN: yaml_token_type_t = 20;
pub const YAML_SCALAR_TOKEN: yaml_token_type_t = 21;

pub type yaml_event_type_t = c_uint;
pub const YAML_NO_EVENT: yaml_event_type_t = 0;
pub const YAML_STREAM_START_EVENT: yaml_event_type_t = 1;
pub const YAML_STREAM_END_EVENT: yaml_event_type_t = 2;
pub const YAML_DOCUMENT_START_EVENT: yaml_event_type_t = 3;
pub const YAML_DOCUMENT_END_EVENT: yaml_event_type_t = 4;
pub const YAML_ALIAS_EVENT: yaml_event_type_t = 5;
pub const YAML_SCALAR_EVENT: yaml_event_type_t = 6;
pub const YAML_SEQUENCE_START_EVENT: yaml_event_type_t = 7;
pub const YAML_SEQUENCE_END_EVENT: yaml_event_type_t = 8;
pub const YAML_MAPPING_START_EVENT: yaml_event_type_t = 9;
pub const YAML_MAPPING_END_EVENT: yaml_event_type_t = 10;

pub type yaml_node_type_t = c_uint;
pub const YAML_NO_NODE: yaml_node_type_t = 0;
pub const YAML_SCALAR_NODE: yaml_node_type_t = 1;
pub const YAML_SEQUENCE_NODE: yaml_node_type_t = 2;
pub const YAML_MAPPING_NODE: yaml_node_type_t = 3;

pub type yaml_parser_state_t = c_uint;
pub const YAML_PARSE_STREAM_START_STATE: yaml_parser_state_t = 0;
pub const YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE: yaml_parser_state_t = 1;
pub const YAML_PARSE_DOCUMENT_START_STATE: yaml_parser_state_t = 2;
pub const YAML_PARSE_DOCUMENT_CONTENT_STATE: yaml_parser_state_t = 3;
pub const YAML_PARSE_DOCUMENT_END_STATE: yaml_parser_state_t = 4;
pub const YAML_PARSE_BLOCK_NODE_STATE: yaml_parser_state_t = 5;
pub const YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE: yaml_parser_state_t = 6;
pub const YAML_PARSE_FLOW_NODE_STATE: yaml_parser_state_t = 7;
pub const YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_t = 8;
pub const YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE: yaml_parser_state_t = 9;
pub const YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE: yaml_parser_state_t = 10;
pub const YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_parser_state_t = 11;
pub const YAML_PARSE_BLOCK_MAPPING_KEY_STATE: yaml_parser_state_t = 12;
pub const YAML_PARSE_BLOCK_MAPPING_VALUE_STATE: yaml_parser_state_t = 13;
pub const YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_t = 14;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE: yaml_parser_state_t = 15;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE: yaml_parser_state_t = 16;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE: yaml_parser_state_t = 17;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE: yaml_parser_state_t = 18;
pub const YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE: yaml_parser_state_t = 19;
pub const YAML_PARSE_FLOW_MAPPING_KEY_STATE: yaml_parser_state_t = 20;
pub const YAML_PARSE_FLOW_MAPPING_VALUE_STATE: yaml_parser_state_t = 21;
pub const YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE: yaml_parser_state_t = 22;
pub const YAML_PARSE_END_STATE: yaml_parser_state_t = 23;

pub type yaml_emitter_state_t = c_uint;
pub const YAML_EMIT_STREAM_START_STATE: yaml_emitter_state_t = 0;
pub const YAML_EMIT_FIRST_DOCUMENT_START_STATE: yaml_emitter_state_t = 1;
pub const YAML_EMIT_DOCUMENT_START_STATE: yaml_emitter_state_t = 2;
pub const YAML_EMIT_DOCUMENT_CONTENT_STATE: yaml_emitter_state_t = 3;
pub const YAML_EMIT_DOCUMENT_END_STATE: yaml_emitter_state_t = 4;
pub const YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_t = 5;
pub const YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE: yaml_emitter_state_t = 6;
pub const YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_t = 7;
pub const YAML_EMIT_FLOW_MAPPING_KEY_STATE: yaml_emitter_state_t = 8;
pub const YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_t = 9;
pub const YAML_EMIT_FLOW_MAPPING_VALUE_STATE: yaml_emitter_state_t = 10;
pub const YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_t = 11;
pub const YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE: yaml_emitter_state_t = 12;
pub const YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_t = 13;
pub const YAML_EMIT_BLOCK_MAPPING_KEY_STATE: yaml_emitter_state_t = 14;
pub const YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_t = 15;
pub const YAML_EMIT_BLOCK_MAPPING_VALUE_STATE: yaml_emitter_state_t = 16;
pub const YAML_EMIT_END_STATE: yaml_emitter_state_t = 17;

/* --------------------------------------------------------------------- */
/*  Token                                                                 */
/* --------------------------------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_stream_start_s {
    pub encoding: yaml_encoding_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_alias_s {
    pub value: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_anchor_s {
    pub value: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_tag_s {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_scalar_s {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_version_directive_s {
    pub major: c_int,
    pub minor: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_token_data_t {
    pub stream_start: yaml_token_stream_start_s,
    pub alias: yaml_token_alias_s,
    pub anchor: yaml_token_anchor_s,
    pub tag: yaml_token_tag_s,
    pub scalar: yaml_token_scalar_s,
    pub version_directive: yaml_token_version_directive_s,
    pub tag_directive: yaml_token_tag_directive_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_t {
    pub type_: yaml_token_type_t,
    pub data: yaml_token_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

/* --------------------------------------------------------------------- */
/*  Event                                                                 */
/* --------------------------------------------------------------------- */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_stream_start_s {
    pub encoding: yaml_encoding_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_document_start_tag_directives_s {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_document_start_s {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: yaml_event_document_start_tag_directives_s,
    pub implicit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_document_end_s {
    pub implicit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_alias_s {
    pub anchor: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_scalar_s {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub plain_implicit: c_int,
    pub quoted_implicit: c_int,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_sequence_start_s {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: c_int,
    pub style: yaml_sequence_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_mapping_start_s {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: c_int,
    pub style: yaml_mapping_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_event_data_t {
    pub stream_start: yaml_event_stream_start_s,
    pub document_start: yaml_event_document_start_s,
    pub document_end: yaml_event_document_end_s,
    pub alias: yaml_event_alias_s,
    pub scalar: yaml_event_scalar_s,
    pub sequence_start: yaml_event_sequence_start_s,
    pub mapping_start: yaml_event_mapping_start_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_t {
    pub type_: yaml_event_type_t,
    pub data: yaml_event_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

/* --------------------------------------------------------------------- */
/*  Node / document                                                       */
/* --------------------------------------------------------------------- */

pub type yaml_node_item_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_pair_t {
    pub key: c_int,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_scalar_s {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_sequence_items_s {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_sequence_s {
    pub items: yaml_node_sequence_items_s,
    pub style: yaml_sequence_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_mapping_pairs_s {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_mapping_s {
    pub pairs: yaml_node_mapping_pairs_s,
    pub style: yaml_mapping_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_node_data_t {
    pub scalar: yaml_node_scalar_s,
    pub sequence: yaml_node_sequence_s,
    pub mapping: yaml_node_mapping_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_t {
    pub type_: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: yaml_node_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_document_nodes_s {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_document_tag_directives_s {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_document_t {
    pub nodes: yaml_document_nodes_s,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: yaml_document_tag_directives_s,
    pub start_implicit: c_int,
    pub end_implicit: c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

/* --------------------------------------------------------------------- */
/*  Parser                                                                */
/* --------------------------------------------------------------------- */

pub type yaml_read_handler_t = unsafe extern "C" fn(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
    size_read: *mut size_t,
) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_simple_key_t {
    pub possible: c_int,
    pub required: c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_alias_data_t {
    pub anchor: *mut yaml_char_t,
    pub index: c_int,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_parser_input_string_s {
    pub start: *const c_uchar,
    pub end: *const c_uchar,
    pub current: *const c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_parser_input_t {
    pub string: yaml_parser_input_string_s,
    pub file: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_buffer_char_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_buffer_uchar_t {
    pub start: *mut c_uchar,
    pub end: *mut c_uchar,
    pub pointer: *mut c_uchar,
    pub last: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_queue_token_t {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_int_t {
    pub start: *mut c_int,
    pub end: *mut c_int,
    pub top: *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_simple_key_t {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_parser_state_t {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_mark_t {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_tag_directive_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_alias_data_t {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_parser_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,
    pub problem_offset: size_t,
    pub problem_value: c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const c_char,
    pub context_mark: yaml_mark_t,

    pub read_handler: Option<yaml_read_handler_t>,
    pub read_handler_data: *mut c_void,

    pub input: yaml_parser_input_t,
    pub eof: c_int,
    pub buffer: yaml_buffer_char_t,
    pub unread: size_t,
    pub raw_buffer: yaml_buffer_uchar_t,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,

    pub stream_start_produced: c_int,
    pub stream_end_produced: c_int,
    pub flow_level: c_int,
    pub tokens: yaml_queue_token_t,
    pub tokens_parsed: size_t,
    pub token_available: c_int,

    pub indents: yaml_stack_int_t,
    pub indent: c_int,
    pub simple_key_allowed: c_int,
    pub simple_keys: yaml_stack_simple_key_t,

    pub states: yaml_stack_parser_state_t,
    pub state: yaml_parser_state_t,
    pub marks: yaml_stack_mark_t,
    pub tag_directives: yaml_stack_tag_directive_t,

    pub aliases: yaml_stack_alias_data_t,
    pub document: *mut yaml_document_t,
}

/* --------------------------------------------------------------------- */
/*  Emitter                                                               */
/* --------------------------------------------------------------------- */

pub type yaml_write_handler_t =
    unsafe extern "C" fn(data: *mut c_void, buffer: *mut c_uchar, size: size_t) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_anchors_t {
    pub references: c_int,
    pub anchor: c_int,
    pub serialized: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_output_string_s {
    pub buffer: *mut c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_emitter_output_t {
    pub string: yaml_emitter_output_string_s,
    pub file: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_stack_emitter_state_t {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_queue_event_t {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_anchor_data_t {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_tag_data_t {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_scalar_data_t {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub multiline: c_int,
    pub flow_plain_allowed: c_int,
    pub block_plain_allowed: c_int,
    pub single_quoted_allowed: c_int,
    pub block_allowed: c_int,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,

    pub write_handler: Option<yaml_write_handler_t>,
    pub write_handler_data: *mut c_void,

    pub output: yaml_emitter_output_t,
    pub buffer: yaml_buffer_char_t,
    pub raw_buffer: yaml_buffer_uchar_t,
    pub encoding: yaml_encoding_t,

    pub canonical: c_int,
    pub best_indent: c_int,
    pub best_width: c_int,
    pub unicode: c_int,
    pub line_break: yaml_break_t,

    pub states: yaml_stack_emitter_state_t,
    pub state: yaml_emitter_state_t,
    pub events: yaml_queue_event_t,
    pub indents: yaml_stack_int_t,
    pub tag_directives: yaml_stack_tag_directive_t,

    pub indent: c_int,
    pub flow_level: c_int,
    pub root_context: c_int,
    pub sequence_context: c_int,
    pub mapping_context: c_int,
    pub simple_key_context: c_int,

    pub line: c_int,
    pub column: c_int,
    pub whitespace: c_int,
    pub indention: c_int,
    pub open_ended: c_int,

    pub anchor_data: yaml_emitter_anchor_data_t,
    pub tag_data: yaml_emitter_tag_data_t,
    pub scalar_data: yaml_emitter_scalar_data_t,

    pub opened: c_int,
    pub closed: c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: c_int,
    pub document: *mut yaml_document_t,
}
