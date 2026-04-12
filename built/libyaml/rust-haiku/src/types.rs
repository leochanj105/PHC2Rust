// Complete C-compatible type definitions for libyaml
// All types use #[repr(C)] for C interoperability and are Copy+Clone for direct C passing

use std::ffi::c_void;

// ============================================================================
// Basic Types
// ============================================================================

/// The character type (UTF-8 octet)
pub type yaml_char_t = u8;

/// The stream encoding
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_encoding_t {
    /// Let the parser choose the encoding
    YAML_ANY_ENCODING = 0,
    /// The default UTF-8 encoding
    YAML_UTF8_ENCODING = 1,
    /// The UTF-16-LE encoding with BOM
    YAML_UTF16LE_ENCODING = 2,
    /// The UTF-16-BE encoding with BOM
    YAML_UTF16BE_ENCODING = 3,
}

/// Line break types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_break_t {
    /// Let the parser choose the break type
    YAML_ANY_BREAK = 0,
    /// Use CR for line breaks (Mac style)
    YAML_CR_BREAK = 1,
    /// Use LN for line breaks (Unix style)
    YAML_LN_BREAK = 2,
    /// Use CR LN for line breaks (DOS style)
    YAML_CRLN_BREAK = 3,
}

/// Error types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_error_type_t {
    /// No error is produced
    YAML_NO_ERROR = 0,
    /// Cannot allocate or reallocate a block of memory
    YAML_MEMORY_ERROR = 1,
    /// Cannot read or decode the input stream
    YAML_READER_ERROR = 2,
    /// Cannot scan the input stream
    YAML_SCANNER_ERROR = 3,
    /// Cannot parse the input stream
    YAML_PARSER_ERROR = 4,
    /// Cannot compose a YAML document
    YAML_COMPOSER_ERROR = 5,
    /// Cannot write to the output stream
    YAML_WRITER_ERROR = 6,
    /// Cannot emit a YAML stream
    YAML_EMITTER_ERROR = 7,
}

/// The position marker
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_mark_t {
    /// The position index
    pub index: usize,
    /// The position line
    pub line: usize,
    /// The position column
    pub column: usize,
}

/// The version directive data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_version_directive_t {
    /// The major version number
    pub major: i32,
    /// The minor version number
    pub minor: i32,
}

/// The tag directive data
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_tag_directive_t {
    /// The tag handle
    pub handle: *mut yaml_char_t,
    /// The tag prefix
    pub prefix: *mut yaml_char_t,
}

// ============================================================================
// Node Styles
// ============================================================================

/// Scalar styles
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_scalar_style_t {
    /// Let the emitter choose the style
    YAML_ANY_SCALAR_STYLE = 0,
    /// The plain scalar style
    YAML_PLAIN_SCALAR_STYLE = 1,
    /// The single-quoted scalar style
    YAML_SINGLE_QUOTED_SCALAR_STYLE = 2,
    /// The double-quoted scalar style
    YAML_DOUBLE_QUOTED_SCALAR_STYLE = 3,
    /// The literal scalar style
    YAML_LITERAL_SCALAR_STYLE = 4,
    /// The folded scalar style
    YAML_FOLDED_SCALAR_STYLE = 5,
}

/// Sequence styles
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_sequence_style_t {
    /// Let the emitter choose the style
    YAML_ANY_SEQUENCE_STYLE = 0,
    /// The block sequence style
    YAML_BLOCK_SEQUENCE_STYLE = 1,
    /// The flow sequence style
    YAML_FLOW_SEQUENCE_STYLE = 2,
}

/// Mapping styles
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_mapping_style_t {
    /// Let the emitter choose the style
    YAML_ANY_MAPPING_STYLE = 0,
    /// The block mapping style
    YAML_BLOCK_MAPPING_STYLE = 1,
    /// The flow mapping style
    YAML_FLOW_MAPPING_STYLE = 2,
}

// ============================================================================
// Tokens
// ============================================================================

/// Token types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_token_type_t {
    /// An empty token
    YAML_NO_TOKEN = 0,
    /// A STREAM-START token
    YAML_STREAM_START_TOKEN = 1,
    /// A STREAM-END token
    YAML_STREAM_END_TOKEN = 2,
    /// A VERSION-DIRECTIVE token
    YAML_VERSION_DIRECTIVE_TOKEN = 3,
    /// A TAG-DIRECTIVE token
    YAML_TAG_DIRECTIVE_TOKEN = 4,
    /// A DOCUMENT-START token
    YAML_DOCUMENT_START_TOKEN = 5,
    /// A DOCUMENT-END token
    YAML_DOCUMENT_END_TOKEN = 6,
    /// A BLOCK-SEQUENCE-START token
    YAML_BLOCK_SEQUENCE_START_TOKEN = 7,
    /// A BLOCK-MAPPING-START token
    YAML_BLOCK_MAPPING_START_TOKEN = 8,
    /// A BLOCK-END token
    YAML_BLOCK_END_TOKEN = 9,
    /// A FLOW-SEQUENCE-START token
    YAML_FLOW_SEQUENCE_START_TOKEN = 10,
    /// A FLOW-SEQUENCE-END token
    YAML_FLOW_SEQUENCE_END_TOKEN = 11,
    /// A FLOW-MAPPING-START token
    YAML_FLOW_MAPPING_START_TOKEN = 12,
    /// A FLOW-MAPPING-END token
    YAML_FLOW_MAPPING_END_TOKEN = 13,
    /// A BLOCK-ENTRY token
    YAML_BLOCK_ENTRY_TOKEN = 14,
    /// A FLOW-ENTRY token
    YAML_FLOW_ENTRY_TOKEN = 15,
    /// A KEY token
    YAML_KEY_TOKEN = 16,
    /// A VALUE token
    YAML_VALUE_TOKEN = 17,
    /// An ALIAS token
    YAML_ALIAS_TOKEN = 18,
    /// An ANCHOR token
    YAML_ANCHOR_TOKEN = 19,
    /// A TAG token
    YAML_TAG_TOKEN = 20,
    /// A SCALAR token
    YAML_SCALAR_TOKEN = 21,
}

/// Token data for STREAM-START
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_stream_start_t {
    pub encoding: yaml_encoding_t,
}

/// Token data for ALIAS
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_alias_t {
    pub value: *mut yaml_char_t,
}

/// Token data for ANCHOR
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_anchor_t {
    pub value: *mut yaml_char_t,
}

/// Token data for TAG
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_tag_t {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}

/// Token data for SCALAR
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_scalar_t {
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub style: yaml_scalar_style_t,
}

/// Token data for VERSION-DIRECTIVE
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_version_directive_t {
    pub major: i32,
    pub minor: i32,
}

/// Token data for TAG-DIRECTIVE
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_token_tag_directive_t {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}

/// The token structure
#[repr(C)]
pub struct yaml_token_t {
    pub token_type: yaml_token_type_t,
    pub data: yaml_token_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

/// Union-like structure for token data (stored as all fields)
#[repr(C)]
pub struct yaml_token_data_t {
    pub stream_start: yaml_token_stream_start_t,
    pub alias: yaml_token_alias_t,
    pub anchor: yaml_token_anchor_t,
    pub tag: yaml_token_tag_t,
    pub scalar: yaml_token_scalar_t,
    pub version_directive: yaml_token_version_directive_t,
    pub tag_directive: yaml_token_tag_directive_t,
}

impl Copy for yaml_token_t {}
impl Clone for yaml_token_t {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for yaml_token_data_t {}
impl Clone for yaml_token_data_t {
    fn clone(&self) -> Self {
        *self
    }
}

// ============================================================================
// Events
// ============================================================================

/// Event types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_event_type_t {
    /// An empty event
    YAML_NO_EVENT = 0,
    /// A STREAM-START event
    YAML_STREAM_START_EVENT = 1,
    /// A STREAM-END event
    YAML_STREAM_END_EVENT = 2,
    /// A DOCUMENT-START event
    YAML_DOCUMENT_START_EVENT = 3,
    /// A DOCUMENT-END event
    YAML_DOCUMENT_END_EVENT = 4,
    /// An ALIAS event
    YAML_ALIAS_EVENT = 5,
    /// A SCALAR event
    YAML_SCALAR_EVENT = 6,
    /// A SEQUENCE-START event
    YAML_SEQUENCE_START_EVENT = 7,
    /// A SEQUENCE-END event
    YAML_SEQUENCE_END_EVENT = 8,
    /// A MAPPING-START event
    YAML_MAPPING_START_EVENT = 9,
    /// A MAPPING-END event
    YAML_MAPPING_END_EVENT = 10,
}

/// Event data for STREAM-START
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_stream_start_t {
    pub encoding: yaml_encoding_t,
}

/// Event data for DOCUMENT-START
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_document_start_t {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives_start: *mut yaml_tag_directive_t,
    pub tag_directives_end: *mut yaml_tag_directive_t,
    pub implicit: i32,
}

/// Event data for DOCUMENT-END
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_document_end_t {
    pub implicit: i32,
}

/// Event data for ALIAS
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_alias_t {
    pub anchor: *mut yaml_char_t,
}

/// Event data for SCALAR
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_scalar_t {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub plain_implicit: i32,
    pub quoted_implicit: i32,
    pub style: yaml_scalar_style_t,
}

/// Event data for SEQUENCE-START
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_sequence_start_t {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_sequence_style_t,
}

/// Event data for MAPPING-START
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_event_mapping_start_t {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: i32,
    pub style: yaml_mapping_style_t,
}

/// The event structure
#[repr(C)]
pub struct yaml_event_t {
    pub event_type: yaml_event_type_t,
    pub data: yaml_event_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

/// Union-like structure for event data (stored as all fields)
#[repr(C)]
pub struct yaml_event_data_t {
    pub stream_start: yaml_event_stream_start_t,
    pub document_start: yaml_event_document_start_t,
    pub document_end: yaml_event_document_end_t,
    pub alias: yaml_event_alias_t,
    pub scalar: yaml_event_scalar_t,
    pub sequence_start: yaml_event_sequence_start_t,
    pub mapping_start: yaml_event_mapping_start_t,
}

impl Copy for yaml_event_t {}
impl Clone for yaml_event_t {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for yaml_event_data_t {}
impl Clone for yaml_event_data_t {
    fn clone(&self) -> Self {
        *self
    }
}

// ============================================================================
// Nodes and Documents
// ============================================================================

/// Node types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum yaml_node_type_t {
    /// An empty node
    YAML_NO_NODE = 0,
    /// A scalar node
    YAML_SCALAR_NODE = 1,
    /// A sequence node
    YAML_SEQUENCE_NODE = 2,
    /// A mapping node
    YAML_MAPPING_NODE = 3,
}

/// An element of a sequence node
pub type yaml_node_item_t = i32;

/// An element of a mapping node
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_node_pair_t {
    /// The key of the element
    pub key: i32,
    /// The value of the element
    pub value: i32,
}

/// Node data for SCALAR
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_node_scalar_t {
    pub value: *mut yaml_char_t,
    pub length: usize,
    pub style: yaml_scalar_style_t,
}

/// Stack structure for sequence items
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_node_items_t {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}

/// Node data for SEQUENCE
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_node_sequence_t {
    pub items: yaml_node_items_t,
    pub style: yaml_sequence_style_t,
}

/// Stack structure for mapping pairs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_node_pairs_t {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}

/// Node data for MAPPING
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_node_mapping_t {
    pub pairs: yaml_node_pairs_t,
    pub style: yaml_mapping_style_t,
}

/// The node structure
#[repr(C)]
pub struct yaml_node_t {
    pub node_type: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: yaml_node_data_t,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}

/// Union-like structure for node data (stored as all fields)
#[repr(C)]
pub struct yaml_node_data_t {
    pub scalar: yaml_node_scalar_t,
    pub sequence: yaml_node_sequence_t,
    pub mapping: yaml_node_mapping_t,
}

impl Copy for yaml_node_t {}
impl Clone for yaml_node_t {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for yaml_node_data_t {}
impl Clone for yaml_node_data_t {
    fn clone(&self) -> Self {
        *self
    }
}

/// Stack structure for document nodes
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_document_nodes_t {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}

/// Stack structure for tag directives
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_document_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}

/// The document structure
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

impl Copy for yaml_document_t {}
impl Clone for yaml_document_t {
    fn clone(&self) -> Self {
        *self
    }
}

// ============================================================================
// Parser
// ============================================================================

/// Parser states
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

/// Simple key data structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_simple_key_t {
    pub possible: i32,
    pub required: i32,
    pub token_number: usize,
    pub mark: yaml_mark_t,
}

/// Alias data structure
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_alias_data_t {
    pub anchor: *mut yaml_char_t,
    pub index: i32,
    pub mark: yaml_mark_t,
}

/// Read handler type
pub type yaml_read_handler_t = extern "C" fn(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
    size_read: *mut usize,
) -> i32;

/// Stack structure for tokens
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_tokens_t {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}

/// Stack structure for buffer
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_buffer_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}

/// Stack structure for raw buffer
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_raw_buffer_t {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}

/// Stack structure for indents
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_indents_t {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}

/// Stack structure for simple keys
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_simple_keys_t {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}

/// Input union for parser
#[repr(C)]
pub union yaml_parser_input_t {
    pub string: yaml_parser_string_input_t,
    pub file: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_string_input_t {
    pub start: *const u8,
    pub end: *const u8,
    pub current: *const u8,
}

impl Copy for yaml_parser_input_t {}
impl Clone for yaml_parser_input_t {
    fn clone(&self) -> Self {
        *self
    }
}

/// Stack structure for parser states
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_states_t {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}

/// Stack structure for marks
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_marks_t {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}

/// Stack structure for tag directives
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}

/// Stack structure for aliases
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_parser_aliases_t {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}

/// The parser structure
#[repr(C)]
pub struct yaml_parser_t {
    // Error handling
    pub error: yaml_error_type_t,
    pub problem: *const i8,
    pub problem_offset: usize,
    pub problem_value: i32,
    pub problem_mark: yaml_mark_t,
    pub context: *const i8,
    pub context_mark: yaml_mark_t,

    // Reader stuff
    pub read_handler: *mut yaml_read_handler_t,
    pub read_handler_data: *mut c_void,
    pub input: yaml_parser_input_t,
    pub eof: i32,
    pub buffer: yaml_parser_buffer_t,
    pub unread: usize,
    pub raw_buffer: yaml_parser_raw_buffer_t,
    pub encoding: yaml_encoding_t,
    pub offset: usize,
    pub mark: yaml_mark_t,

    // Scanner stuff
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

    // Parser stuff
    pub states: yaml_parser_states_t,
    pub state: yaml_parser_state_t,
    pub marks: yaml_parser_marks_t,
    pub tag_directives: yaml_parser_tag_directives_t,

    // Dumper stuff
    pub aliases: yaml_parser_aliases_t,
    pub document: *mut yaml_document_t,
}

impl Copy for yaml_parser_t {}
impl Clone for yaml_parser_t {
    fn clone(&self) -> Self {
        *self
    }
}

// ============================================================================
// Emitter
// ============================================================================

/// Emitter states
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

/// Anchor information
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_anchors_t {
    pub references: i32,
    pub anchor: i32,
    pub serialized: i32,
}

/// Write handler type
pub type yaml_write_handler_t = extern "C" fn(
    data: *mut c_void,
    buffer: *mut u8,
    size: usize,
) -> i32;

/// Output union for emitter
#[repr(C)]
pub union yaml_emitter_output_t {
    pub string: yaml_emitter_string_output_t,
    pub file: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_string_output_t {
    pub buffer: *mut u8,
    pub size: usize,
    pub size_written: *mut usize,
}

impl Copy for yaml_emitter_output_t {}
impl Clone for yaml_emitter_output_t {
    fn clone(&self) -> Self {
        *self
    }
}

/// Stack structure for emitter buffer
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_buffer_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}

/// Stack structure for emitter raw buffer
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_raw_buffer_t {
    pub start: *mut u8,
    pub end: *mut u8,
    pub pointer: *mut u8,
    pub last: *mut u8,
}

/// Stack structure for emitter states
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_states_t {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}

/// Stack structure for emitter events
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_events_t {
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}

/// Stack structure for emitter indents
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_indents_t {
    pub start: *mut i32,
    pub end: *mut i32,
    pub top: *mut i32,
}

/// Stack structure for emitter tag directives
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_tag_directives_t {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}

/// Anchor data for emitter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_anchor_data_t {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: usize,
    pub alias: i32,
}

/// Tag data for emitter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct yaml_emitter_tag_data_t {
    pub handle: *mut yaml_char_t,
    pub handle_length: usize,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: usize,
}

/// Scalar data for emitter
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

/// The emitter structure
#[repr(C)]
pub struct yaml_emitter_t {
    // Error handling
    pub error: yaml_error_type_t,
    pub problem: *const i8,

    // Writer stuff
    pub write_handler: *mut yaml_write_handler_t,
    pub write_handler_data: *mut c_void,
    pub output: yaml_emitter_output_t,
    pub buffer: yaml_emitter_buffer_t,
    pub raw_buffer: yaml_emitter_raw_buffer_t,
    pub encoding: yaml_encoding_t,

    // Emitter stuff
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

    // Dumper stuff
    pub opened: i32,
    pub closed: i32,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: i32,
    pub document: *mut yaml_document_t,
}

impl Copy for yaml_emitter_t {}
impl Clone for yaml_emitter_t {
    fn clone(&self) -> Self {
        *self
    }
}

// ============================================================================
// Constants (defined as C macros)
// ============================================================================

pub const YAML_NULL_TAG: &[u8] = b"tag:yaml.org,2002:null";
pub const YAML_BOOL_TAG: &[u8] = b"tag:yaml.org,2002:bool";
pub const YAML_STR_TAG: &[u8] = b"tag:yaml.org,2002:str";
pub const YAML_INT_TAG: &[u8] = b"tag:yaml.org,2002:int";
pub const YAML_FLOAT_TAG: &[u8] = b"tag:yaml.org,2002:float";
pub const YAML_TIMESTAMP_TAG: &[u8] = b"tag:yaml.org,2002:timestamp";
pub const YAML_SEQ_TAG: &[u8] = b"tag:yaml.org,2002:seq";
pub const YAML_MAP_TAG: &[u8] = b"tag:yaml.org,2002:map";

pub const YAML_DEFAULT_SCALAR_TAG: &[u8] = YAML_STR_TAG;
pub const YAML_DEFAULT_SEQUENCE_TAG: &[u8] = YAML_SEQ_TAG;
pub const YAML_DEFAULT_MAPPING_TAG: &[u8] = YAML_MAP_TAG;
