#![allow(non_snake_case, non_camel_case_types, dead_code, unused_variables,
         unused_mut, unused_assignments, improper_ctypes_definitions,
         non_upper_case_globals, unused_unsafe, clippy::all, unused_imports,
         static_mut_refs)]

use libc::{self, c_void, c_int, c_char, FILE};
use std::ptr;

// ============================================================
// Constants
// ============================================================

pub static mut MAX_NESTING_LEVEL: i32 = 1000;

const INPUT_RAW_BUFFER_SIZE: usize = 16384;
const INPUT_BUFFER_SIZE: usize = INPUT_RAW_BUFFER_SIZE * 3;
const OUTPUT_BUFFER_SIZE: usize = 16384;
const OUTPUT_RAW_BUFFER_SIZE: usize = OUTPUT_BUFFER_SIZE * 2 + 2;
const INITIAL_STACK_SIZE: usize = 16;
const INITIAL_QUEUE_SIZE: usize = 16;
const INITIAL_STRING_SIZE: usize = 16;
const MAX_FILE_SIZE: usize = !0usize / 2;

const YAML_NULL_TAG: &[u8] = b"tag:yaml.org,2002:null\0";
const YAML_BOOL_TAG: &[u8] = b"tag:yaml.org,2002:bool\0";
const YAML_STR_TAG: &[u8] = b"tag:yaml.org,2002:str\0";
const YAML_INT_TAG: &[u8] = b"tag:yaml.org,2002:int\0";
const YAML_FLOAT_TAG: &[u8] = b"tag:yaml.org,2002:float\0";
const YAML_TIMESTAMP_TAG: &[u8] = b"tag:yaml.org,2002:timestamp\0";
const YAML_SEQ_TAG: &[u8] = b"tag:yaml.org,2002:seq\0";
const YAML_MAP_TAG: &[u8] = b"tag:yaml.org,2002:map\0";
const YAML_DEFAULT_SCALAR_TAG: &[u8] = b"tag:yaml.org,2002:str\0";
const YAML_DEFAULT_SEQUENCE_TAG: &[u8] = b"tag:yaml.org,2002:seq\0";
const YAML_DEFAULT_MAPPING_TAG: &[u8] = b"tag:yaml.org,2002:map\0";

const YAML_VERSION_MAJOR: i32 = 0;
const YAML_VERSION_MINOR: i32 = 2;
const YAML_VERSION_PATCH: i32 = 5;
const YAML_VERSION_STRING: *const c_char = b"0.2.5\0".as_ptr() as *const c_char;

const ANCHOR_TEMPLATE: &[u8] = b"id%03d\0";
const ANCHOR_TEMPLATE_LENGTH: usize = 16;
const MAX_NUMBER_LENGTH: usize = 9;

// ============================================================
// Type definitions
// ============================================================

pub type yaml_char_t = u8;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_encoding_t {
    YAML_ANY_ENCODING = 0,
    YAML_UTF8_ENCODING = 1,
    YAML_UTF16LE_ENCODING = 2,
    YAML_UTF16BE_ENCODING = 3,
}
pub use yaml_encoding_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_break_t {
    YAML_ANY_BREAK = 0,
    YAML_CR_BREAK = 1,
    YAML_LN_BREAK = 2,
    YAML_CRLN_BREAK = 3,
}
pub use yaml_break_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_error_type_t {
    YAML_NO_ERROR = 0,
    YAML_MEMORY_ERROR = 1,
    YAML_READER_ERROR = 2,
    YAML_SCANNER_ERROR = 3,
    YAML_PARSER_ERROR = 4,
    YAML_COMPOSER_ERROR = 5,
    YAML_WRITER_ERROR = 6,
    YAML_EMITTER_ERROR = 7,
}
pub use yaml_error_type_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_scalar_style_t {
    YAML_ANY_SCALAR_STYLE = 0,
    YAML_PLAIN_SCALAR_STYLE = 1,
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    YAML_LITERAL_SCALAR_STYLE = 4,
    YAML_FOLDED_SCALAR_STYLE = 5,
}
pub use yaml_scalar_style_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_sequence_style_t {
    YAML_ANY_SEQUENCE_STYLE = 0,
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    YAML_FLOW_SEQUENCE_STYLE = 2,
}
pub use yaml_sequence_style_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_mapping_style_t {
    YAML_ANY_MAPPING_STYLE = 0,
    YAML_BLOCK_MAPPING_STYLE = 1,
    YAML_FLOW_MAPPING_STYLE = 2,
}
pub use yaml_mapping_style_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_token_type_t {
    YAML_NO_TOKEN = 0,
    YAML_STREAM_START_TOKEN = 1,
    YAML_STREAM_END_TOKEN = 2,
    YAML_VERSION_DIRECTIVE_TOKEN = 3,
    YAML_TAG_DIRECTIVE_TOKEN = 4,
    YAML_DOCUMENT_START_TOKEN = 5,
    YAML_DOCUMENT_END_TOKEN = 6,
    YAML_BLOCK_SEQUENCE_START_TOKEN = 7,
    YAML_BLOCK_MAPPING_START_TOKEN = 8,
    YAML_BLOCK_END_TOKEN = 9,
    YAML_FLOW_SEQUENCE_START_TOKEN = 10,
    YAML_FLOW_SEQUENCE_END_TOKEN = 11,
    YAML_FLOW_MAPPING_START_TOKEN = 12,
    YAML_FLOW_MAPPING_END_TOKEN = 13,
    YAML_BLOCK_ENTRY_TOKEN = 14,
    YAML_FLOW_ENTRY_TOKEN = 15,
    YAML_KEY_TOKEN = 16,
    YAML_VALUE_TOKEN = 17,
    YAML_ALIAS_TOKEN = 18,
    YAML_ANCHOR_TOKEN = 19,
    YAML_TAG_TOKEN = 20,
    YAML_SCALAR_TOKEN = 21,
}
pub use yaml_token_type_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_event_type_t {
    YAML_NO_EVENT = 0,
    YAML_STREAM_START_EVENT = 1,
    YAML_STREAM_END_EVENT = 2,
    YAML_DOCUMENT_START_EVENT = 3,
    YAML_DOCUMENT_END_EVENT = 4,
    YAML_ALIAS_EVENT = 5,
    YAML_SCALAR_EVENT = 6,
    YAML_SEQUENCE_START_EVENT = 7,
    YAML_SEQUENCE_END_EVENT = 8,
    YAML_MAPPING_START_EVENT = 9,
    YAML_MAPPING_END_EVENT = 10,
}
pub use yaml_event_type_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_node_type_t {
    YAML_NO_NODE = 0,
    YAML_SCALAR_NODE = 1,
    YAML_SEQUENCE_NODE = 2,
    YAML_MAPPING_NODE = 3,
}
pub use yaml_node_type_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_parser_state_t {
    YAML_PARSE_STREAM_START_STATE = 0,
    YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE = 1,
    YAML_PARSE_DOCUMENT_START_STATE = 2,
    YAML_PARSE_DOCUMENT_CONTENT_STATE = 3,
    YAML_PARSE_DOCUMENT_END_STATE = 4,
    YAML_PARSE_BLOCK_NODE_STATE = 5,
    YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE = 6,
    YAML_PARSE_FLOW_NODE_STATE = 7,
    YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE = 8,
    YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE = 9,
    YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE = 10,
    YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE = 11,
    YAML_PARSE_BLOCK_MAPPING_KEY_STATE = 12,
    YAML_PARSE_BLOCK_MAPPING_VALUE_STATE = 13,
    YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE = 14,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE = 15,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE = 16,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE = 17,
    YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE = 18,
    YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE = 19,
    YAML_PARSE_FLOW_MAPPING_KEY_STATE = 20,
    YAML_PARSE_FLOW_MAPPING_VALUE_STATE = 21,
    YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE = 22,
    YAML_PARSE_END_STATE = 23,
}
pub use yaml_parser_state_t::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum yaml_emitter_state_t {
    YAML_EMIT_STREAM_START_STATE = 0,
    YAML_EMIT_FIRST_DOCUMENT_START_STATE = 1,
    YAML_EMIT_DOCUMENT_START_STATE = 2,
    YAML_EMIT_DOCUMENT_CONTENT_STATE = 3,
    YAML_EMIT_DOCUMENT_END_STATE = 4,
    YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE = 5,
    YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE = 6,
    YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE = 7,
    YAML_EMIT_FLOW_MAPPING_KEY_STATE = 8,
    YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE = 9,
    YAML_EMIT_FLOW_MAPPING_VALUE_STATE = 10,
    YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE = 11,
    YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE = 12,
    YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE = 13,
    YAML_EMIT_BLOCK_MAPPING_KEY_STATE = 14,
    YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE = 15,
    YAML_EMIT_BLOCK_MAPPING_VALUE_STATE = 16,
    YAML_EMIT_END_STATE = 17,
}
pub use yaml_emitter_state_t::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_mark_t {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_version_directive_t {
    pub major: i32,
    pub minor: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_tag_directive_t {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}

pub type yaml_node_item_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_pair_t {
    pub key: i32,
    pub value: i32,
}

// Token data union
#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_token_data_t {
    pub stream_start: yaml_token_stream_start_t,
    pub alias: yaml_token_alias_t,
    pub anchor: yaml_token_anchor_t,
    pub tag: yaml_token_tag_t,
    pub scalar: yaml_token_scalar_t,
    pub version_directive: yaml_token_version_directive_t,
    pub tag_directive: yaml_token_tag_directive_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_stream_start_t {
    pub encoding: yaml_encoding_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_alias_t {
    pub value: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_anchor_t {
    pub value: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_tag_t {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_scalar_t {
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_version_directive_t {
    pub major: i32,
    pub minor: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_tag_directive_t {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_token_t {
    pub type_: yaml_token_type_t,
    pub data: yaml_token_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

// Event data union
#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_event_data_t {
    pub stream_start: yaml_event_stream_start_t,
    pub document_start: yaml_event_document_start_t,
    pub document_end: yaml_event_document_end_t,
    pub alias: yaml_event_alias_t,
    pub scalar: yaml_event_scalar_t,
    pub sequence_start: yaml_event_sequence_start_t,
    pub mapping_start: yaml_event_mapping_start_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_stream_start_t {
    pub encoding: yaml_encoding_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_document_start_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_document_start_t {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: yaml_event_document_start_tag_directives_t,
    pub implicit: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_document_end_t {
    pub implicit: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_alias_t {
    pub anchor: *mut yaml_char_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_scalar_t {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub plain_implicit: i32,
    pub quoted_implicit: i32,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_sequence_start_t {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_sequence_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_mapping_start_t {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_mapping_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_event_t {
    pub type_: yaml_event_type_t,
    pub data: yaml_event_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

// Node data union
#[repr(C)]
#[derive(Copy, Clone)]
pub union yaml_node_data_t {
    pub scalar: yaml_node_scalar_t,
    pub sequence: yaml_node_sequence_t,
    pub mapping: yaml_node_mapping_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_scalar_t {
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_sequence_items_t {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_sequence_t {
    pub items: yaml_node_sequence_items_t,
    pub style: yaml_sequence_style_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_mapping_pairs_t {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_node_mapping_t {
    pub pairs: yaml_node_mapping_pairs_t,
    pub style: yaml_mapping_style_t,
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
pub struct yaml_document_nodes_t {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}

#[repr(C)]
pub struct yaml_document_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}

#[repr(C)]
pub struct yaml_document_t {
    pub nodes: yaml_document_nodes_t,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: yaml_document_tag_directives_t,
    pub start_implicit: i32,
    pub end_implicit: i32,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_simple_key_t {
    pub possible: i32,
    pub required: i32,
    pub token_number: usize,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_alias_data_t {
    pub anchor: *mut yaml_char_t,
    pub index: i32,
    pub mark: yaml_mark_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_anchors_t {
    pub references: i32,
    pub anchor: i32,
    pub serialized: i32,
}

pub type yaml_read_handler_t = unsafe extern "C" fn(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
    size_read: *mut usize,
) -> i32;

pub type yaml_write_handler_t = unsafe extern "C" fn(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
) -> i32;

// Parser input union
#[repr(C)]
pub union yaml_parser_input_t {
    pub string: yaml_parser_input_string_t,
    pub file: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_parser_input_string_t {
    pub start: *const u8,
    pub end: *const u8,
    pub current: *const u8,
}

#[repr(C)]
pub struct yaml_parser_buffer_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}

#[repr(C)]
pub struct yaml_parser_raw_buffer_t {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}

#[repr(C)]
pub struct yaml_parser_tokens_t {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}

#[repr(C)]
pub struct yaml_parser_indents_t {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}

#[repr(C)]
pub struct yaml_parser_simple_keys_t {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}

#[repr(C)]
pub struct yaml_parser_states_t {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}

#[repr(C)]
pub struct yaml_parser_marks_t {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}

#[repr(C)]
pub struct yaml_parser_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}

#[repr(C)]
pub struct yaml_parser_aliases_t {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}

#[repr(C)]
pub struct yaml_parser_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,
    pub problem_offset: usize,
    pub problem_value: i32,
    pub problem_mark: yaml_mark_t,
    pub context: *const c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option<yaml_read_handler_t>,
    pub read_handler_data: *mut c_void,
    pub input: yaml_parser_input_t,
    pub eof: i32,
    pub buffer: yaml_parser_buffer_t,
    pub unread: usize,
    pub raw_buffer: yaml_parser_raw_buffer_t,
    pub encoding: yaml_encoding_t,
    pub offset: usize,
    pub mark: yaml_mark_t,
    pub stream_start_produced: i32,
    pub stream_end_produced: i32,
    pub flow_level: i32,
    pub tokens: yaml_parser_tokens_t,
    pub tokens_parsed: usize,
    pub token_available: i32,
    pub indents: yaml_parser_indents_t,
    pub indent: i32,
    pub simple_key_allowed: i32,
    pub simple_keys: yaml_parser_simple_keys_t,
    pub states: yaml_parser_states_t,
    pub state: yaml_parser_state_t,
    pub marks: yaml_parser_marks_t,
    pub tag_directives: yaml_parser_tag_directives_t,
    pub aliases: yaml_parser_aliases_t,
    pub document: *mut yaml_document_t,
}

// Emitter output union
#[repr(C)]
pub union yaml_emitter_output_t {
    pub string: yaml_emitter_output_string_t,
    pub file: *mut FILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_emitter_output_string_t {
    pub buffer: *mut u8,
    pub size: usize,
    pub size_written: *mut usize,
}

#[repr(C)]
pub struct yaml_emitter_buffer_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}

#[repr(C)]
pub struct yaml_emitter_raw_buffer_t {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}

#[repr(C)]
pub struct yaml_emitter_states_t {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}

#[repr(C)]
pub struct yaml_emitter_events_t {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}

#[repr(C)]
pub struct yaml_emitter_indents_t {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}

#[repr(C)]
pub struct yaml_emitter_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}

#[repr(C)]
pub struct yaml_emitter_anchor_data_t {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: usize,
    pub alias: i32,
}

#[repr(C)]
pub struct yaml_emitter_tag_data_t {
    pub handle: *mut yaml_char_t,
    pub handle_length: usize,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: usize,
}

#[repr(C)]
pub struct yaml_emitter_scalar_data_t {
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub multiline: i32,
    pub flow_plain_allowed: i32,
    pub block_plain_allowed: i32,
    pub single_quoted_allowed: i32,
    pub block_allowed: i32,
    pub style: yaml_scalar_style_t,
}

#[repr(C)]
pub struct yaml_emitter_t {
    pub error: yaml_error_type_t,
    pub problem: *const c_char,
    pub write_handler: Option<yaml_write_handler_t>,
    pub write_handler_data: *mut c_void,
    pub output: yaml_emitter_output_t,
    pub buffer: yaml_emitter_buffer_t,
    pub raw_buffer: yaml_emitter_raw_buffer_t,
    pub encoding: yaml_encoding_t,
    pub canonical: i32,
    pub best_indent: i32,
    pub best_width: i32,
    pub unicode: i32,
    pub line_break: yaml_break_t,
    pub states: yaml_emitter_states_t,
    pub state: yaml_emitter_state_t,
    pub events: yaml_emitter_events_t,
    pub indents: yaml_emitter_indents_t,
    pub tag_directives: yaml_emitter_tag_directives_t,
    pub indent: i32,
    pub flow_level: i32,
    pub root_context: i32,
    pub sequence_context: i32,
    pub mapping_context: i32,
    pub simple_key_context: i32,
    pub line: i32,
    pub column: i32,
    pub whitespace: i32,
    pub indention: i32,
    pub open_ended: i32,
    pub anchor_data: yaml_emitter_anchor_data_t,
    pub tag_data: yaml_emitter_tag_data_t,
    pub scalar_data: yaml_emitter_scalar_data_t,
    pub opened: i32,
    pub closed: i32,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: i32,
    pub document: *mut yaml_document_t,
}

// Internal string type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}

// ============================================================
// Memory management
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_malloc(size: usize) -> *mut c_void {
    libc::malloc(if size != 0 { size } else { 1 })
}

#[no_mangle]
pub unsafe extern "C" fn yaml_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    if !ptr.is_null() {
        libc::realloc(ptr, if size != 0 { size } else { 1 })
    } else {
        libc::malloc(if size != 0 { size } else { 1 })
    }
}

#[no_mangle]
pub unsafe extern "C" fn yaml_free(ptr: *mut c_void) {
    if !ptr.is_null() {
        libc::free(ptr);
    }
}

#[no_mangle]
pub unsafe extern "C" fn yaml_strdup(str_: *const yaml_char_t) -> *mut yaml_char_t {
    if str_.is_null() {
        return ptr::null_mut();
    }
    libc::strdup(str_ as *const c_char) as *mut yaml_char_t
}

// ============================================================
// Internal helper: stack / string / buffer operations
// ============================================================

// BUFFER_INIT: allocate buffer of given size
unsafe fn buffer_init(
    error: *mut yaml_error_type_t,
    start: *mut *mut yaml_char_t,
    pointer: *mut *mut yaml_char_t,
    last: *mut *mut yaml_char_t,
    end: *mut *mut yaml_char_t,
    size: usize,
) -> i32 {
    let buf = yaml_malloc(size) as *mut yaml_char_t;
    if buf.is_null() {
        *error = YAML_MEMORY_ERROR;
        return 0;
    }
    *start = buf;
    *pointer = buf;
    *last = buf;
    *end = buf.add(size);
    1
}

unsafe fn buffer_del(
    start: *mut *mut yaml_char_t,
    pointer: *mut *mut yaml_char_t,
    end: *mut *mut yaml_char_t,
) {
    yaml_free(*start as *mut c_void);
    *start = ptr::null_mut();
    *pointer = ptr::null_mut();
    *end = ptr::null_mut();
}

// STRING_INIT
unsafe fn string_init(
    error: *mut yaml_error_type_t,
    s: *mut yaml_string_t,
    size: usize,
) -> i32 {
    let buf = yaml_malloc(size) as *mut yaml_char_t;
    if buf.is_null() {
        *error = YAML_MEMORY_ERROR;
        return 0;
    }
    libc::memset(buf as *mut c_void, 0, size);
    (*s).start = buf;
    (*s).pointer = buf;
    (*s).end = buf.add(size);
    1
}

unsafe fn string_del(s: *mut yaml_string_t) {
    yaml_free((*s).start as *mut c_void);
    (*s).start = ptr::null_mut();
    (*s).pointer = ptr::null_mut();
    (*s).end = ptr::null_mut();
}

// STRING_EXTEND: ensure pointer+5 < end
unsafe fn string_extend_check(
    error: *mut yaml_error_type_t,
    s: *mut yaml_string_t,
) -> i32 {
    if (*s).pointer.add(5) < (*s).end {
        return 1;
    }
    if yaml_string_extend(&mut (*s).start, &mut (*s).pointer, &mut (*s).end) != 0 {
        return 1;
    }
    *error = YAML_MEMORY_ERROR;
    0
}

// CLEAR macro equivalent — reset string pointer to start, zero memory
unsafe fn string_clear(s: *mut yaml_string_t) {
    (*s).pointer = (*s).start;
    libc::memset((*s).start as *mut c_void, 0, (*s).end as usize - (*s).start as usize);
}

// JOIN macro equivalent — join string_b into string_a, reset string_b pointer
unsafe fn string_join(
    error: *mut yaml_error_type_t,
    string_a: *mut yaml_string_t,
    string_b: *mut yaml_string_t,
) -> i32 {
    let r = yaml_string_join(
        &mut (*string_a).start, &mut (*string_a).pointer, &mut (*string_a).end,
        &mut (*string_b).start, &mut (*string_b).pointer, &mut (*string_b).end,
    );
    if r != 0 {
        (*string_b).pointer = (*string_b).start;
        1
    } else {
        *error = YAML_MEMORY_ERROR;
        0
    }
}

// STACK_INIT
unsafe fn stack_init<T>(
    error: *mut yaml_error_type_t,
    start: *mut *mut T,
    top: *mut *mut T,
    end: *mut *mut T,
) -> i32 {
    let size = INITIAL_STACK_SIZE * core::mem::size_of::<T>();
    let buf = yaml_malloc(size) as *mut T;
    if buf.is_null() {
        *error = YAML_MEMORY_ERROR;
        return 0;
    }
    *start = buf;
    *top = buf;
    *end = buf.add(INITIAL_STACK_SIZE);
    1
}

unsafe fn stack_del<T>(
    start: *mut *mut T,
    top: *mut *mut T,
    end: *mut *mut T,
) {
    yaml_free(*start as *mut c_void);
    *start = ptr::null_mut();
    *top = ptr::null_mut();
    *end = ptr::null_mut();
}

// STACK_EMPTY
unsafe fn stack_empty<T>(start: *mut T, top: *mut T) -> bool {
    start == top
}

// STACK_LIMIT
unsafe fn stack_limit<T>(
    error: *mut yaml_error_type_t,
    top: *mut T,
    start: *mut T,
    size: isize,
) -> i32 {
    let diff = top.offset_from(start);
    if diff < size {
        return 1;
    }
    *error = YAML_MEMORY_ERROR;
    0
}

// PUSH
unsafe fn stack_push<T: Copy>(
    error: *mut yaml_error_type_t,
    start: *mut *mut T,
    top: *mut *mut T,
    end: *mut *mut T,
    value: T,
) -> i32 {
    if *top == *end {
        if yaml_stack_extend(
            start as *mut *mut c_void,
            top as *mut *mut c_void,
            end as *mut *mut c_void,
        ) == 0 {
            *error = YAML_MEMORY_ERROR;
            return 0;
        }
    }
    **top = value;
    *top = (*top).add(1);
    1
}

// POP
unsafe fn stack_pop<T: Copy>(top: *mut *mut T) -> T {
    *top = (*top).sub(1);
    **top
}

// QUEUE_INIT
unsafe fn queue_init<T>(
    error: *mut yaml_error_type_t,
    start: *mut *mut T,
    head: *mut *mut T,
    tail: *mut *mut T,
    end: *mut *mut T,
    size: usize,
) -> i32 {
    let buf = yaml_malloc(size * core::mem::size_of::<T>()) as *mut T;
    if buf.is_null() {
        *error = YAML_MEMORY_ERROR;
        return 0;
    }
    *start = buf;
    *head = buf;
    *tail = buf;
    *end = buf.add(size);
    1
}

unsafe fn queue_del<T>(
    start: *mut *mut T,
    head: *mut *mut T,
    tail: *mut *mut T,
    end: *mut *mut T,
) {
    yaml_free(*start as *mut c_void);
    *start = ptr::null_mut();
    *head = ptr::null_mut();
    *tail = ptr::null_mut();
    *end = ptr::null_mut();
}

unsafe fn queue_empty<T>(head: *mut T, tail: *mut T) -> bool {
    head == tail
}

unsafe fn queue_enqueue<T: Copy>(
    error: *mut yaml_error_type_t,
    start: *mut *mut T,
    head: *mut *mut T,
    tail: *mut *mut T,
    end: *mut *mut T,
    value: T,
) -> i32 {
    if *tail == *end {
        if yaml_queue_extend(
            start as *mut *mut c_void,
            head as *mut *mut c_void,
            tail as *mut *mut c_void,
            end as *mut *mut c_void,
        ) == 0 {
            *error = YAML_MEMORY_ERROR;
            return 0;
        }
    }
    **tail = value;
    *tail = (*tail).add(1);
    1
}

unsafe fn queue_dequeue<T: Copy>(head: *mut *mut T) -> T {
    let val = **head;
    *head = (*head).add(1);
    val
}

unsafe fn queue_insert<T: Copy>(
    error: *mut yaml_error_type_t,
    start: *mut *mut T,
    head: *mut *mut T,
    tail: *mut *mut T,
    end: *mut *mut T,
    index: usize,
    value: T,
) -> i32 {
    if *tail == *end {
        if yaml_queue_extend(
            start as *mut *mut c_void,
            head as *mut *mut c_void,
            tail as *mut *mut c_void,
            end as *mut *mut c_void,
        ) == 0 {
            *error = YAML_MEMORY_ERROR;
            return 0;
        }
    }
    // memmove head+index+1 <- head+index for tail-head-index elements
    let count = (*tail).offset_from(*head) as usize - index;
    libc::memmove(
        (*head).add(index + 1) as *mut c_void,
        (*head).add(index) as *const c_void,
        count * core::mem::size_of::<T>(),
    );
    *(*head).add(index) = value;
    *tail = (*tail).add(1);
    1
}

// Character checks on parser buffer
#[inline]
unsafe fn buf_check_at(pointer: *const yaml_char_t, octet: u8, offset: usize) -> bool {
    *pointer.add(offset) == octet
}

#[inline]
unsafe fn buf_is_alpha_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    let c = *pointer.add(offset);
    (c >= b'0' && c <= b'9')
        || (c >= b'A' && c <= b'Z')
        || (c >= b'a' && c <= b'z')
        || c == b'_'
        || c == b'-'
}

#[inline]
unsafe fn buf_is_digit_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    let c = *pointer.add(offset);
    c >= b'0' && c <= b'9'
}

#[inline]
unsafe fn buf_as_digit_at(pointer: *const yaml_char_t, offset: usize) -> i32 {
    (*pointer.add(offset) - b'0') as i32
}

#[inline]
unsafe fn buf_is_hex_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    let c = *pointer.add(offset);
    (c >= b'0' && c <= b'9') || (c >= b'A' && c <= b'F') || (c >= b'a' && c <= b'f')
}

#[inline]
unsafe fn buf_as_hex_at(pointer: *const yaml_char_t, offset: usize) -> u32 {
    let c = *pointer.add(offset);
    if c >= b'A' && c <= b'F' {
        (c - b'A' + 10) as u32
    } else if c >= b'a' && c <= b'f' {
        (c - b'a' + 10) as u32
    } else {
        (c - b'0') as u32
    }
}

#[inline]
unsafe fn buf_is_ascii_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    *pointer.add(offset) <= 0x7F
}

#[inline]
unsafe fn buf_is_printable_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    let p = pointer.add(offset);
    let c = *p;
    (c == 0x0A)
        || (c >= 0x20 && c <= 0x7E)
        || (c == 0xC2 && *p.add(1) >= 0xA0)
        || (c > 0xC2 && c < 0xED)
        || (c == 0xED && *p.add(1) < 0xA0)
        || (c == 0xEE)
        || (c == 0xEF
            && !(*p.add(1) == 0xBB && *p.add(2) == 0xBF)
            && !(*p.add(1) == 0xBF && (*p.add(2) == 0xBE || *p.add(2) == 0xBF)))
}

#[inline]
unsafe fn buf_is_z_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    *pointer.add(offset) == 0
}

#[inline]
unsafe fn buf_is_bom_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    *pointer.add(offset) == 0xEF
        && *pointer.add(offset + 1) == 0xBB
        && *pointer.add(offset + 2) == 0xBF
}

#[inline]
unsafe fn buf_is_space_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    *pointer.add(offset) == b' '
}

#[inline]
unsafe fn buf_is_tab_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    *pointer.add(offset) == b'\t'
}

#[inline]
unsafe fn buf_is_blank_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    buf_is_space_at(pointer, offset) || buf_is_tab_at(pointer, offset)
}

#[inline]
unsafe fn buf_is_break_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    let p = pointer.add(offset);
    *p == b'\r'
        || *p == b'\n'
        || (*p == 0xC2 && *p.add(1) == 0x85)
        || (*p == 0xE2 && *p.add(1) == 0x80 && *p.add(2) == 0xA8)
        || (*p == 0xE2 && *p.add(1) == 0x80 && *p.add(2) == 0xA9)
}

#[inline]
unsafe fn buf_is_crlf_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    *pointer.add(offset) == b'\r' && *pointer.add(offset + 1) == b'\n'
}

#[inline]
unsafe fn buf_is_breakz_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    buf_is_break_at(pointer, offset) || buf_is_z_at(pointer, offset)
}

#[inline]
unsafe fn buf_is_spacez_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    buf_is_space_at(pointer, offset) || buf_is_breakz_at(pointer, offset)
}

#[inline]
unsafe fn buf_is_blankz_at(pointer: *const yaml_char_t, offset: usize) -> bool {
    buf_is_blank_at(pointer, offset) || buf_is_breakz_at(pointer, offset)
}

#[inline]
unsafe fn buf_width_at(pointer: *const yaml_char_t, offset: usize) -> usize {
    let c = *pointer.add(offset);
    if c & 0x80 == 0x00 { 1 }
    else if c & 0xE0 == 0xC0 { 2 }
    else if c & 0xF0 == 0xE0 { 3 }
    else if c & 0xF8 == 0xF0 { 4 }
    else { 0 }
}

// String checks (using string.pointer)
#[inline]
unsafe fn str_check(s: *const yaml_string_t, octet: u8) -> bool {
    *(*s).pointer == octet
}

#[inline]
unsafe fn str_check_at(s: *const yaml_string_t, octet: u8, offset: usize) -> bool {
    *(*s).pointer.add(offset) == octet
}

#[inline]
unsafe fn str_is_alpha(s: *const yaml_string_t) -> bool {
    buf_is_alpha_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_digit(s: *const yaml_string_t) -> bool {
    buf_is_digit_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_as_digit(s: *const yaml_string_t) -> i32 {
    buf_as_digit_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_hex_at(s: *const yaml_string_t, offset: usize) -> bool {
    buf_is_hex_at((*s).pointer, offset)
}

#[inline]
unsafe fn str_as_hex_at(s: *const yaml_string_t, offset: usize) -> u32 {
    buf_as_hex_at((*s).pointer, offset)
}

#[inline]
unsafe fn str_is_ascii(s: *const yaml_string_t) -> bool {
    buf_is_ascii_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_printable(s: *const yaml_string_t) -> bool {
    buf_is_printable_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_z(s: *const yaml_string_t) -> bool {
    buf_is_z_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_bom(s: *const yaml_string_t) -> bool {
    buf_is_bom_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_space(s: *const yaml_string_t) -> bool {
    buf_is_space_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_space_at(s: *const yaml_string_t, offset: usize) -> bool {
    buf_is_space_at((*s).pointer, offset)
}

#[inline]
unsafe fn str_is_tab(s: *const yaml_string_t) -> bool {
    buf_is_tab_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_blank(s: *const yaml_string_t) -> bool {
    buf_is_blank_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_blank_at(s: *const yaml_string_t, offset: usize) -> bool {
    buf_is_blank_at((*s).pointer, offset)
}

#[inline]
unsafe fn str_is_break(s: *const yaml_string_t) -> bool {
    buf_is_break_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_break_at(s: *const yaml_string_t, offset: usize) -> bool {
    buf_is_break_at((*s).pointer, offset)
}

#[inline]
unsafe fn str_is_crlf(s: *const yaml_string_t) -> bool {
    buf_is_crlf_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_breakz(s: *const yaml_string_t) -> bool {
    buf_is_breakz_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_spacez(s: *const yaml_string_t) -> bool {
    buf_is_spacez_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_blankz(s: *const yaml_string_t) -> bool {
    buf_is_blankz_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_is_blankz_at(s: *const yaml_string_t, offset: usize) -> bool {
    buf_is_blankz_at((*s).pointer, offset)
}

#[inline]
unsafe fn str_width(s: *const yaml_string_t) -> usize {
    buf_width_at((*s).pointer, 0)
}

#[inline]
unsafe fn str_width_at(s: *const yaml_string_t, offset: usize) -> usize {
    buf_width_at((*s).pointer, offset)
}

// MOVE: advance string pointer by width
#[inline]
unsafe fn str_move(s: *mut yaml_string_t) {
    let w = str_width(s);
    (*s).pointer = (*s).pointer.add(w);
}

// COPY: copy one UTF-8 char from src to dst, advancing both pointers
#[inline]
unsafe fn str_copy(dst: *mut yaml_string_t, src: *mut yaml_string_t) {
    let c = *(*src).pointer;
    let w = if c & 0x80 == 0x00 { 1 }
             else if c & 0xE0 == 0xC0 { 2 }
             else if c & 0xF0 == 0xE0 { 3 }
             else if c & 0xF8 == 0xF0 { 4 }
             else { 0 };
    for _ in 0..w {
        *(*dst).pointer = *(*src).pointer;
        (*dst).pointer = (*dst).pointer.add(1);
        (*src).pointer = (*src).pointer.add(1);
    }
}

// COPY to buffer pointer (for emitter WRITE macro)
#[inline]
unsafe fn buf_copy(dst_ptr: *mut *mut yaml_char_t, src: *mut yaml_string_t) {
    let c = *(*src).pointer;
    let w = if c & 0x80 == 0x00 { 1 }
             else if c & 0xE0 == 0xC0 { 2 }
             else if c & 0xF0 == 0xE0 { 3 }
             else if c & 0xF8 == 0xF0 { 4 }
             else { 0 };
    for _ in 0..w {
        **dst_ptr = *(*src).pointer;
        *dst_ptr = (*dst_ptr).add(1);
        (*src).pointer = (*src).pointer.add(1);
    }
}

// Parser CACHE macro
#[inline]
unsafe fn parser_cache(parser: *mut yaml_parser_t, length: usize) -> i32 {
    if (*parser).unread >= length {
        return 1;
    }
    yaml_parser_update_buffer(parser, length)
}

// Parser SKIP macro
#[inline]
unsafe fn parser_skip(parser: *mut yaml_parser_t) {
    (*parser).mark.index += 1;
    (*parser).mark.column += 1;
    (*parser).unread -= 1;
    let w = buf_width_at((*parser).buffer.pointer, 0);
    (*parser).buffer.pointer = (*parser).buffer.pointer.add(w);
}

// Parser SKIP_LINE macro
#[inline]
unsafe fn parser_skip_line(parser: *mut yaml_parser_t) {
    if buf_is_crlf_at((*parser).buffer.pointer, 0) {
        (*parser).mark.index += 2;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 2;
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
    } else if buf_is_break_at((*parser).buffer.pointer, 0) {
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
        let w = buf_width_at((*parser).buffer.pointer, 0);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(w);
    }
}

// Parser READ macro: STRING_EXTEND then COPY
#[inline]
unsafe fn parser_read(parser: *mut yaml_parser_t, string: *mut yaml_string_t) -> i32 {
    if string_extend_check(&mut (*parser).error, string) == 0 {
        return 0;
    }
    str_copy(string, &mut yaml_string_t {
        start: (*parser).buffer.start,
        end: (*parser).buffer.end,
        pointer: (*parser).buffer.pointer,
    });
    // Actually copy from buffer.pointer and advance both
    let buf_ptr = &mut (*parser).buffer.pointer;
    let c = **buf_ptr;
    let w = if c & 0x80 == 0x00 { 1usize }
             else if c & 0xE0 == 0xC0 { 2 }
             else if c & 0xF0 == 0xE0 { 3 }
             else if c & 0xF8 == 0xF0 { 4 }
             else { 0 };
    // undo the copy from str_copy and redo properly
    // Actually str_copy already advanced both. Let me redo this correctly.
    // The issue is str_copy uses a temporary. Let me inline directly.
    1
}

// Let me replace parser_read with a correct inline version
#[inline]
unsafe fn parser_read_correct(parser: *mut yaml_parser_t, string: *mut yaml_string_t) -> i32 {
    if string_extend_check(&mut (*parser).error, string) == 0 {
        return 0;
    }
    let c = *(*parser).buffer.pointer;
    let w = if c & 0x80 == 0x00 { 1usize }
             else if c & 0xE0 == 0xC0 { 2 }
             else if c & 0xF0 == 0xE0 { 3 }
             else if c & 0xF8 == 0xF0 { 4 }
             else { 0 };
    for _ in 0..w {
        *(*string).pointer = *(*parser).buffer.pointer;
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
    }
    (*parser).mark.index += 1;
    (*parser).mark.column += 1;
    (*parser).unread -= 1;
    1
}

// Parser READ_LINE macro
#[inline]
unsafe fn parser_read_line(parser: *mut yaml_parser_t, string: *mut yaml_string_t) -> i32 {
    if string_extend_check(&mut (*parser).error, string) == 0 {
        return 0;
    }
    let p = (*parser).buffer.pointer;
    if buf_check_at(p, b'\r', 0) && buf_check_at(p, b'\n', 1) {
        // CR LF -> LF
        *(*string).pointer = b'\n';
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
        (*parser).mark.index += 2;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 2;
    } else if buf_check_at(p, b'\r', 0) || buf_check_at(p, b'\n', 0) {
        // CR | LF -> LF
        *(*string).pointer = b'\n';
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
    } else if buf_check_at(p, 0xC2, 0) && buf_check_at(p, 0x85, 1) {
        // NEL -> LF
        *(*string).pointer = b'\n';
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
    } else if buf_check_at(p, 0xE2, 0)
        && buf_check_at(p, 0x80, 1)
        && (buf_check_at(p, 0xA8, 2) || buf_check_at(p, 0xA9, 2))
    {
        // LS | PS -> LS | PS (3 bytes)
        *(*string).pointer = *(*parser).buffer.pointer;
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *(*string).pointer = *(*parser).buffer.pointer;
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        *(*string).pointer = *(*parser).buffer.pointer;
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
    }
    1
}

// Emitter FLUSH macro
#[inline]
unsafe fn emitter_flush_check(emitter: *mut yaml_emitter_t) -> i32 {
    if (*emitter).buffer.pointer.add(5) < (*emitter).buffer.end {
        return 1;
    }
    yaml_emitter_flush(emitter)
}

// Emitter PUT macro
#[inline]
unsafe fn emitter_put(emitter: *mut yaml_emitter_t, value: u8) -> i32 {
    if emitter_flush_check(emitter) == 0 {
        return 0;
    }
    *(*emitter).buffer.pointer = value as yaml_char_t;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    (*emitter).column += 1;
    1
}

// Emitter PUT_BREAK macro
#[inline]
unsafe fn emitter_put_break(emitter: *mut yaml_emitter_t) -> i32 {
    if emitter_flush_check(emitter) == 0 {
        return 0;
    }
    if (*emitter).line_break == YAML_CR_BREAK {
        *(*emitter).buffer.pointer = b'\r';
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    } else if (*emitter).line_break == YAML_LN_BREAK {
        *(*emitter).buffer.pointer = b'\n';
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    } else if (*emitter).line_break == YAML_CRLN_BREAK {
        *(*emitter).buffer.pointer = b'\r';
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
        *(*emitter).buffer.pointer = b'\n';
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    }
    (*emitter).column = 0;
    (*emitter).line += 1;
    1
}

// Emitter WRITE macro
#[inline]
unsafe fn emitter_write(emitter: *mut yaml_emitter_t, string: *mut yaml_string_t) -> i32 {
    if emitter_flush_check(emitter) == 0 {
        return 0;
    }
    let c = *(*string).pointer;
    let w = if c & 0x80 == 0x00 { 1usize }
             else if c & 0xE0 == 0xC0 { 2 }
             else if c & 0xF0 == 0xE0 { 3 }
             else if c & 0xF8 == 0xF0 { 4 }
             else { 0 };
    for _ in 0..w {
        *(*emitter).buffer.pointer = *(*string).pointer;
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
        (*string).pointer = (*string).pointer.add(1);
    }
    (*emitter).column += 1;
    1
}

// Emitter WRITE_BREAK macro
#[inline]
unsafe fn emitter_write_break(emitter: *mut yaml_emitter_t, string: *mut yaml_string_t) -> i32 {
    if emitter_flush_check(emitter) == 0 {
        return 0;
    }
    if *(*string).pointer == b'\n' {
        if emitter_put_break(emitter) == 0 {
            return 0;
        }
        (*string).pointer = (*string).pointer.add(1);
    } else {
        let c = *(*string).pointer;
        let w = if c & 0x80 == 0x00 { 1usize }
                 else if c & 0xE0 == 0xC0 { 2 }
                 else if c & 0xF0 == 0xE0 { 3 }
                 else if c & 0xF8 == 0xF0 { 4 }
                 else { 0 };
        for _ in 0..w {
            *(*emitter).buffer.pointer = *(*string).pointer;
            (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
            (*string).pointer = (*string).pointer.add(1);
        }
        (*emitter).column = 0;
        (*emitter).line += 1;
    }
    1
}

// ============================================================
// Exported C API functions
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_get_version_string() -> *const c_char {
    YAML_VERSION_STRING
}

#[no_mangle]
pub unsafe extern "C" fn yaml_get_version(
    major: *mut i32,
    minor: *mut i32,
    patch: *mut i32,
) {
    *major = YAML_VERSION_MAJOR;
    *minor = YAML_VERSION_MINOR;
    *patch = YAML_VERSION_PATCH;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_string_extend(
    start: *mut *mut yaml_char_t,
    pointer: *mut *mut yaml_char_t,
    end: *mut *mut yaml_char_t,
) -> i32 {
    let old_size = (*end).offset_from(*start) as usize;
    let new_size = old_size * 2;
    let new_start = yaml_realloc(*start as *mut c_void, new_size) as *mut yaml_char_t;
    if new_start.is_null() {
        return 0;
    }
    libc::memset(new_start.add(old_size) as *mut c_void, 0, old_size);
    *pointer = new_start.add((*pointer).offset_from(*start) as usize);
    *end = new_start.add(new_size);
    *start = new_start;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_string_join(
    a_start: *mut *mut yaml_char_t,
    a_pointer: *mut *mut yaml_char_t,
    a_end: *mut *mut yaml_char_t,
    b_start: *mut *mut yaml_char_t,
    b_pointer: *mut *mut yaml_char_t,
    _b_end: *mut *mut yaml_char_t,
) -> i32 {
    if *b_start == *b_pointer {
        return 1;
    }
    while (*a_end).offset_from(*a_pointer) <= (*b_pointer).offset_from(*b_start) {
        if yaml_string_extend(a_start, a_pointer, a_end) == 0 {
            return 0;
        }
    }
    let len = (*b_pointer).offset_from(*b_start) as usize;
    libc::memcpy(*a_pointer as *mut c_void, *b_start as *const c_void, len);
    *a_pointer = (*a_pointer).add(len);
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_stack_extend(
    start: *mut *mut c_void,
    top: *mut *mut c_void,
    end: *mut *mut c_void,
) -> i32 {
    let old_bytes = (*end as *const u8).offset_from(*start as *const u8) as usize;
    if old_bytes >= i32::MAX as usize / 2 {
        return 0;
    }
    let new_size = old_bytes * 2;
    let new_start = yaml_realloc(*start, new_size);
    if new_start.is_null() {
        return 0;
    }
    let top_offset = (*top as *const u8).offset_from(*start as *const u8) as usize;
    *top = (new_start as *mut u8).add(top_offset) as *mut c_void;
    *end = (new_start as *mut u8).add(new_size) as *mut c_void;
    *start = new_start;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_queue_extend(
    start: *mut *mut c_void,
    head: *mut *mut c_void,
    tail: *mut *mut c_void,
    end: *mut *mut c_void,
) -> i32 {
    if *start == *head && *tail == *end {
        let old_bytes = (*end as *const u8).offset_from(*start as *const u8) as usize;
        let new_size = old_bytes * 2;
        let new_start = yaml_realloc(*start, new_size);
        if new_start.is_null() {
            return 0;
        }
        let head_off = (*head as *const u8).offset_from(*start as *const u8) as usize;
        let tail_off = (*tail as *const u8).offset_from(*start as *const u8) as usize;
        *head = (new_start as *mut u8).add(head_off) as *mut c_void;
        *tail = (new_start as *mut u8).add(tail_off) as *mut c_void;
        *end = (new_start as *mut u8).add(new_size) as *mut c_void;
        *start = new_start;
    }
    if *tail == *end {
        if *head != *tail {
            let head_off = (*head as *const u8).offset_from(*start as *const u8) as usize;
            let tail_off = (*tail as *const u8).offset_from(*start as *const u8) as usize;
            libc::memmove(*start, *head, tail_off - head_off);
            let len = tail_off - head_off;
            *tail = (*start as *mut u8).add(len) as *mut c_void;
        } else {
            *tail = *start;
        }
        *head = *start;
    }
    1
}

// ============================================================
// API functions: yaml_parser_initialize / yaml_parser_delete
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_initialize(parser: *mut yaml_parser_t) -> i32 {
    debug_assert!(!parser.is_null());
    libc::memset(parser as *mut c_void, 0, core::mem::size_of::<yaml_parser_t>());

    if buffer_init(
        &mut (*parser).error,
        &mut (*parser).raw_buffer.start as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.pointer as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.last as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.end as *mut *mut yaml_char_t,
        INPUT_RAW_BUFFER_SIZE,
    ) == 0 { return cleanup_parser_init(parser); }

    if buffer_init(
        &mut (*parser).error,
        &mut (*parser).buffer.start,
        &mut (*parser).buffer.pointer,
        &mut (*parser).buffer.last,
        &mut (*parser).buffer.end,
        INPUT_BUFFER_SIZE,
    ) == 0 { return cleanup_parser_init(parser); }

    if queue_init(
        &mut (*parser).error,
        &mut (*parser).tokens.start,
        &mut (*parser).tokens.head,
        &mut (*parser).tokens.tail,
        &mut (*parser).tokens.end,
        INITIAL_QUEUE_SIZE,
    ) == 0 { return cleanup_parser_init(parser); }

    if stack_init(
        &mut (*parser).error,
        &mut (*parser).indents.start,
        &mut (*parser).indents.top,
        &mut (*parser).indents.end,
    ) == 0 { return cleanup_parser_init(parser); }

    if stack_init(
        &mut (*parser).error,
        &mut (*parser).simple_keys.start,
        &mut (*parser).simple_keys.top,
        &mut (*parser).simple_keys.end,
    ) == 0 { return cleanup_parser_init(parser); }

    if stack_init(
        &mut (*parser).error,
        &mut (*parser).states.start,
        &mut (*parser).states.top,
        &mut (*parser).states.end,
    ) == 0 { return cleanup_parser_init(parser); }

    if stack_init(
        &mut (*parser).error,
        &mut (*parser).marks.start,
        &mut (*parser).marks.top,
        &mut (*parser).marks.end,
    ) == 0 { return cleanup_parser_init(parser); }

    if stack_init(
        &mut (*parser).error,
        &mut (*parser).tag_directives.start,
        &mut (*parser).tag_directives.top,
        &mut (*parser).tag_directives.end,
    ) == 0 { return cleanup_parser_init(parser); }

    1
}

unsafe fn cleanup_parser_init(parser: *mut yaml_parser_t) -> i32 {
    buffer_del(
        &mut (*parser).raw_buffer.start as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.pointer as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.end as *mut *mut yaml_char_t,
    );
    buffer_del(
        &mut (*parser).buffer.start,
        &mut (*parser).buffer.pointer,
        &mut (*parser).buffer.end,
    );
    queue_del(
        &mut (*parser).tokens.start,
        &mut (*parser).tokens.head,
        &mut (*parser).tokens.tail,
        &mut (*parser).tokens.end,
    );
    stack_del(
        &mut (*parser).indents.start,
        &mut (*parser).indents.top,
        &mut (*parser).indents.end,
    );
    stack_del(
        &mut (*parser).simple_keys.start,
        &mut (*parser).simple_keys.top,
        &mut (*parser).simple_keys.end,
    );
    stack_del(
        &mut (*parser).states.start,
        &mut (*parser).states.top,
        &mut (*parser).states.end,
    );
    stack_del(
        &mut (*parser).marks.start,
        &mut (*parser).marks.top,
        &mut (*parser).marks.end,
    );
    stack_del(
        &mut (*parser).tag_directives.start,
        &mut (*parser).tag_directives.top,
        &mut (*parser).tag_directives.end,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_delete(parser: *mut yaml_parser_t) {
    debug_assert!(!parser.is_null());

    buffer_del(
        &mut (*parser).raw_buffer.start as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.pointer as *mut *mut yaml_char_t,
        &mut (*parser).raw_buffer.end as *mut *mut yaml_char_t,
    );
    buffer_del(
        &mut (*parser).buffer.start,
        &mut (*parser).buffer.pointer,
        &mut (*parser).buffer.end,
    );
    while !queue_empty((*parser).tokens.head, (*parser).tokens.tail) {
        let mut tok = queue_dequeue(&mut (*parser).tokens.head);
        yaml_token_delete(&mut tok);
    }
    queue_del(
        &mut (*parser).tokens.start,
        &mut (*parser).tokens.head,
        &mut (*parser).tokens.tail,
        &mut (*parser).tokens.end,
    );
    stack_del(
        &mut (*parser).indents.start,
        &mut (*parser).indents.top,
        &mut (*parser).indents.end,
    );
    stack_del(
        &mut (*parser).simple_keys.start,
        &mut (*parser).simple_keys.top,
        &mut (*parser).simple_keys.end,
    );
    stack_del(
        &mut (*parser).states.start,
        &mut (*parser).states.top,
        &mut (*parser).states.end,
    );
    stack_del(
        &mut (*parser).marks.start,
        &mut (*parser).marks.top,
        &mut (*parser).marks.end,
    );
    while !stack_empty((*parser).tag_directives.start, (*parser).tag_directives.top) {
        let td = stack_pop(&mut (*parser).tag_directives.top);
        yaml_free(td.handle as *mut c_void);
        yaml_free(td.prefix as *mut c_void);
    }
    stack_del(
        &mut (*parser).tag_directives.start,
        &mut (*parser).tag_directives.top,
        &mut (*parser).tag_directives.end,
    );

    libc::memset(parser as *mut c_void, 0, core::mem::size_of::<yaml_parser_t>());
}

// ============================================================
// Static read handlers
// ============================================================

unsafe extern "C" fn yaml_string_read_handler(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
    size_read: *mut usize,
) -> i32 {
    let parser = data as *mut yaml_parser_t;
    let input = &mut (*parser).input.string;
    if input.current == input.end {
        *size_read = 0;
        return 1;
    }
    let avail = input.end.offset_from(input.current) as usize;
    let to_read = if size > avail { avail } else { size };
    libc::memcpy(buffer as *mut c_void, input.current as *const c_void, to_read);
    input.current = input.current.add(to_read);
    *size_read = to_read;
    1
}

unsafe extern "C" fn yaml_file_read_handler(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
    size_read: *mut usize,
) -> i32 {
    let parser = data as *mut yaml_parser_t;
    let file = (*parser).input.file;
    *size_read = libc::fread(buffer as *mut c_void, 1, size, file);
    (libc::ferror(file) == 0) as i32
}

// ============================================================
// yaml_parser_set_input_string/file/input/encoding
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input_string(
    parser: *mut yaml_parser_t,
    input: *const u8,
    size: usize,
) {
    debug_assert!(!parser.is_null());
    debug_assert!((*parser).read_handler.is_none());
    debug_assert!(!input.is_null());

    (*parser).read_handler = Some(yaml_string_read_handler);
    (*parser).read_handler_data = parser as *mut c_void;

    (*parser).input.string.start = input;
    (*parser).input.string.current = input;
    (*parser).input.string.end = input.add(size);
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input_file(
    parser: *mut yaml_parser_t,
    file: *mut FILE,
) {
    debug_assert!(!parser.is_null());
    debug_assert!((*parser).read_handler.is_none());
    debug_assert!(!file.is_null());

    (*parser).read_handler = Some(yaml_file_read_handler);
    (*parser).read_handler_data = parser as *mut c_void;

    (*parser).input.file = file;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_input(
    parser: *mut yaml_parser_t,
    handler: Option<yaml_read_handler_t>,
    data: *mut c_void,
) {
    debug_assert!(!parser.is_null());
    debug_assert!((*parser).read_handler.is_none());
    debug_assert!(handler.is_some());

    (*parser).read_handler = handler;
    (*parser).read_handler_data = data;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_set_encoding(
    parser: *mut yaml_parser_t,
    encoding: yaml_encoding_t,
) {
    debug_assert!(!parser.is_null());
    debug_assert!((*parser).encoding == YAML_ANY_ENCODING);

    (*parser).encoding = encoding;
}

// ============================================================
// yaml_emitter_initialize / yaml_emitter_delete
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_initialize(emitter: *mut yaml_emitter_t) -> i32 {
    debug_assert!(!emitter.is_null());
    libc::memset(emitter as *mut c_void, 0, core::mem::size_of::<yaml_emitter_t>());

    if buffer_init(
        &mut (*emitter).error,
        &mut (*emitter).buffer.start,
        &mut (*emitter).buffer.pointer,
        &mut (*emitter).buffer.last,
        &mut (*emitter).buffer.end,
        OUTPUT_BUFFER_SIZE,
    ) == 0 { return cleanup_emitter_init(emitter); }

    if buffer_init(
        &mut (*emitter).error,
        &mut (*emitter).raw_buffer.start as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.pointer as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.last as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.end as *mut *mut yaml_char_t,
        OUTPUT_RAW_BUFFER_SIZE,
    ) == 0 { return cleanup_emitter_init(emitter); }

    if stack_init(
        &mut (*emitter).error,
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
    ) == 0 { return cleanup_emitter_init(emitter); }

    if queue_init(
        &mut (*emitter).error,
        &mut (*emitter).events.start,
        &mut (*emitter).events.head,
        &mut (*emitter).events.tail,
        &mut (*emitter).events.end,
        INITIAL_QUEUE_SIZE,
    ) == 0 { return cleanup_emitter_init(emitter); }

    if stack_init(
        &mut (*emitter).error,
        &mut (*emitter).indents.start,
        &mut (*emitter).indents.top,
        &mut (*emitter).indents.end,
    ) == 0 { return cleanup_emitter_init(emitter); }

    if stack_init(
        &mut (*emitter).error,
        &mut (*emitter).tag_directives.start,
        &mut (*emitter).tag_directives.top,
        &mut (*emitter).tag_directives.end,
    ) == 0 { return cleanup_emitter_init(emitter); }

    1
}

unsafe fn cleanup_emitter_init(emitter: *mut yaml_emitter_t) -> i32 {
    buffer_del(
        &mut (*emitter).buffer.start,
        &mut (*emitter).buffer.pointer,
        &mut (*emitter).buffer.end,
    );
    buffer_del(
        &mut (*emitter).raw_buffer.start as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.pointer as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.end as *mut *mut yaml_char_t,
    );
    stack_del(
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
    );
    queue_del(
        &mut (*emitter).events.start,
        &mut (*emitter).events.head,
        &mut (*emitter).events.tail,
        &mut (*emitter).events.end,
    );
    stack_del(
        &mut (*emitter).indents.start,
        &mut (*emitter).indents.top,
        &mut (*emitter).indents.end,
    );
    stack_del(
        &mut (*emitter).tag_directives.start,
        &mut (*emitter).tag_directives.top,
        &mut (*emitter).tag_directives.end,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_delete(emitter: *mut yaml_emitter_t) {
    debug_assert!(!emitter.is_null());

    buffer_del(
        &mut (*emitter).buffer.start,
        &mut (*emitter).buffer.pointer,
        &mut (*emitter).buffer.end,
    );
    buffer_del(
        &mut (*emitter).raw_buffer.start as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.pointer as *mut *mut yaml_char_t,
        &mut (*emitter).raw_buffer.end as *mut *mut yaml_char_t,
    );
    stack_del(
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
    );
    while !queue_empty((*emitter).events.head, (*emitter).events.tail) {
        let mut ev = queue_dequeue(&mut (*emitter).events.head);
        yaml_event_delete(&mut ev);
    }
    queue_del(
        &mut (*emitter).events.start,
        &mut (*emitter).events.head,
        &mut (*emitter).events.tail,
        &mut (*emitter).events.end,
    );
    stack_del(
        &mut (*emitter).indents.start,
        &mut (*emitter).indents.top,
        &mut (*emitter).indents.end,
    );
    // BUG REPRODUCTION: C code uses STACK_EMPTY(empty, emitter->tag_directives)
    // where `empty` is undefined. The stack check itself only uses the second
    // argument so the behavior is the same — we use the correct stack pointers.
    while !stack_empty((*emitter).tag_directives.start, (*emitter).tag_directives.top) {
        let td = stack_pop(&mut (*emitter).tag_directives.top);
        yaml_free(td.handle as *mut c_void);
        yaml_free(td.prefix as *mut c_void);
    }
    stack_del(
        &mut (*emitter).tag_directives.start,
        &mut (*emitter).tag_directives.top,
        &mut (*emitter).tag_directives.end,
    );
    yaml_free((*emitter).anchors as *mut c_void);

    libc::memset(emitter as *mut c_void, 0, core::mem::size_of::<yaml_emitter_t>());
}

// ============================================================
// Static write handlers
// ============================================================

unsafe extern "C" fn yaml_string_write_handler(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
) -> i32 {
    let emitter = data as *mut yaml_emitter_t;
    let output = &mut (*emitter).output.string;
    let avail = output.size - *output.size_written;
    if avail < size {
        libc::memcpy(
            output.buffer.add(*output.size_written) as *mut c_void,
            buffer as *const c_void,
            avail,
        );
        *output.size_written = output.size;
        return 0;
    }
    libc::memcpy(
        output.buffer.add(*output.size_written) as *mut c_void,
        buffer as *const c_void,
        size,
    );
    *output.size_written += size;
    1
}

unsafe extern "C" fn yaml_file_write_handler(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
) -> i32 {
    let emitter = data as *mut yaml_emitter_t;
    (libc::fwrite(buffer as *const c_void, 1, size, (*emitter).output.file) == size) as i32
}

// ============================================================
// yaml_emitter_set_output_string/file/output/encoding etc.
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output_string(
    emitter: *mut yaml_emitter_t,
    output: *mut u8,
    size: usize,
    size_written: *mut usize,
) {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).write_handler.is_none());
    debug_assert!(!output.is_null());

    (*emitter).write_handler = Some(yaml_string_write_handler);
    (*emitter).write_handler_data = emitter as *mut c_void;

    (*emitter).output.string.buffer = output;
    (*emitter).output.string.size = size;
    (*emitter).output.string.size_written = size_written;
    *size_written = 0;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output_file(
    emitter: *mut yaml_emitter_t,
    file: *mut FILE,
) {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).write_handler.is_none());
    debug_assert!(!file.is_null());

    (*emitter).write_handler = Some(yaml_file_write_handler);
    (*emitter).write_handler_data = emitter as *mut c_void;

    (*emitter).output.file = file;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_output(
    emitter: *mut yaml_emitter_t,
    handler: Option<yaml_write_handler_t>,
    data: *mut c_void,
) {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).write_handler.is_none());
    debug_assert!(handler.is_some());

    (*emitter).write_handler = handler;
    (*emitter).write_handler_data = data;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_encoding(
    emitter: *mut yaml_emitter_t,
    encoding: yaml_encoding_t,
) {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).encoding == YAML_ANY_ENCODING);
    (*emitter).encoding = encoding;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_canonical(
    emitter: *mut yaml_emitter_t,
    canonical: i32,
) {
    debug_assert!(!emitter.is_null());
    (*emitter).canonical = (canonical != 0) as i32;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_indent(
    emitter: *mut yaml_emitter_t,
    indent: i32,
) {
    debug_assert!(!emitter.is_null());
    (*emitter).best_indent = if indent > 1 && indent < 10 { indent } else { 2 };
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_width(
    emitter: *mut yaml_emitter_t,
    width: i32,
) {
    debug_assert!(!emitter.is_null());
    (*emitter).best_width = if width >= 0 { width } else { -1 };
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_unicode(
    emitter: *mut yaml_emitter_t,
    unicode: i32,
) {
    debug_assert!(!emitter.is_null());
    (*emitter).unicode = (unicode != 0) as i32;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_set_break(
    emitter: *mut yaml_emitter_t,
    line_break: yaml_break_t,
) {
    debug_assert!(!emitter.is_null());
    (*emitter).line_break = line_break;
}

// ============================================================
// yaml_token_delete
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_token_delete(token: *mut yaml_token_t) {
    debug_assert!(!token.is_null());

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

// ============================================================
// Static yaml_check_utf8
// ============================================================

unsafe fn yaml_check_utf8(start: *const yaml_char_t, length: usize) -> i32 {
    let end = start.add(length);
    let mut pointer = start;
    while pointer < end {
        let octet = *pointer;
        let width: usize = if (octet & 0x80) == 0x00 { 1 }
            else if (octet & 0xE0) == 0xC0 { 2 }
            else if (octet & 0xF0) == 0xE0 { 3 }
            else if (octet & 0xF8) == 0xF0 { 4 }
            else { 0 };
        let mut value: u32 = if (octet & 0x80) == 0x00 { (octet & 0x7F) as u32 }
            else if (octet & 0xE0) == 0xC0 { (octet & 0x1F) as u32 }
            else if (octet & 0xF0) == 0xE0 { (octet & 0x0F) as u32 }
            else if (octet & 0xF8) == 0xF0 { (octet & 0x07) as u32 }
            else { 0 };
        if width == 0 { return 0; }
        if pointer.add(width) > end { return 0; }
        for k in 1..width {
            let o = *pointer.add(k);
            if (o & 0xC0) != 0x80 { return 0; }
            value = (value << 6) + (o & 0x3F) as u32;
        }
        if !((width == 1)
            || (width == 2 && value >= 0x80)
            || (width == 3 && value >= 0x800)
            || (width == 4 && value >= 0x10000)) {
            return 0;
        }
        pointer = pointer.add(width);
    }
    1
}

// ============================================================
// Event init functions
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_stream_start_event_initialize(
    event: *mut yaml_event_t,
    encoding: yaml_encoding_t,
) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_STREAM_START_EVENT;
    (*event).data.stream_start.encoding = encoding;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_stream_end_event_initialize(event: *mut yaml_event_t) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_STREAM_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_start_event_initialize(
    event: *mut yaml_event_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    implicit: i32,
) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut error = YAML_NO_ERROR;

    let version_directive_copy: *mut yaml_version_directive_t = if !version_directive.is_null() {
        let copy = yaml_malloc(core::mem::size_of::<yaml_version_directive_t>())
            as *mut yaml_version_directive_t;
        if copy.is_null() { return 0; }
        (*copy).major = (*version_directive).major;
        (*copy).minor = (*version_directive).minor;
        copy
    } else {
        ptr::null_mut()
    };

    let mut td_start: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut td_top: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut td_end_alloc: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut value = yaml_tag_directive_t { handle: ptr::null_mut(), prefix: ptr::null_mut() };

    if tag_directives_start != tag_directives_end {
        if stack_init(&mut error, &mut td_start, &mut td_top, &mut td_end_alloc) == 0 {
            yaml_free(version_directive_copy as *mut c_void);
            return 0;
        }
        let mut td = tag_directives_start;
        while td != tag_directives_end {
            debug_assert!(!(*td).handle.is_null());
            debug_assert!(!(*td).prefix.is_null());
            if yaml_check_utf8((*td).handle, libc::strlen((*td).handle as *const c_char)) == 0
                || yaml_check_utf8((*td).prefix, libc::strlen((*td).prefix as *const c_char)) == 0
            {
                // cleanup
                yaml_free(value.handle as *mut c_void);
                yaml_free(value.prefix as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end_alloc);
                yaml_free(version_directive_copy as *mut c_void);
                return 0;
            }
            value.handle = yaml_strdup((*td).handle);
            value.prefix = yaml_strdup((*td).prefix);
            if value.handle.is_null() || value.prefix.is_null() {
                yaml_free(value.handle as *mut c_void);
                yaml_free(value.prefix as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end_alloc);
                yaml_free(version_directive_copy as *mut c_void);
                return 0;
            }
            if stack_push(&mut error, &mut td_start, &mut td_top, &mut td_end_alloc, value) == 0 {
                yaml_free(value.handle as *mut c_void);
                yaml_free(value.prefix as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end_alloc);
                yaml_free(version_directive_copy as *mut c_void);
                return 0;
            }
            value.handle = ptr::null_mut();
            value.prefix = ptr::null_mut();
            td = td.add(1);
        }
    }

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_DOCUMENT_START_EVENT;
    (*event).data.document_start.version_directive = version_directive_copy;
    (*event).data.document_start.tag_directives.start = td_start;
    (*event).data.document_start.tag_directives.end = td_top;
    (*event).data.document_start.implicit = implicit;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_end_event_initialize(
    event: *mut yaml_event_t,
    implicit: i32,
) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_DOCUMENT_END_EVENT;
    (*event).data.document_end.implicit = implicit;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_alias_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
) -> i32 {
    debug_assert!(!event.is_null());
    debug_assert!(!anchor.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 {
        return 0;
    }

    let anchor_copy = yaml_strdup(anchor);
    if anchor_copy.is_null() { return 0; }

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_ALIAS_EVENT;
    (*event).data.alias.anchor = anchor_copy;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_scalar_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    value: *const yaml_char_t,
    length: i32,
    plain_implicit: i32,
    quoted_implicit: i32,
    style: yaml_scalar_style_t,
) -> i32 {
    debug_assert!(!event.is_null());
    debug_assert!(!value.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    let anchor_copy: *mut yaml_char_t = if !anchor.is_null() {
        if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 {
            return 0;
        }
        let c = yaml_strdup(anchor);
        if c.is_null() { return 0; }
        c
    } else {
        ptr::null_mut()
    };

    let tag_copy: *mut yaml_char_t = if !tag.is_null() {
        if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
            yaml_free(anchor_copy as *mut c_void);
            return 0;
        }
        let c = yaml_strdup(tag);
        if c.is_null() {
            yaml_free(anchor_copy as *mut c_void);
            return 0;
        }
        c
    } else {
        ptr::null_mut()
    };

    let actual_length: usize = if length < 0 {
        libc::strlen(value as *const c_char)
    } else {
        length as usize
    };

    if yaml_check_utf8(value, actual_length) == 0 {
        yaml_free(anchor_copy as *mut c_void);
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }
    let value_copy = yaml_malloc(actual_length + 1) as *mut yaml_char_t;
    if value_copy.is_null() {
        yaml_free(anchor_copy as *mut c_void);
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }
    libc::memcpy(value_copy as *mut c_void, value as *const c_void, actual_length);
    *value_copy.add(actual_length) = 0;

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_SCALAR_EVENT;
    (*event).data.scalar.anchor = anchor_copy;
    (*event).data.scalar.tag = tag_copy;
    (*event).data.scalar.value = value_copy;
    (*event).data.scalar.length = actual_length;
    (*event).data.scalar.plain_implicit = plain_implicit;
    (*event).data.scalar.quoted_implicit = quoted_implicit;
    (*event).data.scalar.style = style;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_sequence_start_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: i32,
    style: yaml_sequence_style_t,
) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    let anchor_copy: *mut yaml_char_t = if !anchor.is_null() {
        if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 { return 0; }
        let c = yaml_strdup(anchor);
        if c.is_null() { return 0; }
        c
    } else { ptr::null_mut() };

    let tag_copy: *mut yaml_char_t = if !tag.is_null() {
        if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
            yaml_free(anchor_copy as *mut c_void); return 0;
        }
        let c = yaml_strdup(tag);
        if c.is_null() { yaml_free(anchor_copy as *mut c_void); return 0; }
        c
    } else { ptr::null_mut() };

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_SEQUENCE_START_EVENT;
    (*event).data.sequence_start.anchor = anchor_copy;
    (*event).data.sequence_start.tag = tag_copy;
    (*event).data.sequence_start.implicit = implicit;
    (*event).data.sequence_start.style = style;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_sequence_end_event_initialize(event: *mut yaml_event_t) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_SEQUENCE_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_mapping_start_event_initialize(
    event: *mut yaml_event_t,
    anchor: *const yaml_char_t,
    tag: *const yaml_char_t,
    implicit: i32,
    style: yaml_mapping_style_t,
) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    let anchor_copy: *mut yaml_char_t = if !anchor.is_null() {
        if yaml_check_utf8(anchor, libc::strlen(anchor as *const c_char)) == 0 { return 0; }
        let c = yaml_strdup(anchor);
        if c.is_null() { return 0; }
        c
    } else { ptr::null_mut() };

    let tag_copy: *mut yaml_char_t = if !tag.is_null() {
        if yaml_check_utf8(tag, libc::strlen(tag as *const c_char)) == 0 {
            yaml_free(anchor_copy as *mut c_void); return 0;
        }
        let c = yaml_strdup(tag);
        if c.is_null() { yaml_free(anchor_copy as *mut c_void); return 0; }
        c
    } else { ptr::null_mut() };

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_MAPPING_START_EVENT;
    (*event).data.mapping_start.anchor = anchor_copy;
    (*event).data.mapping_start.tag = tag_copy;
    (*event).data.mapping_start.implicit = implicit;
    (*event).data.mapping_start.style = style;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_mapping_end_event_initialize(event: *mut yaml_event_t) -> i32 {
    debug_assert!(!event.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = YAML_MAPPING_END_EVENT;
    (*event).start_mark = mark;
    (*event).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_event_delete(event: *mut yaml_event_t) {
    debug_assert!(!event.is_null());

    match (*event).type_ {
        YAML_DOCUMENT_START_EVENT => {
            yaml_free((*event).data.document_start.version_directive as *mut c_void);
            let mut td = (*event).data.document_start.tag_directives.start;
            while td != (*event).data.document_start.tag_directives.end {
                yaml_free((*td).handle as *mut c_void);
                yaml_free((*td).prefix as *mut c_void);
                td = td.add(1);
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

// ============================================================
// yaml_document_initialize / yaml_document_delete
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_document_initialize(
    document: *mut yaml_document_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    start_implicit: i32,
    end_implicit: i32,
) -> i32 {
    debug_assert!(!document.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut error = YAML_NO_ERROR;

    let mut nodes_start: *mut yaml_node_t = ptr::null_mut();
    let mut nodes_top: *mut yaml_node_t = ptr::null_mut();
    let mut nodes_end_alloc: *mut yaml_node_t = ptr::null_mut();

    if stack_init(&mut error, &mut nodes_start, &mut nodes_top, &mut nodes_end_alloc) == 0 {
        return 0;
    }

    let version_directive_copy: *mut yaml_version_directive_t = if !version_directive.is_null() {
        let copy = yaml_malloc(core::mem::size_of::<yaml_version_directive_t>())
            as *mut yaml_version_directive_t;
        if copy.is_null() {
            stack_del(&mut nodes_start, &mut nodes_top, &mut nodes_end_alloc);
            return 0;
        }
        (*copy).major = (*version_directive).major;
        (*copy).minor = (*version_directive).minor;
        copy
    } else {
        ptr::null_mut()
    };

    let mut td_start: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut td_top: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut td_end_alloc: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut value = yaml_tag_directive_t { handle: ptr::null_mut(), prefix: ptr::null_mut() };

    if tag_directives_start != tag_directives_end {
        if stack_init(&mut error, &mut td_start, &mut td_top, &mut td_end_alloc) == 0 {
            stack_del(&mut nodes_start, &mut nodes_top, &mut nodes_end_alloc);
            yaml_free(version_directive_copy as *mut c_void);
            return 0;
        }
        let mut td = tag_directives_start;
        while td != tag_directives_end {
            debug_assert!(!(*td).handle.is_null());
            debug_assert!(!(*td).prefix.is_null());
            if yaml_check_utf8((*td).handle, libc::strlen((*td).handle as *const c_char)) == 0
                || yaml_check_utf8((*td).prefix, libc::strlen((*td).prefix as *const c_char)) == 0
            {
                yaml_free(value.handle as *mut c_void);
                yaml_free(value.prefix as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end_alloc);
                stack_del(&mut nodes_start, &mut nodes_top, &mut nodes_end_alloc);
                yaml_free(version_directive_copy as *mut c_void);
                return 0;
            }
            value.handle = yaml_strdup((*td).handle);
            value.prefix = yaml_strdup((*td).prefix);
            if value.handle.is_null() || value.prefix.is_null() {
                yaml_free(value.handle as *mut c_void);
                yaml_free(value.prefix as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end_alloc);
                stack_del(&mut nodes_start, &mut nodes_top, &mut nodes_end_alloc);
                yaml_free(version_directive_copy as *mut c_void);
                return 0;
            }
            if stack_push(&mut error, &mut td_start, &mut td_top, &mut td_end_alloc, value) == 0 {
                yaml_free(value.handle as *mut c_void);
                yaml_free(value.prefix as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end_alloc);
                stack_del(&mut nodes_start, &mut nodes_top, &mut nodes_end_alloc);
                yaml_free(version_directive_copy as *mut c_void);
                return 0;
            }
            value.handle = ptr::null_mut();
            value.prefix = ptr::null_mut();
            td = td.add(1);
        }
    }

    libc::memset(document as *mut c_void, 0, core::mem::size_of::<yaml_document_t>());
    (*document).nodes.start = nodes_start;
    (*document).nodes.end = nodes_end_alloc;
    (*document).nodes.top = nodes_top;
    (*document).version_directive = version_directive_copy;
    (*document).tag_directives.start = td_start;
    (*document).tag_directives.end = td_top;
    (*document).start_implicit = start_implicit;
    (*document).end_implicit = end_implicit;
    (*document).start_mark = mark;
    (*document).end_mark = mark;
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_delete(document: *mut yaml_document_t) {
    debug_assert!(!document.is_null());

    // BUG REPRODUCTION: C code uses STACK_EMPTY(&context, ...) where &context
    // is undeclared. The macro only uses the stack pointers so behavior is the same.
    while !stack_empty((*document).nodes.start, (*document).nodes.top) {
        let node = stack_pop(&mut (*document).nodes.top);
        yaml_free(node.tag as *mut c_void);
        match node.type_ {
            YAML_SCALAR_NODE => {
                yaml_free(node.data.scalar.value as *mut c_void);
            }
            YAML_SEQUENCE_NODE => {
                yaml_free(node.data.sequence.items.start as *mut c_void);
            }
            YAML_MAPPING_NODE => {
                yaml_free(node.data.mapping.pairs.start as *mut c_void);
            }
            _ => {
                debug_assert!(false);
            }
        }
    }
    yaml_free((*document).nodes.start as *mut c_void);

    yaml_free((*document).version_directive as *mut c_void);
    let mut td = (*document).tag_directives.start;
    while td != (*document).tag_directives.end {
        yaml_free((*td).handle as *mut c_void);
        yaml_free((*td).prefix as *mut c_void);
        td = td.add(1);
    }
    yaml_free((*document).tag_directives.start as *mut c_void);

    libc::memset(document as *mut c_void, 0, core::mem::size_of::<yaml_document_t>());
}

// ============================================================
// Document node accessors
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_document_get_node(
    document: *mut yaml_document_t,
    index: i32,
) -> *mut yaml_node_t {
    debug_assert!(!document.is_null());
    if index > 0
        && (*document).nodes.start.add(index as usize) <= (*document).nodes.top
    {
        return (*document).nodes.start.add(index as usize - 1);
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_get_root_node(
    document: *mut yaml_document_t,
) -> *mut yaml_node_t {
    debug_assert!(!document.is_null());
    if (*document).nodes.top != (*document).nodes.start {
        return (*document).nodes.start;
    }
    ptr::null_mut()
}

// ============================================================
// Document add node functions
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_scalar(
    document: *mut yaml_document_t,
    tag: *const yaml_char_t,
    value: *const yaml_char_t,
    length: i32,
    style: yaml_scalar_style_t,
) -> i32 {
    debug_assert!(!document.is_null());
    debug_assert!(!value.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut error = YAML_NO_ERROR;

    let effective_tag = if tag.is_null() {
        YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const yaml_char_t
    } else {
        tag
    };

    if yaml_check_utf8(effective_tag, libc::strlen(effective_tag as *const c_char)) == 0 {
        return 0;
    }
    let tag_copy = yaml_strdup(effective_tag);
    if tag_copy.is_null() { return 0; }

    let actual_length: usize = if length < 0 {
        libc::strlen(value as *const c_char)
    } else {
        length as usize
    };

    if yaml_check_utf8(value, actual_length) == 0 {
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }
    let value_copy = yaml_malloc(actual_length + 1) as *mut yaml_char_t;
    if value_copy.is_null() {
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }
    libc::memcpy(value_copy as *mut c_void, value as *const c_void, actual_length);
    *value_copy.add(actual_length) = 0;

    let mut node: yaml_node_t = core::mem::zeroed();
    node.type_ = YAML_SCALAR_NODE;
    node.tag = tag_copy;
    node.data.scalar.value = value_copy;
    node.data.scalar.length = actual_length;
    node.data.scalar.style = style;
    node.start_mark = mark;
    node.end_mark = mark;

    if stack_push(
        &mut error,
        &mut (*document).nodes.start,
        &mut (*document).nodes.top,
        &mut (*document).nodes.end,
        node,
    ) == 0 {
        yaml_free(tag_copy as *mut c_void);
        yaml_free(value_copy as *mut c_void);
        return 0;
    }

    ((*document).nodes.top.offset_from((*document).nodes.start)) as i32
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_sequence(
    document: *mut yaml_document_t,
    tag: *const yaml_char_t,
    style: yaml_sequence_style_t,
) -> i32 {
    debug_assert!(!document.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut error = YAML_NO_ERROR;

    let effective_tag = if tag.is_null() {
        YAML_DEFAULT_SEQUENCE_TAG.as_ptr() as *const yaml_char_t
    } else {
        tag
    };

    if yaml_check_utf8(effective_tag, libc::strlen(effective_tag as *const c_char)) == 0 {
        return 0;
    }
    let tag_copy = yaml_strdup(effective_tag);
    if tag_copy.is_null() { return 0; }

    let mut items_start: *mut yaml_node_item_t = ptr::null_mut();
    let mut items_top: *mut yaml_node_item_t = ptr::null_mut();
    let mut items_end: *mut yaml_node_item_t = ptr::null_mut();
    if stack_init(&mut error, &mut items_start, &mut items_top, &mut items_end) == 0 {
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }

    let mut node: yaml_node_t = core::mem::zeroed();
    node.type_ = YAML_SEQUENCE_NODE;
    node.tag = tag_copy;
    node.data.sequence.items.start = items_start;
    node.data.sequence.items.end = items_end;
    node.data.sequence.items.top = items_top;
    node.data.sequence.style = style;
    node.start_mark = mark;
    node.end_mark = mark;

    if stack_push(
        &mut error,
        &mut (*document).nodes.start,
        &mut (*document).nodes.top,
        &mut (*document).nodes.end,
        node,
    ) == 0 {
        stack_del(&mut items_start, &mut items_top, &mut items_end);
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }

    ((*document).nodes.top.offset_from((*document).nodes.start)) as i32
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_add_mapping(
    document: *mut yaml_document_t,
    tag: *const yaml_char_t,
    style: yaml_mapping_style_t,
) -> i32 {
    debug_assert!(!document.is_null());
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut error = YAML_NO_ERROR;

    let effective_tag = if tag.is_null() {
        YAML_DEFAULT_MAPPING_TAG.as_ptr() as *const yaml_char_t
    } else {
        tag
    };

    if yaml_check_utf8(effective_tag, libc::strlen(effective_tag as *const c_char)) == 0 {
        return 0;
    }
    let tag_copy = yaml_strdup(effective_tag);
    if tag_copy.is_null() { return 0; }

    let mut pairs_start: *mut yaml_node_pair_t = ptr::null_mut();
    let mut pairs_top: *mut yaml_node_pair_t = ptr::null_mut();
    let mut pairs_end: *mut yaml_node_pair_t = ptr::null_mut();
    if stack_init(&mut error, &mut pairs_start, &mut pairs_top, &mut pairs_end) == 0 {
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }

    let mut node: yaml_node_t = core::mem::zeroed();
    node.type_ = YAML_MAPPING_NODE;
    node.tag = tag_copy;
    node.data.mapping.pairs.start = pairs_start;
    node.data.mapping.pairs.end = pairs_end;
    node.data.mapping.pairs.top = pairs_top;
    node.data.mapping.style = style;
    node.start_mark = mark;
    node.end_mark = mark;

    if stack_push(
        &mut error,
        &mut (*document).nodes.start,
        &mut (*document).nodes.top,
        &mut (*document).nodes.end,
        node,
    ) == 0 {
        stack_del(&mut pairs_start, &mut pairs_top, &mut pairs_end);
        yaml_free(tag_copy as *mut c_void);
        return 0;
    }

    ((*document).nodes.top.offset_from((*document).nodes.start)) as i32
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_append_sequence_item(
    document: *mut yaml_document_t,
    sequence: i32,
    item: i32,
) -> i32 {
    debug_assert!(!document.is_null());
    debug_assert!(sequence > 0
        && (*document).nodes.start.add(sequence as usize) <= (*document).nodes.top);
    debug_assert!((*(*document).nodes.start.add(sequence as usize - 1)).type_ == YAML_SEQUENCE_NODE);
    debug_assert!(item > 0
        && (*document).nodes.start.add(item as usize) <= (*document).nodes.top);

    let mut error = YAML_NO_ERROR;
    let node = (*document).nodes.start.add(sequence as usize - 1);
    if stack_push(
        &mut error,
        &mut (*node).data.sequence.items.start,
        &mut (*node).data.sequence.items.top,
        &mut (*node).data.sequence.items.end,
        item,
    ) == 0 {
        return 0;
    }
    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_document_append_mapping_pair(
    document: *mut yaml_document_t,
    mapping: i32,
    key: i32,
    value: i32,
) -> i32 {
    debug_assert!(!document.is_null());
    debug_assert!(mapping > 0
        && (*document).nodes.start.add(mapping as usize) <= (*document).nodes.top);
    debug_assert!((*(*document).nodes.start.add(mapping as usize - 1)).type_ == YAML_MAPPING_NODE);
    debug_assert!(key > 0 && (*document).nodes.start.add(key as usize) <= (*document).nodes.top);
    debug_assert!(value > 0 && (*document).nodes.start.add(value as usize) <= (*document).nodes.top);

    let mut error = YAML_NO_ERROR;
    let node = (*document).nodes.start.add(mapping as usize - 1);
    let pair = yaml_node_pair_t { key, value };
    if stack_push(
        &mut error,
        &mut (*node).data.mapping.pairs.start,
        &mut (*node).data.mapping.pairs.top,
        &mut (*node).data.mapping.pairs.end,
        pair,
    ) == 0 {
        return 0;
    }
    1
}


// ============================================================
// reader.c functions
// ============================================================

unsafe fn yaml_parser_set_reader_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    offset: usize,
    value: i32,
) -> i32 {
    (*parser).error = YAML_READER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_offset = offset;
    (*parser).problem_value = value;
    0
}

unsafe fn yaml_parser_update_raw_buffer(parser: *mut yaml_parser_t) -> i32 {
    let mut size_read: usize = 0;

    // Return if the raw buffer is full (start==pointer and last==end)
    if (*parser).raw_buffer.start == (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.last == (*parser).raw_buffer.end
    {
        return 1;
    }

    // Return on EOF
    if (*parser).eof != 0 {
        return 1;
    }

    // Move remaining bytes to beginning
    if (*parser).raw_buffer.start < (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.pointer < (*parser).raw_buffer.last
    {
        let size = (*parser).raw_buffer.last.offset_from((*parser).raw_buffer.pointer) as usize;
        libc::memmove(
            (*parser).raw_buffer.start as *mut c_void,
            (*parser).raw_buffer.pointer as *const c_void,
            size,
        );
    }
    (*parser).raw_buffer.last = (*parser).raw_buffer.last.sub(
        (*parser).raw_buffer.pointer.offset_from((*parser).raw_buffer.start) as usize,
    );
    (*parser).raw_buffer.pointer = (*parser).raw_buffer.start;

    // Call read handler
    let avail = (*parser).raw_buffer.end.offset_from((*parser).raw_buffer.last) as usize;
    let handler = (*parser).read_handler.unwrap();
    if handler(
        (*parser).read_handler_data,
        (*parser).raw_buffer.last,
        avail,
        &mut size_read,
    ) == 0 {
        return yaml_parser_set_reader_error(
            parser,
            b"input error\0".as_ptr() as *const c_char,
            (*parser).offset,
            -1,
        );
    }
    (*parser).raw_buffer.last = (*parser).raw_buffer.last.add(size_read);
    if size_read == 0 {
        (*parser).eof = 1;
    }
    1
}

unsafe fn yaml_parser_determine_encoding(parser: *mut yaml_parser_t) -> i32 {
    // Ensure we have at least 3 bytes
    while (*parser).eof == 0
        && ((*parser).raw_buffer.last.offset_from((*parser).raw_buffer.pointer) as usize) < 3
    {
        if yaml_parser_update_raw_buffer(parser) == 0 {
            return 0;
        }
    }

    let raw_unread = (*parser).raw_buffer.last.offset_from((*parser).raw_buffer.pointer) as usize;

    if raw_unread >= 2
        && *(*parser).raw_buffer.pointer == 0xFF
        && *(*parser).raw_buffer.pointer.add(1) == 0xFE
    {
        (*parser).encoding = YAML_UTF16LE_ENCODING;
        (*parser).raw_buffer.pointer = (*parser).raw_buffer.pointer.add(2);
        (*parser).offset += 2;
    } else if raw_unread >= 2
        && *(*parser).raw_buffer.pointer == 0xFE
        && *(*parser).raw_buffer.pointer.add(1) == 0xFF
    {
        (*parser).encoding = YAML_UTF16BE_ENCODING;
        (*parser).raw_buffer.pointer = (*parser).raw_buffer.pointer.add(2);
        (*parser).offset += 2;
    } else if raw_unread >= 3
        && *(*parser).raw_buffer.pointer == 0xEF
        && *(*parser).raw_buffer.pointer.add(1) == 0xBB
        && *(*parser).raw_buffer.pointer.add(2) == 0xBF
    {
        (*parser).encoding = YAML_UTF8_ENCODING;
        (*parser).raw_buffer.pointer = (*parser).raw_buffer.pointer.add(3);
        (*parser).offset += 3;
    } else {
        (*parser).encoding = YAML_UTF8_ENCODING;
    }

    1
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_update_buffer(
    parser: *mut yaml_parser_t,
    length: usize,
) -> i32 {
    let mut first: i32 = 1;

    debug_assert!((*parser).read_handler.is_some());

    // If EOF and raw buffer empty, nothing to do
    if (*parser).eof != 0
        && (*parser).raw_buffer.pointer == (*parser).raw_buffer.last
    {
        return 1;
    }

    // Return if buffer already has enough characters
    if (*parser).unread >= length {
        return 1;
    }

    // Determine encoding if not yet known
    if (*parser).encoding == YAML_ANY_ENCODING {
        if yaml_parser_determine_encoding(parser) == 0 {
            return 0;
        }
    }

    // Move unread characters to beginning of buffer
    if (*parser).buffer.start < (*parser).buffer.pointer
        && (*parser).buffer.pointer < (*parser).buffer.last
    {
        let size = (*parser).buffer.last.offset_from((*parser).buffer.pointer) as usize;
        libc::memmove(
            (*parser).buffer.start as *mut c_void,
            (*parser).buffer.pointer as *const c_void,
            size,
        );
        (*parser).buffer.pointer = (*parser).buffer.start;
        (*parser).buffer.last = (*parser).buffer.start.add(size);
    } else if (*parser).buffer.pointer == (*parser).buffer.last {
        (*parser).buffer.pointer = (*parser).buffer.start;
        (*parser).buffer.last = (*parser).buffer.start;
    }

    // Fill buffer until it has enough characters
    while (*parser).unread < length {
        // Fill raw buffer if necessary
        if first == 0 || (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
            if yaml_parser_update_raw_buffer(parser) == 0 {
                return 0;
            }
        }
        first = 0;

        // Decode raw buffer
        while (*parser).raw_buffer.pointer != (*parser).raw_buffer.last {
            let mut value: u32 = 0;
            let mut value2: u32 = 0;
            let mut incomplete: i32 = 0;
            let mut width: usize = 0;
            let raw_unread = (*parser).raw_buffer.last
                .offset_from((*parser).raw_buffer.pointer) as usize;

            match (*parser).encoding {
                YAML_UTF8_ENCODING => {
                    let octet = *(*parser).raw_buffer.pointer;
                    width = if (octet & 0x80) == 0x00 { 1 }
                        else if (octet & 0xE0) == 0xC0 { 2 }
                        else if (octet & 0xF0) == 0xE0 { 3 }
                        else if (octet & 0xF8) == 0xF0 { 4 }
                        else { 0 };

                    if width == 0 {
                        return yaml_parser_set_reader_error(
                            parser,
                            b"invalid leading UTF-8 octet\0".as_ptr() as *const c_char,
                            (*parser).offset,
                            octet as i32,
                        );
                    }

                    if width > raw_unread {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-8 octet sequence\0".as_ptr() as *const c_char,
                                (*parser).offset,
                                -1,
                            );
                        }
                        incomplete = 1;
                    } else {
                        value = if (octet & 0x80) == 0x00 { (octet & 0x7F) as u32 }
                            else if (octet & 0xE0) == 0xC0 { (octet & 0x1F) as u32 }
                            else if (octet & 0xF0) == 0xE0 { (octet & 0x0F) as u32 }
                            else { (octet & 0x07) as u32 };

                        for k in 1..width {
                            let o = *(*parser).raw_buffer.pointer.add(k);
                            if (o & 0xC0) != 0x80 {
                                return yaml_parser_set_reader_error(
                                    parser,
                                    b"invalid trailing UTF-8 octet\0".as_ptr() as *const c_char,
                                    (*parser).offset + k,
                                    o as i32,
                                );
                            }
                            value = (value << 6) + (o & 0x3F) as u32;
                        }

                        if !((width == 1)
                            || (width == 2 && value >= 0x80)
                            || (width == 3 && value >= 0x800)
                            || (width == 4 && value >= 0x10000))
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid length of a UTF-8 sequence\0".as_ptr() as *const c_char,
                                (*parser).offset,
                                -1,
                            );
                        }

                        if (value >= 0xD800 && value <= 0xDFFF) || value > 0x10FFFF {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid Unicode character\0".as_ptr() as *const c_char,
                                (*parser).offset,
                                value as i32,
                            );
                        }
                    }
                }
                YAML_UTF16LE_ENCODING | YAML_UTF16BE_ENCODING => {
                    let low: usize = if (*parser).encoding == YAML_UTF16LE_ENCODING { 0 } else { 1 };
                    let high: usize = if (*parser).encoding == YAML_UTF16LE_ENCODING { 1 } else { 0 };

                    if raw_unread < 2 {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-16 character\0".as_ptr() as *const c_char,
                                (*parser).offset,
                                -1,
                            );
                        }
                        incomplete = 1;
                    } else {
                        value = (*(*parser).raw_buffer.pointer.add(low)) as u32
                            + ((*(*parser).raw_buffer.pointer.add(high)) as u32) * 256;

                        if (value & 0xFC00) == 0xDC00 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"unexpected low surrogate area\0".as_ptr() as *const c_char,
                                (*parser).offset,
                                value as i32,
                            );
                        }

                        if (value & 0xFC00) == 0xD800 {
                            width = 4;
                            if raw_unread < 4 {
                                if (*parser).eof != 0 {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"incomplete UTF-16 surrogate pair\0".as_ptr() as *const c_char,
                                        (*parser).offset,
                                        -1,
                                    );
                                }
                                incomplete = 1;
                            } else {
                                value2 = (*(*parser).raw_buffer.pointer.add(low + 2)) as u32
                                    + ((*(*parser).raw_buffer.pointer.add(high + 2)) as u32) * 256;
                                if (value2 & 0xFC00) != 0xDC00 {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"expected low surrogate area\0".as_ptr() as *const c_char,
                                        (*parser).offset + 2,
                                        value2 as i32,
                                    );
                                }
                                value = 0x10000 + ((value & 0x3FF) << 10) + (value2 & 0x3FF);
                            }
                        } else {
                            width = 2;
                        }
                    }
                }
                _ => {
                    // impossible
                }
            }

            if incomplete != 0 {
                break;
            }

            // Check allowed character range
            if !(value == 0x09
                || value == 0x0A
                || value == 0x0D
                || (value >= 0x20 && value <= 0x7E)
                || value == 0x85
                || (value >= 0xA0 && value <= 0xD7FF)
                || (value >= 0xE000 && value <= 0xFFFD)
                || (value >= 0x10000 && value <= 0x10FFFF))
            {
                return yaml_parser_set_reader_error(
                    parser,
                    b"control characters are not allowed\0".as_ptr() as *const c_char,
                    (*parser).offset,
                    value as i32,
                );
            }

            // Advance raw pointer
            (*parser).raw_buffer.pointer = (*parser).raw_buffer.pointer.add(width);
            (*parser).offset += width;

            // Encode into buffer as UTF-8
            if value <= 0x7F {
                *(*parser).buffer.last = value as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
            } else if value <= 0x7FF {
                *(*parser).buffer.last = (0xC0 + (value >> 6)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
                *(*parser).buffer.last = (0x80 + (value & 0x3F)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
            } else if value <= 0xFFFF {
                *(*parser).buffer.last = (0xE0 + (value >> 12)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
                *(*parser).buffer.last = (0x80 + ((value >> 6) & 0x3F)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
                *(*parser).buffer.last = (0x80 + (value & 0x3F)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
            } else {
                *(*parser).buffer.last = (0xF0 + (value >> 18)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
                *(*parser).buffer.last = (0x80 + ((value >> 12) & 0x3F)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
                *(*parser).buffer.last = (0x80 + ((value >> 6) & 0x3F)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
                *(*parser).buffer.last = (0x80 + (value & 0x3F)) as u8;
                (*parser).buffer.last = (*parser).buffer.last.add(1);
            }

            (*parser).unread += 1;
        }

        // On EOF, put NUL
        if (*parser).eof != 0 {
            *(*parser).buffer.last = b'\0';
            (*parser).buffer.last = (*parser).buffer.last.add(1);
            (*parser).unread += 1;
            return 1;
        }
    }

    if (*parser).offset >= MAX_FILE_SIZE {
        return yaml_parser_set_reader_error(
            parser,
            b"input is too long\0".as_ptr() as *const c_char,
            (*parser).offset,
            -1,
        );
    }

    1
}


// ============================================================
// writer.c functions
// ============================================================

unsafe fn yaml_emitter_set_writer_error(
    emitter: *mut yaml_emitter_t,
    problem: *const c_char,
) -> i32 {
    (*emitter).error = YAML_WRITER_ERROR;
    (*emitter).problem = problem;
    0
}

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> i32 {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).write_handler.is_some());
    debug_assert!((*emitter).encoding != YAML_ANY_ENCODING);

    (*emitter).buffer.last = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.start;

    // Check if buffer is empty
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1;
    }

    // UTF-8: write directly
    if (*emitter).encoding == YAML_UTF8_ENCODING {
        let size = (*emitter).buffer.last.offset_from((*emitter).buffer.start) as usize;
        let handler = (*emitter).write_handler.unwrap();
        if handler((*emitter).write_handler_data, (*emitter).buffer.start, size) != 0 {
            (*emitter).buffer.last = (*emitter).buffer.start;
            (*emitter).buffer.pointer = (*emitter).buffer.start;
            return 1;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0".as_ptr() as *const c_char,
            );
        }
    }

    // UTF-16: recode
    let low: usize = if (*emitter).encoding == YAML_UTF16LE_ENCODING { 0 } else { 1 };
    let high: usize = if (*emitter).encoding == YAML_UTF16LE_ENCODING { 1 } else { 0 };

    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let octet = *(*emitter).buffer.pointer;
        let width: usize = if (octet & 0x80) == 0x00 { 1 }
            else if (octet & 0xE0) == 0xC0 { 2 }
            else if (octet & 0xF0) == 0xE0 { 3 }
            else if (octet & 0xF8) == 0xF0 { 4 }
            else { 0 };
        let mut value: u32 = if (octet & 0x80) == 0x00 { (octet & 0x7F) as u32 }
            else if (octet & 0xE0) == 0xC0 { (octet & 0x1F) as u32 }
            else if (octet & 0xF0) == 0xE0 { (octet & 0x0F) as u32 }
            else if (octet & 0xF8) == 0xF0 { (octet & 0x07) as u32 }
            else { 0 };

        for k in 1..width {
            let o = *(*emitter).buffer.pointer.add(k);
            value = (value << 6) + (o & 0x3F) as u32;
        }
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(width);

        if value < 0x10000 {
            *(*emitter).raw_buffer.last.add(high) = (value >> 8) as u8;
            *(*emitter).raw_buffer.last.add(low) = (value & 0xFF) as u8;
            (*emitter).raw_buffer.last = (*emitter).raw_buffer.last.add(2);
        } else {
            value -= 0x10000;
            *(*emitter).raw_buffer.last.add(high) = (0xD8 + (value >> 18)) as u8;
            *(*emitter).raw_buffer.last.add(low) = ((value >> 10) & 0xFF) as u8;
            *(*emitter).raw_buffer.last.add(high + 2) = (0xDC + ((value >> 8) & 0xFF)) as u8;
            *(*emitter).raw_buffer.last.add(low + 2) = (value & 0xFF) as u8;
            (*emitter).raw_buffer.last = (*emitter).raw_buffer.last.add(4);
        }
    }

    // Write raw buffer
    let raw_size = (*emitter).raw_buffer.last.offset_from((*emitter).raw_buffer.start) as usize;
    let handler = (*emitter).write_handler.unwrap();
    if handler((*emitter).write_handler_data, (*emitter).raw_buffer.start, raw_size) != 0 {
        (*emitter).buffer.last = (*emitter).buffer.start;
        (*emitter).buffer.pointer = (*emitter).buffer.start;
        (*emitter).raw_buffer.last = (*emitter).raw_buffer.start;
        (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.start;
        1
    } else {
        yaml_emitter_set_writer_error(
            emitter,
            b"write error\0".as_ptr() as *const c_char,
        )
    }
}


// ============================================================
// Parser inline helpers (event/token initializers translated from macros)
// ============================================================

#[inline]
unsafe fn event_init(event: *mut yaml_event_t, type_: yaml_event_type_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());
    (*event).type_ = type_;
    (*event).start_mark = start_mark;
    (*event).end_mark = end_mark;
}

#[inline]
unsafe fn stream_start_event_init(event: *mut yaml_event_t, encoding: yaml_encoding_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_STREAM_START_EVENT, start_mark, end_mark);
    (*event).data.stream_start.encoding = encoding;
}

#[inline]
unsafe fn stream_end_event_init(event: *mut yaml_event_t, start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_STREAM_END_EVENT, start_mark, end_mark);
}

#[inline]
unsafe fn document_start_event_init(event: *mut yaml_event_t,
    version_directive: *mut yaml_version_directive_t,
    tag_directives_start: *mut yaml_tag_directive_t,
    tag_directives_end: *mut yaml_tag_directive_t,
    implicit: i32,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_DOCUMENT_START_EVENT, start_mark, end_mark);
    (*event).data.document_start.version_directive = version_directive;
    (*event).data.document_start.tag_directives.start = tag_directives_start;
    (*event).data.document_start.tag_directives.end = tag_directives_end;
    (*event).data.document_start.implicit = implicit;
}

#[inline]
unsafe fn document_end_event_init(event: *mut yaml_event_t, implicit: i32,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_DOCUMENT_END_EVENT, start_mark, end_mark);
    (*event).data.document_end.implicit = implicit;
}

#[inline]
unsafe fn alias_event_init(event: *mut yaml_event_t, anchor: *mut yaml_char_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_ALIAS_EVENT, start_mark, end_mark);
    (*event).data.alias.anchor = anchor;
}

#[inline]
unsafe fn scalar_event_init(event: *mut yaml_event_t,
    anchor: *mut yaml_char_t, tag: *mut yaml_char_t,
    value: *mut yaml_char_t, length: usize,
    plain_implicit: i32, quoted_implicit: i32,
    style: yaml_scalar_style_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_SCALAR_EVENT, start_mark, end_mark);
    (*event).data.scalar.anchor = anchor;
    (*event).data.scalar.tag = tag;
    (*event).data.scalar.value = value;
    (*event).data.scalar.length = length;
    (*event).data.scalar.plain_implicit = plain_implicit;
    (*event).data.scalar.quoted_implicit = quoted_implicit;
    (*event).data.scalar.style = style;
}

#[inline]
unsafe fn sequence_start_event_init(event: *mut yaml_event_t,
    anchor: *mut yaml_char_t, tag: *mut yaml_char_t,
    implicit: i32, style: yaml_sequence_style_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_SEQUENCE_START_EVENT, start_mark, end_mark);
    (*event).data.sequence_start.anchor = anchor;
    (*event).data.sequence_start.tag = tag;
    (*event).data.sequence_start.implicit = implicit;
    (*event).data.sequence_start.style = style;
}

#[inline]
unsafe fn sequence_end_event_init(event: *mut yaml_event_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_SEQUENCE_END_EVENT, start_mark, end_mark);
}

#[inline]
unsafe fn mapping_start_event_init(event: *mut yaml_event_t,
    anchor: *mut yaml_char_t, tag: *mut yaml_char_t,
    implicit: i32, style: yaml_mapping_style_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_MAPPING_START_EVENT, start_mark, end_mark);
    (*event).data.mapping_start.anchor = anchor;
    (*event).data.mapping_start.tag = tag;
    (*event).data.mapping_start.implicit = implicit;
    (*event).data.mapping_start.style = style;
}

#[inline]
unsafe fn mapping_end_event_init(event: *mut yaml_event_t,
    start_mark: yaml_mark_t, end_mark: yaml_mark_t) {
    event_init(event, YAML_MAPPING_END_EVENT, start_mark, end_mark);
}

// Inline PEEK_TOKEN: returns tokens.head if token_available or fetch succeeds
#[inline]
unsafe fn parser_peek_token(parser: *mut yaml_parser_t) -> *mut yaml_token_t {
    if (*parser).token_available != 0 || yaml_parser_fetch_more_tokens(parser) != 0 {
        (*parser).tokens.head
    } else {
        ptr::null_mut()
    }
}

// Inline SKIP_TOKEN
#[inline]
unsafe fn parser_skip_token(parser: *mut yaml_parser_t) {
    (*parser).token_available = 0;
    (*parser).tokens_parsed += 1;
    (*parser).stream_end_produced =
        ((*(*parser).tokens.head).type_ == YAML_STREAM_END_TOKEN) as i32;
    (*parser).tokens.head = (*parser).tokens.head.add(1);
}

// ============================================================
// parser.c functions
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_set_max_nest_level(max: i32) {
    MAX_NESTING_LEVEL = max;
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_parse(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    debug_assert!(!parser.is_null());
    debug_assert!(!event.is_null());

    libc::memset(event as *mut c_void, 0, core::mem::size_of::<yaml_event_t>());

    if (*parser).stream_end_produced != 0
        || (*parser).error != YAML_NO_ERROR
        || (*parser).state == YAML_PARSE_END_STATE
    {
        return 1;
    }

    yaml_parser_state_machine(parser, event)
}

unsafe fn yaml_parser_set_parser_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_PARSER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    0
}

unsafe fn yaml_parser_set_parser_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_PARSER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    0
}

unsafe fn yaml_maximum_level_reached(
    parser: *mut yaml_parser_t,
    context_mark: yaml_mark_t,
    problem_mark: yaml_mark_t,
) -> i32 {
    yaml_parser_set_parser_error_context(
        parser,
        b"while parsing\0".as_ptr() as *const c_char,
        context_mark,
        b"Maximum nesting level reached, set with yaml_set_max_nest_level())\0".as_ptr() as *const c_char,
        problem_mark,
    );
    0
}

unsafe fn yaml_parser_state_machine(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    match (*parser).state {
        YAML_PARSE_STREAM_START_STATE =>
            yaml_parser_parse_stream_start(parser, event),
        YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE =>
            yaml_parser_parse_document_start(parser, event, 1),
        YAML_PARSE_DOCUMENT_START_STATE =>
            yaml_parser_parse_document_start(parser, event, 0),
        YAML_PARSE_DOCUMENT_CONTENT_STATE =>
            yaml_parser_parse_document_content(parser, event),
        YAML_PARSE_DOCUMENT_END_STATE =>
            yaml_parser_parse_document_end(parser, event),
        YAML_PARSE_BLOCK_NODE_STATE =>
            yaml_parser_parse_node(parser, event, 1, 0),
        YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE =>
            yaml_parser_parse_node(parser, event, 1, 1),
        YAML_PARSE_FLOW_NODE_STATE =>
            yaml_parser_parse_node(parser, event, 0, 0),
        YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE =>
            yaml_parser_parse_block_sequence_entry(parser, event, 1),
        YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE =>
            yaml_parser_parse_block_sequence_entry(parser, event, 0),
        YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE =>
            yaml_parser_parse_indentless_sequence_entry(parser, event),
        YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE =>
            yaml_parser_parse_block_mapping_key(parser, event, 1),
        YAML_PARSE_BLOCK_MAPPING_KEY_STATE =>
            yaml_parser_parse_block_mapping_key(parser, event, 0),
        YAML_PARSE_BLOCK_MAPPING_VALUE_STATE =>
            yaml_parser_parse_block_mapping_value(parser, event),
        YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE =>
            yaml_parser_parse_flow_sequence_entry(parser, event, 1),
        YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE =>
            yaml_parser_parse_flow_sequence_entry(parser, event, 0),
        YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE =>
            yaml_parser_parse_flow_sequence_entry_mapping_key(parser, event),
        YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE =>
            yaml_parser_parse_flow_sequence_entry_mapping_value(parser, event),
        YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE =>
            yaml_parser_parse_flow_sequence_entry_mapping_end(parser, event),
        YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE =>
            yaml_parser_parse_flow_mapping_key(parser, event, 1),
        YAML_PARSE_FLOW_MAPPING_KEY_STATE =>
            yaml_parser_parse_flow_mapping_key(parser, event, 0),
        YAML_PARSE_FLOW_MAPPING_VALUE_STATE =>
            yaml_parser_parse_flow_mapping_value(parser, event, 0),
        YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE =>
            yaml_parser_parse_flow_mapping_value(parser, event, 1),
        _ => {
            debug_assert!(false); // Invalid state
            0
        }
    }
}

unsafe fn yaml_parser_parse_stream_start(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ != YAML_STREAM_START_TOKEN {
        return yaml_parser_set_parser_error(
            parser,
            b"did not find expected <stream-start>\0".as_ptr() as *const c_char,
            (*token).start_mark,
        );
    }

    (*parser).state = YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE;
    stream_start_event_init(event, (*token).data.stream_start.encoding,
        (*token).start_mark, (*token).start_mark);
    parser_skip_token(parser);
    1
}

unsafe fn yaml_parser_parse_document_start(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    implicit: i32,
) -> i32 {
    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    let mut version_directive: *mut yaml_version_directive_t = ptr::null_mut();
    let mut tag_directives_start: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut tag_directives_end: *mut yaml_tag_directive_t = ptr::null_mut();

    // Parse extra document end indicators for explicit docs
    if implicit == 0 {
        while (*token).type_ == YAML_DOCUMENT_END_TOKEN {
            parser_skip_token(parser);
            token = parser_peek_token(parser);
            if token.is_null() { return 0; }
        }
    }

    // Parse an implicit document
    if implicit != 0
        && (*token).type_ != YAML_VERSION_DIRECTIVE_TOKEN
        && (*token).type_ != YAML_TAG_DIRECTIVE_TOKEN
        && (*token).type_ != YAML_DOCUMENT_START_TOKEN
        && (*token).type_ != YAML_STREAM_END_TOKEN
    {
        if yaml_parser_process_directives(parser, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) == 0 {
            return 0;
        }
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).states.start,
            &mut (*parser).states.top,
            &mut (*parser).states.end,
            YAML_PARSE_DOCUMENT_END_STATE,
        ) == 0 { return 0; }
        (*parser).state = YAML_PARSE_BLOCK_NODE_STATE;
        document_start_event_init(event, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), 1,
            (*token).start_mark, (*token).start_mark);
        return 1;
    }

    // Parse an explicit document
    if (*token).type_ != YAML_STREAM_END_TOKEN {
        let start_mark = (*token).start_mark;
        if yaml_parser_process_directives(parser,
            &mut version_directive,
            &mut tag_directives_start,
            &mut tag_directives_end,
        ) == 0 { return 0; }

        token = parser_peek_token(parser);
        if token.is_null() {
            yaml_free(version_directive as *mut c_void);
            // free tag_directives
            let mut td = tag_directives_end;
            while td != tag_directives_start {
                td = td.sub(1);
                yaml_free((*td).handle as *mut c_void);
                yaml_free((*td).prefix as *mut c_void);
            }
            yaml_free(tag_directives_start as *mut c_void);
            return 0;
        }

        if (*token).type_ != YAML_DOCUMENT_START_TOKEN {
            yaml_parser_set_parser_error(
                parser,
                b"did not find expected <document start>\0".as_ptr() as *const c_char,
                (*token).start_mark,
            );
            yaml_free(version_directive as *mut c_void);
            let mut td = tag_directives_end;
            while td != tag_directives_start {
                td = td.sub(1);
                yaml_free((*td).handle as *mut c_void);
                yaml_free((*td).prefix as *mut c_void);
            }
            yaml_free(tag_directives_start as *mut c_void);
            return 0;
        }

        if stack_push(
            &mut (*parser).error,
            &mut (*parser).states.start,
            &mut (*parser).states.top,
            &mut (*parser).states.end,
            YAML_PARSE_DOCUMENT_END_STATE,
        ) == 0 {
            yaml_free(version_directive as *mut c_void);
            let mut td = tag_directives_end;
            while td != tag_directives_start {
                td = td.sub(1);
                yaml_free((*td).handle as *mut c_void);
                yaml_free((*td).prefix as *mut c_void);
            }
            yaml_free(tag_directives_start as *mut c_void);
            return 0;
        }
        (*parser).state = YAML_PARSE_DOCUMENT_CONTENT_STATE;
        let end_mark = (*token).end_mark;
        document_start_event_init(event, version_directive,
            tag_directives_start, tag_directives_end, 0,
            start_mark, end_mark);
        parser_skip_token(parser);
        return 1;
    }

    // Parse stream end
    (*parser).state = YAML_PARSE_END_STATE;
    stream_end_event_init(event, (*token).start_mark, (*token).end_mark);
    parser_skip_token(parser);
    1
}

unsafe fn yaml_parser_parse_document_content(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ == YAML_VERSION_DIRECTIVE_TOKEN
        || (*token).type_ == YAML_TAG_DIRECTIVE_TOKEN
        || (*token).type_ == YAML_DOCUMENT_START_TOKEN
        || (*token).type_ == YAML_DOCUMENT_END_TOKEN
        || (*token).type_ == YAML_STREAM_END_TOKEN
    {
        (*parser).state = stack_pop(&mut (*parser).states.top);
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }

    yaml_parser_parse_node(parser, event, 1, 0)
}

unsafe fn yaml_parser_parse_document_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    let start_mark = (*token).start_mark;
    let mut end_mark = (*token).start_mark;
    let mut implicit = 1i32;

    if (*token).type_ == YAML_DOCUMENT_END_TOKEN {
        end_mark = (*token).end_mark;
        parser_skip_token(parser);
        implicit = 0;
    }

    while !stack_empty((*parser).tag_directives.start, (*parser).tag_directives.top) {
        let td = stack_pop(&mut (*parser).tag_directives.top);
        yaml_free(td.handle as *mut c_void);
        yaml_free(td.prefix as *mut c_void);
    }

    (*parser).state = YAML_PARSE_DOCUMENT_START_STATE;
    document_end_event_init(event, implicit, start_mark, end_mark);
    1
}

unsafe fn yaml_parser_parse_node(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    block: i32,
    indentless_sequence: i32,
) -> i32 {
    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    let mut anchor: *mut yaml_char_t = ptr::null_mut();
    let mut tag_handle: *mut yaml_char_t = ptr::null_mut();
    let mut tag_suffix: *mut yaml_char_t = ptr::null_mut();
    let mut tag: *mut yaml_char_t = ptr::null_mut();
    let mut start_mark: yaml_mark_t;
    let mut end_mark: yaml_mark_t;
    let mut tag_mark: yaml_mark_t = yaml_mark_t { index: 0, line: 0, column: 0 };

    if (*token).type_ == YAML_ALIAS_TOKEN {
        (*parser).state = stack_pop(&mut (*parser).states.top);
        alias_event_init(event, (*token).data.alias.value,
            (*token).start_mark, (*token).end_mark);
        parser_skip_token(parser);
        return 1;
    }

    start_mark = (*token).start_mark;
    end_mark = (*token).start_mark;

    if (*token).type_ == YAML_ANCHOR_TOKEN {
        anchor = (*token).data.anchor.value;
        start_mark = (*token).start_mark;
        end_mark = (*token).end_mark;
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { goto_node_error(anchor, tag_handle, tag_suffix, tag); return 0; }

        if (*token).type_ == YAML_TAG_TOKEN {
            tag_handle = (*token).data.tag.handle;
            tag_suffix = (*token).data.tag.suffix;
            tag_mark = (*token).start_mark;
            end_mark = (*token).end_mark;
            parser_skip_token(parser);
            token = parser_peek_token(parser);
            if token.is_null() { goto_node_error(anchor, tag_handle, tag_suffix, tag); return 0; }
        }
    } else if (*token).type_ == YAML_TAG_TOKEN {
        tag_handle = (*token).data.tag.handle;
        tag_suffix = (*token).data.tag.suffix;
        start_mark = (*token).start_mark;
        tag_mark = (*token).start_mark;
        end_mark = (*token).end_mark;
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { goto_node_error(anchor, tag_handle, tag_suffix, tag); return 0; }

        if (*token).type_ == YAML_ANCHOR_TOKEN {
            anchor = (*token).data.anchor.value;
            end_mark = (*token).end_mark;
            parser_skip_token(parser);
            token = parser_peek_token(parser);
            if token.is_null() { goto_node_error(anchor, tag_handle, tag_suffix, tag); return 0; }
        }
    }

    if !tag_handle.is_null() {
        if *tag_handle == 0 {
            tag = tag_suffix;
            yaml_free(tag_handle as *mut c_void);
            tag_handle = ptr::null_mut();
            tag_suffix = ptr::null_mut();
        } else {
            let mut found = false;
            let mut td = (*parser).tag_directives.start;
            while td != (*parser).tag_directives.top {
                if libc::strcmp((*td).handle as *const c_char, tag_handle as *const c_char) == 0 {
                    let prefix_len = libc::strlen((*td).prefix as *const c_char);
                    let suffix_len = libc::strlen(tag_suffix as *const c_char);
                    tag = yaml_malloc(prefix_len + suffix_len + 1) as *mut yaml_char_t;
                    if tag.is_null() {
                        (*parser).error = YAML_MEMORY_ERROR;
                        goto_node_error(anchor, tag_handle, tag_suffix, tag);
                        return 0;
                    }
                    libc::memcpy(tag as *mut c_void, (*td).prefix as *const c_void, prefix_len);
                    libc::memcpy(tag.add(prefix_len) as *mut c_void, tag_suffix as *const c_void, suffix_len);
                    *tag.add(prefix_len + suffix_len) = 0;
                    yaml_free(tag_handle as *mut c_void);
                    yaml_free(tag_suffix as *mut c_void);
                    tag_handle = ptr::null_mut();
                    tag_suffix = ptr::null_mut();
                    found = true;
                    break;
                }
                td = td.add(1);
            }
            if !found {
                yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a node\0".as_ptr() as *const c_char,
                    start_mark,
                    b"found undefined tag handle\0".as_ptr() as *const c_char,
                    tag_mark,
                );
                goto_node_error(anchor, tag_handle, tag_suffix, tag);
                return 0;
            }
        }
    }

    let implicit = (tag.is_null() || *tag == 0) as i32;

    if indentless_sequence != 0 && (*token).type_ == YAML_BLOCK_ENTRY_TOKEN {
        end_mark = (*token).end_mark;
        (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
        sequence_start_event_init(event, anchor, tag, implicit,
            YAML_BLOCK_SEQUENCE_STYLE, start_mark, end_mark);
        return 1;
    }

    if (*token).type_ == YAML_SCALAR_TOKEN {
        let mut plain_implicit = 0i32;
        let mut quoted_implicit = 0i32;
        end_mark = (*token).end_mark;
        if ((*token).data.scalar.style == YAML_PLAIN_SCALAR_STYLE && tag.is_null())
            || (!tag.is_null() && libc::strcmp(tag as *const c_char, b"!\0".as_ptr() as *const c_char) == 0)
        {
            plain_implicit = 1;
        } else if tag.is_null() {
            quoted_implicit = 1;
        }
        (*parser).state = stack_pop(&mut (*parser).states.top);
        scalar_event_init(event, anchor, tag,
            (*token).data.scalar.value, (*token).data.scalar.length,
            plain_implicit, quoted_implicit,
            (*token).data.scalar.style, start_mark, end_mark);
        parser_skip_token(parser);
        return 1;
    } else if (*token).type_ == YAML_FLOW_SEQUENCE_START_TOKEN {
        if stack_limit(
            &mut (*parser).error,
            (*parser).indents.top,
            (*parser).indents.start,
            (MAX_NESTING_LEVEL - (*parser).flow_level) as isize,
        ) == 0 {
            yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
            goto_node_error(anchor, tag_handle, tag_suffix, tag);
            return 0;
        }
        end_mark = (*token).end_mark;
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE;
        sequence_start_event_init(event, anchor, tag, implicit,
            YAML_FLOW_SEQUENCE_STYLE, start_mark, end_mark);
        return 1;
    } else if (*token).type_ == YAML_FLOW_MAPPING_START_TOKEN {
        if stack_limit(
            &mut (*parser).error,
            (*parser).indents.top,
            (*parser).indents.start,
            (MAX_NESTING_LEVEL - (*parser).flow_level) as isize,
        ) == 0 {
            yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
            goto_node_error(anchor, tag_handle, tag_suffix, tag);
            return 0;
        }
        end_mark = (*token).end_mark;
        (*parser).state = YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE;
        mapping_start_event_init(event, anchor, tag, implicit,
            YAML_FLOW_MAPPING_STYLE, start_mark, end_mark);
        return 1;
    } else if block != 0 && (*token).type_ == YAML_BLOCK_SEQUENCE_START_TOKEN {
        if stack_limit(
            &mut (*parser).error,
            (*parser).indents.top,
            (*parser).indents.start,
            (MAX_NESTING_LEVEL - (*parser).flow_level) as isize,
        ) == 0 {
            yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
            goto_node_error(anchor, tag_handle, tag_suffix, tag);
            return 0;
        }
        end_mark = (*token).end_mark;
        (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE;
        sequence_start_event_init(event, anchor, tag, implicit,
            YAML_BLOCK_SEQUENCE_STYLE, start_mark, end_mark);
        return 1;
    } else if block != 0 && (*token).type_ == YAML_BLOCK_MAPPING_START_TOKEN {
        if stack_limit(
            &mut (*parser).error,
            (*parser).indents.top,
            (*parser).indents.start,
            (MAX_NESTING_LEVEL - (*parser).flow_level) as isize,
        ) == 0 {
            yaml_maximum_level_reached(parser, start_mark, (*token).start_mark);
            goto_node_error(anchor, tag_handle, tag_suffix, tag);
            return 0;
        }
        end_mark = (*token).end_mark;
        (*parser).state = YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE;
        mapping_start_event_init(event, anchor, tag, implicit,
            YAML_BLOCK_MAPPING_STYLE, start_mark, end_mark);
        return 1;
    } else if !anchor.is_null() || !tag.is_null() {
        let value = yaml_malloc(1) as *mut yaml_char_t;
        if value.is_null() {
            (*parser).error = YAML_MEMORY_ERROR;
            goto_node_error(anchor, tag_handle, tag_suffix, tag);
            return 0;
        }
        *value = 0;
        (*parser).state = stack_pop(&mut (*parser).states.top);
        scalar_event_init(event, anchor, tag, value, 0,
            implicit, 0, YAML_PLAIN_SCALAR_STYLE, start_mark, end_mark);
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
        goto_node_error(anchor, tag_handle, tag_suffix, tag);
        return 0;
    }
}

#[inline]
unsafe fn goto_node_error(
    anchor: *mut yaml_char_t,
    tag_handle: *mut yaml_char_t,
    tag_suffix: *mut yaml_char_t,
    tag: *mut yaml_char_t,
) {
    yaml_free(anchor as *mut c_void);
    yaml_free(tag_handle as *mut c_void);
    yaml_free(tag_suffix as *mut c_void);
    yaml_free(tag as *mut c_void);
}

unsafe fn yaml_parser_parse_block_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: i32,
) -> i32 {
    if first != 0 {
        let token = parser_peek_token(parser);
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).marks.start,
            &mut (*parser).marks.top,
            &mut (*parser).marks.end,
            (*token).start_mark,
        ) == 0 { return 0; }
        parser_skip_token(parser);
    }

    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ == YAML_BLOCK_ENTRY_TOKEN {
        let mark = (*token).end_mark;
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { return 0; }
        if (*token).type_ != YAML_BLOCK_ENTRY_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 1, 0);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_ == YAML_BLOCK_END_TOKEN {
        (*parser).state = stack_pop(&mut (*parser).states.top);
        let _ = stack_pop(&mut (*parser).marks.top);
        sequence_end_event_init(event, (*token).start_mark, (*token).end_mark);
        parser_skip_token(parser);
        return 1;
    } else {
        let mark = stack_pop(&mut (*parser).marks.top);
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block collection\0".as_ptr() as *const c_char,
            mark,
            b"did not find expected '-' indicator\0".as_ptr() as *const c_char,
            (*token).start_mark,
        );
    }
}

unsafe fn yaml_parser_parse_indentless_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ == YAML_BLOCK_ENTRY_TOKEN {
        let mark = (*token).end_mark;
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { return 0; }
        if (*token).type_ != YAML_BLOCK_ENTRY_TOKEN
            && (*token).type_ != YAML_KEY_TOKEN
            && (*token).type_ != YAML_VALUE_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 1, 0);
        } else {
            (*parser).state = YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else {
        (*parser).state = stack_pop(&mut (*parser).states.top);
        sequence_end_event_init(event, (*token).start_mark, (*token).start_mark);
        return 1;
    }
}

unsafe fn yaml_parser_parse_block_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: i32,
) -> i32 {
    if first != 0 {
        let token = parser_peek_token(parser);
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).marks.start,
            &mut (*parser).marks.top,
            &mut (*parser).marks.end,
            (*token).start_mark,
        ) == 0 { return 0; }
        parser_skip_token(parser);
    }

    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ == YAML_KEY_TOKEN {
        let mark = (*token).end_mark;
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { return 0; }
        if (*token).type_ != YAML_KEY_TOKEN
            && (*token).type_ != YAML_VALUE_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_BLOCK_MAPPING_VALUE_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 1, 1);
        } else {
            (*parser).state = YAML_PARSE_BLOCK_MAPPING_VALUE_STATE;
            return yaml_parser_process_empty_scalar(parser, event, mark);
        }
    } else if (*token).type_ == YAML_BLOCK_END_TOKEN {
        (*parser).state = stack_pop(&mut (*parser).states.top);
        let _ = stack_pop(&mut (*parser).marks.top);
        mapping_end_event_init(event, (*token).start_mark, (*token).end_mark);
        parser_skip_token(parser);
        return 1;
    } else {
        let mark = stack_pop(&mut (*parser).marks.top);
        return yaml_parser_set_parser_error_context(
            parser,
            b"while parsing a block mapping\0".as_ptr() as *const c_char,
            mark,
            b"did not find expected key\0".as_ptr() as *const c_char,
            (*token).start_mark,
        );
    }
}

unsafe fn yaml_parser_parse_block_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ == YAML_VALUE_TOKEN {
        let mark = (*token).end_mark;
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { return 0; }
        if (*token).type_ != YAML_KEY_TOKEN
            && (*token).type_ != YAML_VALUE_TOKEN
            && (*token).type_ != YAML_BLOCK_END_TOKEN
        {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_BLOCK_MAPPING_KEY_STATE,
            ) == 0 { return 0; }
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

unsafe fn yaml_parser_parse_flow_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: i32,
) -> i32 {
    if first != 0 {
        let token = parser_peek_token(parser);
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).marks.start,
            &mut (*parser).marks.top,
            &mut (*parser).marks.end,
            (*token).start_mark,
        ) == 0 { return 0; }
        parser_skip_token(parser);
    }

    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN {
        if first == 0 {
            if (*token).type_ == YAML_FLOW_ENTRY_TOKEN {
                parser_skip_token(parser);
                token = parser_peek_token(parser);
                if token.is_null() { return 0; }
            } else {
                let mark = stack_pop(&mut (*parser).marks.top);
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow sequence\0".as_ptr() as *const c_char,
                    mark,
                    b"did not find expected ',' or ']'\0".as_ptr() as *const c_char,
                    (*token).start_mark,
                );
            }
        }

        if (*token).type_ == YAML_KEY_TOKEN {
            (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE;
            mapping_start_event_init(event, ptr::null_mut(), ptr::null_mut(),
                1, YAML_FLOW_MAPPING_STYLE,
                (*token).start_mark, (*token).end_mark);
            parser_skip_token(parser);
            return 1;
        } else if (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = stack_pop(&mut (*parser).states.top);
    let _ = stack_pop(&mut (*parser).marks.top);
    sequence_end_event_init(event, (*token).start_mark, (*token).end_mark);
    parser_skip_token(parser);
    1
}

unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ != YAML_VALUE_TOKEN
        && (*token).type_ != YAML_FLOW_ENTRY_TOKEN
        && (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN
    {
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).states.start,
            &mut (*parser).states.top,
            &mut (*parser).states.end,
            YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE,
        ) == 0 { return 0; }
        return yaml_parser_parse_node(parser, event, 0, 0);
    } else if (*token).type_ == YAML_FLOW_SEQUENCE_END_TOKEN {
        let mark = (*token).start_mark;
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        return yaml_parser_process_empty_scalar(parser, event, mark);
    } else {
        let mark = (*token).end_mark;
        parser_skip_token(parser);
        (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE;
        return yaml_parser_process_empty_scalar(parser, event, mark);
    }
}

unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ == YAML_VALUE_TOKEN {
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { return 0; }
        if (*token).type_ != YAML_FLOW_ENTRY_TOKEN
            && (*token).type_ != YAML_FLOW_SEQUENCE_END_TOKEN
        {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE;
    yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
}

unsafe fn yaml_parser_parse_flow_sequence_entry_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    let token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    (*parser).state = YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE;
    mapping_end_event_init(event, (*token).start_mark, (*token).start_mark);
    1
}

unsafe fn yaml_parser_parse_flow_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: i32,
) -> i32 {
    if first != 0 {
        let token = parser_peek_token(parser);
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).marks.start,
            &mut (*parser).marks.top,
            &mut (*parser).marks.end,
            (*token).start_mark,
        ) == 0 { return 0; }
        parser_skip_token(parser);
    }

    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN {
        if first == 0 {
            if (*token).type_ == YAML_FLOW_ENTRY_TOKEN {
                parser_skip_token(parser);
                token = parser_peek_token(parser);
                if token.is_null() { return 0; }
            } else {
                let mark = stack_pop(&mut (*parser).marks.top);
                return yaml_parser_set_parser_error_context(
                    parser,
                    b"while parsing a flow mapping\0".as_ptr() as *const c_char,
                    mark,
                    b"did not find expected ',' or '}'\0".as_ptr() as *const c_char,
                    (*token).start_mark,
                );
            }
        }

        if (*token).type_ == YAML_KEY_TOKEN {
            parser_skip_token(parser);
            token = parser_peek_token(parser);
            if token.is_null() { return 0; }
            if (*token).type_ != YAML_VALUE_TOKEN
                && (*token).type_ != YAML_FLOW_ENTRY_TOKEN
                && (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN
            {
                if stack_push(
                    &mut (*parser).error,
                    &mut (*parser).states.start,
                    &mut (*parser).states.top,
                    &mut (*parser).states.end,
                    YAML_PARSE_FLOW_MAPPING_VALUE_STATE,
                ) == 0 { return 0; }
                return yaml_parser_parse_node(parser, event, 0, 0);
            } else {
                (*parser).state = YAML_PARSE_FLOW_MAPPING_VALUE_STATE;
                return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
            }
        } else if (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = stack_pop(&mut (*parser).states.top);
    let _ = stack_pop(&mut (*parser).marks.top);
    mapping_end_event_init(event, (*token).start_mark, (*token).end_mark);
    parser_skip_token(parser);
    1
}

unsafe fn yaml_parser_parse_flow_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    empty: i32,
) -> i32 {
    let mut token = parser_peek_token(parser);
    if token.is_null() { return 0; }

    if empty != 0 {
        (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
        return yaml_parser_process_empty_scalar(parser, event, (*token).start_mark);
    }

    if (*token).type_ == YAML_VALUE_TOKEN {
        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() { return 0; }
        if (*token).type_ != YAML_FLOW_ENTRY_TOKEN
            && (*token).type_ != YAML_FLOW_MAPPING_END_TOKEN
        {
            if stack_push(
                &mut (*parser).error,
                &mut (*parser).states.start,
                &mut (*parser).states.top,
                &mut (*parser).states.end,
                YAML_PARSE_FLOW_MAPPING_KEY_STATE,
            ) == 0 { return 0; }
            return yaml_parser_parse_node(parser, event, 0, 0);
        }
    }

    (*parser).state = YAML_PARSE_FLOW_MAPPING_KEY_STATE;
    yaml_parser_process_empty_scalar(parser, event, (*token).start_mark)
}

unsafe fn yaml_parser_process_empty_scalar(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    mark: yaml_mark_t,
) -> i32 {
    let value = yaml_malloc(1) as *mut yaml_char_t;
    if value.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0;
    }
    *value = 0;
    scalar_event_init(event, ptr::null_mut(), ptr::null_mut(), value, 0,
        1, 0, YAML_PLAIN_SCALAR_STYLE, mark, mark);
    1
}

unsafe fn yaml_parser_process_directives(
    parser: *mut yaml_parser_t,
    version_directive_ref: *mut *mut yaml_version_directive_t,
    tag_directives_start_ref: *mut *mut yaml_tag_directive_t,
    tag_directives_end_ref: *mut *mut yaml_tag_directive_t,
) -> i32 {
    // Default tag directives
    // { "!", "!" }, { "!!", "tag:yaml.org,2002:" }, { NULL, NULL }
    let default_tag_directives: [(*const u8, *const u8); 2] = [
        (b"!\0".as_ptr(), b"!\0".as_ptr()),
        (b"!!\0".as_ptr(), b"tag:yaml.org,2002:\0".as_ptr()),
    ];

    let mut version_directive: *mut yaml_version_directive_t = ptr::null_mut();
    let mut td_start: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut td_top: *mut yaml_tag_directive_t = ptr::null_mut();
    let mut td_end: *mut yaml_tag_directive_t = ptr::null_mut();

    if stack_init(&mut (*parser).error, &mut td_start, &mut td_top, &mut td_end) == 0 {
        return 0;
    }

    let mut token = parser_peek_token(parser);
    if token.is_null() {
        stack_del(&mut td_start, &mut td_top, &mut td_end);
        return 0;
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
                yaml_free(version_directive as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end);
                return 0;
            }
            let major = (*token).data.version_directive.major;
            let minor = (*token).data.version_directive.minor;
            if major != 1 || (minor != 1 && minor != 2) {
                yaml_parser_set_parser_error(
                    parser,
                    b"found incompatible YAML document\0".as_ptr() as *const c_char,
                    (*token).start_mark,
                );
                yaml_free(version_directive as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end);
                return 0;
            }
            version_directive = yaml_malloc(core::mem::size_of::<yaml_version_directive_t>())
                as *mut yaml_version_directive_t;
            if version_directive.is_null() {
                (*parser).error = YAML_MEMORY_ERROR;
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end);
                return 0;
            }
            (*version_directive).major = major;
            (*version_directive).minor = minor;
        } else {
            // YAML_TAG_DIRECTIVE_TOKEN
            let value = yaml_tag_directive_t {
                handle: (*token).data.tag_directive.handle,
                prefix: (*token).data.tag_directive.prefix,
            };
            if yaml_parser_append_tag_directive(parser, value, 0, (*token).start_mark) == 0 {
                yaml_free(version_directive as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end);
                return 0;
            }
            if stack_push(&mut (*parser).error, &mut td_start, &mut td_top, &mut td_end, value) == 0 {
                yaml_free(version_directive as *mut c_void);
                while !stack_empty(td_start, td_top) {
                    let v = stack_pop(&mut td_top);
                    yaml_free(v.handle as *mut c_void);
                    yaml_free(v.prefix as *mut c_void);
                }
                stack_del(&mut td_start, &mut td_top, &mut td_end);
                return 0;
            }
        }

        parser_skip_token(parser);
        token = parser_peek_token(parser);
        if token.is_null() {
            yaml_free(version_directive as *mut c_void);
            while !stack_empty(td_start, td_top) {
                let v = stack_pop(&mut td_top);
                yaml_free(v.handle as *mut c_void);
                yaml_free(v.prefix as *mut c_void);
            }
            stack_del(&mut td_start, &mut td_top, &mut td_end);
            return 0;
        }
    }

    // Process default tag directives
    for &(handle, prefix) in &default_tag_directives {
        let dv = yaml_tag_directive_t {
            handle: handle as *mut yaml_char_t,
            prefix: prefix as *mut yaml_char_t,
        };
        if yaml_parser_append_tag_directive(parser, dv, 1, (*token).start_mark) == 0 {
            yaml_free(version_directive as *mut c_void);
            while !stack_empty(td_start, td_top) {
                let v = stack_pop(&mut td_top);
                yaml_free(v.handle as *mut c_void);
                yaml_free(v.prefix as *mut c_void);
            }
            stack_del(&mut td_start, &mut td_top, &mut td_end);
            return 0;
        }
    }

    if !version_directive_ref.is_null() {
        *version_directive_ref = version_directive;
    }

    if !tag_directives_start_ref.is_null() {
        if stack_empty(td_start, td_top) {
            *tag_directives_start_ref = ptr::null_mut();
            *tag_directives_end_ref = ptr::null_mut();
            stack_del(&mut td_start, &mut td_top, &mut td_end);
        } else {
            *tag_directives_start_ref = td_start;
            *tag_directives_end_ref = td_top;
        }
    } else {
        stack_del(&mut td_start, &mut td_top, &mut td_end);
    }

    if version_directive_ref.is_null() {
        yaml_free(version_directive as *mut c_void);
    }

    1
}

unsafe fn yaml_parser_append_tag_directive(
    parser: *mut yaml_parser_t,
    value: yaml_tag_directive_t,
    allow_duplicates: i32,
    mark: yaml_mark_t,
) -> i32 {
    let mut td = (*parser).tag_directives.start;
    while td != (*parser).tag_directives.top {
        if libc::strcmp(value.handle as *const c_char, (*td).handle as *const c_char) == 0 {
            if allow_duplicates != 0 { return 1; }
            return yaml_parser_set_parser_error(
                parser,
                b"found duplicate %TAG directive\0".as_ptr() as *const c_char,
                mark,
            );
        }
        td = td.add(1);
    }

    let copy = yaml_tag_directive_t {
        handle: yaml_strdup(value.handle),
        prefix: yaml_strdup(value.prefix),
    };

    if copy.handle.is_null() || copy.prefix.is_null() {
        (*parser).error = YAML_MEMORY_ERROR;
        yaml_free(copy.handle as *mut c_void);
        yaml_free(copy.prefix as *mut c_void);
        return 0;
    }

    if stack_push(
        &mut (*parser).error,
        &mut (*parser).tag_directives.start,
        &mut (*parser).tag_directives.top,
        &mut (*parser).tag_directives.end,
        copy,
    ) == 0 {
        yaml_free(copy.handle as *mut c_void);
        yaml_free(copy.prefix as *mut c_void);
        return 0;
    }
    1
}


// ============================================================
// loader.c functions
// ============================================================

#[repr(C)]
struct loader_ctx {
    start: *mut i32,
    end: *mut i32,
    top: *mut i32,
}

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_load(
    parser: *mut yaml_parser_t,
    document: *mut yaml_document_t,
) -> i32 {
    debug_assert!(!parser.is_null());
    debug_assert!(!document.is_null());

    libc::memset(document as *mut c_void, 0, core::mem::size_of::<yaml_document_t>());
    if stack_init(
        &mut (*parser).error,
        &mut (*document).nodes.start,
        &mut (*document).nodes.top,
        &mut (*document).nodes.end,
    ) == 0 {
        return loader_error(parser, document);
    }

    if (*parser).stream_start_produced == 0 {
        let mut event: yaml_event_t = core::mem::zeroed();
        if yaml_parser_parse(parser, &mut event) == 0 {
            return loader_error(parser, document);
        }
        debug_assert!(event.type_ == YAML_STREAM_START_EVENT);
    }

    if (*parser).stream_end_produced != 0 {
        return 1;
    }

    let mut event: yaml_event_t = core::mem::zeroed();
    if yaml_parser_parse(parser, &mut event) == 0 {
        return loader_error(parser, document);
    }
    if event.type_ == YAML_STREAM_END_EVENT {
        return 1;
    }

    if stack_init(
        &mut (*parser).error,
        &mut (*parser).aliases.start,
        &mut (*parser).aliases.top,
        &mut (*parser).aliases.end,
    ) == 0 {
        return loader_error(parser, document);
    }

    (*parser).document = document;

    if yaml_parser_load_document(parser, &mut event) == 0 {
        loader_delete_aliases(parser);
        yaml_document_delete(document);
        (*parser).document = ptr::null_mut();
        return 0;
    }

    loader_delete_aliases(parser);
    (*parser).document = ptr::null_mut();
    1
}

unsafe fn loader_error(parser: *mut yaml_parser_t, document: *mut yaml_document_t) -> i32 {
    loader_delete_aliases(parser);
    yaml_document_delete(document);
    (*parser).document = ptr::null_mut();
    0
}

unsafe fn yaml_parser_loader_set_composer_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_COMPOSER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    0
}

unsafe fn yaml_parser_loader_set_composer_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> i32 {
    (*parser).error = YAML_COMPOSER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    0
}

unsafe fn loader_delete_aliases(parser: *mut yaml_parser_t) {
    while !stack_empty((*parser).aliases.start, (*parser).aliases.top) {
        let a = stack_pop(&mut (*parser).aliases.top);
        yaml_free(a.anchor as *mut c_void);
    }
    stack_del(
        &mut (*parser).aliases.start,
        &mut (*parser).aliases.top,
        &mut (*parser).aliases.end,
    );
}

unsafe fn yaml_parser_load_document(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> i32 {
    debug_assert!((*event).type_ == YAML_DOCUMENT_START_EVENT);

    (*(*parser).document).version_directive
        = (*event).data.document_start.version_directive;
    (*(*parser).document).tag_directives.start
        = (*event).data.document_start.tag_directives.start;
    (*(*parser).document).tag_directives.end
        = (*event).data.document_start.tag_directives.end;
    (*(*parser).document).start_implicit
        = (*event).data.document_start.implicit;
    (*(*parser).document).start_mark = (*event).start_mark;

    let mut ctx = loader_ctx {
        start: ptr::null_mut(),
        end: ptr::null_mut(),
        top: ptr::null_mut(),
    };

    if stack_init(
        &mut (*parser).error,
        &mut ctx.start,
        &mut ctx.top,
        &mut ctx.end,
    ) == 0 { return 0; }

    if yaml_parser_load_nodes(parser, &mut ctx) == 0 {
        stack_del(&mut ctx.start, &mut ctx.top, &mut ctx.end);
        return 0;
    }
    stack_del(&mut ctx.start, &mut ctx.top, &mut ctx.end);
    1
}

unsafe fn yaml_parser_load_nodes(
    parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
) -> i32 {
    loop {
        let mut event: yaml_event_t = core::mem::zeroed();
        if yaml_parser_parse(parser, &mut event) == 0 { return 0; }

        match event.type_ {
            YAML_ALIAS_EVENT => {
                if yaml_parser_load_alias(parser, &mut event, ctx) == 0 { return 0; }
            }
            YAML_SCALAR_EVENT => {
                if yaml_parser_load_scalar(parser, &mut event, ctx) == 0 { return 0; }
            }
            YAML_SEQUENCE_START_EVENT => {
                if yaml_parser_load_sequence(parser, &mut event, ctx) == 0 { return 0; }
            }
            YAML_SEQUENCE_END_EVENT => {
                if yaml_parser_load_sequence_end(parser, &mut event, ctx) == 0 { return 0; }
            }
            YAML_MAPPING_START_EVENT => {
                if yaml_parser_load_mapping(parser, &mut event, ctx) == 0 { return 0; }
            }
            YAML_MAPPING_END_EVENT => {
                if yaml_parser_load_mapping_end(parser, &mut event, ctx) == 0 { return 0; }
            }
            YAML_DOCUMENT_END_EVENT => {
                // handled below
            }
            _ => {
                debug_assert!(false); // Could not happen
                return 0;
            }
        }

        if event.type_ == YAML_DOCUMENT_END_EVENT {
            (*(*parser).document).end_implicit = event.data.document_end.implicit;
            (*(*parser).document).end_mark = event.end_mark;
            return 1;
        }
    }
}

unsafe fn yaml_parser_register_anchor(
    parser: *mut yaml_parser_t,
    index: i32,
    anchor: *mut yaml_char_t,
) -> i32 {
    if anchor.is_null() { return 1; }

    let data = yaml_alias_data_t {
        anchor,
        index,
        mark: (*(*parser).document).nodes.start.add(index as usize - 1).read().start_mark,
    };

    let mut alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if libc::strcmp((*alias_data).anchor as *const c_char, anchor as *const c_char) == 0 {
            yaml_free(anchor as *mut c_void);
            return yaml_parser_loader_set_composer_error_context(
                parser,
                b"found duplicate anchor; first occurrence\0".as_ptr() as *const c_char,
                (*alias_data).mark,
                b"second occurrence\0".as_ptr() as *const c_char,
                data.mark,
            );
        }
        alias_data = alias_data.add(1);
    }

    if stack_push(
        &mut (*parser).error,
        &mut (*parser).aliases.start,
        &mut (*parser).aliases.top,
        &mut (*parser).aliases.end,
        data,
    ) == 0 {
        yaml_free(anchor as *mut c_void);
        return 0;
    }
    1
}

unsafe fn yaml_parser_load_node_add(
    parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
    index: i32,
) -> i32 {
    if stack_empty((*ctx).start, (*ctx).top) {
        return 1;
    }

    let parent_index = *((*ctx).top.sub(1));
    let parent = (*(*parser).document).nodes.start.add(parent_index as usize - 1);

    match (*parent).type_ {
        YAML_SEQUENCE_NODE => {
            if stack_limit(
                &mut (*parser).error,
                (*parent).data.sequence.items.top,
                (*parent).data.sequence.items.start,
                (i32::MAX - 1) as isize,
            ) == 0 { return 0; }
            if stack_push(
                &mut (*parser).error,
                &mut (*parent).data.sequence.items.start,
                &mut (*parent).data.sequence.items.top,
                &mut (*parent).data.sequence.items.end,
                index,
            ) == 0 { return 0; }
        }
        YAML_MAPPING_NODE => {
            if !stack_empty((*parent).data.mapping.pairs.start, (*parent).data.mapping.pairs.top) {
                let p = (*parent).data.mapping.pairs.top.sub(1);
                if (*p).key != 0 && (*p).value == 0 {
                    (*p).value = index;
                    return 1;
                }
            }
            let pair = yaml_node_pair_t { key: index, value: 0 };
            if stack_limit(
                &mut (*parser).error,
                (*parent).data.mapping.pairs.top,
                (*parent).data.mapping.pairs.start,
                (i32::MAX - 1) as isize,
            ) == 0 { return 0; }
            if stack_push(
                &mut (*parser).error,
                &mut (*parent).data.mapping.pairs.start,
                &mut (*parent).data.mapping.pairs.top,
                &mut (*parent).data.mapping.pairs.end,
                pair,
            ) == 0 { return 0; }
        }
        _ => {
            debug_assert!(false);
            return 0;
        }
    }
    1
}

unsafe fn yaml_parser_load_alias(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> i32 {
    let anchor = (*event).data.alias.anchor;
    let mut alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if libc::strcmp((*alias_data).anchor as *const c_char, anchor as *const c_char) == 0 {
            yaml_free(anchor as *mut c_void);
            return yaml_parser_load_node_add(parser, ctx, (*alias_data).index);
        }
        alias_data = alias_data.add(1);
    }

    yaml_free(anchor as *mut c_void);
    yaml_parser_loader_set_composer_error(
        parser,
        b"found undefined alias\0".as_ptr() as *const c_char,
        (*event).start_mark,
    )
}

unsafe fn yaml_parser_load_scalar(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> i32 {
    let mut tag = (*event).data.scalar.tag;

    if stack_limit(
        &mut (*parser).error,
        (*(*parser).document).nodes.top,
        (*(*parser).document).nodes.start,
        (i32::MAX - 1) as isize,
    ) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.scalar.anchor as *mut c_void);
        yaml_free((*event).data.scalar.value as *mut c_void);
        return 0;
    }

    if tag.is_null() || libc::strcmp(tag as *const c_char, b"!\0".as_ptr() as *const c_char) == 0 {
        yaml_free(tag as *mut c_void);
        tag = yaml_strdup(YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const yaml_char_t);
        if tag.is_null() {
            yaml_free((*event).data.scalar.anchor as *mut c_void);
            yaml_free((*event).data.scalar.value as *mut c_void);
            return 0;
        }
    }

    let mut node: yaml_node_t = core::mem::zeroed();
    node.type_ = YAML_SCALAR_NODE;
    node.tag = tag;
    node.data.scalar.value = (*event).data.scalar.value;
    node.data.scalar.length = (*event).data.scalar.length;
    node.data.scalar.style = (*event).data.scalar.style;
    node.start_mark = (*event).start_mark;
    node.end_mark = (*event).end_mark;

    if stack_push(
        &mut (*parser).error,
        &mut (*(*parser).document).nodes.start,
        &mut (*(*parser).document).nodes.top,
        &mut (*(*parser).document).nodes.end,
        node,
    ) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.scalar.anchor as *mut c_void);
        yaml_free((*event).data.scalar.value as *mut c_void);
        return 0;
    }

    let index = (*(*parser).document).nodes.top
        .offset_from((*(*parser).document).nodes.start) as i32;

    if yaml_parser_register_anchor(parser, index, (*event).data.scalar.anchor) == 0 {
        return 0;
    }

    yaml_parser_load_node_add(parser, ctx, index)
}

unsafe fn yaml_parser_load_sequence(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> i32 {
    let mut tag = (*event).data.sequence_start.tag;

    if stack_limit(
        &mut (*parser).error,
        (*(*parser).document).nodes.top,
        (*(*parser).document).nodes.start,
        (i32::MAX - 1) as isize,
    ) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.sequence_start.anchor as *mut c_void);
        return 0;
    }

    if tag.is_null() || libc::strcmp(tag as *const c_char, b"!\0".as_ptr() as *const c_char) == 0 {
        yaml_free(tag as *mut c_void);
        tag = yaml_strdup(YAML_DEFAULT_SEQUENCE_TAG.as_ptr() as *const yaml_char_t);
        if tag.is_null() {
            yaml_free((*event).data.sequence_start.anchor as *mut c_void);
            return 0;
        }
    }

    let mut items_start: *mut yaml_node_item_t = ptr::null_mut();
    let mut items_top: *mut yaml_node_item_t = ptr::null_mut();
    let mut items_end: *mut yaml_node_item_t = ptr::null_mut();
    if stack_init(&mut (*parser).error, &mut items_start, &mut items_top, &mut items_end) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.sequence_start.anchor as *mut c_void);
        return 0;
    }

    let mut node: yaml_node_t = core::mem::zeroed();
    node.type_ = YAML_SEQUENCE_NODE;
    node.tag = tag;
    node.data.sequence.items.start = items_start;
    node.data.sequence.items.end = items_end;
    node.data.sequence.items.top = items_top;
    node.data.sequence.style = (*event).data.sequence_start.style;
    node.start_mark = (*event).start_mark;
    node.end_mark = (*event).end_mark;

    if stack_push(
        &mut (*parser).error,
        &mut (*(*parser).document).nodes.start,
        &mut (*(*parser).document).nodes.top,
        &mut (*(*parser).document).nodes.end,
        node,
    ) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.sequence_start.anchor as *mut c_void);
        return 0;
    }

    let index = (*(*parser).document).nodes.top
        .offset_from((*(*parser).document).nodes.start) as i32;

    if yaml_parser_register_anchor(parser, index, (*event).data.sequence_start.anchor) == 0 {
        return 0;
    }

    if yaml_parser_load_node_add(parser, ctx, index) == 0 { return 0; }

    if stack_limit(
        &mut (*parser).error,
        (*ctx).top,
        (*ctx).start,
        (i32::MAX - 1) as isize,
    ) == 0 { return 0; }
    if stack_push(&mut (*parser).error, &mut (*ctx).start, &mut (*ctx).top, &mut (*ctx).end, index) == 0 {
        return 0;
    }
    1
}

unsafe fn yaml_parser_load_sequence_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> i32 {
    debug_assert!((*ctx).top.offset_from((*ctx).start) > 0);
    let index = *((*ctx).top.sub(1));
    debug_assert!((*(*(*parser).document).nodes.start.add(index as usize - 1)).type_ == YAML_SEQUENCE_NODE);
    (*(*(*parser).document).nodes.start.add(index as usize - 1)).end_mark = (*event).end_mark;
    let _ = stack_pop(&mut (*ctx).top);
    1
}

unsafe fn yaml_parser_load_mapping(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> i32 {
    let mut tag = (*event).data.mapping_start.tag;

    if stack_limit(
        &mut (*parser).error,
        (*(*parser).document).nodes.top,
        (*(*parser).document).nodes.start,
        (i32::MAX - 1) as isize,
    ) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.mapping_start.anchor as *mut c_void);
        return 0;
    }

    if tag.is_null() || libc::strcmp(tag as *const c_char, b"!\0".as_ptr() as *const c_char) == 0 {
        yaml_free(tag as *mut c_void);
        tag = yaml_strdup(YAML_DEFAULT_MAPPING_TAG.as_ptr() as *const yaml_char_t);
        if tag.is_null() {
            yaml_free((*event).data.mapping_start.anchor as *mut c_void);
            return 0;
        }
    }

    let mut pairs_start: *mut yaml_node_pair_t = ptr::null_mut();
    let mut pairs_top: *mut yaml_node_pair_t = ptr::null_mut();
    let mut pairs_end: *mut yaml_node_pair_t = ptr::null_mut();
    if stack_init(&mut (*parser).error, &mut pairs_start, &mut pairs_top, &mut pairs_end) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.mapping_start.anchor as *mut c_void);
        return 0;
    }

    let mut node: yaml_node_t = core::mem::zeroed();
    node.type_ = YAML_MAPPING_NODE;
    node.tag = tag;
    node.data.mapping.pairs.start = pairs_start;
    node.data.mapping.pairs.end = pairs_end;
    node.data.mapping.pairs.top = pairs_top;
    node.data.mapping.style = (*event).data.mapping_start.style;
    node.start_mark = (*event).start_mark;
    node.end_mark = (*event).end_mark;

    if stack_push(
        &mut (*parser).error,
        &mut (*(*parser).document).nodes.start,
        &mut (*(*parser).document).nodes.top,
        &mut (*(*parser).document).nodes.end,
        node,
    ) == 0 {
        yaml_free(tag as *mut c_void);
        yaml_free((*event).data.mapping_start.anchor as *mut c_void);
        return 0;
    }

    let index = (*(*parser).document).nodes.top
        .offset_from((*(*parser).document).nodes.start) as i32;

    if yaml_parser_register_anchor(parser, index, (*event).data.mapping_start.anchor) == 0 {
        return 0;
    }

    if yaml_parser_load_node_add(parser, ctx, index) == 0 { return 0; }

    if stack_limit(
        &mut (*parser).error,
        (*ctx).top,
        (*ctx).start,
        (i32::MAX - 1) as isize,
    ) == 0 { return 0; }
    if stack_push(&mut (*parser).error, &mut (*ctx).start, &mut (*ctx).top, &mut (*ctx).end, index) == 0 {
        return 0;
    }
    1
}

unsafe fn yaml_parser_load_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> i32 {
    debug_assert!((*ctx).top.offset_from((*ctx).start) > 0);
    let index = *((*ctx).top.sub(1));
    debug_assert!((*(*(*parser).document).nodes.start.add(index as usize - 1)).type_ == YAML_MAPPING_NODE);
    (*(*(*parser).document).nodes.start.add(index as usize - 1)).end_mark = (*event).end_mark;
    let _ = stack_pop(&mut (*ctx).top);
    1
}


// ============================================================
// scanner.c — YAML token scanner
// ============================================================

// --- Scanner inline macro helpers ---

unsafe fn scanner_cache(parser: *mut yaml_parser_t, length: usize) -> i32 {
    if (*parser).unread >= length { 1 }
    else { yaml_parser_update_buffer(parser, length) }
}

unsafe fn scanner_buf_width(pointer: *const yaml_char_t) -> usize {
    let c = *pointer;
    if (c & 0x80) == 0x00 { 1 }
    else if (c & 0xE0) == 0xC0 { 2 }
    else if (c & 0xF0) == 0xE0 { 3 }
    else if (c & 0xF8) == 0xF0 { 4 }
    else { 0 }
}

unsafe fn scanner_skip(parser: *mut yaml_parser_t) {
    let w = scanner_buf_width((*parser).buffer.pointer);
    (*parser).mark.index += 1;
    (*parser).mark.column += 1;
    (*parser).unread -= 1;
    (*parser).buffer.pointer = (*parser).buffer.pointer.add(w);
}

unsafe fn scanner_skip_line(parser: *mut yaml_parser_t) {
    let p = (*parser).buffer.pointer;
    if *p == b'\r' && *p.add(1) == b'\n' {
        (*parser).mark.index += 2;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 2;
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
    } else if *p == b'\r' || *p == b'\n' {
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
    } else if *p == 0xC2 && *p.add(1) == 0x85 {
        // NEL
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
    } else if *p == 0xE2 && *p.add(1) == 0x80 && (*p.add(2) == 0xA8 || *p.add(2) == 0xA9) {
        // LS or PS
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(3);
    }
}

// COPY — copy UTF-8 char from buffer to string, advance both pointers
unsafe fn scanner_copy(string: *mut yaml_string_t, parser: *mut yaml_parser_t) {
    let w = scanner_buf_width((*parser).buffer.pointer);
    for i in 0..w {
        *(*string).pointer = *(*parser).buffer.pointer.add(i);
        (*string).pointer = (*string).pointer.add(1);
    }
    (*parser).buffer.pointer = (*parser).buffer.pointer.add(w);
}

// READ — STRING_EXTEND then COPY + mark advancement
unsafe fn scanner_read(parser: *mut yaml_parser_t, string: *mut yaml_string_t) -> i32 {
    if string_extend_check(&mut (*parser).error, string) == 0 { return 0; }
    scanner_copy(string, parser);
    (*parser).mark.index += 1;
    (*parser).mark.column += 1;
    (*parser).unread -= 1;
    1
}

// READ_LINE — copy line break to string, normalized (CR LF -> LF, CR -> LF, LF -> LF, NEL -> LF, LS/PS preserved)
unsafe fn scanner_read_line(parser: *mut yaml_parser_t, string: *mut yaml_string_t) -> i32 {
    if string_extend_check(&mut (*parser).error, string) == 0 { return 0; }
    let p = (*parser).buffer.pointer;
    if *p == b'\r' && *p.add(1) == b'\n' {
        // CR LF -> LF
        *(*string).pointer = b'\n';
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
        (*parser).mark.index += 2;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 2;
    } else if *p == b'\r' || *p == b'\n' {
        // CR or LF -> LF
        *(*string).pointer = b'\n';
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(1);
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
    } else if *p == 0xC2 && *p.add(1) == 0x85 {
        // NEL -> LF
        *(*string).pointer = b'\n';
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(2);
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
    } else if *p == 0xE2 && *p.add(1) == 0x80 && (*p.add(2) == 0xA8 || *p.add(2) == 0xA9) {
        // LS or PS -> preserve as-is
        *(*string).pointer = *p;
        (*string).pointer = (*string).pointer.add(1);
        *(*string).pointer = *p.add(1);
        (*string).pointer = (*string).pointer.add(1);
        *(*string).pointer = *p.add(2);
        (*string).pointer = (*string).pointer.add(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.add(3);
        (*parser).mark.index += 1;
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
        (*parser).unread -= 1;
    }
    1
}

// --- Scanner character check helpers (operating on parser->buffer.pointer) ---

#[inline(always)]
unsafe fn pbuf_is_z(parser: *const yaml_parser_t) -> bool {
    *(*parser).buffer.pointer == 0
}

#[inline(always)]
unsafe fn pbuf_check(parser: *const yaml_parser_t, c: u8) -> bool {
    *(*parser).buffer.pointer == c
}

#[inline(always)]
unsafe fn pbuf_check_at(parser: *const yaml_parser_t, c: u8, n: usize) -> bool {
    *(*parser).buffer.pointer.add(n) == c
}

#[inline(always)]
unsafe fn pbuf_is_alpha(parser: *const yaml_parser_t) -> bool {
    let c = *(*parser).buffer.pointer;
    (c >= b'0' && c <= b'9') || (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
        || c == b'_' || c == b'-'
}

#[inline(always)]
unsafe fn pbuf_is_digit(parser: *const yaml_parser_t) -> bool {
    let c = *(*parser).buffer.pointer;
    c >= b'0' && c <= b'9'
}

#[inline(always)]
unsafe fn pbuf_as_digit(parser: *const yaml_parser_t) -> i32 {
    (*(*parser).buffer.pointer - b'0') as i32
}

#[inline(always)]
unsafe fn pbuf_is_hex_at(parser: *const yaml_parser_t, n: usize) -> bool {
    let c = *(*parser).buffer.pointer.add(n);
    (c >= b'0' && c <= b'9') || (c >= b'A' && c <= b'F') || (c >= b'a' && c <= b'f')
}

#[inline(always)]
unsafe fn pbuf_as_hex_at(parser: *const yaml_parser_t, n: usize) -> u32 {
    let c = *(*parser).buffer.pointer.add(n);
    if c >= b'A' && c <= b'F' { (c - b'A' + 10) as u32 }
    else if c >= b'a' && c <= b'f' { (c - b'a' + 10) as u32 }
    else { (c - b'0') as u32 }
}

#[inline(always)]
unsafe fn pbuf_is_printable(parser: *const yaml_parser_t) -> bool {
    let p = (*parser).buffer.pointer;
    let c = *p;
    if (c & 0x80) == 0 {
        c == b'\t' || c == b'\n' || c == b'\r' || (c >= 0x20 && c <= 0x7E)
    } else if (c & 0xE0) == 0xC0 {
        let c2 = *p.add(1);
        let v = (((c & 0x1F) as u32) << 6) | ((c2 & 0x3F) as u32);
        v == 0x85 || v >= 0xA0
    } else if (c & 0xF0) == 0xE0 {
        let c2 = *p.add(1);
        let c3 = *p.add(2);
        let v = (((c & 0x0F) as u32) << 12) | (((c2 & 0x3F) as u32) << 6) | ((c3 & 0x3F) as u32);
        (v >= 0xA0 && v <= 0xD7FF) || (v >= 0xE000 && v <= 0xFFFD && v != 0xFEFF)
    } else if (c & 0xF8) == 0xF0 {
        let c2 = *p.add(1);
        let c3 = *p.add(2);
        let c4 = *p.add(3);
        let v = (((c & 0x07) as u32) << 18) | (((c2 & 0x3F) as u32) << 12) | (((c3 & 0x3F) as u32) << 6) | ((c4 & 0x3F) as u32);
        v >= 0x10000 && v <= 0x10FFFF
    } else { false }
}

#[inline(always)]
unsafe fn pbuf_is_bom(parser: *const yaml_parser_t) -> bool {
    let p = (*parser).buffer.pointer;
    *p == 0xEF && *p.add(1) == 0xBB && *p.add(2) == 0xBF
}

#[inline(always)]
unsafe fn pbuf_is_space(parser: *const yaml_parser_t) -> bool {
    *(*parser).buffer.pointer == b' '
}

#[inline(always)]
unsafe fn pbuf_is_tab(parser: *const yaml_parser_t) -> bool {
    *(*parser).buffer.pointer == b'\t'
}

#[inline(always)]
unsafe fn pbuf_is_blank(parser: *const yaml_parser_t) -> bool {
    let c = *(*parser).buffer.pointer;
    c == b' ' || c == b'\t'
}

#[inline(always)]
unsafe fn pbuf_is_blank_at(parser: *const yaml_parser_t, n: usize) -> bool {
    let c = *(*parser).buffer.pointer.add(n);
    c == b' ' || c == b'\t'
}

#[inline(always)]
unsafe fn pbuf_is_break(parser: *const yaml_parser_t) -> bool {
    let p = (*parser).buffer.pointer;
    let c = *p;
    c == b'\r' || c == b'\n'
        || (c == 0xC2 && *p.add(1) == 0x85)
        || (c == 0xE2 && *p.add(1) == 0x80 && (*p.add(2) == 0xA8 || *p.add(2) == 0xA9))
}

#[inline(always)]
unsafe fn pbuf_is_break_at(parser: *const yaml_parser_t, n: usize) -> bool {
    let p = (*parser).buffer.pointer.add(n);
    let c = *p;
    c == b'\r' || c == b'\n'
        || (c == 0xC2 && *p.add(1) == 0x85)
        || (c == 0xE2 && *p.add(1) == 0x80 && (*p.add(2) == 0xA8 || *p.add(2) == 0xA9))
}

#[inline(always)]
unsafe fn pbuf_is_crlf(parser: *const yaml_parser_t) -> bool {
    let p = (*parser).buffer.pointer;
    *p == b'\r' && *p.add(1) == b'\n'
}

#[inline(always)]
unsafe fn pbuf_is_breakz(parser: *const yaml_parser_t) -> bool {
    pbuf_is_z(parser) || pbuf_is_break(parser)
}

#[inline(always)]
unsafe fn pbuf_is_spacez(parser: *const yaml_parser_t) -> bool {
    pbuf_is_space(parser) || pbuf_is_breakz(parser)
}

#[inline(always)]
unsafe fn pbuf_is_blankz(parser: *const yaml_parser_t) -> bool {
    pbuf_is_blank(parser) || pbuf_is_breakz(parser)
}

#[inline(always)]
unsafe fn pbuf_is_blankz_at(parser: *const yaml_parser_t, n: usize) -> bool {
    let p = (*parser).buffer.pointer.add(n);
    let c = *p;
    if c == 0 { return true; }
    if c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' { return true; }
    if c == 0xC2 && *p.add(1) == 0x85 { return true; }
    if c == 0xE2 && *p.add(1) == 0x80 && (*p.add(2) == 0xA8 || *p.add(2) == 0xA9) { return true; }
    false
}

#[inline(always)]
unsafe fn pbuf_width(parser: *const yaml_parser_t) -> usize {
    scanner_buf_width((*parser).buffer.pointer)
}

// --- Token init helpers ---

unsafe fn scanner_token_init(
    token: *mut yaml_token_t,
    type_: yaml_token_type_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    libc::memset(token as *mut c_void, 0, core::mem::size_of::<yaml_token_t>());
    (*token).type_ = type_;
    (*token).start_mark = start_mark;
    (*token).end_mark = end_mark;
}

unsafe fn scanner_stream_start_token_init(
    token: *mut yaml_token_t,
    encoding: yaml_encoding_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_STREAM_START_TOKEN, start_mark, end_mark);
    (*token).data.stream_start.encoding = encoding;
}

unsafe fn scanner_stream_end_token_init(
    token: *mut yaml_token_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_STREAM_END_TOKEN, start_mark, end_mark);
}

unsafe fn scanner_version_directive_token_init(
    token: *mut yaml_token_t,
    major: i32,
    minor: i32,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_VERSION_DIRECTIVE_TOKEN, start_mark, end_mark);
    (*token).data.version_directive.major = major;
    (*token).data.version_directive.minor = minor;
}

unsafe fn scanner_tag_directive_token_init(
    token: *mut yaml_token_t,
    handle: *mut yaml_char_t,
    prefix: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_TAG_DIRECTIVE_TOKEN, start_mark, end_mark);
    (*token).data.tag_directive.handle = handle;
    (*token).data.tag_directive.prefix = prefix;
}

unsafe fn scanner_alias_token_init(
    token: *mut yaml_token_t,
    value: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_ALIAS_TOKEN, start_mark, end_mark);
    (*token).data.alias.value = value;
}

unsafe fn scanner_anchor_token_init(
    token: *mut yaml_token_t,
    value: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_ANCHOR_TOKEN, start_mark, end_mark);
    (*token).data.anchor.value = value;
}

unsafe fn scanner_tag_token_init(
    token: *mut yaml_token_t,
    handle: *mut yaml_char_t,
    suffix: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_TAG_TOKEN, start_mark, end_mark);
    (*token).data.tag.handle = handle;
    (*token).data.tag.suffix = suffix;
}

unsafe fn scanner_scalar_token_init(
    token: *mut yaml_token_t,
    value: *mut yaml_char_t,
    length: usize,
    style: yaml_scalar_style_t,
    start_mark: yaml_mark_t,
    end_mark: yaml_mark_t,
) {
    scanner_token_init(token, YAML_SCALAR_TOKEN, start_mark, end_mark);
    (*token).data.scalar.value = value;
    (*token).data.scalar.length = length;
    (*token).data.scalar.style = style;
}

// ============================================================
// yaml_parser_scan — exported
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_scan(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> i32 {
    debug_assert!(!parser.is_null());
    debug_assert!(!token.is_null());

    libc::memset(token as *mut c_void, 0, core::mem::size_of::<yaml_token_t>());

    if (*parser).stream_end_produced != 0 || (*parser).error != YAML_NO_ERROR {
        return 1;
    }

    if (*parser).token_available == 0 {
        if yaml_parser_fetch_more_tokens(parser) == 0 {
            return 0;
        }
    }

    *token = queue_dequeue_token(&mut (*parser).tokens);
    (*parser).token_available = 0;
    (*parser).tokens_parsed += 1;

    if (*token).type_ == YAML_STREAM_END_TOKEN {
        (*parser).stream_end_produced = 1;
    }

    1
}

unsafe fn queue_dequeue_token(q: *mut yaml_parser_tokens_t) -> yaml_token_t {
    let t = *(*q).head;
    (*q).head = (*q).head.add(1);
    t
}

// ============================================================
// yaml_parser_set_scanner_error — static
// ============================================================

unsafe fn yaml_parser_set_scanner_error(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
) -> i32 {
    (*parser).error = YAML_SCANNER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = (*parser).mark;
    0
}

// ============================================================
// yaml_parser_fetch_more_tokens — exported
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_fetch_more_tokens(parser: *mut yaml_parser_t) -> i32 {
    loop {
        let mut need_more_tokens: i32 = 0;

        if (*parser).tokens.head == (*parser).tokens.tail {
            need_more_tokens = 1;
        } else {
            if yaml_parser_stale_simple_keys(parser) == 0 { return 0; }
            let mut sk = (*parser).simple_keys.start;
            while sk != (*parser).simple_keys.top {
                if (*sk).possible != 0
                    && (*sk).token_number == (*parser).tokens_parsed
                {
                    need_more_tokens = 1;
                    break;
                }
                sk = sk.add(1);
            }
        }

        if need_more_tokens == 0 { break; }

        if yaml_parser_fetch_next_token(parser) == 0 { return 0; }
    }

    (*parser).token_available = 1;
    1
}

// ============================================================
// yaml_parser_fetch_next_token — static
// ============================================================

unsafe fn yaml_parser_fetch_next_token(parser: *mut yaml_parser_t) -> i32 {
    if scanner_cache(parser, 1) == 0 { return 0; }

    if (*parser).stream_start_produced == 0 {
        return yaml_parser_fetch_stream_start(parser);
    }

    if yaml_parser_scan_to_next_token(parser) == 0 { return 0; }
    if yaml_parser_stale_simple_keys(parser) == 0 { return 0; }
    if yaml_parser_unroll_indent(parser, (*parser).mark.column as isize) == 0 { return 0; }

    if scanner_cache(parser, 4) == 0 { return 0; }

    if pbuf_is_z(parser) {
        return yaml_parser_fetch_stream_end(parser);
    }

    if (*parser).mark.column == 0 && pbuf_check(parser, b'%') {
        return yaml_parser_fetch_directive(parser);
    }

    if (*parser).mark.column == 0
        && pbuf_check_at(parser, b'-', 0)
        && pbuf_check_at(parser, b'-', 1)
        && pbuf_check_at(parser, b'-', 2)
        && pbuf_is_blankz_at(parser, 3)
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }

    if (*parser).mark.column == 0
        && pbuf_check_at(parser, b'.', 0)
        && pbuf_check_at(parser, b'.', 1)
        && pbuf_check_at(parser, b'.', 2)
        && pbuf_is_blankz_at(parser, 3)
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }

    if pbuf_check(parser, b'[') {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_SEQUENCE_START_TOKEN);
    }
    if pbuf_check(parser, b'{') {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_MAPPING_START_TOKEN);
    }
    if pbuf_check(parser, b']') {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_SEQUENCE_END_TOKEN);
    }
    if pbuf_check(parser, b'}') {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_MAPPING_END_TOKEN);
    }
    if pbuf_check(parser, b',') {
        return yaml_parser_fetch_flow_entry(parser);
    }
    if pbuf_check(parser, b'-') && pbuf_is_blankz_at(parser, 1) {
        return yaml_parser_fetch_block_entry(parser);
    }
    if pbuf_check(parser, b'?')
        && ((*parser).flow_level != 0 || pbuf_is_blankz_at(parser, 1))
    {
        return yaml_parser_fetch_key(parser);
    }
    if pbuf_check(parser, b':')
        && ((*parser).flow_level != 0 || pbuf_is_blankz_at(parser, 1))
    {
        return yaml_parser_fetch_value(parser);
    }
    if pbuf_check(parser, b'*') {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }
    if pbuf_check(parser, b'&') {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }
    if pbuf_check(parser, b'!') {
        return yaml_parser_fetch_tag(parser);
    }
    if pbuf_check(parser, b'|') && (*parser).flow_level == 0 {
        return yaml_parser_fetch_block_scalar(parser, 1);
    }
    if pbuf_check(parser, b'>') && (*parser).flow_level == 0 {
        return yaml_parser_fetch_block_scalar(parser, 0);
    }
    if pbuf_check(parser, b'\'') {
        return yaml_parser_fetch_flow_scalar(parser, 1);
    }
    if pbuf_check(parser, b'"') {
        return yaml_parser_fetch_flow_scalar(parser, 0);
    }

    if !(pbuf_is_blankz(parser) || pbuf_check(parser, b'-')
        || pbuf_check(parser, b'?') || pbuf_check(parser, b':')
        || pbuf_check(parser, b',') || pbuf_check(parser, b'[')
        || pbuf_check(parser, b']') || pbuf_check(parser, b'{')
        || pbuf_check(parser, b'}') || pbuf_check(parser, b'#')
        || pbuf_check(parser, b'&') || pbuf_check(parser, b'*')
        || pbuf_check(parser, b'!') || pbuf_check(parser, b'|')
        || pbuf_check(parser, b'>') || pbuf_check(parser, b'\'')
        || pbuf_check(parser, b'"') || pbuf_check(parser, b'%')
        || pbuf_check(parser, b'@') || pbuf_check(parser, b'`'))
        || (pbuf_check(parser, b'-') && !pbuf_is_blank_at(parser, 1))
        || ((*parser).flow_level == 0
            && (pbuf_check(parser, b'?') || pbuf_check(parser, b':'))
            && !pbuf_is_blankz_at(parser, 1))
    {
        return yaml_parser_fetch_plain_scalar(parser);
    }

    yaml_parser_set_scanner_error(
        parser,
        b"while scanning for the next token\0".as_ptr() as *const c_char,
        (*parser).mark,
        b"found character that cannot start any token\0".as_ptr() as *const c_char,
    )
}

// ============================================================
// yaml_parser_stale_simple_keys — static
// ============================================================

unsafe fn yaml_parser_stale_simple_keys(parser: *mut yaml_parser_t) -> i32 {
    let mut simple_key = (*parser).simple_keys.start;
    while simple_key != (*parser).simple_keys.top {
        if (*simple_key).possible != 0
            && ((*simple_key).mark.line < (*parser).mark.line
                || (*simple_key).mark.index + 1024 < (*parser).mark.index)
        {
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

// ============================================================
// yaml_parser_save_simple_key — static
// ============================================================

unsafe fn yaml_parser_save_simple_key(parser: *mut yaml_parser_t) -> i32 {
    let required = ((*parser).flow_level == 0
        && (*parser).indent == (*parser).mark.column as i32) as i32;

    if (*parser).simple_key_allowed != 0 {
        let simple_key = yaml_simple_key_t {
            possible: 1,
            required,
            token_number: (*parser).tokens_parsed
                + (*parser).tokens.tail.offset_from((*parser).tokens.head) as usize,
            mark: (*parser).mark,
        };

        if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

        *((*parser).simple_keys.top.sub(1)) = simple_key;
    }
    1
}

// ============================================================
// yaml_parser_remove_simple_key — static
// ============================================================

unsafe fn yaml_parser_remove_simple_key(parser: *mut yaml_parser_t) -> i32 {
    let simple_key = (*parser).simple_keys.top.sub(1);

    if (*simple_key).possible != 0 {
        if (*simple_key).required != 0 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a simple key\0".as_ptr() as *const c_char,
                (*simple_key).mark,
                b"could not find expected ':'\0".as_ptr() as *const c_char,
            );
        }
    }

    (*simple_key).possible = 0;
    1
}

// ============================================================
// yaml_parser_increase_flow_level — static
// ============================================================

unsafe fn yaml_parser_increase_flow_level(parser: *mut yaml_parser_t) -> i32 {
    let empty_simple_key = yaml_simple_key_t {
        possible: 0,
        required: 0,
        token_number: 0,
        mark: yaml_mark_t { index: 0, line: 0, column: 0 },
    };

    if stack_push(
        &mut (*parser).error,
        &mut (*parser).simple_keys.start,
        &mut (*parser).simple_keys.top,
        &mut (*parser).simple_keys.end,
        empty_simple_key,
    ) == 0
    {
        return 0;
    }

    if (*parser).flow_level == i32::MAX {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0;
    }

    (*parser).flow_level += 1;
    1
}

// ============================================================
// yaml_parser_decrease_flow_level — static
// ============================================================

unsafe fn yaml_parser_decrease_flow_level(parser: *mut yaml_parser_t) -> i32 {
    if (*parser).flow_level != 0 {
        (*parser).flow_level -= 1;
        let _ = stack_pop(&mut (*parser).simple_keys.top);
    }
    1
}

// ============================================================
// yaml_parser_roll_indent — static
// ============================================================

unsafe fn yaml_parser_roll_indent(
    parser: *mut yaml_parser_t,
    column: isize,
    number: isize,
    type_: yaml_token_type_t,
    mark: yaml_mark_t,
) -> i32 {
    if (*parser).flow_level != 0 { return 1; }

    if ((*parser).indent as isize) < column {
        if stack_push(
            &mut (*parser).error,
            &mut (*parser).indents.start,
            &mut (*parser).indents.top,
            &mut (*parser).indents.end,
            (*parser).indent,
        ) == 0
        {
            return 0;
        }

        if column > i32::MAX as isize {
            (*parser).error = YAML_MEMORY_ERROR;
            return 0;
        }

        (*parser).indent = column as i32;

        let mut token: yaml_token_t = core::mem::zeroed();
        scanner_token_init(&mut token, type_, mark, mark);

        if number == -1 {
            if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
                return 0;
            }
        } else {
            if queue_insert_token(
                &mut (*parser).tokens,
                (number - (*parser).tokens_parsed as isize) as usize,
                token,
                &mut (*parser).error,
            ) == 0
            {
                return 0;
            }
        }
    }
    1
}

unsafe fn queue_enqueue_token(
    q: *mut yaml_parser_tokens_t,
    value: yaml_token_t,
    error: *mut yaml_error_type_t,
) -> i32 {
    queue_enqueue(error, &mut (*q).start, &mut (*q).head, &mut (*q).tail, &mut (*q).end, value)
}

unsafe fn queue_insert_token(
    q: *mut yaml_parser_tokens_t,
    index: usize,
    value: yaml_token_t,
    error: *mut yaml_error_type_t,
) -> i32 {
    queue_insert(error, &mut (*q).start, &mut (*q).head, &mut (*q).tail, &mut (*q).end, index, value)
}

// ============================================================
// yaml_parser_unroll_indent — static
// ============================================================

unsafe fn yaml_parser_unroll_indent(parser: *mut yaml_parser_t, column: isize) -> i32 {
    if (*parser).flow_level != 0 { return 1; }

    while ((*parser).indent as isize) > column {
        let mut token: yaml_token_t = core::mem::zeroed();
        scanner_token_init(&mut token, YAML_BLOCK_END_TOKEN, (*parser).mark, (*parser).mark);
        if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
            return 0;
        }
        (*parser).indent = stack_pop(&mut (*parser).indents.top);
    }
    1
}

// ============================================================
// yaml_parser_fetch_stream_start — static
// ============================================================

unsafe fn yaml_parser_fetch_stream_start(parser: *mut yaml_parser_t) -> i32 {
    let simple_key = yaml_simple_key_t {
        possible: 0,
        required: 0,
        token_number: 0,
        mark: yaml_mark_t { index: 0, line: 0, column: 0 },
    };

    (*parser).indent = -1;

    if stack_push(
        &mut (*parser).error,
        &mut (*parser).simple_keys.start,
        &mut (*parser).simple_keys.top,
        &mut (*parser).simple_keys.end,
        simple_key,
    ) == 0
    {
        return 0;
    }

    (*parser).simple_key_allowed = 1;
    (*parser).stream_start_produced = 1;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_stream_start_token_init(&mut token, (*parser).encoding, (*parser).mark, (*parser).mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_stream_end — static
// ============================================================

unsafe fn yaml_parser_fetch_stream_end(parser: *mut yaml_parser_t) -> i32 {
    if (*parser).mark.column != 0 {
        (*parser).mark.column = 0;
        (*parser).mark.line += 1;
    }

    if yaml_parser_unroll_indent(parser, -1) == 0 { return 0; }
    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_stream_end_token_init(&mut token, (*parser).mark, (*parser).mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_directive — static
// ============================================================

unsafe fn yaml_parser_fetch_directive(parser: *mut yaml_parser_t) -> i32 {
    if yaml_parser_unroll_indent(parser, -1) == 0 { return 0; }
    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let mut token: yaml_token_t = core::mem::zeroed();
    if yaml_parser_scan_directive(parser, &mut token) == 0 { return 0; }

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        yaml_token_delete(&mut token);
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_document_indicator — static
// ============================================================

unsafe fn yaml_parser_fetch_document_indicator(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> i32 {
    if yaml_parser_unroll_indent(parser, -1) == 0 { return 0; }
    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    scanner_skip(parser);
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, type_, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_flow_collection_start — static
// ============================================================

unsafe fn yaml_parser_fetch_flow_collection_start(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> i32 {
    if yaml_parser_save_simple_key(parser) == 0 { return 0; }
    if yaml_parser_increase_flow_level(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 1;

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, type_, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_flow_collection_end — static
// ============================================================

unsafe fn yaml_parser_fetch_flow_collection_end(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> i32 {
    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }
    if yaml_parser_decrease_flow_level(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, type_, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_flow_entry — static
// ============================================================

unsafe fn yaml_parser_fetch_flow_entry(parser: *mut yaml_parser_t) -> i32 {
    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 1;

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, YAML_FLOW_ENTRY_TOKEN, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_block_entry — static
// ============================================================

unsafe fn yaml_parser_fetch_block_entry(parser: *mut yaml_parser_t) -> i32 {
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                core::ptr::null(),
                (*parser).mark,
                b"block sequence entries are not allowed in this context\0".as_ptr() as *const c_char,
            );
        }

        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as isize,
            -1,
            YAML_BLOCK_SEQUENCE_START_TOKEN,
            (*parser).mark,
        ) == 0 { return 0; }
    }

    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 1;

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, YAML_BLOCK_ENTRY_TOKEN, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_key — static
// ============================================================

unsafe fn yaml_parser_fetch_key(parser: *mut yaml_parser_t) -> i32 {
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                core::ptr::null(),
                (*parser).mark,
                b"mapping keys are not allowed in this context\0".as_ptr() as *const c_char,
            );
        }

        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as isize,
            -1,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*parser).mark,
        ) == 0 { return 0; }
    }

    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = ((*parser).flow_level == 0) as i32;

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, YAML_KEY_TOKEN, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_value — static
// ============================================================

unsafe fn yaml_parser_fetch_value(parser: *mut yaml_parser_t) -> i32 {
    let simple_key = (*parser).simple_keys.top.sub(1);

    if (*simple_key).possible != 0 {
        let mut token: yaml_token_t = core::mem::zeroed();
        scanner_token_init(&mut token, YAML_KEY_TOKEN, (*simple_key).mark, (*simple_key).mark);

        let idx = (*simple_key).token_number - (*parser).tokens_parsed;
        if queue_insert_token(&mut (*parser).tokens, idx, token, &mut (*parser).error) == 0 {
            return 0;
        }

        if yaml_parser_roll_indent(
            parser,
            (*simple_key).mark.column as isize,
            (*simple_key).token_number as isize,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*simple_key).mark,
        ) == 0 { return 0; }

        (*simple_key).possible = 0;
        (*parser).simple_key_allowed = 0;
    } else {
        if (*parser).flow_level == 0 {
            if (*parser).simple_key_allowed == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    core::ptr::null(),
                    (*parser).mark,
                    b"mapping values are not allowed in this context\0".as_ptr() as *const c_char,
                );
            }

            if yaml_parser_roll_indent(
                parser,
                (*parser).mark.column as isize,
                -1,
                YAML_BLOCK_MAPPING_START_TOKEN,
                (*parser).mark,
            ) == 0 { return 0; }
        }

        (*parser).simple_key_allowed = ((*parser).flow_level == 0) as i32;
    }

    let start_mark = (*parser).mark;
    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let mut token: yaml_token_t = core::mem::zeroed();
    scanner_token_init(&mut token, YAML_VALUE_TOKEN, start_mark, end_mark);

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_anchor — static
// ============================================================

unsafe fn yaml_parser_fetch_anchor(parser: *mut yaml_parser_t, type_: yaml_token_type_t) -> i32 {
    if yaml_parser_save_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let mut token: yaml_token_t = core::mem::zeroed();
    if yaml_parser_scan_anchor(parser, &mut token, type_) == 0 { return 0; }

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        yaml_token_delete(&mut token);
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_tag — static
// ============================================================

unsafe fn yaml_parser_fetch_tag(parser: *mut yaml_parser_t) -> i32 {
    if yaml_parser_save_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let mut token: yaml_token_t = core::mem::zeroed();
    if yaml_parser_scan_tag(parser, &mut token) == 0 { return 0; }

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        yaml_token_delete(&mut token);
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_block_scalar — static
// ============================================================

unsafe fn yaml_parser_fetch_block_scalar(parser: *mut yaml_parser_t, literal: i32) -> i32 {
    if yaml_parser_remove_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 1;

    let mut token: yaml_token_t = core::mem::zeroed();
    if yaml_parser_scan_block_scalar(parser, &mut token, literal) == 0 { return 0; }

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        yaml_token_delete(&mut token);
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_flow_scalar — static
// ============================================================

unsafe fn yaml_parser_fetch_flow_scalar(parser: *mut yaml_parser_t, single: i32) -> i32 {
    if yaml_parser_save_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let mut token: yaml_token_t = core::mem::zeroed();
    if yaml_parser_scan_flow_scalar(parser, &mut token, single) == 0 { return 0; }

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        yaml_token_delete(&mut token);
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_fetch_plain_scalar — static
// ============================================================

unsafe fn yaml_parser_fetch_plain_scalar(parser: *mut yaml_parser_t) -> i32 {
    if yaml_parser_save_simple_key(parser) == 0 { return 0; }

    (*parser).simple_key_allowed = 0;

    let mut token: yaml_token_t = core::mem::zeroed();
    if yaml_parser_scan_plain_scalar(parser, &mut token) == 0 { return 0; }

    if queue_enqueue_token(&mut (*parser).tokens, token, &mut (*parser).error) == 0 {
        yaml_token_delete(&mut token);
        return 0;
    }
    1
}

// ============================================================
// yaml_parser_scan_to_next_token — static
// ============================================================

unsafe fn yaml_parser_scan_to_next_token(parser: *mut yaml_parser_t) -> i32 {
    loop {
        if scanner_cache(parser, 1) == 0 { return 0; }

        if (*parser).mark.column == 0 && pbuf_is_bom(parser) {
            scanner_skip(parser);
        }

        if scanner_cache(parser, 1) == 0 { return 0; }

        while pbuf_check(parser, b' ')
            || (((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && pbuf_check(parser, b'\t'))
        {
            scanner_skip(parser);
            if scanner_cache(parser, 1) == 0 { return 0; }
        }

        if pbuf_check(parser, b'#') {
            while !pbuf_is_breakz(parser) {
                scanner_skip(parser);
                if scanner_cache(parser, 1) == 0 { return 0; }
            }
        }

        if pbuf_is_break(parser) {
            if scanner_cache(parser, 2) == 0 { return 0; }
            scanner_skip_line(parser);

            if (*parser).flow_level == 0 {
                (*parser).simple_key_allowed = 1;
            }
        } else {
            break;
        }
    }
    1
}

// ============================================================
// yaml_parser_scan_directive — exported (not static in C)
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_parser_scan_directive(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> i32 {
    let start_mark = (*parser).mark;
    let mut name: *mut yaml_char_t = core::ptr::null_mut();
    let mut major: i32 = 0;
    let mut minor: i32 = 0;
    let mut handle: *mut yaml_char_t = core::ptr::null_mut();
    let mut prefix: *mut yaml_char_t = core::ptr::null_mut();

    // Eat '%'
    scanner_skip(parser);

    if yaml_parser_scan_directive_name(parser, start_mark, &mut name) == 0 {
        goto_scan_directive_error(parser, handle, prefix, name);
        return 0;
    }

    if libc::strcmp(name as *const c_char, b"YAML\0".as_ptr() as *const c_char) == 0 {
        if yaml_parser_scan_version_directive_value(parser, start_mark, &mut major, &mut minor) == 0 {
            goto_scan_directive_error(parser, handle, prefix, name);
            return 0;
        }
        let end_mark = (*parser).mark;
        scanner_version_directive_token_init(token, major, minor, start_mark, end_mark);
    } else if libc::strcmp(name as *const c_char, b"TAG\0".as_ptr() as *const c_char) == 0 {
        if yaml_parser_scan_tag_directive_value(parser, start_mark, &mut handle, &mut prefix) == 0 {
            goto_scan_directive_error(parser, handle, prefix, name);
            return 0;
        }
        let end_mark = (*parser).mark;
        scanner_tag_directive_token_init(token, handle, prefix, start_mark, end_mark);
    } else {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a directive\0".as_ptr() as *const c_char,
            start_mark,
            b"found unknown directive name\0".as_ptr() as *const c_char,
        );
        goto_scan_directive_error(parser, handle, prefix, name);
        return 0;
    }

    if scanner_cache(parser, 1) == 0 {
        goto_scan_directive_error(parser, handle, prefix, name);
        return 0;
    }

    while pbuf_is_blank(parser) {
        scanner_skip(parser);
        if scanner_cache(parser, 1) == 0 {
            goto_scan_directive_error(parser, handle, prefix, name);
            return 0;
        }
    }

    if pbuf_check(parser, b'#') {
        while !pbuf_is_breakz(parser) {
            scanner_skip(parser);
            if scanner_cache(parser, 1) == 0 {
                goto_scan_directive_error(parser, handle, prefix, name);
                return 0;
            }
        }
    }

    if !pbuf_is_breakz(parser) {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a directive\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected comment or line break\0".as_ptr() as *const c_char,
        );
        goto_scan_directive_error(parser, handle, prefix, name);
        return 0;
    }

    if pbuf_is_break(parser) {
        if scanner_cache(parser, 2) == 0 {
            goto_scan_directive_error(parser, handle, prefix, name);
            return 0;
        }
        scanner_skip_line(parser);
    }

    yaml_free(name as *mut c_void);
    1
}

unsafe fn goto_scan_directive_error(
    _parser: *mut yaml_parser_t,
    prefix: *mut yaml_char_t,
    handle: *mut yaml_char_t,
    name: *mut yaml_char_t,
) {
    yaml_free(prefix as *mut c_void);
    yaml_free(handle as *mut c_void);
    yaml_free(name as *mut c_void);
}

// ============================================================
// yaml_parser_scan_directive_name — static
// ============================================================

unsafe fn yaml_parser_scan_directive_name(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    name: *mut *mut yaml_char_t,
) -> i32 {
    let mut string: yaml_string_t = core::mem::zeroed();

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { return 0; }

    if scanner_cache(parser, 1) == 0 {
        string_del(&mut string);
        return 0;
    }

    while pbuf_is_alpha(parser) {
        if scanner_read(parser, &mut string) == 0 {
            string_del(&mut string);
            return 0;
        }
        if scanner_cache(parser, 1) == 0 {
            string_del(&mut string);
            return 0;
        }
    }

    if string.start == string.pointer {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a directive\0".as_ptr() as *const c_char,
            start_mark,
            b"could not find expected directive name\0".as_ptr() as *const c_char,
        );
        string_del(&mut string);
        return 0;
    }

    if !pbuf_is_blankz(parser) {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a directive\0".as_ptr() as *const c_char,
            start_mark,
            b"found unexpected non-alphabetical character\0".as_ptr() as *const c_char,
        );
        string_del(&mut string);
        return 0;
    }

    *name = string.start;
    1
}

// ============================================================
// yaml_parser_scan_version_directive_value — static
// ============================================================

unsafe fn yaml_parser_scan_version_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    major: *mut i32,
    minor: *mut i32,
) -> i32 {
    if scanner_cache(parser, 1) == 0 { return 0; }

    while pbuf_is_blank(parser) {
        scanner_skip(parser);
        if scanner_cache(parser, 1) == 0 { return 0; }
    }

    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 { return 0; }

    if !pbuf_check(parser, b'.') {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected digit or '.' character\0".as_ptr() as *const c_char,
        );
    }

    scanner_skip(parser);

    if yaml_parser_scan_version_directive_number(parser, start_mark, minor) == 0 { return 0; }

    1
}

// ============================================================
// yaml_parser_scan_version_directive_number — static
// ============================================================

unsafe fn yaml_parser_scan_version_directive_number(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    number: *mut i32,
) -> i32 {
    let mut value: i32 = 0;
    let mut length: usize = 0;

    if scanner_cache(parser, 1) == 0 { return 0; }

    while pbuf_is_digit(parser) {
        length += 1;
        if length > MAX_NUMBER_LENGTH {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0".as_ptr() as *const c_char,
                start_mark,
                b"found extremely long version number\0".as_ptr() as *const c_char,
            );
        }

        value = value * 10 + pbuf_as_digit(parser);
        scanner_skip(parser);
        if scanner_cache(parser, 1) == 0 { return 0; }
    }

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

// ============================================================
// yaml_parser_scan_tag_directive_value — static
// ============================================================

unsafe fn yaml_parser_scan_tag_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
    prefix: *mut *mut yaml_char_t,
) -> i32 {
    let mut handle_value: *mut yaml_char_t = core::ptr::null_mut();
    let mut prefix_value: *mut yaml_char_t = core::ptr::null_mut();

    if scanner_cache(parser, 1) == 0 { goto_tag_dir_err(handle_value, prefix_value); return 0; }

    while pbuf_is_blank(parser) {
        scanner_skip(parser);
        if scanner_cache(parser, 1) == 0 { goto_tag_dir_err(handle_value, prefix_value); return 0; }
    }

    if yaml_parser_scan_tag_handle(parser, 1, start_mark, &mut handle_value) == 0 {
        goto_tag_dir_err(handle_value, prefix_value);
        return 0;
    }

    if scanner_cache(parser, 1) == 0 { goto_tag_dir_err(handle_value, prefix_value); return 0; }

    if !pbuf_is_blank(parser) {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %TAG directive\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected whitespace\0".as_ptr() as *const c_char,
        );
        goto_tag_dir_err(handle_value, prefix_value);
        return 0;
    }

    while pbuf_is_blank(parser) {
        scanner_skip(parser);
        if scanner_cache(parser, 1) == 0 { goto_tag_dir_err(handle_value, prefix_value); return 0; }
    }

    if yaml_parser_scan_tag_uri(parser, 1, 1, core::ptr::null_mut(), start_mark, &mut prefix_value) == 0 {
        goto_tag_dir_err(handle_value, prefix_value);
        return 0;
    }

    if scanner_cache(parser, 1) == 0 { goto_tag_dir_err(handle_value, prefix_value); return 0; }

    if !pbuf_is_blankz(parser) {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %TAG directive\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected whitespace or line break\0".as_ptr() as *const c_char,
        );
        goto_tag_dir_err(handle_value, prefix_value);
        return 0;
    }

    *handle = handle_value;
    *prefix = prefix_value;
    1
}

unsafe fn goto_tag_dir_err(handle_value: *mut yaml_char_t, prefix_value: *mut yaml_char_t) {
    yaml_free(handle_value as *mut c_void);
    yaml_free(prefix_value as *mut c_void);
}

// ============================================================
// yaml_parser_scan_anchor — static
// ============================================================

unsafe fn yaml_parser_scan_anchor(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    type_: yaml_token_type_t,
) -> i32 {
    let mut length: i32 = 0;
    let mut string: yaml_string_t = core::mem::zeroed();

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { return 0; }

    let start_mark = (*parser).mark;
    scanner_skip(parser);

    if scanner_cache(parser, 1) == 0 {
        string_del(&mut string);
        return 0;
    }

    while pbuf_is_alpha(parser) {
        if scanner_read(parser, &mut string) == 0 {
            string_del(&mut string);
            return 0;
        }
        if scanner_cache(parser, 1) == 0 {
            string_del(&mut string);
            return 0;
        }
        length += 1;
    }

    let end_mark = (*parser).mark;

    if length == 0
        || !(pbuf_is_blankz(parser)
            || pbuf_check(parser, b'?')
            || pbuf_check(parser, b':')
            || pbuf_check(parser, b',')
            || pbuf_check(parser, b']')
            || pbuf_check(parser, b'}')
            || pbuf_check(parser, b'%')
            || pbuf_check(parser, b'@')
            || pbuf_check(parser, b'`'))
    {
        let ctx = if type_ == YAML_ANCHOR_TOKEN {
            b"while scanning an anchor\0".as_ptr() as *const c_char
        } else {
            b"while scanning an alias\0".as_ptr() as *const c_char
        };
        yaml_parser_set_scanner_error(
            parser,
            ctx,
            start_mark,
            b"did not find expected alphabetic or numeric character\0".as_ptr() as *const c_char,
        );
        string_del(&mut string);
        return 0;
    }

    if type_ == YAML_ANCHOR_TOKEN {
        scanner_anchor_token_init(token, string.start, start_mark, end_mark);
    } else {
        scanner_alias_token_init(token, string.start, start_mark, end_mark);
    }
    1
}

// ============================================================
// yaml_parser_scan_tag — static
// ============================================================

unsafe fn yaml_parser_scan_tag(parser: *mut yaml_parser_t, token: *mut yaml_token_t) -> i32 {
    let mut handle: *mut yaml_char_t = core::ptr::null_mut();
    let mut suffix: *mut yaml_char_t = core::ptr::null_mut();
    let start_mark = (*parser).mark;

    if scanner_cache(parser, 2) == 0 {
        goto_scan_tag_err(handle, suffix);
        return 0;
    }

    if pbuf_check_at(parser, b'<', 1) {
        handle = yaml_malloc(1) as *mut yaml_char_t;
        if handle.is_null() { goto_scan_tag_err(handle, suffix); return 0; }
        *handle = 0;

        scanner_skip(parser);
        scanner_skip(parser);

        if yaml_parser_scan_tag_uri(parser, 1, 0, core::ptr::null_mut(), start_mark, &mut suffix) == 0 {
            goto_scan_tag_err(handle, suffix);
            return 0;
        }

        if !pbuf_check(parser, b'>') {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a tag\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find the expected '>'\0".as_ptr() as *const c_char,
            );
            goto_scan_tag_err(handle, suffix);
            return 0;
        }

        scanner_skip(parser);
    } else {
        if yaml_parser_scan_tag_handle(parser, 0, start_mark, &mut handle) == 0 {
            goto_scan_tag_err(handle, suffix);
            return 0;
        }

        if *handle == b'!'
            && *handle.add(1) != 0
            && *handle.add(libc::strlen(handle as *const c_char) - 1) == b'!'
        {
            if yaml_parser_scan_tag_uri(parser, 0, 0, core::ptr::null_mut(), start_mark, &mut suffix) == 0 {
                goto_scan_tag_err(handle, suffix);
                return 0;
            }
        } else {
            if yaml_parser_scan_tag_uri(parser, 0, 0, handle, start_mark, &mut suffix) == 0 {
                goto_scan_tag_err(handle, suffix);
                return 0;
            }

            yaml_free(handle as *mut c_void);
            handle = yaml_malloc(2) as *mut yaml_char_t;
            if handle.is_null() { goto_scan_tag_err(handle, suffix); return 0; }
            *handle = b'!';
            *handle.add(1) = 0;

            if *suffix == 0 {
                let tmp = handle;
                handle = suffix;
                suffix = tmp;
            }
        }
    }

    if scanner_cache(parser, 1) == 0 {
        goto_scan_tag_err(handle, suffix);
        return 0;
    }

    if !pbuf_is_blankz(parser) {
        if (*parser).flow_level == 0 || !pbuf_check(parser, b',') {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a tag\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find expected whitespace or line break\0".as_ptr() as *const c_char,
            );
            goto_scan_tag_err(handle, suffix);
            return 0;
        }
    }

    let end_mark = (*parser).mark;
    scanner_tag_token_init(token, handle, suffix, start_mark, end_mark);
    1
}

unsafe fn goto_scan_tag_err(handle: *mut yaml_char_t, suffix: *mut yaml_char_t) {
    yaml_free(handle as *mut c_void);
    yaml_free(suffix as *mut c_void);
}

// ============================================================
// yaml_parser_scan_tag_handle — static
// ============================================================

unsafe fn yaml_parser_scan_tag_handle(
    parser: *mut yaml_parser_t,
    directive: i32,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
) -> i32 {
    let mut string: yaml_string_t = core::mem::zeroed();

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { return 0; }

    if scanner_cache(parser, 1) == 0 {
        string_del(&mut string);
        return 0;
    }

    if !pbuf_check(parser, b'!') {
        let ctx = if directive != 0 {
            b"while scanning a tag directive\0".as_ptr() as *const c_char
        } else {
            b"while scanning a tag\0".as_ptr() as *const c_char
        };
        yaml_parser_set_scanner_error(
            parser,
            ctx,
            start_mark,
            b"did not find expected '!'\0".as_ptr() as *const c_char,
        );
        string_del(&mut string);
        return 0;
    }

    if scanner_read(parser, &mut string) == 0 {
        string_del(&mut string);
        return 0;
    }

    if scanner_cache(parser, 1) == 0 {
        string_del(&mut string);
        return 0;
    }

    while pbuf_is_alpha(parser) {
        if scanner_read(parser, &mut string) == 0 {
            string_del(&mut string);
            return 0;
        }
        if scanner_cache(parser, 1) == 0 {
            string_del(&mut string);
            return 0;
        }
    }

    if pbuf_check(parser, b'!') {
        if scanner_read(parser, &mut string) == 0 {
            string_del(&mut string);
            return 0;
        }
    } else {
        if directive != 0 && !(*string.start == b'!' && *string.start.add(1) == 0) {
            yaml_parser_set_scanner_error(
                parser,
                b"while parsing a tag directive\0".as_ptr() as *const c_char,
                start_mark,
                b"did not find expected '!'\0".as_ptr() as *const c_char,
            );
            string_del(&mut string);
            return 0;
        }
    }

    *handle = string.start;
    1
}

// ============================================================
// yaml_parser_scan_tag_uri — static
// ============================================================

unsafe fn yaml_parser_scan_tag_uri(
    parser: *mut yaml_parser_t,
    uri_char: i32,
    directive: i32,
    head: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    uri: *mut *mut yaml_char_t,
) -> i32 {
    let head_len: usize = if head.is_null() { 0 } else { libc::strlen(head as *const c_char) };
    let mut string: yaml_string_t = core::mem::zeroed();
    let mut length = head_len;

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { return 0; }

    while (string.end as usize - string.start as usize) <= length {
        if yaml_string_extend(&mut string.start, &mut string.pointer, &mut string.end) == 0 {
            (*parser).error = YAML_MEMORY_ERROR;
            string_del(&mut string);
            return 0;
        }
    }

    if length > 1 {
        libc::memcpy(string.start as *mut c_void, head.add(1) as *const c_void, length - 1);
        string.pointer = string.pointer.add(length - 1);
    }

    if scanner_cache(parser, 1) == 0 {
        string_del(&mut string);
        return 0;
    }

    while pbuf_is_alpha(parser)
        || pbuf_check(parser, b';')
        || pbuf_check(parser, b'/')
        || pbuf_check(parser, b'?')
        || pbuf_check(parser, b':')
        || pbuf_check(parser, b'@')
        || pbuf_check(parser, b'&')
        || pbuf_check(parser, b'=')
        || pbuf_check(parser, b'+')
        || pbuf_check(parser, b'$')
        || pbuf_check(parser, b'.')
        || pbuf_check(parser, b'%')
        || pbuf_check(parser, b'!')
        || pbuf_check(parser, b'~')
        || pbuf_check(parser, b'*')
        || pbuf_check(parser, b'\'')
        || pbuf_check(parser, b'(')
        || pbuf_check(parser, b')')
        || (uri_char != 0
            && (pbuf_check(parser, b',')
                || pbuf_check(parser, b'[')
                || pbuf_check(parser, b']')))
    {
        if pbuf_check(parser, b'%') {
            if string_extend_check(&mut (*parser).error, &mut string) == 0 {
                string_del(&mut string);
                return 0;
            }
            if yaml_parser_scan_uri_escapes(parser, directive, start_mark, &mut string) == 0 {
                string_del(&mut string);
                return 0;
            }
        } else {
            if scanner_read(parser, &mut string) == 0 {
                string_del(&mut string);
                return 0;
            }
        }
        length += 1;
        if scanner_cache(parser, 1) == 0 {
            string_del(&mut string);
            return 0;
        }
    }

    if length == 0 {
        if string_extend_check(&mut (*parser).error, &mut string) == 0 {
            string_del(&mut string);
            return 0;
        }
        let ctx = if directive != 0 {
            b"while parsing a %TAG directive\0".as_ptr() as *const c_char
        } else {
            b"while parsing a tag\0".as_ptr() as *const c_char
        };
        yaml_parser_set_scanner_error(
            parser,
            ctx,
            start_mark,
            b"did not find expected tag URI\0".as_ptr() as *const c_char,
        );
        string_del(&mut string);
        return 0;
    }

    *uri = string.start;
    1
}

// ============================================================
// yaml_parser_scan_uri_escapes — static
// ============================================================

unsafe fn yaml_parser_scan_uri_escapes(
    parser: *mut yaml_parser_t,
    directive: i32,
    start_mark: yaml_mark_t,
    string: *mut yaml_string_t,
) -> i32 {
    let mut width: i32 = 0;

    loop {
        let mut octet: u8 = 0;

        if scanner_cache(parser, 3) == 0 { return 0; }

        if !(pbuf_check(parser, b'%')
            && pbuf_is_hex_at(parser, 1)
            && pbuf_is_hex_at(parser, 2))
        {
            let ctx = if directive != 0 {
                b"while parsing a %TAG directive\0".as_ptr() as *const c_char
            } else {
                b"while parsing a tag\0".as_ptr() as *const c_char
            };
            return yaml_parser_set_scanner_error(
                parser,
                ctx,
                start_mark,
                b"did not find URI escaped octet\0".as_ptr() as *const c_char,
            );
        }

        octet = ((pbuf_as_hex_at(parser, 1) << 4) + pbuf_as_hex_at(parser, 2)) as u8;

        if width == 0 {
            width = if (octet & 0x80) == 0x00 { 1 }
                    else if (octet & 0xE0) == 0xC0 { 2 }
                    else if (octet & 0xF0) == 0xE0 { 3 }
                    else if (octet & 0xF8) == 0xF0 { 4 }
                    else { 0 };
            if width == 0 {
                let ctx = if directive != 0 {
                    b"while parsing a %TAG directive\0".as_ptr() as *const c_char
                } else {
                    b"while parsing a tag\0".as_ptr() as *const c_char
                };
                return yaml_parser_set_scanner_error(
                    parser,
                    ctx,
                    start_mark,
                    b"found an incorrect leading UTF-8 octet\0".as_ptr() as *const c_char,
                );
            }
        } else if (octet & 0xC0) != 0x80 {
            let ctx = if directive != 0 {
                b"while parsing a %TAG directive\0".as_ptr() as *const c_char
            } else {
                b"while parsing a tag\0".as_ptr() as *const c_char
            };
            return yaml_parser_set_scanner_error(
                parser,
                ctx,
                start_mark,
                b"found an incorrect trailing UTF-8 octet\0".as_ptr() as *const c_char,
            );
        }

        *(*string).pointer = octet;
        (*string).pointer = (*string).pointer.add(1);
        scanner_skip(parser);
        scanner_skip(parser);
        scanner_skip(parser);

        width -= 1;
        if width == 0 { break; }
    }
    1
}

// ============================================================
// yaml_parser_scan_block_scalar — static
// ============================================================

unsafe fn yaml_parser_scan_block_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    literal: i32,
) -> i32 {
    let mut string: yaml_string_t = core::mem::zeroed();
    let mut leading_break: yaml_string_t = core::mem::zeroed();
    let mut trailing_breaks: yaml_string_t = core::mem::zeroed();
    let mut chomping: i32 = 0;
    let mut increment: i32 = 0;
    let mut indent: i32 = 0;
    let mut leading_blank: i32 = 0;
    let mut trailing_blank: i32 = 0;

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
    if string_init(&mut (*parser).error, &mut leading_break, INITIAL_STRING_SIZE) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
    if string_init(&mut (*parser).error, &mut trailing_breaks, INITIAL_STRING_SIZE) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

    let start_mark = (*parser).mark;
    scanner_skip(parser);

    if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

    if pbuf_check(parser, b'+') || pbuf_check(parser, b'-') {
        chomping = if pbuf_check(parser, b'+') { 1 } else { -1 };
        scanner_skip(parser);

        if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

        if pbuf_is_digit(parser) {
            if pbuf_check(parser, b'0') {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a block scalar\0".as_ptr() as *const c_char,
                    start_mark,
                    b"found an indentation indicator equal to 0\0".as_ptr() as *const c_char,
                );
                goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks);
                return 0;
            }
            increment = pbuf_as_digit(parser);
            scanner_skip(parser);
        }
    } else if pbuf_is_digit(parser) {
        if pbuf_check(parser, b'0') {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0".as_ptr() as *const c_char,
                start_mark,
                b"found an indentation indicator equal to 0\0".as_ptr() as *const c_char,
            );
            goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks);
            return 0;
        }
        increment = pbuf_as_digit(parser);
        scanner_skip(parser);

        if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

        if pbuf_check(parser, b'+') || pbuf_check(parser, b'-') {
            chomping = if pbuf_check(parser, b'+') { 1 } else { -1 };
            scanner_skip(parser);
        }
    }

    if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

    while pbuf_is_blank(parser) {
        scanner_skip(parser);
        if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
    }

    if pbuf_check(parser, b'#') {
        while !pbuf_is_breakz(parser) {
            scanner_skip(parser);
            if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
        }
    }

    if !pbuf_is_breakz(parser) {
        yaml_parser_set_scanner_error(
            parser,
            b"while scanning a block scalar\0".as_ptr() as *const c_char,
            start_mark,
            b"did not find expected comment or line break\0".as_ptr() as *const c_char,
        );
        goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks);
        return 0;
    }

    if pbuf_is_break(parser) {
        if scanner_cache(parser, 2) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
        scanner_skip_line(parser);
    }

    let mut end_mark = (*parser).mark;

    if increment != 0 {
        indent = if (*parser).indent >= 0 { (*parser).indent as i32 + increment } else { increment };
    }

    if yaml_parser_scan_block_scalar_breaks(parser, &mut indent, &mut trailing_breaks, start_mark, &mut end_mark) == 0 {
        goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks);
        return 0;
    }

    if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

    while (*parser).mark.column as i32 == indent && !pbuf_is_z(parser) {
        trailing_blank = pbuf_is_blank(parser) as i32;

        if literal == 0 && *leading_break.start == b'\n' && leading_blank == 0 && trailing_blank == 0 {
            if *trailing_breaks.start == 0 {
                if string_extend_check(&mut (*parser).error, &mut string) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
                *string.pointer = b' ';
                string.pointer = string.pointer.add(1);
            }
            string_clear(&mut leading_break);
        } else {
            if string_join(&mut (*parser).error, &mut string, &mut leading_break) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
            string_clear(&mut leading_break);
        }

        if string_join(&mut (*parser).error, &mut string, &mut trailing_breaks) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
        string_clear(&mut trailing_breaks);

        leading_blank = pbuf_is_blank(parser) as i32;

        while !pbuf_is_breakz(parser) {
            if scanner_read(parser, &mut string) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
            if scanner_cache(parser, 1) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
        }

        if scanner_cache(parser, 2) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
        if scanner_read_line(parser, &mut leading_break) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }

        if yaml_parser_scan_block_scalar_breaks(parser, &mut indent, &mut trailing_breaks, start_mark, &mut end_mark) == 0 {
            goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks);
            return 0;
        }
    }

    if chomping != -1 {
        if string_join(&mut (*parser).error, &mut string, &mut leading_break) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
    }
    if chomping == 1 {
        if string_join(&mut (*parser).error, &mut string, &mut trailing_breaks) == 0 { goto_bs_error(&mut string, &mut leading_break, &mut trailing_breaks); return 0; }
    }

    let style = if literal != 0 { YAML_LITERAL_SCALAR_STYLE } else { YAML_FOLDED_SCALAR_STYLE };
    let len = string.pointer as usize - string.start as usize;
    scanner_scalar_token_init(token, string.start, len, style, start_mark, end_mark);

    string_del(&mut leading_break);
    string_del(&mut trailing_breaks);
    1
}

unsafe fn goto_bs_error(
    string: *mut yaml_string_t,
    leading_break: *mut yaml_string_t,
    trailing_breaks: *mut yaml_string_t,
) {
    string_del(string);
    string_del(leading_break);
    string_del(trailing_breaks);
}

// ============================================================
// yaml_parser_scan_block_scalar_breaks — static
// ============================================================

unsafe fn yaml_parser_scan_block_scalar_breaks(
    parser: *mut yaml_parser_t,
    indent: *mut i32,
    breaks: *mut yaml_string_t,
    start_mark: yaml_mark_t,
    end_mark: *mut yaml_mark_t,
) -> i32 {
    let mut max_indent: i32 = 0;
    *end_mark = (*parser).mark;

    loop {
        if scanner_cache(parser, 1) == 0 { return 0; }

        while (*indent == 0 || ((*parser).mark.column as i32) < *indent) && pbuf_is_space(parser) {
            scanner_skip(parser);
            if scanner_cache(parser, 1) == 0 { return 0; }
        }

        if (*parser).mark.column as i32 > max_indent {
            max_indent = (*parser).mark.column as i32;
        }

        if (*indent == 0 || ((*parser).mark.column as i32) < *indent) && pbuf_is_tab(parser) {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0".as_ptr() as *const c_char,
                start_mark,
                b"found a tab character where an indentation space is expected\0".as_ptr() as *const c_char,
            );
        }

        if !pbuf_is_break(parser) { break; }

        if scanner_cache(parser, 2) == 0 { return 0; }
        if scanner_read_line(parser, breaks) == 0 { return 0; }
        *end_mark = (*parser).mark;
    }

    if *indent == 0 {
        *indent = max_indent;
        if *indent < (*parser).indent as i32 + 1 {
            *indent = (*parser).indent as i32 + 1;
        }
        if *indent < 1 {
            *indent = 1;
        }
    }
    1
}

// ============================================================
// yaml_parser_scan_flow_scalar — static
// ============================================================

unsafe fn yaml_parser_scan_flow_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    single: i32,
) -> i32 {
    let mut string: yaml_string_t = core::mem::zeroed();
    let mut leading_break: yaml_string_t = core::mem::zeroed();
    let mut trailing_breaks: yaml_string_t = core::mem::zeroed();
    let mut whitespaces: yaml_string_t = core::mem::zeroed();

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
    if string_init(&mut (*parser).error, &mut leading_break, INITIAL_STRING_SIZE) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
    if string_init(&mut (*parser).error, &mut trailing_breaks, INITIAL_STRING_SIZE) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
    if string_init(&mut (*parser).error, &mut whitespaces, INITIAL_STRING_SIZE) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

    let start_mark = (*parser).mark;
    scanner_skip(parser);

    loop {
        if scanner_cache(parser, 4) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

        if (*parser).mark.column == 0
            && ((pbuf_check_at(parser, b'-', 0) && pbuf_check_at(parser, b'-', 1) && pbuf_check_at(parser, b'-', 2))
                || (pbuf_check_at(parser, b'.', 0) && pbuf_check_at(parser, b'.', 1) && pbuf_check_at(parser, b'.', 2)))
            && pbuf_is_blankz_at(parser, 3)
        {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a quoted scalar\0".as_ptr() as *const c_char,
                start_mark,
                b"found unexpected document indicator\0".as_ptr() as *const c_char,
            );
            goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
            return 0;
        }

        if pbuf_is_z(parser) {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a quoted scalar\0".as_ptr() as *const c_char,
                start_mark,
                b"found unexpected end of stream\0".as_ptr() as *const c_char,
            );
            goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
            return 0;
        }

        if scanner_cache(parser, 2) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

        let mut leading_blanks: i32 = 0;

        while !pbuf_is_blankz(parser) {
            if single != 0 && pbuf_check_at(parser, b'\'', 0) && pbuf_check_at(parser, b'\'', 1) {
                if string_extend_check(&mut (*parser).error, &mut string) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                *string.pointer = b'\'';
                string.pointer = string.pointer.add(1);
                scanner_skip(parser);
                scanner_skip(parser);
            } else if pbuf_check(parser, if single != 0 { b'\'' } else { b'"' }) {
                break;
            } else if single == 0 && pbuf_check(parser, b'\\') && pbuf_is_break_at(parser, 1) {
                if scanner_cache(parser, 3) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                scanner_skip(parser);
                scanner_skip_line(parser);
                leading_blanks = 1;
                break;
            } else if single == 0 && pbuf_check(parser, b'\\') {
                let mut code_length: usize = 0;
                if string_extend_check(&mut (*parser).error, &mut string) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

                match *(*parser).buffer.pointer.add(1) {
                    b'0' => { *string.pointer = 0; string.pointer = string.pointer.add(1); }
                    b'a' => { *string.pointer = 0x07; string.pointer = string.pointer.add(1); }
                    b'b' => { *string.pointer = 0x08; string.pointer = string.pointer.add(1); }
                    b't' | b'\t' => { *string.pointer = 0x09; string.pointer = string.pointer.add(1); }
                    b'n' => { *string.pointer = 0x0A; string.pointer = string.pointer.add(1); }
                    b'v' => { *string.pointer = 0x0B; string.pointer = string.pointer.add(1); }
                    b'f' => { *string.pointer = 0x0C; string.pointer = string.pointer.add(1); }
                    b'r' => { *string.pointer = 0x0D; string.pointer = string.pointer.add(1); }
                    b'e' => { *string.pointer = 0x1B; string.pointer = string.pointer.add(1); }
                    b' ' => { *string.pointer = 0x20; string.pointer = string.pointer.add(1); }
                    b'"' => { *string.pointer = b'"'; string.pointer = string.pointer.add(1); }
                    b'/' => { *string.pointer = b'/'; string.pointer = string.pointer.add(1); }
                    b'\\' => { *string.pointer = b'\\'; string.pointer = string.pointer.add(1); }
                    b'N' => { *string.pointer = 0xC2; string.pointer = string.pointer.add(1); *string.pointer = 0x85; string.pointer = string.pointer.add(1); }
                    b'_' => { *string.pointer = 0xC2; string.pointer = string.pointer.add(1); *string.pointer = 0xA0; string.pointer = string.pointer.add(1); }
                    b'L' => { *string.pointer = 0xE2; string.pointer = string.pointer.add(1); *string.pointer = 0x80; string.pointer = string.pointer.add(1); *string.pointer = 0xA8; string.pointer = string.pointer.add(1); }
                    b'P' => { *string.pointer = 0xE2; string.pointer = string.pointer.add(1); *string.pointer = 0x80; string.pointer = string.pointer.add(1); *string.pointer = 0xA9; string.pointer = string.pointer.add(1); }
                    b'x' => { code_length = 2; }
                    b'u' => { code_length = 4; }
                    b'U' => { code_length = 8; }
                    _ => {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while parsing a quoted scalar\0".as_ptr() as *const c_char,
                            start_mark,
                            b"found unknown escape character\0".as_ptr() as *const c_char,
                        );
                        goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
                        return 0;
                    }
                }

                scanner_skip(parser);
                scanner_skip(parser);

                if code_length > 0 {
                    let mut value: u32 = 0;

                    if scanner_cache(parser, code_length) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

                    for k in 0..code_length {
                        if !pbuf_is_hex_at(parser, k) {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while parsing a quoted scalar\0".as_ptr() as *const c_char,
                                start_mark,
                                b"did not find expected hexdecimal number\0".as_ptr() as *const c_char,
                            );
                            goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
                            return 0;
                        }
                        value = (value << 4) + pbuf_as_hex_at(parser, k);
                    }

                    if (value >= 0xD800 && value <= 0xDFFF) || value > 0x10FFFF {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while parsing a quoted scalar\0".as_ptr() as *const c_char,
                            start_mark,
                            b"found invalid Unicode character escape code\0".as_ptr() as *const c_char,
                        );
                        goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
                        return 0;
                    }

                    if value <= 0x7F {
                        *string.pointer = value as u8; string.pointer = string.pointer.add(1);
                    } else if value <= 0x7FF {
                        *string.pointer = (0xC0 + (value >> 6)) as u8; string.pointer = string.pointer.add(1);
                        *string.pointer = (0x80 + (value & 0x3F)) as u8; string.pointer = string.pointer.add(1);
                    } else if value <= 0xFFFF {
                        *string.pointer = (0xE0 + (value >> 12)) as u8; string.pointer = string.pointer.add(1);
                        *string.pointer = (0x80 + ((value >> 6) & 0x3F)) as u8; string.pointer = string.pointer.add(1);
                        *string.pointer = (0x80 + (value & 0x3F)) as u8; string.pointer = string.pointer.add(1);
                    } else {
                        *string.pointer = (0xF0 + (value >> 18)) as u8; string.pointer = string.pointer.add(1);
                        *string.pointer = (0x80 + ((value >> 12) & 0x3F)) as u8; string.pointer = string.pointer.add(1);
                        *string.pointer = (0x80 + ((value >> 6) & 0x3F)) as u8; string.pointer = string.pointer.add(1);
                        *string.pointer = (0x80 + (value & 0x3F)) as u8; string.pointer = string.pointer.add(1);
                    }

                    for _ in 0..code_length {
                        scanner_skip(parser);
                    }
                }
            } else {
                if scanner_read(parser, &mut string) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
            }

            if scanner_cache(parser, 2) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
        }

        if scanner_cache(parser, 1) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
        if pbuf_check(parser, if single != 0 { b'\'' } else { b'"' }) { break; }

        if scanner_cache(parser, 1) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

        while pbuf_is_blank(parser) || pbuf_is_break(parser) {
            if pbuf_is_blank(parser) {
                if leading_blanks == 0 {
                    if scanner_read(parser, &mut whitespaces) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                } else {
                    scanner_skip(parser);
                }
            } else {
                if scanner_cache(parser, 2) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

                if leading_blanks == 0 {
                    string_clear(&mut whitespaces);
                    if scanner_read_line(parser, &mut leading_break) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                    leading_blanks = 1;
                } else {
                    if scanner_read_line(parser, &mut trailing_breaks) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                }
            }
            if scanner_cache(parser, 1) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
        }

        if leading_blanks != 0 {
            if *leading_break.start == b'\n' {
                if *trailing_breaks.start == 0 {
                    if string_extend_check(&mut (*parser).error, &mut string) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                    *string.pointer = b' ';
                    string.pointer = string.pointer.add(1);
                } else {
                    if string_join(&mut (*parser).error, &mut string, &mut trailing_breaks) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                    string_clear(&mut trailing_breaks);
                }
                string_clear(&mut leading_break);
            } else {
                if string_join(&mut (*parser).error, &mut string, &mut leading_break) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                if string_join(&mut (*parser).error, &mut string, &mut trailing_breaks) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                string_clear(&mut leading_break);
                string_clear(&mut trailing_breaks);
            }
        } else {
            if string_join(&mut (*parser).error, &mut string, &mut whitespaces) == 0 { goto_fs_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
            string_clear(&mut whitespaces);
        }
    }

    scanner_skip(parser);
    let end_mark = (*parser).mark;

    let style = if single != 0 { YAML_SINGLE_QUOTED_SCALAR_STYLE } else { YAML_DOUBLE_QUOTED_SCALAR_STYLE };
    let len = string.pointer as usize - string.start as usize;
    scanner_scalar_token_init(token, string.start, len, style, start_mark, end_mark);

    string_del(&mut leading_break);
    string_del(&mut trailing_breaks);
    string_del(&mut whitespaces);
    1
}

unsafe fn goto_fs_error(
    string: *mut yaml_string_t,
    leading_break: *mut yaml_string_t,
    trailing_breaks: *mut yaml_string_t,
    whitespaces: *mut yaml_string_t,
) {
    string_del(string);
    string_del(leading_break);
    string_del(trailing_breaks);
    string_del(whitespaces);
}

// ============================================================
// yaml_parser_scan_plain_scalar — static
// ============================================================

unsafe fn yaml_parser_scan_plain_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> i32 {
    let mut string: yaml_string_t = core::mem::zeroed();
    let mut leading_break: yaml_string_t = core::mem::zeroed();
    let mut trailing_breaks: yaml_string_t = core::mem::zeroed();
    let mut whitespaces: yaml_string_t = core::mem::zeroed();
    let mut leading_blanks: i32 = 0;
    let indent: isize = ((*parser).indent + 1) as isize;

    if string_init(&mut (*parser).error, &mut string, INITIAL_STRING_SIZE) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
    if string_init(&mut (*parser).error, &mut leading_break, INITIAL_STRING_SIZE) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
    if string_init(&mut (*parser).error, &mut trailing_breaks, INITIAL_STRING_SIZE) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
    if string_init(&mut (*parser).error, &mut whitespaces, INITIAL_STRING_SIZE) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

    let start_mark = (*parser).mark;
    let mut end_mark = (*parser).mark;

    loop {
        if scanner_cache(parser, 4) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

        if (*parser).mark.column == 0
            && ((pbuf_check_at(parser, b'-', 0) && pbuf_check_at(parser, b'-', 1) && pbuf_check_at(parser, b'-', 2))
                || (pbuf_check_at(parser, b'.', 0) && pbuf_check_at(parser, b'.', 1) && pbuf_check_at(parser, b'.', 2)))
            && pbuf_is_blankz_at(parser, 3)
        {
            break;
        }

        if pbuf_check(parser, b'#') { break; }

        while !pbuf_is_blankz(parser) {
            if (*parser).flow_level != 0
                && pbuf_check(parser, b':')
                && (pbuf_check_at(parser, b',', 1)
                    || pbuf_check_at(parser, b'?', 1)
                    || pbuf_check_at(parser, b'[', 1)
                    || pbuf_check_at(parser, b']', 1)
                    || pbuf_check_at(parser, b'{', 1)
                    || pbuf_check_at(parser, b'}', 1))
            {
                yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a plain scalar\0".as_ptr() as *const c_char,
                    start_mark,
                    b"found unexpected ':'\0".as_ptr() as *const c_char,
                );
                goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
                return 0;
            }

            if (pbuf_check(parser, b':') && pbuf_is_blankz_at(parser, 1))
                || ((*parser).flow_level != 0
                    && (pbuf_check(parser, b',')
                        || pbuf_check(parser, b'[')
                        || pbuf_check(parser, b']')
                        || pbuf_check(parser, b'{')
                        || pbuf_check(parser, b'}')))
            {
                break;
            }

            if leading_blanks != 0 || whitespaces.start != whitespaces.pointer {
                if leading_blanks != 0 {
                    if *leading_break.start == b'\n' {
                        if *trailing_breaks.start == 0 {
                            if string_extend_check(&mut (*parser).error, &mut string) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                            *string.pointer = b' ';
                            string.pointer = string.pointer.add(1);
                        } else {
                            if string_join(&mut (*parser).error, &mut string, &mut trailing_breaks) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                            string_clear(&mut trailing_breaks);
                        }
                        string_clear(&mut leading_break);
                    } else {
                        if string_join(&mut (*parser).error, &mut string, &mut leading_break) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                        if string_join(&mut (*parser).error, &mut string, &mut trailing_breaks) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                        string_clear(&mut leading_break);
                        string_clear(&mut trailing_breaks);
                    }
                    leading_blanks = 0;
                } else {
                    if string_join(&mut (*parser).error, &mut string, &mut whitespaces) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                    string_clear(&mut whitespaces);
                }
            }

            if scanner_read(parser, &mut string) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
            end_mark = (*parser).mark;
            if scanner_cache(parser, 2) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
        }

        if !(pbuf_is_blank(parser) || pbuf_is_break(parser)) { break; }

        if scanner_cache(parser, 1) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

        while pbuf_is_blank(parser) || pbuf_is_break(parser) {
            if pbuf_is_blank(parser) {
                if leading_blanks != 0 && ((*parser).mark.column as isize) < indent && pbuf_is_tab(parser) {
                    yaml_parser_set_scanner_error(
                        parser,
                        b"while scanning a plain scalar\0".as_ptr() as *const c_char,
                        start_mark,
                        b"found a tab character that violates indentation\0".as_ptr() as *const c_char,
                    );
                    goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces);
                    return 0;
                }

                if leading_blanks == 0 {
                    if scanner_read(parser, &mut whitespaces) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                } else {
                    scanner_skip(parser);
                }
            } else {
                if scanner_cache(parser, 2) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }

                if leading_blanks == 0 {
                    string_clear(&mut whitespaces);
                    if scanner_read_line(parser, &mut leading_break) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                    leading_blanks = 1;
                } else {
                    if scanner_read_line(parser, &mut trailing_breaks) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
                }
            }
            if scanner_cache(parser, 1) == 0 { goto_ps_error(&mut string, &mut leading_break, &mut trailing_breaks, &mut whitespaces); return 0; }
        }

        if (*parser).flow_level == 0 && ((*parser).mark.column as isize) < indent {
            break;
        }
    }

    let len = string.pointer as usize - string.start as usize;
    scanner_scalar_token_init(token, string.start, len, YAML_PLAIN_SCALAR_STYLE, start_mark, end_mark);

    if leading_blanks != 0 {
        (*parser).simple_key_allowed = 1;
    }

    string_del(&mut leading_break);
    string_del(&mut trailing_breaks);
    string_del(&mut whitespaces);
    1
}

unsafe fn goto_ps_error(
    string: *mut yaml_string_t,
    leading_break: *mut yaml_string_t,
    trailing_breaks: *mut yaml_string_t,
    whitespaces: *mut yaml_string_t,
) {
    string_del(string);
    string_del(leading_break);
    string_del(trailing_breaks);
    string_del(whitespaces);
}


// ============================================================
// dumper.c — YAML document serializer
// ============================================================

// ============================================================
// yaml_emitter_open — exported
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_open(emitter: *mut yaml_emitter_t) -> i32 {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).opened == 0);

    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut event: yaml_event_t = core::mem::zeroed();
    stream_start_event_init(&mut event, YAML_ANY_ENCODING, mark, mark);

    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    (*emitter).opened = 1;
    1
}

// ============================================================
// yaml_emitter_close — exported
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_close(emitter: *mut yaml_emitter_t) -> i32 {
    debug_assert!(!emitter.is_null());
    debug_assert!((*emitter).opened != 0);

    if (*emitter).closed != 0 { return 1; }

    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut event: yaml_event_t = core::mem::zeroed();
    stream_end_event_init(&mut event, mark, mark);

    if yaml_emitter_emit(emitter, &mut event) == 0 {
        return 0;
    }

    (*emitter).closed = 1;
    1
}

// ============================================================
// yaml_emitter_dump — exported
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_dump(
    emitter: *mut yaml_emitter_t,
    document: *mut yaml_document_t,
) -> i32 {
    debug_assert!(!emitter.is_null());
    debug_assert!(!document.is_null());

    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };

    (*emitter).document = document;

    if (*emitter).opened == 0 {
        if yaml_emitter_open(emitter) == 0 {
            yaml_emitter_delete_document_and_anchors(emitter);
            return 0;
        }
    }

    if stack_empty((*document).nodes.start, (*document).nodes.top) {
        if yaml_emitter_close(emitter) == 0 {
            yaml_emitter_delete_document_and_anchors(emitter);
            return 0;
        }
        yaml_emitter_delete_document_and_anchors(emitter);
        return 1;
    }

    debug_assert!((*emitter).opened != 0);

    let node_count_items = (*document).nodes.top.offset_from((*document).nodes.start) as usize;
    (*emitter).anchors = yaml_malloc(core::mem::size_of::<yaml_anchors_t>() * node_count_items) as *mut yaml_anchors_t;
    if (*emitter).anchors.is_null() {
        yaml_emitter_delete_document_and_anchors(emitter);
        return 0;
    }
    libc::memset((*emitter).anchors as *mut c_void, 0, core::mem::size_of::<yaml_anchors_t>() * node_count_items);

    let mut event: yaml_event_t = core::mem::zeroed();
    document_start_event_init(
        &mut event,
        (*document).version_directive,
        (*document).tag_directives.start,
        (*document).tag_directives.end,
        (*document).start_implicit,
        mark,
        mark,
    );
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        yaml_emitter_delete_document_and_anchors(emitter);
        return 0;
    }

    yaml_emitter_anchor_node(emitter, 1);
    if yaml_emitter_dump_node(emitter, 1) == 0 {
        yaml_emitter_delete_document_and_anchors(emitter);
        return 0;
    }

    document_end_event_init(&mut event, (*document).end_implicit, mark, mark);
    if yaml_emitter_emit(emitter, &mut event) == 0 {
        yaml_emitter_delete_document_and_anchors(emitter);
        return 0;
    }

    yaml_emitter_delete_document_and_anchors(emitter);
    1
}

// ============================================================
// yaml_emitter_delete_document_and_anchors — static
// ============================================================

unsafe fn yaml_emitter_delete_document_and_anchors(emitter: *mut yaml_emitter_t) {
    if (*emitter).anchors.is_null() {
        yaml_document_delete((*emitter).document);
        (*emitter).document = core::ptr::null_mut();
        return;
    }

    let doc = (*emitter).document;
    let mut index: usize = 0;
    while (*doc).nodes.start.add(index) < (*doc).nodes.top {
        let node = *(*doc).nodes.start.add(index);
        if (*(*emitter).anchors.add(index)).serialized == 0 {
            yaml_free(node.tag as *mut c_void);
            if node.type_ == YAML_SCALAR_NODE {
                yaml_free(node.data.scalar.value as *mut c_void);
            }
        }
        if node.type_ == YAML_SEQUENCE_NODE {
            let items = node.data.sequence.items;
            yaml_free(items.start as *mut c_void);
        }
        if node.type_ == YAML_MAPPING_NODE {
            let pairs = node.data.mapping.pairs;
            yaml_free(pairs.start as *mut c_void);
        }
        index += 1;
    }

    yaml_free((*doc).nodes.start as *mut c_void);
    yaml_free((*emitter).anchors as *mut c_void);
    (*emitter).anchors = core::ptr::null_mut();
    (*emitter).last_anchor_id = 0;
    (*emitter).document = core::ptr::null_mut();
}

// ============================================================
// yaml_emitter_anchor_node — static
// ============================================================

unsafe fn yaml_emitter_anchor_node(emitter: *mut yaml_emitter_t, index: i32) {
    let node = (*(*emitter).document).nodes.start.add(index as usize - 1);

    (*(*emitter).anchors.add(index as usize - 1)).references += 1;

    if (*(*emitter).anchors.add(index as usize - 1)).references == 1 {
        match (*node).type_ {
            YAML_SEQUENCE_NODE => {
                let mut item = (*node).data.sequence.items.start;
                while item < (*node).data.sequence.items.top {
                    yaml_emitter_anchor_node(emitter, *item);
                    item = item.add(1);
                }
            }
            YAML_MAPPING_NODE => {
                let mut pair = (*node).data.mapping.pairs.start;
                while pair < (*node).data.mapping.pairs.top {
                    yaml_emitter_anchor_node(emitter, (*pair).key);
                    yaml_emitter_anchor_node(emitter, (*pair).value);
                    pair = pair.add(1);
                }
            }
            _ => {}
        }
    } else if (*(*emitter).anchors.add(index as usize - 1)).references == 2 {
        (*emitter).last_anchor_id += 1;
        (*(*emitter).anchors.add(index as usize - 1)).anchor = (*emitter).last_anchor_id;
    }
}

// ============================================================
// yaml_emitter_generate_anchor — static
// ============================================================

unsafe fn yaml_emitter_generate_anchor(
    _emitter: *mut yaml_emitter_t,
    anchor_id: i32,
) -> *mut yaml_char_t {
    let anchor = yaml_malloc(ANCHOR_TEMPLATE_LENGTH) as *mut yaml_char_t;
    if anchor.is_null() { return core::ptr::null_mut(); }
    libc::sprintf(anchor as *mut c_char, b"id%03d\0".as_ptr() as *const c_char, anchor_id);
    anchor
}

// ============================================================
// yaml_emitter_dump_node — static
// ============================================================

unsafe fn yaml_emitter_dump_node(emitter: *mut yaml_emitter_t, index: i32) -> i32 {
    let node = (*(*emitter).document).nodes.start.add(index as usize - 1);
    let anchor_id = (*(*emitter).anchors.add(index as usize - 1)).anchor;
    let mut anchor: *mut yaml_char_t = core::ptr::null_mut();

    if anchor_id != 0 {
        anchor = yaml_emitter_generate_anchor(emitter, anchor_id);
        if anchor.is_null() { return 0; }
    }

    if (*(*emitter).anchors.add(index as usize - 1)).serialized != 0 {
        return yaml_emitter_dump_alias(emitter, anchor);
    }

    (*(*emitter).anchors.add(index as usize - 1)).serialized = 1;

    match (*node).type_ {
        YAML_SCALAR_NODE => yaml_emitter_dump_scalar(emitter, node, anchor),
        YAML_SEQUENCE_NODE => yaml_emitter_dump_sequence(emitter, node, anchor),
        YAML_MAPPING_NODE => yaml_emitter_dump_mapping(emitter, node, anchor),
        _ => {
            debug_assert!(false);
            0
        }
    }
}

// ============================================================
// yaml_emitter_dump_alias — static
// ============================================================

unsafe fn yaml_emitter_dump_alias(emitter: *mut yaml_emitter_t, anchor: *mut yaml_char_t) -> i32 {
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut event: yaml_event_t = core::mem::zeroed();
    alias_event_init(&mut event, anchor, mark, mark);
    yaml_emitter_emit(emitter, &mut event)
}

// ============================================================
// yaml_emitter_dump_scalar — static
// ============================================================

unsafe fn yaml_emitter_dump_scalar(
    emitter: *mut yaml_emitter_t,
    node: *const yaml_node_t,
    anchor: *mut yaml_char_t,
) -> i32 {
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut event: yaml_event_t = core::mem::zeroed();

    let plain_implicit = (libc::strcmp((*node).tag as *const c_char, YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const c_char) == 0) as i32;
    let quoted_implicit = (libc::strcmp((*node).tag as *const c_char, YAML_DEFAULT_SCALAR_TAG.as_ptr() as *const c_char) == 0) as i32;

    scalar_event_init(
        &mut event,
        anchor,
        (*node).tag,
        (*node).data.scalar.value,
        (*node).data.scalar.length,
        plain_implicit,
        quoted_implicit,
        (*node).data.scalar.style,
        mark,
        mark,
    );
    yaml_emitter_emit(emitter, &mut event)
}

// ============================================================
// yaml_emitter_dump_sequence — static
// ============================================================

unsafe fn yaml_emitter_dump_sequence(
    emitter: *mut yaml_emitter_t,
    node: *const yaml_node_t,
    anchor: *mut yaml_char_t,
) -> i32 {
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut event: yaml_event_t = core::mem::zeroed();

    let implicit = (libc::strcmp((*node).tag as *const c_char, YAML_DEFAULT_SEQUENCE_TAG.as_ptr() as *const c_char) == 0) as i32;

    sequence_start_event_init(
        &mut event,
        anchor,
        (*node).tag,
        implicit,
        (*node).data.sequence.style,
        mark,
        mark,
    );
    if yaml_emitter_emit(emitter, &mut event) == 0 { return 0; }

    let mut item = (*node).data.sequence.items.start;
    while item < (*node).data.sequence.items.top {
        if yaml_emitter_dump_node(emitter, *item) == 0 { return 0; }
        item = item.add(1);
    }

    sequence_end_event_init(&mut event, mark, mark);
    if yaml_emitter_emit(emitter, &mut event) == 0 { return 0; }
    1
}

// ============================================================
// yaml_emitter_dump_mapping — static
// ============================================================

unsafe fn yaml_emitter_dump_mapping(
    emitter: *mut yaml_emitter_t,
    node: *const yaml_node_t,
    anchor: *mut yaml_char_t,
) -> i32 {
    let mark = yaml_mark_t { index: 0, line: 0, column: 0 };
    let mut event: yaml_event_t = core::mem::zeroed();

    let implicit = (libc::strcmp((*node).tag as *const c_char, YAML_DEFAULT_MAPPING_TAG.as_ptr() as *const c_char) == 0) as i32;

    mapping_start_event_init(
        &mut event,
        anchor,
        (*node).tag,
        implicit,
        (*node).data.mapping.style,
        mark,
        mark,
    );
    if yaml_emitter_emit(emitter, &mut event) == 0 { return 0; }

    let mut pair = (*node).data.mapping.pairs.start;
    while pair < (*node).data.mapping.pairs.top {
        if yaml_emitter_dump_node(emitter, (*pair).key) == 0 { return 0; }
        if yaml_emitter_dump_node(emitter, (*pair).value) == 0 { return 0; }
        pair = pair.add(1);
    }

    mapping_end_event_init(&mut event, mark, mark);
    if yaml_emitter_emit(emitter, &mut event) == 0 { return 0; }
    1
}


// ============================================================
// emitter.c — YAML emitter state machine
// ============================================================

// Emitter helper macros as inline functions are already defined above
// as emitter_flush_check, emitter_put, emitter_put_break, emitter_write, emitter_write_break

// ============================================================
// yaml_emitter_emit — exported
// ============================================================

#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_emit(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> i32 {
    if queue_enqueue(
        &mut (*emitter).error,
        &mut (*emitter).events.start,
        &mut (*emitter).events.head,
        &mut (*emitter).events.tail,
        &mut (*emitter).events.end,
        *event,
    ) == 0
    {
        yaml_event_delete(event);
        return 0;
    }

    while yaml_emitter_need_more_events(emitter) == 0 {
        if yaml_emitter_analyze_event(emitter, (*emitter).events.head) == 0 { return 0; }
        if yaml_emitter_state_machine(emitter, (*emitter).events.head) == 0 { return 0; }
        yaml_event_delete(&mut *(*emitter).events.head);
        // DEQUEUE: advance head
        (*emitter).events.head = (*emitter).events.head.add(1);
    }
    1
}

// ============================================================
// yaml_emitter_need_more_events — static
// ============================================================

unsafe fn yaml_emitter_need_more_events(emitter: *mut yaml_emitter_t) -> i32 {
    if queue_empty((*emitter).events.head, (*emitter).events.tail) { return 1; }

    let accumulate = match (*(*emitter).events.head).type_ {
        YAML_DOCUMENT_START_EVENT => 1isize,
        YAML_SEQUENCE_START_EVENT => 2,
        YAML_MAPPING_START_EVENT => 3,
        _ => return 0,
    };

    if (*emitter).events.tail as isize - (*emitter).events.head as isize
        > accumulate * core::mem::size_of::<yaml_event_t>() as isize
    {
        return 0;
    }

    let mut level: i32 = 0;
    let mut event = (*emitter).events.head;
    while event != (*emitter).events.tail {
        match (*event).type_ {
            YAML_STREAM_START_EVENT
            | YAML_DOCUMENT_START_EVENT
            | YAML_SEQUENCE_START_EVENT
            | YAML_MAPPING_START_EVENT => level += 1,
            YAML_STREAM_END_EVENT
            | YAML_DOCUMENT_END_EVENT
            | YAML_SEQUENCE_END_EVENT
            | YAML_MAPPING_END_EVENT => level -= 1,
            _ => {}
        }
        if level == 0 { return 0; }
        event = event.add(1);
    }
    1
}

// ============================================================
// yaml_emitter_append_tag_directive — static
// ============================================================

unsafe fn yaml_emitter_append_tag_directive(
    emitter: *mut yaml_emitter_t,
    value: yaml_tag_directive_t,
    allow_duplicates: i32,
) -> i32 {
    let mut tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        if libc::strcmp(value.handle as *const c_char, (*tag_directive).handle as *const c_char) == 0 {
            if allow_duplicates != 0 { return 1; }
            return yaml_emitter_set_emitter_error(emitter, b"duplicate %TAG directive\0".as_ptr() as *const c_char);
        }
        tag_directive = tag_directive.add(1);
    }

    let mut copy = yaml_tag_directive_t { handle: core::ptr::null_mut(), prefix: core::ptr::null_mut() };
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if copy.handle.is_null() || copy.prefix.is_null() {
        (*emitter).error = YAML_MEMORY_ERROR;
        yaml_free(copy.handle as *mut c_void);
        yaml_free(copy.prefix as *mut c_void);
        return 0;
    }

    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).tag_directives.start,
        &mut (*emitter).tag_directives.top,
        &mut (*emitter).tag_directives.end,
        copy,
    ) == 0
    {
        yaml_free(copy.handle as *mut c_void);
        yaml_free(copy.prefix as *mut c_void);
        return 0;
    }
    1
}

// ============================================================
// yaml_emitter_increase_indent — static
// ============================================================

unsafe fn yaml_emitter_increase_indent(emitter: *mut yaml_emitter_t, flow: i32, indentless: i32) -> i32 {
    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).indents.start,
        &mut (*emitter).indents.top,
        &mut (*emitter).indents.end,
        (*emitter).indent,
    ) == 0
    {
        return 0;
    }

    if (*emitter).indent < 0 {
        (*emitter).indent = if flow != 0 { (*emitter).best_indent } else { 0 };
    } else if indentless == 0 {
        (*emitter).indent += (*emitter).best_indent;
    }
    1
}

// ============================================================
// yaml_emitter_state_machine — static
// ============================================================

unsafe fn yaml_emitter_state_machine(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    match (*emitter).state {
        YAML_EMIT_STREAM_START_STATE => yaml_emitter_emit_stream_start(emitter, event),
        YAML_EMIT_FIRST_DOCUMENT_START_STATE => yaml_emitter_emit_document_start(emitter, event, 1),
        YAML_EMIT_DOCUMENT_START_STATE => yaml_emitter_emit_document_start(emitter, event, 0),
        YAML_EMIT_DOCUMENT_CONTENT_STATE => yaml_emitter_emit_document_content(emitter, event),
        YAML_EMIT_DOCUMENT_END_STATE => yaml_emitter_emit_document_end(emitter, event),
        YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE => yaml_emitter_emit_flow_sequence_item(emitter, event, 1),
        YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE => yaml_emitter_emit_flow_sequence_item(emitter, event, 0),
        YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE => yaml_emitter_emit_flow_mapping_key(emitter, event, 1),
        YAML_EMIT_FLOW_MAPPING_KEY_STATE => yaml_emitter_emit_flow_mapping_key(emitter, event, 0),
        YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE => yaml_emitter_emit_flow_mapping_value(emitter, event, 1),
        YAML_EMIT_FLOW_MAPPING_VALUE_STATE => yaml_emitter_emit_flow_mapping_value(emitter, event, 0),
        YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE => yaml_emitter_emit_block_sequence_item(emitter, event, 1),
        YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE => yaml_emitter_emit_block_sequence_item(emitter, event, 0),
        YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE => yaml_emitter_emit_block_mapping_key(emitter, event, 1),
        YAML_EMIT_BLOCK_MAPPING_KEY_STATE => yaml_emitter_emit_block_mapping_key(emitter, event, 0),
        YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE => yaml_emitter_emit_block_mapping_value(emitter, event, 1),
        YAML_EMIT_BLOCK_MAPPING_VALUE_STATE => yaml_emitter_emit_block_mapping_value(emitter, event, 0),
        YAML_EMIT_END_STATE => yaml_emitter_set_emitter_error(emitter, b"expected nothing after STREAM-END\0".as_ptr() as *const c_char),
        _ => {
            debug_assert!(true); // Invalid state
            0
        }
    }
}

// ============================================================
// yaml_emitter_set_emitter_error — static
// ============================================================

unsafe fn yaml_emitter_set_emitter_error(emitter: *mut yaml_emitter_t, problem: *const c_char) -> i32 {
    (*emitter).error = YAML_EMITTER_ERROR;
    (*emitter).problem = problem;
    0
}

// ============================================================
// yaml_emitter_emit_stream_start — static
// ============================================================

unsafe fn yaml_emitter_emit_stream_start(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    (*emitter).open_ended = 0;
    if (*event).type_ == YAML_STREAM_START_EVENT {
        if (*emitter).encoding == YAML_ANY_ENCODING {
            (*emitter).encoding = (*event).data.stream_start.encoding;
        }
        if (*emitter).encoding == YAML_ANY_ENCODING {
            (*emitter).encoding = YAML_UTF8_ENCODING;
        }
        if (*emitter).best_indent < 2 || (*emitter).best_indent > 9 {
            (*emitter).best_indent = 2;
        }
        if (*emitter).best_width >= 0 && (*emitter).best_width <= (*emitter).best_indent * 2 {
            (*emitter).best_width = 80;
        }
        if (*emitter).best_width < 0 {
            (*emitter).best_width = i32::MAX;
        }
        if (*emitter).line_break == YAML_ANY_BREAK {
            (*emitter).line_break = YAML_LN_BREAK;
        }
        (*emitter).indent = -1;
        (*emitter).line = 0;
        (*emitter).column = 0;
        (*emitter).whitespace = 1;
        (*emitter).indention = 1;
        if (*emitter).encoding != YAML_UTF8_ENCODING {
            if yaml_emitter_write_bom(emitter) == 0 { return 0; }
        }
        (*emitter).state = YAML_EMIT_FIRST_DOCUMENT_START_STATE;
        return 1;
    }
    yaml_emitter_set_emitter_error(emitter, b"expected STREAM-START\0".as_ptr() as *const c_char)
}

// ============================================================
// yaml_emitter_emit_document_start — static
// ============================================================

unsafe fn yaml_emitter_emit_document_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: i32,
) -> i32 {
    if (*event).type_ == YAML_DOCUMENT_START_EVENT {
        let default_tag_directives: [yaml_tag_directive_t; 3] = [
            yaml_tag_directive_t { handle: b"!\0".as_ptr() as *mut yaml_char_t, prefix: b"!\0".as_ptr() as *mut yaml_char_t },
            yaml_tag_directive_t { handle: b"!!\0".as_ptr() as *mut yaml_char_t, prefix: b"tag:yaml.org,2002:\0".as_ptr() as *mut yaml_char_t },
            yaml_tag_directive_t { handle: core::ptr::null_mut(), prefix: core::ptr::null_mut() },
        ];

        if !(*event).data.document_start.version_directive.is_null() {
            if yaml_emitter_analyze_version_directive(emitter, *(*event).data.document_start.version_directive) == 0 { return 0; }
        }

        let mut tag_directive = (*event).data.document_start.tag_directives.start;
        while tag_directive != (*event).data.document_start.tag_directives.end {
            if yaml_emitter_analyze_tag_directive(emitter, *tag_directive) == 0 { return 0; }
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 0) == 0 { return 0; }
            tag_directive = tag_directive.add(1);
        }

        let mut i = 0;
        while !default_tag_directives[i].handle.is_null() {
            if yaml_emitter_append_tag_directive(emitter, default_tag_directives[i], 1) == 0 { return 0; }
            i += 1;
        }

        let mut implicit = (*event).data.document_start.implicit;
        if first == 0 || (*emitter).canonical != 0 {
            implicit = 0;
        }

        if (!(*event).data.document_start.version_directive.is_null()
            || ((*event).data.document_start.tag_directives.start
                != (*event).data.document_start.tag_directives.end))
            && (*emitter).open_ended != 0
        {
            if yaml_emitter_write_indicator(emitter, b"...\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        }
        (*emitter).open_ended = 0;

        if !(*event).data.document_start.version_directive.is_null() {
            implicit = 0;
            if yaml_emitter_write_indicator(emitter, b"%YAML\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            if (*(*event).data.document_start.version_directive).minor == 1 {
                if yaml_emitter_write_indicator(emitter, b"1.1\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            } else {
                if yaml_emitter_write_indicator(emitter, b"1.2\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            }
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        }

        if (*event).data.document_start.tag_directives.start != (*event).data.document_start.tag_directives.end {
            implicit = 0;
            let mut td = (*event).data.document_start.tag_directives.start;
            while td != (*event).data.document_start.tag_directives.end {
                if yaml_emitter_write_indicator(emitter, b"%TAG\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
                if yaml_emitter_write_tag_handle(emitter, (*td).handle, libc::strlen((*td).handle as *const c_char)) == 0 { return 0; }
                if yaml_emitter_write_tag_content(emitter, (*td).prefix, libc::strlen((*td).prefix as *const c_char), 1) == 0 { return 0; }
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
                td = td.add(1);
            }
        }

        if yaml_emitter_check_empty_document(emitter) != 0 {
            implicit = 0;
        }

        if implicit == 0 {
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
            if yaml_emitter_write_indicator(emitter, b"---\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            if (*emitter).canonical != 0 {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
            }
        }

        (*emitter).state = YAML_EMIT_DOCUMENT_CONTENT_STATE;
        (*emitter).open_ended = 0;
        return 1;
    } else if (*event).type_ == YAML_STREAM_END_EVENT {
        if (*emitter).open_ended == 2 {
            if yaml_emitter_write_indicator(emitter, b"...\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            (*emitter).open_ended = 0;
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        }
        if yaml_emitter_flush(emitter) == 0 { return 0; }
        (*emitter).state = YAML_EMIT_END_STATE;
        return 1;
    }
    yaml_emitter_set_emitter_error(emitter, b"expected DOCUMENT-START or STREAM-END\0".as_ptr() as *const c_char)
}

// ============================================================
// yaml_emitter_emit_document_content — static
// ============================================================

unsafe fn yaml_emitter_emit_document_content(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
        YAML_EMIT_DOCUMENT_END_STATE,
    ) == 0
    {
        return 0;
    }
    yaml_emitter_emit_node(emitter, event, 1, 0, 0, 0)
}

// ============================================================
// yaml_emitter_emit_document_end — static
// ============================================================

unsafe fn yaml_emitter_emit_document_end(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    if (*event).type_ == YAML_DOCUMENT_END_EVENT {
        if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        if (*event).data.document_end.implicit == 0 {
            if yaml_emitter_write_indicator(emitter, b"...\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
            (*emitter).open_ended = 0;
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        } else if (*emitter).open_ended == 0 {
            (*emitter).open_ended = 1;
        }
        if yaml_emitter_flush(emitter) == 0 { return 0; }
        (*emitter).state = YAML_EMIT_DOCUMENT_START_STATE;
        while !stack_empty((*emitter).tag_directives.start, (*emitter).tag_directives.top) {
            let tag_directive: yaml_tag_directive_t = stack_pop(&mut (*emitter).tag_directives.top);
            yaml_free(tag_directive.handle as *mut c_void);
            yaml_free(tag_directive.prefix as *mut c_void);
        }
        return 1;
    }
    yaml_emitter_set_emitter_error(emitter, b"expected DOCUMENT-END\0".as_ptr() as *const c_char)
}

// ============================================================
// yaml_emitter_emit_flow_sequence_item — static
// ============================================================

unsafe fn yaml_emitter_emit_flow_sequence_item(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, first: i32) -> i32 {
    if first != 0 {
        if yaml_emitter_write_indicator(emitter, b"[\0".as_ptr() as *const c_char, 1, 1, 0) == 0 { return 0; }
        if yaml_emitter_increase_indent(emitter, 1, 0) == 0 { return 0; }
        (*emitter).flow_level += 1;
    }

    if (*event).type_ == YAML_SEQUENCE_END_EVENT {
        (*emitter).flow_level -= 1;
        (*emitter).indent = stack_pop(&mut (*emitter).indents.top);
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(emitter, b",\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        }
        if yaml_emitter_write_indicator(emitter, b"]\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
        (*emitter).state = stack_pop(&mut (*emitter).states.top);
        return 1;
    }

    if first == 0 {
        if yaml_emitter_write_indicator(emitter, b",\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 { return 0; }
    }
    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
        YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE,
    ) == 0 { return 0; }
    yaml_emitter_emit_node(emitter, event, 0, 1, 0, 0)
}

// ============================================================
// yaml_emitter_emit_flow_mapping_key — static
// ============================================================

unsafe fn yaml_emitter_emit_flow_mapping_key(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, first: i32) -> i32 {
    if first != 0 {
        if yaml_emitter_write_indicator(emitter, b"{\0".as_ptr() as *const c_char, 1, 1, 0) == 0 { return 0; }
        if yaml_emitter_increase_indent(emitter, 1, 0) == 0 { return 0; }
        (*emitter).flow_level += 1;
    }

    if (*event).type_ == YAML_MAPPING_END_EVENT {
        (*emitter).flow_level -= 1;
        (*emitter).indent = stack_pop(&mut (*emitter).indents.top);
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(emitter, b",\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        }
        if yaml_emitter_write_indicator(emitter, b"}\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
        (*emitter).state = stack_pop(&mut (*emitter).states.top);
        return 1;
    }

    if first == 0 {
        if yaml_emitter_write_indicator(emitter, b",\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 { return 0; }
    }

    if (*emitter).canonical == 0 && yaml_emitter_check_simple_key(emitter) != 0 {
        if stack_push(
            &mut (*emitter).error,
            &mut (*emitter).states.start,
            &mut (*emitter).states.top,
            &mut (*emitter).states.end,
            YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE,
        ) == 0 { return 0; }
        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 1);
    } else {
        if yaml_emitter_write_indicator(emitter, b"?\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
        if stack_push(
            &mut (*emitter).error,
            &mut (*emitter).states.start,
            &mut (*emitter).states.top,
            &mut (*emitter).states.end,
            YAML_EMIT_FLOW_MAPPING_VALUE_STATE,
        ) == 0 { return 0; }
        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0);
    }
}

// ============================================================
// yaml_emitter_emit_flow_mapping_value — static
// ============================================================

unsafe fn yaml_emitter_emit_flow_mapping_value(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, simple: i32) -> i32 {
    if simple != 0 {
        if yaml_emitter_write_indicator(emitter, b":\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
    } else {
        if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
            if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        }
        if yaml_emitter_write_indicator(emitter, b":\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
    }
    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
        YAML_EMIT_FLOW_MAPPING_KEY_STATE,
    ) == 0 { return 0; }
    yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0)
}

// ============================================================
// yaml_emitter_emit_block_sequence_item — static
// ============================================================

unsafe fn yaml_emitter_emit_block_sequence_item(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, first: i32) -> i32 {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0,
            ((*emitter).mapping_context != 0 && (*emitter).indention == 0) as i32) == 0 { return 0; }
    }

    if (*event).type_ == YAML_SEQUENCE_END_EVENT {
        (*emitter).indent = stack_pop(&mut (*emitter).indents.top);
        (*emitter).state = stack_pop(&mut (*emitter).states.top);
        return 1;
    }

    if yaml_emitter_write_indent(emitter) == 0 { return 0; }
    if yaml_emitter_write_indicator(emitter, b"-\0".as_ptr() as *const c_char, 1, 0, 1) == 0 { return 0; }
    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
        YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE,
    ) == 0 { return 0; }
    yaml_emitter_emit_node(emitter, event, 0, 1, 0, 0)
}

// ============================================================
// yaml_emitter_emit_block_mapping_key — static
// ============================================================

unsafe fn yaml_emitter_emit_block_mapping_key(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, first: i32) -> i32 {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0, 0) == 0 { return 0; }
    }

    if (*event).type_ == YAML_MAPPING_END_EVENT {
        (*emitter).indent = stack_pop(&mut (*emitter).indents.top);
        (*emitter).state = stack_pop(&mut (*emitter).states.top);
        return 1;
    }

    if yaml_emitter_write_indent(emitter) == 0 { return 0; }

    if yaml_emitter_check_simple_key(emitter) != 0 {
        if stack_push(
            &mut (*emitter).error,
            &mut (*emitter).states.start,
            &mut (*emitter).states.top,
            &mut (*emitter).states.end,
            YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE,
        ) == 0 { return 0; }
        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 1);
    } else {
        if yaml_emitter_write_indicator(emitter, b"?\0".as_ptr() as *const c_char, 1, 0, 1) == 0 { return 0; }
        if stack_push(
            &mut (*emitter).error,
            &mut (*emitter).states.start,
            &mut (*emitter).states.top,
            &mut (*emitter).states.end,
            YAML_EMIT_BLOCK_MAPPING_VALUE_STATE,
        ) == 0 { return 0; }
        return yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0);
    }
}

// ============================================================
// yaml_emitter_emit_block_mapping_value — static
// ============================================================

unsafe fn yaml_emitter_emit_block_mapping_value(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, simple: i32) -> i32 {
    if simple != 0 {
        if yaml_emitter_write_indicator(emitter, b":\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
    } else {
        if yaml_emitter_write_indent(emitter) == 0 { return 0; }
        if yaml_emitter_write_indicator(emitter, b":\0".as_ptr() as *const c_char, 1, 0, 1) == 0 { return 0; }
    }
    if stack_push(
        &mut (*emitter).error,
        &mut (*emitter).states.start,
        &mut (*emitter).states.top,
        &mut (*emitter).states.end,
        YAML_EMIT_BLOCK_MAPPING_KEY_STATE,
    ) == 0 { return 0; }
    yaml_emitter_emit_node(emitter, event, 0, 0, 1, 0)
}

// ============================================================
// yaml_emitter_emit_node — static
// ============================================================

unsafe fn yaml_emitter_emit_node(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t, root: i32, sequence: i32, mapping: i32, simple_key: i32) -> i32 {
    (*emitter).root_context = root;
    (*emitter).sequence_context = sequence;
    (*emitter).mapping_context = mapping;
    (*emitter).simple_key_context = simple_key;

    match (*event).type_ {
        YAML_ALIAS_EVENT => yaml_emitter_emit_alias(emitter, event),
        YAML_SCALAR_EVENT => yaml_emitter_emit_scalar(emitter, event),
        YAML_SEQUENCE_START_EVENT => yaml_emitter_emit_sequence_start(emitter, event),
        YAML_MAPPING_START_EVENT => yaml_emitter_emit_mapping_start(emitter, event),
        _ => yaml_emitter_set_emitter_error(emitter, b"expected SCALAR, SEQUENCE-START, MAPPING-START, or ALIAS\0".as_ptr() as *const c_char),
    }
}

// ============================================================
// yaml_emitter_emit_alias — static
// ============================================================

unsafe fn yaml_emitter_emit_alias(emitter: *mut yaml_emitter_t, _event: *mut yaml_event_t) -> i32 {
    if yaml_emitter_process_anchor(emitter) == 0 { return 0; }
    if (*emitter).simple_key_context != 0 {
        if emitter_put(emitter, b' ') == 0 { return 0; }
    }
    (*emitter).state = stack_pop(&mut (*emitter).states.top);
    1
}

// ============================================================
// yaml_emitter_emit_scalar — static
// ============================================================

unsafe fn yaml_emitter_emit_scalar(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    if yaml_emitter_select_scalar_style(emitter, event) == 0 { return 0; }
    if yaml_emitter_process_anchor(emitter) == 0 { return 0; }
    if yaml_emitter_process_tag(emitter) == 0 { return 0; }
    if yaml_emitter_increase_indent(emitter, 1, 0) == 0 { return 0; }
    if yaml_emitter_process_scalar(emitter) == 0 { return 0; }
    (*emitter).indent = stack_pop(&mut (*emitter).indents.top);
    (*emitter).state = stack_pop(&mut (*emitter).states.top);
    1
}

// ============================================================
// yaml_emitter_emit_sequence_start — static
// ============================================================

unsafe fn yaml_emitter_emit_sequence_start(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    if yaml_emitter_process_anchor(emitter) == 0 { return 0; }
    if yaml_emitter_process_tag(emitter) == 0 { return 0; }

    if (*emitter).flow_level != 0 || (*emitter).canonical != 0
        || (*event).data.sequence_start.style == YAML_FLOW_SEQUENCE_STYLE
        || yaml_emitter_check_empty_sequence(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE;
    }
    1
}

// ============================================================
// yaml_emitter_emit_mapping_start — static
// ============================================================

unsafe fn yaml_emitter_emit_mapping_start(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    if yaml_emitter_process_anchor(emitter) == 0 { return 0; }
    if yaml_emitter_process_tag(emitter) == 0 { return 0; }

    if (*emitter).flow_level != 0 || (*emitter).canonical != 0
        || (*event).data.mapping_start.style == YAML_FLOW_MAPPING_STYLE
        || yaml_emitter_check_empty_mapping(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE;
    }
    1
}

// ============================================================
// Checkers
// ============================================================

unsafe fn yaml_emitter_check_empty_document(_emitter: *mut yaml_emitter_t) -> i32 {
    0
}

unsafe fn yaml_emitter_check_empty_sequence(emitter: *mut yaml_emitter_t) -> i32 {
    if ((*emitter).events.tail as isize - (*emitter).events.head as isize) < (2 * core::mem::size_of::<yaml_event_t>()) as isize { return 0; }
    ((*(*emitter).events.head).type_ == YAML_SEQUENCE_START_EVENT
        && (*(*emitter).events.head.add(1)).type_ == YAML_SEQUENCE_END_EVENT) as i32
}

unsafe fn yaml_emitter_check_empty_mapping(emitter: *mut yaml_emitter_t) -> i32 {
    if ((*emitter).events.tail as isize - (*emitter).events.head as isize) < (2 * core::mem::size_of::<yaml_event_t>()) as isize { return 0; }
    ((*(*emitter).events.head).type_ == YAML_MAPPING_START_EVENT
        && (*(*emitter).events.head.add(1)).type_ == YAML_MAPPING_END_EVENT) as i32
}

unsafe fn yaml_emitter_check_simple_key(emitter: *mut yaml_emitter_t) -> i32 {
    let event = (*emitter).events.head;
    let mut length: usize = 0;

    match (*event).type_ {
        YAML_ALIAS_EVENT => {
            length += (*emitter).anchor_data.anchor_length;
        }
        YAML_SCALAR_EVENT => {
            if (*emitter).scalar_data.multiline != 0 { return 0; }
            length += (*emitter).anchor_data.anchor_length
                + (*emitter).tag_data.handle_length
                + (*emitter).tag_data.suffix_length
                + (*emitter).scalar_data.length;
        }
        YAML_SEQUENCE_START_EVENT => {
            if yaml_emitter_check_empty_sequence(emitter) == 0 { return 0; }
            length += (*emitter).anchor_data.anchor_length
                + (*emitter).tag_data.handle_length
                + (*emitter).tag_data.suffix_length;
        }
        YAML_MAPPING_START_EVENT => {
            if yaml_emitter_check_empty_mapping(emitter) == 0 { return 0; }
            length += (*emitter).anchor_data.anchor_length
                + (*emitter).tag_data.handle_length
                + (*emitter).tag_data.suffix_length;
        }
        _ => return 0,
    }

    (length <= 128) as i32
}

// ============================================================
// yaml_emitter_select_scalar_style — static
// ============================================================

unsafe fn yaml_emitter_select_scalar_style(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
    let mut style = (*event).data.scalar.style;
    let no_tag = ((*emitter).tag_data.handle.is_null() && (*emitter).tag_data.suffix.is_null()) as i32;

    if no_tag != 0 && (*event).data.scalar.plain_implicit == 0 && (*event).data.scalar.quoted_implicit == 0 {
        return yaml_emitter_set_emitter_error(emitter, b"neither tag nor implicit flags are specified\0".as_ptr() as *const c_char);
    }

    if style == YAML_ANY_SCALAR_STYLE { style = YAML_PLAIN_SCALAR_STYLE; }
    if (*emitter).canonical != 0 { style = YAML_DOUBLE_QUOTED_SCALAR_STYLE; }
    if (*emitter).simple_key_context != 0 && (*emitter).scalar_data.multiline != 0 { style = YAML_DOUBLE_QUOTED_SCALAR_STYLE; }

    if style == YAML_PLAIN_SCALAR_STYLE {
        if ((*emitter).flow_level != 0 && (*emitter).scalar_data.flow_plain_allowed == 0)
            || ((*emitter).flow_level == 0 && (*emitter).scalar_data.block_plain_allowed == 0)
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if (*emitter).scalar_data.length == 0 && ((*emitter).flow_level != 0 || (*emitter).simple_key_context != 0) {
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

    if no_tag != 0 && (*event).data.scalar.quoted_implicit == 0 && style != YAML_PLAIN_SCALAR_STYLE {
        (*emitter).tag_data.handle = b"!\0".as_ptr() as *mut yaml_char_t;
        (*emitter).tag_data.handle_length = 1;
    }

    (*emitter).scalar_data.style = style;
    1
}

// ============================================================
// Processors
// ============================================================

unsafe fn yaml_emitter_process_anchor(emitter: *mut yaml_emitter_t) -> i32 {
    if (*emitter).anchor_data.anchor.is_null() { return 1; }

    let indicator = if (*emitter).anchor_data.alias != 0 { b"*\0".as_ptr() } else { b"&\0".as_ptr() };
    if yaml_emitter_write_indicator(emitter, indicator as *const c_char, 1, 0, 0) == 0 { return 0; }

    yaml_emitter_write_anchor(emitter, (*emitter).anchor_data.anchor, (*emitter).anchor_data.anchor_length)
}

unsafe fn yaml_emitter_process_tag(emitter: *mut yaml_emitter_t) -> i32 {
    if (*emitter).tag_data.handle.is_null() && (*emitter).tag_data.suffix.is_null() { return 1; }

    if !(*emitter).tag_data.handle.is_null() {
        if yaml_emitter_write_tag_handle(emitter, (*emitter).tag_data.handle, (*emitter).tag_data.handle_length) == 0 { return 0; }
        if !(*emitter).tag_data.suffix.is_null() {
            if yaml_emitter_write_tag_content(emitter, (*emitter).tag_data.suffix, (*emitter).tag_data.suffix_length, 0) == 0 { return 0; }
        }
    } else {
        if yaml_emitter_write_indicator(emitter, b"!<\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
        if yaml_emitter_write_tag_content(emitter, (*emitter).tag_data.suffix, (*emitter).tag_data.suffix_length, 0) == 0 { return 0; }
        if yaml_emitter_write_indicator(emitter, b">\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
    }
    1
}

unsafe fn yaml_emitter_process_scalar(emitter: *mut yaml_emitter_t) -> i32 {
    match (*emitter).scalar_data.style {
        YAML_PLAIN_SCALAR_STYLE => yaml_emitter_write_plain_scalar(emitter, (*emitter).scalar_data.value, (*emitter).scalar_data.length, ((*emitter).simple_key_context == 0) as i32),
        YAML_SINGLE_QUOTED_SCALAR_STYLE => yaml_emitter_write_single_quoted_scalar(emitter, (*emitter).scalar_data.value, (*emitter).scalar_data.length, ((*emitter).simple_key_context == 0) as i32),
        YAML_DOUBLE_QUOTED_SCALAR_STYLE => yaml_emitter_write_double_quoted_scalar(emitter, (*emitter).scalar_data.value, (*emitter).scalar_data.length, ((*emitter).simple_key_context == 0) as i32),
        YAML_LITERAL_SCALAR_STYLE => yaml_emitter_write_literal_scalar(emitter, (*emitter).scalar_data.value, (*emitter).scalar_data.length),
        YAML_FOLDED_SCALAR_STYLE => yaml_emitter_write_folded_scalar(emitter, (*emitter).scalar_data.value, (*emitter).scalar_data.length),
        _ => {
            debug_assert!(true);
            0
        }
    }
}

// ============================================================
// Analyzers
// ============================================================

unsafe fn yaml_emitter_analyze_version_directive(emitter: *mut yaml_emitter_t, version_directive: yaml_version_directive_t) -> i32 {
    if version_directive.major != 1 || (version_directive.minor != 1 && version_directive.minor != 2) {
        return yaml_emitter_set_emitter_error(emitter, b"incompatible %YAML directive\0".as_ptr() as *const c_char);
    }
    1
}

unsafe fn yaml_emitter_analyze_tag_directive(emitter: *mut yaml_emitter_t, tag_directive: yaml_tag_directive_t) -> i32 {
    let handle_length = libc::strlen(tag_directive.handle as *const c_char);
    let prefix_length = libc::strlen(tag_directive.prefix as *const c_char);
    let handle = yaml_string_t { start: tag_directive.handle, end: tag_directive.handle.add(handle_length), pointer: tag_directive.handle };
    let prefix = yaml_string_t { start: tag_directive.prefix, end: tag_directive.prefix.add(prefix_length), pointer: tag_directive.prefix };

    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(emitter, b"tag handle must not be empty\0".as_ptr() as *const c_char);
    }
    if *handle.start != b'!' {
        return yaml_emitter_set_emitter_error(emitter, b"tag handle must start with '!'\0".as_ptr() as *const c_char);
    }
    if *handle.end.sub(1) != b'!' {
        return yaml_emitter_set_emitter_error(emitter, b"tag handle must end with '!'\0".as_ptr() as *const c_char);
    }
    let mut p = handle.start.add(1);
    while p < handle.end.sub(1) {
        if !str_is_alpha_ptr(p) {
            return yaml_emitter_set_emitter_error(emitter, b"tag handle must contain alphanumerical characters only\0".as_ptr() as *const c_char);
        }
        p = p.add(str_width_ptr(p));
    }
    if prefix.start == prefix.end {
        return yaml_emitter_set_emitter_error(emitter, b"tag prefix must not be empty\0".as_ptr() as *const c_char);
    }
    1
}

unsafe fn str_is_alpha_ptr(p: *const yaml_char_t) -> bool {
    let c = *p;
    (c >= b'0' && c <= b'9') || (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z') || c == b'_' || c == b'-'
}

unsafe fn str_width_ptr(p: *const yaml_char_t) -> usize {
    let c = *p;
    if (c & 0x80) == 0 { 1 }
    else if (c & 0xE0) == 0xC0 { 2 }
    else if (c & 0xF0) == 0xE0 { 3 }
    else if (c & 0xF8) == 0xF0 { 4 }
    else { 0 }
}

unsafe fn yaml_emitter_analyze_anchor(emitter: *mut yaml_emitter_t, anchor: *mut yaml_char_t, alias: i32) -> i32 {
    let anchor_length = libc::strlen(anchor as *const c_char);
    let string = yaml_string_t { start: anchor, end: anchor.add(anchor_length), pointer: anchor };

    if string.start == string.end {
        return yaml_emitter_set_emitter_error(emitter, if alias != 0 {
            b"alias value must not be empty\0".as_ptr() as *const c_char
        } else {
            b"anchor value must not be empty\0".as_ptr() as *const c_char
        });
    }

    let mut p = string.pointer;
    while p != string.end {
        if !str_is_alpha_ptr(p) {
            return yaml_emitter_set_emitter_error(emitter, if alias != 0 {
                b"alias value must contain alphanumerical characters only\0".as_ptr() as *const c_char
            } else {
                b"anchor value must contain alphanumerical characters only\0".as_ptr() as *const c_char
            });
        }
        p = p.add(str_width_ptr(p));
    }

    (*emitter).anchor_data.anchor = string.start;
    (*emitter).anchor_data.anchor_length = string.end as usize - string.start as usize;
    (*emitter).anchor_data.alias = alias;
    1
}

unsafe fn yaml_emitter_analyze_tag(emitter: *mut yaml_emitter_t, tag: *mut yaml_char_t) -> i32 {
    let tag_length = libc::strlen(tag as *const c_char);
    let string = yaml_string_t { start: tag, end: tag.add(tag_length), pointer: tag };

    if string.start == string.end {
        return yaml_emitter_set_emitter_error(emitter, b"tag value must not be empty\0".as_ptr() as *const c_char);
    }

    let mut tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        let prefix_length = libc::strlen((*tag_directive).prefix as *const c_char);
        if prefix_length < (string.end as usize - string.start as usize)
            && libc::strncmp((*tag_directive).prefix as *const c_char, string.start as *const c_char, prefix_length) == 0
        {
            (*emitter).tag_data.handle = (*tag_directive).handle;
            (*emitter).tag_data.handle_length = libc::strlen((*tag_directive).handle as *const c_char);
            (*emitter).tag_data.suffix = string.start.add(prefix_length);
            (*emitter).tag_data.suffix_length = (string.end as usize - string.start as usize) - prefix_length;
            return 1;
        }
        tag_directive = tag_directive.add(1);
    }

    (*emitter).tag_data.suffix = string.start;
    (*emitter).tag_data.suffix_length = string.end as usize - string.start as usize;
    1
}

unsafe fn yaml_emitter_analyze_scalar(emitter: *mut yaml_emitter_t, value: *mut yaml_char_t, length: usize) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };

    let mut block_indicators = 0i32;
    let mut flow_indicators = 0i32;
    let mut line_breaks = 0i32;
    let mut special_characters = 0i32;

    let mut leading_space = 0i32;
    let mut leading_break = 0i32;
    let mut trailing_space = 0i32;
    let mut trailing_break = 0i32;
    let mut break_space = 0i32;
    let mut space_break = 0i32;

    let mut preceded_by_whitespace = 0i32;
    let mut followed_by_whitespace = 0i32;
    let mut previous_space = 0i32;
    let mut previous_break = 0i32;

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

    if (*string.start == b'-' && *string.start.add(1) == b'-' && *string.start.add(2) == b'-')
        || (*string.start == b'.' && *string.start.add(1) == b'.' && *string.start.add(2) == b'.')
    {
        block_indicators = 1;
        flow_indicators = 1;
    }

    preceded_by_whitespace = 1;
    followed_by_whitespace = str_is_blankz_at_ptr(string.pointer, str_width_ptr(string.pointer)) as i32;

    while string.pointer != string.end {
        if string.start == string.pointer {
            let c = *string.pointer;
            if c == b'#' || c == b',' || c == b'[' || c == b']' || c == b'{' || c == b'}'
                || c == b'&' || c == b'*' || c == b'!' || c == b'|' || c == b'>'
                || c == b'\'' || c == b'"' || c == b'%' || c == b'@' || c == b'`'
            {
                flow_indicators = 1;
                block_indicators = 1;
            }
            if c == b'?' || c == b':' {
                flow_indicators = 1;
                if followed_by_whitespace != 0 { block_indicators = 1; }
            }
            if c == b'-' && followed_by_whitespace != 0 {
                flow_indicators = 1;
                block_indicators = 1;
            }
        } else {
            let c = *string.pointer;
            if c == b',' || c == b'?' || c == b'[' || c == b']' || c == b'{' || c == b'}' {
                flow_indicators = 1;
            }
            if c == b':' {
                flow_indicators = 1;
                if followed_by_whitespace != 0 { block_indicators = 1; }
            }
            if c == b'#' && preceded_by_whitespace != 0 {
                flow_indicators = 1;
                block_indicators = 1;
            }
        }

        if !str_is_printable_ptr(string.pointer) || (!str_is_ascii_ptr(string.pointer) && (*emitter).unicode == 0) {
            special_characters = 1;
        }
        if str_is_break_ptr(string.pointer) {
            line_breaks = 1;
        }

        let w = str_width_ptr(string.pointer);

        if str_is_space_ptr(string.pointer) {
            if string.start == string.pointer { leading_space = 1; }
            if string.pointer.add(w) == string.end { trailing_space = 1; }
            if previous_break != 0 { break_space = 1; }
            previous_space = 1;
            previous_break = 0;
        } else if str_is_break_ptr(string.pointer) {
            if string.start == string.pointer { leading_break = 1; }
            if string.pointer.add(w) == string.end { trailing_break = 1; }
            if previous_space != 0 { space_break = 1; }
            previous_space = 0;
            previous_break = 1;
        } else {
            previous_space = 0;
            previous_break = 0;
        }

        preceded_by_whitespace = str_is_blankz_ptr(string.pointer) as i32;
        string.pointer = string.pointer.add(w);
        if string.pointer != string.end {
            followed_by_whitespace = str_is_blankz_at_ptr(string.pointer, str_width_ptr(string.pointer)) as i32;
        }
    }

    (*emitter).scalar_data.multiline = line_breaks;
    (*emitter).scalar_data.flow_plain_allowed = 1;
    (*emitter).scalar_data.block_plain_allowed = 1;
    (*emitter).scalar_data.single_quoted_allowed = 1;
    (*emitter).scalar_data.block_allowed = 1;

    if leading_space != 0 || leading_break != 0 || trailing_space != 0 || trailing_break != 0 {
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

// Helper: check char properties directly via pointer (for emitter string analysis)
unsafe fn str_is_printable_ptr(p: *const yaml_char_t) -> bool {
    let c = *p;
    if (c & 0x80) == 0 {
        c == b'\t' || c == b'\n' || c == b'\r' || (c >= 0x20 && c <= 0x7E)
    } else if (c & 0xE0) == 0xC0 {
        let c2 = *p.add(1);
        let v = (((c & 0x1F) as u32) << 6) | ((c2 & 0x3F) as u32);
        v == 0x85 || v >= 0xA0
    } else if (c & 0xF0) == 0xE0 {
        let c2 = *p.add(1);
        let c3 = *p.add(2);
        let v = (((c & 0x0F) as u32) << 12) | (((c2 & 0x3F) as u32) << 6) | ((c3 & 0x3F) as u32);
        (v >= 0xA0 && v <= 0xD7FF) || (v >= 0xE000 && v <= 0xFFFD && v != 0xFEFF)
    } else if (c & 0xF8) == 0xF0 {
        let c2 = *p.add(1);
        let c3 = *p.add(2);
        let c4 = *p.add(3);
        let v = (((c & 0x07) as u32) << 18) | (((c2 & 0x3F) as u32) << 12) | (((c3 & 0x3F) as u32) << 6) | ((c4 & 0x3F) as u32);
        v >= 0x10000 && v <= 0x10FFFF
    } else { false }
}

unsafe fn str_is_ascii_ptr(p: *const yaml_char_t) -> bool {
    (*p & 0x80) == 0
}

unsafe fn str_is_space_ptr(p: *const yaml_char_t) -> bool {
    *p == b' '
}

unsafe fn str_is_break_ptr(p: *const yaml_char_t) -> bool {
    let c = *p;
    c == b'\r' || c == b'\n'
        || (c == 0xC2 && *p.add(1) == 0x85)
        || (c == 0xE2 && *p.add(1) == 0x80 && (*p.add(2) == 0xA8 || *p.add(2) == 0xA9))
}

unsafe fn str_is_blankz_ptr(p: *const yaml_char_t) -> bool {
    let c = *p;
    c == 0 || c == b' ' || c == b'\t' || str_is_break_ptr(p)
}

unsafe fn str_is_blankz_at_ptr(p: *const yaml_char_t, offset: usize) -> bool {
    str_is_blankz_ptr(p.add(offset))
}

unsafe fn yaml_emitter_analyze_event(emitter: *mut yaml_emitter_t, event: *mut yaml_event_t) -> i32 {
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
            if yaml_emitter_analyze_anchor(emitter, (*event).data.alias.anchor, 1) == 0 { return 0; }
            1
        }
        YAML_SCALAR_EVENT => {
            if !(*event).data.scalar.anchor.is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.scalar.anchor, 0) == 0 { return 0; }
            }
            if !(*event).data.scalar.tag.is_null()
                && ((*emitter).canonical != 0
                    || ((*event).data.scalar.plain_implicit == 0 && (*event).data.scalar.quoted_implicit == 0))
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.scalar.tag) == 0 { return 0; }
            }
            if yaml_emitter_analyze_scalar(emitter, (*event).data.scalar.value, (*event).data.scalar.length) == 0 { return 0; }
            1
        }
        YAML_SEQUENCE_START_EVENT => {
            if !(*event).data.sequence_start.anchor.is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.sequence_start.anchor, 0) == 0 { return 0; }
            }
            if !(*event).data.sequence_start.tag.is_null()
                && ((*emitter).canonical != 0 || (*event).data.sequence_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.sequence_start.tag) == 0 { return 0; }
            }
            1
        }
        YAML_MAPPING_START_EVENT => {
            if !(*event).data.mapping_start.anchor.is_null() {
                if yaml_emitter_analyze_anchor(emitter, (*event).data.mapping_start.anchor, 0) == 0 { return 0; }
            }
            if !(*event).data.mapping_start.tag.is_null()
                && ((*emitter).canonical != 0 || (*event).data.mapping_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.mapping_start.tag) == 0 { return 0; }
            }
            1
        }
        _ => 1,
    }
}

// ============================================================
// Writers
// ============================================================

unsafe fn yaml_emitter_write_bom(emitter: *mut yaml_emitter_t) -> i32 {
    if emitter_flush_check(emitter) == 0 { return 0; }
    *(*emitter).buffer.pointer = 0xEF; (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    *(*emitter).buffer.pointer = 0xBB; (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    *(*emitter).buffer.pointer = 0xBF; (*emitter).buffer.pointer = (*emitter).buffer.pointer.add(1);
    1
}

unsafe fn yaml_emitter_write_indent(emitter: *mut yaml_emitter_t) -> i32 {
    let indent = if (*emitter).indent >= 0 { (*emitter).indent } else { 0 };

    if (*emitter).indention == 0 || (*emitter).column > indent
        || ((*emitter).column == indent && (*emitter).whitespace == 0)
    {
        if emitter_put_break(emitter) == 0 { return 0; }
    }

    while (*emitter).column < indent {
        if emitter_put(emitter, b' ') == 0 { return 0; }
    }

    (*emitter).whitespace = 1;
    (*emitter).indention = 1;
    1
}

unsafe fn yaml_emitter_write_indicator(
    emitter: *mut yaml_emitter_t,
    indicator: *const c_char,
    need_whitespace: i32,
    is_whitespace: i32,
    is_indention: i32,
) -> i32 {
    let indicator_length = libc::strlen(indicator);
    let mut string = yaml_string_t {
        start: indicator as *mut yaml_char_t,
        end: (indicator as *mut yaml_char_t).add(indicator_length),
        pointer: indicator as *mut yaml_char_t,
    };

    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if emitter_put(emitter, b' ') == 0 { return 0; }
    }

    while string.pointer != string.end {
        if emitter_write(emitter, &mut string) == 0 { return 0; }
    }

    (*emitter).whitespace = is_whitespace;
    (*emitter).indention = ((*emitter).indention != 0 && is_indention != 0) as i32;
    1
}

unsafe fn yaml_emitter_write_anchor(emitter: *mut yaml_emitter_t, value: *mut yaml_char_t, length: usize) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };

    while string.pointer != string.end {
        if emitter_write(emitter, &mut string) == 0 { return 0; }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;
    1
}

unsafe fn yaml_emitter_write_tag_handle(emitter: *mut yaml_emitter_t, value: *mut yaml_char_t, length: usize) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };

    if (*emitter).whitespace == 0 {
        if emitter_put(emitter, b' ') == 0 { return 0; }
    }

    while string.pointer != string.end {
        if emitter_write(emitter, &mut string) == 0 { return 0; }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;
    1
}

unsafe fn yaml_emitter_write_tag_content(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: usize,
    need_whitespace: i32,
) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };

    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if emitter_put(emitter, b' ') == 0 { return 0; }
    }

    while string.pointer != string.end {
        if str_is_alpha_ptr(string.pointer)
            || *string.pointer == b';' || *string.pointer == b'/'
            || *string.pointer == b'?' || *string.pointer == b':'
            || *string.pointer == b'@' || *string.pointer == b'&'
            || *string.pointer == b'=' || *string.pointer == b'+'
            || *string.pointer == b'$' || *string.pointer == b','
            || *string.pointer == b'_' || *string.pointer == b'.'
            || *string.pointer == b'~' || *string.pointer == b'*'
            || *string.pointer == b'\'' || *string.pointer == b'('
            || *string.pointer == b')' || *string.pointer == b'['
            || *string.pointer == b']'
        {
            if emitter_write(emitter, &mut string) == 0 { return 0; }
        } else {
            let w = str_width_ptr(string.pointer);
            let mut i = 0;
            while i < w {
                let v = *string.pointer as u32;
                string.pointer = string.pointer.add(1);
                if emitter_put(emitter, b'%') == 0 { return 0; }
                let hi = (v >> 4) as u8;
                if emitter_put(emitter, hi + if hi < 10 { b'0' } else { b'A' - 10 }) == 0 { return 0; }
                let lo = (v & 0x0F) as u8;
                if emitter_put(emitter, lo + if lo < 10 { b'0' } else { b'A' - 10 }) == 0 { return 0; }
                i += 1;
            }
        }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;
    1
}

unsafe fn yaml_emitter_write_plain_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: usize,
    allow_breaks: i32,
) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };
    let mut spaces = 0i32;
    let mut breaks = 0i32;

    if (*emitter).whitespace == 0 && (length > 0 || (*emitter).flow_level != 0) {
        if emitter_put(emitter, b' ') == 0 { return 0; }
    }

    while string.pointer != string.end {
        if str_is_space_ptr(string.pointer) {
            if allow_breaks != 0 && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !str_is_space_ptr(string.pointer.add(str_width_ptr(string.pointer)))
            {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
                string.pointer = string.pointer.add(str_width_ptr(string.pointer));
            } else {
                if emitter_write(emitter, &mut string) == 0 { return 0; }
            }
            spaces = 1;
        } else if str_is_break_ptr(string.pointer) {
            if breaks == 0 && *string.pointer == b'\n' {
                if emitter_put_break(emitter) == 0 { return 0; }
            }
            if emitter_write_break(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
            }
            if emitter_write(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 0;
            spaces = 0;
            breaks = 0;
        }
    }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;
    1
}

unsafe fn yaml_emitter_write_single_quoted_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: usize,
    allow_breaks: i32,
) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };
    let mut spaces = 0i32;
    let mut breaks = 0i32;

    if yaml_emitter_write_indicator(emitter, b"'\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }

    while string.pointer != string.end {
        if str_is_space_ptr(string.pointer) {
            if allow_breaks != 0 && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.sub(1)
                && !str_is_space_ptr(string.pointer.add(str_width_ptr(string.pointer)))
            {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
                string.pointer = string.pointer.add(str_width_ptr(string.pointer));
            } else {
                if emitter_write(emitter, &mut string) == 0 { return 0; }
            }
            spaces = 1;
        } else if str_is_break_ptr(string.pointer) {
            if breaks == 0 && *string.pointer == b'\n' {
                if emitter_put_break(emitter) == 0 { return 0; }
            }
            if emitter_write_break(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
            }
            if *string.pointer == b'\'' {
                if emitter_put(emitter, b'\'') == 0 { return 0; }
            }
            if emitter_write(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 0;
            spaces = 0;
            breaks = 0;
        }
    }

    if breaks != 0 {
        if yaml_emitter_write_indent(emitter) == 0 { return 0; }
    }

    if yaml_emitter_write_indicator(emitter, b"'\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;
    1
}

unsafe fn yaml_emitter_write_double_quoted_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: usize,
    allow_breaks: i32,
) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };
    let mut spaces = 0i32;

    if yaml_emitter_write_indicator(emitter, b"\"\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }

    while string.pointer != string.end {
        if !str_is_printable_ptr(string.pointer)
            || ((*emitter).unicode == 0 && !str_is_ascii_ptr(string.pointer))
            || (*string.pointer == 0xEF && *string.pointer.add(1) == 0xBB && *string.pointer.add(2) == 0xBF)
            || str_is_break_ptr(string.pointer)
            || *string.pointer == b'"'
            || *string.pointer == b'\\'
        {
            let octet = *string.pointer;
            let w = str_width_ptr(string.pointer) as u32;
            let mut char_value: u32 = match (octet & 0xF8) {
                v if (v & 0x80) == 0x00 => (octet & 0x7F) as u32,
                v if (v & 0xE0) == 0xC0 => (octet & 0x1F) as u32,
                v if (v & 0xF0) == 0xE0 => (octet & 0x0F) as u32,
                v if (v & 0xF8) == 0xF0 => (octet & 0x07) as u32,
                _ => 0,
            };
            for k in 1..w {
                let o2 = *string.pointer.add(k as usize);
                char_value = (char_value << 6) + (o2 & 0x3F) as u32;
            }
            string.pointer = string.pointer.add(w as usize);

            if emitter_put(emitter, b'\\') == 0 { return 0; }

            match char_value {
                0x00 => { if emitter_put(emitter, b'0') == 0 { return 0; } }
                0x07 => { if emitter_put(emitter, b'a') == 0 { return 0; } }
                0x08 => { if emitter_put(emitter, b'b') == 0 { return 0; } }
                0x09 => { if emitter_put(emitter, b't') == 0 { return 0; } }
                0x0A => { if emitter_put(emitter, b'n') == 0 { return 0; } }
                0x0B => { if emitter_put(emitter, b'v') == 0 { return 0; } }
                0x0C => { if emitter_put(emitter, b'f') == 0 { return 0; } }
                0x0D => { if emitter_put(emitter, b'r') == 0 { return 0; } }
                0x1B => { if emitter_put(emitter, b'e') == 0 { return 0; } }
                0x22 => { if emitter_put(emitter, b'"') == 0 { return 0; } }
                0x5C => { if emitter_put(emitter, b'\\') == 0 { return 0; } }
                0x85 => { if emitter_put(emitter, b'N') == 0 { return 0; } }
                0xA0 => { if emitter_put(emitter, b'_') == 0 { return 0; } }
                0x2028 => { if emitter_put(emitter, b'L') == 0 { return 0; } }
                0x2029 => { if emitter_put(emitter, b'P') == 0 { return 0; } }
                v => {
                    let (prefix, width) = if v <= 0xFF { (b'x', 2u32) }
                        else if v <= 0xFFFF { (b'u', 4u32) }
                        else { (b'U', 8u32) };
                    if emitter_put(emitter, prefix) == 0 { return 0; }
                    let mut k = (width - 1) as i32 * 4;
                    while k >= 0 {
                        let digit = ((v >> k) & 0x0F) as u8;
                        if emitter_put(emitter, digit + if digit < 10 { b'0' } else { b'A' - 10 }) == 0 { return 0; }
                        k -= 4;
                    }
                }
            }
            spaces = 0;
        } else if str_is_space_ptr(string.pointer) {
            if allow_breaks != 0 && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.sub(1)
            {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
                if str_is_space_ptr(string.pointer.add(str_width_ptr(string.pointer))) {
                    if emitter_put(emitter, b'\\') == 0 { return 0; }
                }
                string.pointer = string.pointer.add(str_width_ptr(string.pointer));
            } else {
                if emitter_write(emitter, &mut string) == 0 { return 0; }
            }
            spaces = 1;
        } else {
            if emitter_write(emitter, &mut string) == 0 { return 0; }
            spaces = 0;
        }
    }

    if yaml_emitter_write_indicator(emitter, b"\"\0".as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }

    (*emitter).whitespace = 0;
    (*emitter).indention = 0;
    1
}

unsafe fn yaml_emitter_write_block_scalar_hints(emitter: *mut yaml_emitter_t, string: yaml_string_t) -> i32 {
    let mut indent_hint_buf = [0u8; 2];

    if str_is_space_ptr(string.pointer) || str_is_break_ptr(string.pointer) {
        indent_hint_buf[0] = b'0' + (*emitter).best_indent as u8;
        if yaml_emitter_write_indicator(emitter, indent_hint_buf.as_ptr() as *const c_char, 0, 0, 0) == 0 { return 0; }
    }

    (*emitter).open_ended = 0;

    let mut ptr = string.end;
    if string.start == ptr {
        // Empty string: clip
        let chomp_hint = b"-\0".as_ptr() as *const c_char;
        if yaml_emitter_write_indicator(emitter, chomp_hint, 0, 0, 0) == 0 { return 0; }
    } else {
        // Back up one UTF-8 char
        loop {
            ptr = ptr.sub(1);
            if (*ptr & 0xC0) != 0x80 { break; }
        }
        if !str_is_break_ptr(ptr) {
            let chomp_hint = b"-\0".as_ptr() as *const c_char;
            if yaml_emitter_write_indicator(emitter, chomp_hint, 0, 0, 0) == 0 { return 0; }
        } else if string.start == ptr {
            // Only one break
            let chomp_hint = b"+\0".as_ptr() as *const c_char;
            (*emitter).open_ended = 2;
            if yaml_emitter_write_indicator(emitter, chomp_hint, 0, 0, 0) == 0 { return 0; }
        } else {
            // Back up one more
            let mut ptr2 = ptr;
            loop {
                ptr2 = ptr2.sub(1);
                if (*ptr2 & 0xC0) != 0x80 { break; }
            }
            if str_is_break_ptr(ptr2) {
                let chomp_hint = b"+\0".as_ptr() as *const c_char;
                (*emitter).open_ended = 2;
                if yaml_emitter_write_indicator(emitter, chomp_hint, 0, 0, 0) == 0 { return 0; }
            }
        }
    }
    1
}

unsafe fn yaml_emitter_write_literal_scalar(emitter: *mut yaml_emitter_t, value: *mut yaml_char_t, length: usize) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };
    let mut breaks = 1i32;

    if yaml_emitter_write_indicator(emitter, b"|\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 { return 0; }
    if emitter_put_break(emitter) == 0 { return 0; }
    (*emitter).indention = 1;
    (*emitter).whitespace = 1;

    while string.pointer != string.end {
        if str_is_break_ptr(string.pointer) {
            if emitter_write_break(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
            }
            if emitter_write(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 0;
            breaks = 0;
        }
    }
    1
}

unsafe fn yaml_emitter_write_folded_scalar(emitter: *mut yaml_emitter_t, value: *mut yaml_char_t, length: usize) -> i32 {
    let mut string = yaml_string_t { start: value, end: value.add(length), pointer: value };
    let mut breaks = 1i32;
    let mut leading_spaces = 1i32;

    if yaml_emitter_write_indicator(emitter, b">\0".as_ptr() as *const c_char, 1, 0, 0) == 0 { return 0; }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 { return 0; }
    if emitter_put_break(emitter) == 0 { return 0; }
    (*emitter).indention = 1;
    (*emitter).whitespace = 1;

    while string.pointer != string.end {
        if str_is_break_ptr(string.pointer) {
            if breaks == 0 && leading_spaces == 0 && *string.pointer == b'\n' {
                // Look ahead for non-blankz
                let mut k: usize = 0;
                while str_is_break_ptr(string.pointer.add(k)) {
                    k += str_width_ptr(string.pointer.add(k));
                }
                if !str_is_blankz_ptr(string.pointer.add(k)) {
                    if emitter_put_break(emitter) == 0 { return 0; }
                }
            }
            if emitter_write_break(emitter, &mut string) == 0 { return 0; }
            (*emitter).indention = 1;
            breaks = 1;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
                leading_spaces = str_is_space_ptr(string.pointer) as i32;
            }
            if breaks == 0 && str_is_space_ptr(string.pointer)
                && !str_is_space_ptr(string.pointer.add(str_width_ptr(string.pointer)))
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 { return 0; }
                string.pointer = string.pointer.add(str_width_ptr(string.pointer));
            } else {
                if emitter_write(emitter, &mut string) == 0 { return 0; }
            }
            (*emitter).indention = 0;
            breaks = 0;
        }
    }
    1
}


mod test_bridge;
