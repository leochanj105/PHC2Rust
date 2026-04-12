pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;
pub type _IO_wide_data = libc::c_void;

extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn yaml_document_delete(document: *mut yaml_document_t);
    fn yaml_parser_parse(
        parser: *mut yaml_parser_t,
        event: *mut yaml_event_t,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn yaml_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn yaml_free(ptr: *mut ::core::ffi::c_void);
    fn yaml_strdup(_: *const yaml_char_t) -> *mut yaml_char_t;
    fn yaml_stack_extend(
        start: *mut *mut ::core::ffi::c_void,
        top: *mut *mut ::core::ffi::c_void,
        end: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type yaml_char_t = ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_version_directive_s {
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
}
pub type yaml_version_directive_t = yaml_version_directive_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_tag_directive_s {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
pub type yaml_tag_directive_t = yaml_tag_directive_s;
pub type yaml_encoding_e = ::core::ffi::c_uint;
pub const YAML_UTF16BE_ENCODING: yaml_encoding_e = 3;
pub const YAML_UTF16LE_ENCODING: yaml_encoding_e = 2;
pub const YAML_UTF8_ENCODING: yaml_encoding_e = 1;
pub const YAML_ANY_ENCODING: yaml_encoding_e = 0;
pub type yaml_encoding_t = yaml_encoding_e;
pub type yaml_error_type_e = ::core::ffi::c_uint;
pub const YAML_EMITTER_ERROR: yaml_error_type_e = 7;
pub const YAML_WRITER_ERROR: yaml_error_type_e = 6;
pub const YAML_COMPOSER_ERROR: yaml_error_type_e = 5;
pub const YAML_PARSER_ERROR: yaml_error_type_e = 4;
pub const YAML_SCANNER_ERROR: yaml_error_type_e = 3;
pub const YAML_READER_ERROR: yaml_error_type_e = 2;
pub const YAML_MEMORY_ERROR: yaml_error_type_e = 1;
pub const YAML_NO_ERROR: yaml_error_type_e = 0;
pub type yaml_error_type_t = yaml_error_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_mark_s {
    pub index: size_t,
    pub line: size_t,
    pub column: size_t,
}
pub type yaml_mark_t = yaml_mark_s;
pub type yaml_scalar_style_e = ::core::ffi::c_uint;
pub const YAML_FOLDED_SCALAR_STYLE: yaml_scalar_style_e = 5;
pub const YAML_LITERAL_SCALAR_STYLE: yaml_scalar_style_e = 4;
pub const YAML_DOUBLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 3;
pub const YAML_SINGLE_QUOTED_SCALAR_STYLE: yaml_scalar_style_e = 2;
pub const YAML_PLAIN_SCALAR_STYLE: yaml_scalar_style_e = 1;
pub const YAML_ANY_SCALAR_STYLE: yaml_scalar_style_e = 0;
pub type yaml_scalar_style_t = yaml_scalar_style_e;
pub type yaml_sequence_style_e = ::core::ffi::c_uint;
pub const YAML_FLOW_SEQUENCE_STYLE: yaml_sequence_style_e = 2;
pub const YAML_BLOCK_SEQUENCE_STYLE: yaml_sequence_style_e = 1;
pub const YAML_ANY_SEQUENCE_STYLE: yaml_sequence_style_e = 0;
pub type yaml_sequence_style_t = yaml_sequence_style_e;
pub type yaml_mapping_style_e = ::core::ffi::c_uint;
pub const YAML_FLOW_MAPPING_STYLE: yaml_mapping_style_e = 2;
pub const YAML_BLOCK_MAPPING_STYLE: yaml_mapping_style_e = 1;
pub const YAML_ANY_MAPPING_STYLE: yaml_mapping_style_e = 0;
pub type yaml_mapping_style_t = yaml_mapping_style_e;
pub type yaml_token_type_e = ::core::ffi::c_uint;
pub const YAML_SCALAR_TOKEN: yaml_token_type_e = 21;
pub const YAML_TAG_TOKEN: yaml_token_type_e = 20;
pub const YAML_ANCHOR_TOKEN: yaml_token_type_e = 19;
pub const YAML_ALIAS_TOKEN: yaml_token_type_e = 18;
pub const YAML_VALUE_TOKEN: yaml_token_type_e = 17;
pub const YAML_KEY_TOKEN: yaml_token_type_e = 16;
pub const YAML_FLOW_ENTRY_TOKEN: yaml_token_type_e = 15;
pub const YAML_BLOCK_ENTRY_TOKEN: yaml_token_type_e = 14;
pub const YAML_FLOW_MAPPING_END_TOKEN: yaml_token_type_e = 13;
pub const YAML_FLOW_MAPPING_START_TOKEN: yaml_token_type_e = 12;
pub const YAML_FLOW_SEQUENCE_END_TOKEN: yaml_token_type_e = 11;
pub const YAML_FLOW_SEQUENCE_START_TOKEN: yaml_token_type_e = 10;
pub const YAML_BLOCK_END_TOKEN: yaml_token_type_e = 9;
pub const YAML_BLOCK_MAPPING_START_TOKEN: yaml_token_type_e = 8;
pub const YAML_BLOCK_SEQUENCE_START_TOKEN: yaml_token_type_e = 7;
pub const YAML_DOCUMENT_END_TOKEN: yaml_token_type_e = 6;
pub const YAML_DOCUMENT_START_TOKEN: yaml_token_type_e = 5;
pub const YAML_TAG_DIRECTIVE_TOKEN: yaml_token_type_e = 4;
pub const YAML_VERSION_DIRECTIVE_TOKEN: yaml_token_type_e = 3;
pub const YAML_STREAM_END_TOKEN: yaml_token_type_e = 2;
pub const YAML_STREAM_START_TOKEN: yaml_token_type_e = 1;
pub const YAML_NO_TOKEN: yaml_token_type_e = 0;
pub type yaml_token_type_t = yaml_token_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_token_s {
    pub type_0: yaml_token_type_t,
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_6,
    pub alias: C2RustUnnamed_5,
    pub anchor: C2RustUnnamed_4,
    pub tag: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub version_directive: C2RustUnnamed_1,
    pub tag_directive: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub handle: *mut yaml_char_t,
    pub prefix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub major: ::core::ffi::c_int,
    pub minor: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub handle: *mut yaml_char_t,
    pub suffix: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub value: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_token_t = yaml_token_s;
pub type yaml_event_type_e = ::core::ffi::c_uint;
pub const YAML_MAPPING_END_EVENT: yaml_event_type_e = 10;
pub const YAML_MAPPING_START_EVENT: yaml_event_type_e = 9;
pub const YAML_SEQUENCE_END_EVENT: yaml_event_type_e = 8;
pub const YAML_SEQUENCE_START_EVENT: yaml_event_type_e = 7;
pub const YAML_SCALAR_EVENT: yaml_event_type_e = 6;
pub const YAML_ALIAS_EVENT: yaml_event_type_e = 5;
pub const YAML_DOCUMENT_END_EVENT: yaml_event_type_e = 4;
pub const YAML_DOCUMENT_START_EVENT: yaml_event_type_e = 3;
pub const YAML_STREAM_END_EVENT: yaml_event_type_e = 2;
pub const YAML_STREAM_START_EVENT: yaml_event_type_e = 1;
pub const YAML_NO_EVENT: yaml_event_type_e = 0;
pub type yaml_event_type_t = yaml_event_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_event_s {
    pub type_0: yaml_event_type_t,
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub stream_start: C2RustUnnamed_15,
    pub document_start: C2RustUnnamed_13,
    pub document_end: C2RustUnnamed_12,
    pub alias: C2RustUnnamed_11,
    pub scalar: C2RustUnnamed_10,
    pub sequence_start: C2RustUnnamed_9,
    pub mapping_start: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: ::core::ffi::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: ::core::ffi::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub plain_implicit: ::core::ffi::c_int,
    pub quoted_implicit: ::core::ffi::c_int,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub implicit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub implicit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub encoding: yaml_encoding_t,
}
pub type yaml_event_t = yaml_event_s;
pub type yaml_node_type_e = ::core::ffi::c_uint;
pub const YAML_MAPPING_NODE: yaml_node_type_e = 3;
pub const YAML_SEQUENCE_NODE: yaml_node_type_e = 2;
pub const YAML_SCALAR_NODE: yaml_node_type_e = 1;
pub const YAML_NO_NODE: yaml_node_type_e = 0;
pub type yaml_node_type_t = yaml_node_type_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_s {
    pub type_0: yaml_node_type_t,
    pub tag: *mut yaml_char_t,
    pub data: C2RustUnnamed_16,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub scalar: C2RustUnnamed_21,
    pub sequence: C2RustUnnamed_19,
    pub mapping: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pairs: C2RustUnnamed_18,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
pub type yaml_node_pair_t = yaml_node_pair_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_node_pair_s {
    pub key: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub items: C2RustUnnamed_20,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_23,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_22,
    pub start_implicit: ::core::ffi::c_int,
    pub end_implicit: ::core::ffi::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
pub type yaml_read_handler_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_uchar,
    size_t,
    *mut size_t,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_simple_key_s {
    pub possible: ::core::ffi::c_int,
    pub required: ::core::ffi::c_int,
    pub token_number: size_t,
    pub mark: yaml_mark_t,
}
pub type yaml_simple_key_t = yaml_simple_key_s;
pub type yaml_parser_state_e = ::core::ffi::c_uint;
pub const YAML_PARSE_END_STATE: yaml_parser_state_e = 23;
pub const YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE: yaml_parser_state_e = 22;
pub const YAML_PARSE_FLOW_MAPPING_VALUE_STATE: yaml_parser_state_e = 21;
pub const YAML_PARSE_FLOW_MAPPING_KEY_STATE: yaml_parser_state_e = 20;
pub const YAML_PARSE_FLOW_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 19;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_END_STATE: yaml_parser_state_e = 18;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_VALUE_STATE: yaml_parser_state_e = 17;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_MAPPING_KEY_STATE: yaml_parser_state_e = 16;
pub const YAML_PARSE_FLOW_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 15;
pub const YAML_PARSE_FLOW_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 14;
pub const YAML_PARSE_BLOCK_MAPPING_VALUE_STATE: yaml_parser_state_e = 13;
pub const YAML_PARSE_BLOCK_MAPPING_KEY_STATE: yaml_parser_state_e = 12;
pub const YAML_PARSE_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_parser_state_e = 11;
pub const YAML_PARSE_INDENTLESS_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 10;
pub const YAML_PARSE_BLOCK_SEQUENCE_ENTRY_STATE: yaml_parser_state_e = 9;
pub const YAML_PARSE_BLOCK_SEQUENCE_FIRST_ENTRY_STATE: yaml_parser_state_e = 8;
pub const YAML_PARSE_FLOW_NODE_STATE: yaml_parser_state_e = 7;
pub const YAML_PARSE_BLOCK_NODE_OR_INDENTLESS_SEQUENCE_STATE: yaml_parser_state_e = 6;
pub const YAML_PARSE_BLOCK_NODE_STATE: yaml_parser_state_e = 5;
pub const YAML_PARSE_DOCUMENT_END_STATE: yaml_parser_state_e = 4;
pub const YAML_PARSE_DOCUMENT_CONTENT_STATE: yaml_parser_state_e = 3;
pub const YAML_PARSE_DOCUMENT_START_STATE: yaml_parser_state_e = 2;
pub const YAML_PARSE_IMPLICIT_DOCUMENT_START_STATE: yaml_parser_state_e = 1;
pub const YAML_PARSE_STREAM_START_STATE: yaml_parser_state_e = 0;
pub type yaml_parser_state_t = yaml_parser_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_alias_data_s {
    pub anchor: *mut yaml_char_t,
    pub index: ::core::ffi::c_int,
    pub mark: yaml_mark_t,
}
pub type yaml_alias_data_t = yaml_alias_data_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_parser_s {
    pub error: yaml_error_type_t,
    pub problem: *const ::core::ffi::c_char,
    pub problem_offset: size_t,
    pub problem_value: ::core::ffi::c_int,
    pub problem_mark: yaml_mark_t,
    pub context: *const ::core::ffi::c_char,
    pub context_mark: yaml_mark_t,
    pub read_handler: Option<yaml_read_handler_t>,
    pub read_handler_data: *mut ::core::ffi::c_void,
    pub input: C2RustUnnamed_33,
    pub eof: ::core::ffi::c_int,
    pub buffer: C2RustUnnamed_32,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_31,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: ::core::ffi::c_int,
    pub stream_end_produced: ::core::ffi::c_int,
    pub flow_level: ::core::ffi::c_int,
    pub tokens: C2RustUnnamed_30,
    pub tokens_parsed: size_t,
    pub token_available: ::core::ffi::c_int,
    pub indents: C2RustUnnamed_29,
    pub indent: ::core::ffi::c_int,
    pub simple_key_allowed: ::core::ffi::c_int,
    pub simple_keys: C2RustUnnamed_28,
    pub states: C2RustUnnamed_27,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_26,
    pub tag_directives: C2RustUnnamed_25,
    pub aliases: C2RustUnnamed_24,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub start: *mut ::core::ffi::c_int,
    pub end: *mut ::core::ffi::c_int,
    pub top: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_30 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_31 {
    pub start: *mut ::core::ffi::c_uchar,
    pub end: *mut ::core::ffi::c_uchar,
    pub pointer: *mut ::core::ffi::c_uchar,
    pub last: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_32 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub string: C2RustUnnamed_34,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_34 {
    pub start: *const ::core::ffi::c_uchar,
    pub end: *const ::core::ffi::c_uchar,
    pub current: *const ::core::ffi::c_uchar,
}
pub type yaml_parser_t = yaml_parser_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loader_ctx {
    pub start: *mut ::core::ffi::c_int,
    pub end: *mut ::core::ffi::c_int,
    pub top: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_35 {
    pub start: *mut yaml_node_pair_t,
    pub end: *mut yaml_node_pair_t,
    pub top: *mut yaml_node_pair_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_36 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const YAML_STR_TAG: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"tag:yaml.org,2002:str\0")
};
pub const YAML_SEQ_TAG: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"tag:yaml.org,2002:seq\0")
};
pub const YAML_MAP_TAG: [::core::ffi::c_char; 22] = unsafe {
    ::core::mem::transmute::<[u8; 22], [::core::ffi::c_char; 22]>(*b"tag:yaml.org,2002:map\0")
};
pub const YAML_DEFAULT_SCALAR_TAG: [::core::ffi::c_char; 22] = YAML_STR_TAG;
pub const YAML_DEFAULT_SEQUENCE_TAG: [::core::ffi::c_char; 22] = YAML_SEQ_TAG;
pub const YAML_DEFAULT_MAPPING_TAG: [::core::ffi::c_char; 22] = YAML_MAP_TAG;
pub const INITIAL_STACK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_load(
    mut parser: *mut yaml_parser_t,
    mut document: *mut yaml_document_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed_7 {
            stream_start: C2RustUnnamed_15 {
                encoding: YAML_ANY_ENCODING,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    '_c2rust_label: {
        if !parser.is_null() {
        } else {
            __assert_fail(
                b"parser\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_uint,
                b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !document.is_null() {
        } else {
            __assert_fail(
                b"document\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_uint,
                b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        document as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_document_t>() as size_t,
    );
    (*document).nodes.start = yaml_malloc(
        (INITIAL_STACK_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<yaml_node_t>() as size_t),
    ) as *mut yaml_node_t;
    if !(if !(*document).nodes.start.is_null() {
        (*document).nodes.top = (*document).nodes.start;
        (*document).nodes.end = (*document).nodes.start.offset(INITIAL_STACK_SIZE as isize);
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        if (*parser).stream_start_produced == 0 {
            if yaml_parser_parse(parser, &raw mut event) == 0 {
                current_block = 9894560525436162251;
            } else {
                '_c2rust_label_1: {
                    if event.type_0 as ::core::ffi::c_uint
                        == YAML_STREAM_START_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                    } else {
                        __assert_fail(
                            b"event.type == YAML_STREAM_START_EVENT\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            100 as ::core::ffi::c_uint,
                            b"int yaml_parser_load(yaml_parser_t *, yaml_document_t *)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        match current_block {
            9894560525436162251 => {}
            _ => {
                if (*parser).stream_end_produced != 0 {
                    return 1 as ::core::ffi::c_int;
                }
                if !(yaml_parser_parse(parser, &raw mut event) == 0) {
                    if event.type_0 as ::core::ffi::c_uint
                        == YAML_STREAM_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        return 1 as ::core::ffi::c_int;
                    }
                    (*parser).aliases.start = yaml_malloc(
                        (INITIAL_STACK_SIZE as size_t)
                            .wrapping_mul(::core::mem::size_of::<yaml_alias_data_t>() as size_t),
                    ) as *mut yaml_alias_data_t;
                    if !(if !(*parser).aliases.start.is_null() {
                        (*parser).aliases.top = (*parser).aliases.start;
                        (*parser).aliases.end =
                            (*parser).aliases.start.offset(INITIAL_STACK_SIZE as isize);
                        1 as ::core::ffi::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as ::core::ffi::c_int
                    } == 0)
                    {
                        (*parser).document = document;
                        if !(yaml_parser_load_document(parser, &raw mut event) == 0) {
                            yaml_parser_delete_aliases(parser);
                            (*parser).document = ::core::ptr::null_mut::<yaml_document_t>();
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_parser_delete_aliases(parser);
    yaml_document_delete(document);
    (*parser).document = ::core::ptr::null_mut::<yaml_document_t>();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_set_composer_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const ::core::ffi::c_char,
    mut problem_mark: yaml_mark_t,
) -> ::core::ffi::c_int {
    (*parser).error = YAML_COMPOSER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_set_composer_error_context(
    mut parser: *mut yaml_parser_t,
    mut context: *const ::core::ffi::c_char,
    mut context_mark: yaml_mark_t,
    mut problem: *const ::core::ffi::c_char,
    mut problem_mark: yaml_mark_t,
) -> ::core::ffi::c_int {
    (*parser).error = YAML_COMPOSER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = problem_mark;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_delete_aliases(mut parser: *mut yaml_parser_t) {
    while !((*parser).aliases.start == (*parser).aliases.top) {
        (*parser).aliases.top = (*parser).aliases.top.offset(-1);
        yaml_free((*(*parser).aliases.top).anchor as *mut ::core::ffi::c_void);
    }
    yaml_free((*parser).aliases.start as *mut ::core::ffi::c_void);
    (*parser).aliases.end = ::core::ptr::null_mut::<yaml_alias_data_t>();
    (*parser).aliases.top = (*parser).aliases.end;
    (*parser).aliases.start = (*parser).aliases.top;
}
unsafe extern "C" fn yaml_parser_load_document(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    let mut ctx: loader_ctx = loader_ctx {
        start: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        end: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        top: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    };
    '_c2rust_label: {
        if (*event).type_0 as ::core::ffi::c_uint
            == YAML_DOCUMENT_START_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"event->type == YAML_DOCUMENT_START_EVENT\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                189 as ::core::ffi::c_uint,
                b"int yaml_parser_load_document(yaml_parser_t *, yaml_event_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*(*parser).document).version_directive = (*event).data.document_start.version_directive;
    (*(*parser).document).tag_directives.start = (*event).data.document_start.tag_directives.start;
    (*(*parser).document).tag_directives.end = (*event).data.document_start.tag_directives.end;
    (*(*parser).document).start_implicit = (*event).data.document_start.implicit;
    (*(*parser).document).start_mark = (*event).start_mark;
    ctx.start = yaml_malloc(
        (INITIAL_STACK_SIZE as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    if if !ctx.start.is_null() {
        ctx.top = ctx.start;
        ctx.end = ctx.start.offset(INITIAL_STACK_SIZE as isize);
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_load_nodes(parser, &raw mut ctx) == 0 {
        yaml_free(ctx.start as *mut ::core::ffi::c_void);
        ctx.end = ::core::ptr::null_mut::<::core::ffi::c_int>();
        ctx.top = ctx.end;
        ctx.start = ctx.top;
        return 0 as ::core::ffi::c_int;
    }
    yaml_free(ctx.start as *mut ::core::ffi::c_void);
    ctx.end = ::core::ptr::null_mut::<::core::ffi::c_int>();
    ctx.top = ctx.end;
    ctx.start = ctx.top;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_nodes(
    mut parser: *mut yaml_parser_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed_7 {
            stream_start: C2RustUnnamed_15 {
                encoding: YAML_ANY_ENCODING,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    loop {
        if yaml_parser_parse(parser, &raw mut event) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        match event.type_0 as ::core::ffi::c_uint {
            5 => {
                if yaml_parser_load_alias(parser, &raw mut event, ctx) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            6 => {
                if yaml_parser_load_scalar(parser, &raw mut event, ctx) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            7 => {
                if yaml_parser_load_sequence(parser, &raw mut event, ctx) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            8 => {
                if yaml_parser_load_sequence_end(parser, &raw mut event, ctx) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            9 => {
                if yaml_parser_load_mapping(parser, &raw mut event, ctx) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            10 => {
                if yaml_parser_load_mapping_end(parser, &raw mut event, ctx) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            4 => {}
            _ => {
                '_c2rust_label: {
                    __assert_fail(
                        b"0\0" as *const u8 as *const ::core::ffi::c_char,
                        b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                            as *const ::core::ffi::c_char,
                        246 as ::core::ffi::c_uint,
                        b"int yaml_parser_load_nodes(yaml_parser_t *, struct loader_ctx *)\0"
                            as *const u8 as *const ::core::ffi::c_char,
                    );
                };
                return 0 as ::core::ffi::c_int;
            }
        }
        if !(event.type_0 as ::core::ffi::c_uint
            != YAML_DOCUMENT_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            break;
        }
    }
    (*(*parser).document).end_implicit = event.data.document_end.implicit;
    (*(*parser).document).end_mark = event.end_mark;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_register_anchor(
    mut parser: *mut yaml_parser_t,
    mut index: ::core::ffi::c_int,
    mut anchor: *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut data: yaml_alias_data_t = yaml_alias_data_t {
        anchor: ::core::ptr::null_mut::<yaml_char_t>(),
        index: 0,
        mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut alias_data: *mut yaml_alias_data_t = ::core::ptr::null_mut::<yaml_alias_data_t>();
    if anchor.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    data.anchor = anchor;
    data.index = index;
    data.mark = (*(*(*parser).document)
        .nodes
        .start
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .start_mark;
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp(
            (*alias_data).anchor as *mut ::core::ffi::c_char,
            anchor as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            yaml_free(anchor as *mut ::core::ffi::c_void);
            return yaml_parser_set_composer_error_context(
                parser,
                b"found duplicate anchor; first occurrence\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*alias_data).mark,
                b"second occurrence\0" as *const u8 as *const ::core::ffi::c_char,
                data.mark,
            );
        }
        alias_data = alias_data.offset(1);
    }
    if if (*parser).aliases.top != (*parser).aliases.end
        || yaml_stack_extend(
            &raw mut (*parser).aliases.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).aliases.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).aliases.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh4 = (*parser).aliases.top;
        (*parser).aliases.top = (*parser).aliases.top.offset(1);
        *fresh4 = data;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_free(anchor as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_node_add(
    mut parser: *mut yaml_parser_t,
    mut ctx: *mut loader_ctx,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut parent: *mut yaml_node_s = ::core::ptr::null_mut::<yaml_node_s>();
    let mut parent_index: ::core::ffi::c_int = 0;
    if (*ctx).start == (*ctx).top {
        return 1 as ::core::ffi::c_int;
    }
    parent_index = *(*ctx).top.offset(-(1 as ::core::ffi::c_int as isize));
    parent = (*(*parser).document)
        .nodes
        .start
        .offset((parent_index - 1 as ::core::ffi::c_int) as isize) as *mut yaml_node_t
        as *mut yaml_node_s;
    let mut current_block_17: u64;
    match (*parent).type_0 as ::core::ffi::c_uint {
        2 => {
            if if ((*parent)
                .data
                .sequence
                .items
                .top
                .offset_from((*parent).data.sequence.items.start)
                as ::core::ffi::c_long)
                < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_long
            {
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if if (*parent).data.sequence.items.top != (*parent).data.sequence.items.end
                || yaml_stack_extend(
                    &raw mut (*parent).data.sequence.items.start as *mut *mut ::core::ffi::c_void,
                    &raw mut (*parent).data.sequence.items.top as *mut *mut ::core::ffi::c_void,
                    &raw mut (*parent).data.sequence.items.end as *mut *mut ::core::ffi::c_void,
                ) != 0
            {
                let fresh2 = (*parent).data.sequence.items.top;
                (*parent).data.sequence.items.top = (*parent).data.sequence.items.top.offset(1);
                *fresh2 = index as yaml_node_item_t;
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        3 => {
            let mut pair: yaml_node_pair_t = yaml_node_pair_t { key: 0, value: 0 };
            if !((*parent).data.mapping.pairs.start == (*parent).data.mapping.pairs.top) {
                let mut p: *mut yaml_node_pair_t = (*parent)
                    .data
                    .mapping
                    .pairs
                    .top
                    .offset(-(1 as ::core::ffi::c_int as isize));
                if (*p).key != 0 as ::core::ffi::c_int && (*p).value == 0 as ::core::ffi::c_int {
                    (*p).value = index;
                    current_block_17 = 2370887241019905314;
                } else {
                    current_block_17 = 1841672684692190573;
                }
            } else {
                current_block_17 = 1841672684692190573;
            }
            match current_block_17 {
                2370887241019905314 => {}
                _ => {
                    pair.key = index;
                    pair.value = 0 as ::core::ffi::c_int;
                    if if ((*parent)
                        .data
                        .mapping
                        .pairs
                        .top
                        .offset_from((*parent).data.mapping.pairs.start)
                        as ::core::ffi::c_long)
                        < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as ::core::ffi::c_long
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as ::core::ffi::c_int
                    } == 0
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    if if (*parent).data.mapping.pairs.top != (*parent).data.mapping.pairs.end
                        || yaml_stack_extend(
                            &raw mut (*parent).data.mapping.pairs.start
                                as *mut *mut ::core::ffi::c_void,
                            &raw mut (*parent).data.mapping.pairs.top
                                as *mut *mut ::core::ffi::c_void,
                            &raw mut (*parent).data.mapping.pairs.end
                                as *mut *mut ::core::ffi::c_void,
                        ) != 0
                    {
                        let fresh3 = (*parent).data.mapping.pairs.top;
                        (*parent).data.mapping.pairs.top =
                            (*parent).data.mapping.pairs.top.offset(1);
                        *fresh3 = pair;
                        1 as ::core::ffi::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as ::core::ffi::c_int
                    } == 0
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        _ => {
            '_c2rust_label: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    340 as ::core::ffi::c_uint,
                    b"int yaml_parser_load_node_add(yaml_parser_t *, struct loader_ctx *, int)\0"
                        as *const u8 as *const ::core::ffi::c_char,
                );
            };
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_alias(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut anchor: *mut yaml_char_t = (*event).data.alias.anchor;
    let mut alias_data: *mut yaml_alias_data_t = ::core::ptr::null_mut::<yaml_alias_data_t>();
    alias_data = (*parser).aliases.start;
    while alias_data != (*parser).aliases.top {
        if strcmp(
            (*alias_data).anchor as *mut ::core::ffi::c_char,
            anchor as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            yaml_free(anchor as *mut ::core::ffi::c_void);
            return yaml_parser_load_node_add(parser, ctx, (*alias_data).index);
        }
        alias_data = alias_data.offset(1);
    }
    yaml_free(anchor as *mut ::core::ffi::c_void);
    return yaml_parser_set_composer_error(
        parser,
        b"found undefined alias\0" as *const u8 as *const ::core::ffi::c_char,
        (*event).start_mark,
    );
}
unsafe extern "C" fn yaml_parser_load_scalar(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: ::core::ptr::null_mut::<yaml_char_t>(),
        data: C2RustUnnamed_16 {
            scalar: C2RustUnnamed_21 {
                value: ::core::ptr::null_mut::<yaml_char_t>(),
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut index: ::core::ffi::c_int = 0;
    let mut tag: *mut yaml_char_t = (*event).data.scalar.tag;
    if !(if ((*(*parser).document)
        .nodes
        .top
        .offset_from((*(*parser).document).nodes.start) as ::core::ffi::c_long)
        < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
    {
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        if tag.is_null()
            || strcmp(
                tag as *mut ::core::ffi::c_char,
                b"!\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            yaml_free(tag as *mut ::core::ffi::c_void);
            tag = yaml_strdup(YAML_DEFAULT_SCALAR_TAG.as_ptr() as *mut yaml_char_t);
            if tag.is_null() {
                current_block = 1224537838420049293;
            } else {
                current_block = 15427931788582360902;
            }
        } else {
            current_block = 15427931788582360902;
        }
        match current_block {
            1224537838420049293 => {}
            _ => {
                memset(
                    &raw mut node as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<yaml_node_t>() as size_t,
                );
                node.type_0 = YAML_SCALAR_NODE;
                node.tag = tag;
                node.start_mark = (*event).start_mark;
                node.end_mark = (*event).end_mark;
                node.data.scalar.value = (*event).data.scalar.value;
                node.data.scalar.length = (*event).data.scalar.length;
                node.data.scalar.style = (*event).data.scalar.style;
                if !(if (*(*parser).document).nodes.top != (*(*parser).document).nodes.end
                    || yaml_stack_extend(
                        &raw mut (*(*parser).document).nodes.start as *mut *mut ::core::ffi::c_void,
                        &raw mut (*(*parser).document).nodes.top as *mut *mut ::core::ffi::c_void,
                        &raw mut (*(*parser).document).nodes.end as *mut *mut ::core::ffi::c_void,
                    ) != 0
                {
                    let fresh7 = (*(*parser).document).nodes.top;
                    (*(*parser).document).nodes.top = (*(*parser).document).nodes.top.offset(1);
                    *fresh7 = node;
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } == 0)
                {
                    index = (*(*parser).document)
                        .nodes
                        .top
                        .offset_from((*(*parser).document).nodes.start)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
                    if yaml_parser_register_anchor(parser, index, (*event).data.scalar.anchor) == 0
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    return yaml_parser_load_node_add(parser, ctx, index);
                }
            }
        }
    }
    yaml_free(tag as *mut ::core::ffi::c_void);
    yaml_free((*event).data.scalar.anchor as *mut ::core::ffi::c_void);
    yaml_free((*event).data.scalar.value as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_sequence(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: ::core::ptr::null_mut::<yaml_char_t>(),
        data: C2RustUnnamed_16 {
            scalar: C2RustUnnamed_21 {
                value: ::core::ptr::null_mut::<yaml_char_t>(),
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut items: C2RustUnnamed_36 = C2RustUnnamed_36 {
        start: ::core::ptr::null_mut::<yaml_node_item_t>(),
        end: ::core::ptr::null_mut::<yaml_node_item_t>(),
        top: ::core::ptr::null_mut::<yaml_node_item_t>(),
    };
    let mut index: ::core::ffi::c_int = 0;
    let mut tag: *mut yaml_char_t = (*event).data.sequence_start.tag;
    if !(if ((*(*parser).document)
        .nodes
        .top
        .offset_from((*(*parser).document).nodes.start) as ::core::ffi::c_long)
        < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
    {
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        if tag.is_null()
            || strcmp(
                tag as *mut ::core::ffi::c_char,
                b"!\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            yaml_free(tag as *mut ::core::ffi::c_void);
            tag = yaml_strdup(YAML_DEFAULT_SEQUENCE_TAG.as_ptr() as *mut yaml_char_t);
            if tag.is_null() {
                current_block = 4918849390475775327;
            } else {
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        match current_block {
            4918849390475775327 => {}
            _ => {
                items.start = yaml_malloc(
                    (INITIAL_STACK_SIZE as size_t)
                        .wrapping_mul(::core::mem::size_of::<yaml_node_item_t>() as size_t),
                ) as *mut yaml_node_item_t;
                if !(if !items.start.is_null() {
                    items.top = items.start;
                    items.end = items.start.offset(INITIAL_STACK_SIZE as isize);
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } == 0)
                {
                    memset(
                        &raw mut node as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<yaml_node_t>() as size_t,
                    );
                    node.type_0 = YAML_SEQUENCE_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.sequence.items.start = items.start;
                    node.data.sequence.items.end = items.end;
                    node.data.sequence.items.top = items.start;
                    node.data.sequence.style = (*event).data.sequence_start.style;
                    if !(if (*(*parser).document).nodes.top != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            &raw mut (*(*parser).document).nodes.start
                                as *mut *mut ::core::ffi::c_void,
                            &raw mut (*(*parser).document).nodes.top
                                as *mut *mut ::core::ffi::c_void,
                            &raw mut (*(*parser).document).nodes.end
                                as *mut *mut ::core::ffi::c_void,
                        ) != 0
                    {
                        let fresh5 = (*(*parser).document).nodes.top;
                        (*(*parser).document).nodes.top = (*(*parser).document).nodes.top.offset(1);
                        *fresh5 = node;
                        1 as ::core::ffi::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as ::core::ffi::c_int
                    } == 0)
                    {
                        index = (*(*parser).document)
                            .nodes
                            .top
                            .offset_from((*(*parser).document).nodes.start)
                            as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                        if yaml_parser_register_anchor(
                            parser,
                            index,
                            (*event).data.sequence_start.anchor,
                        ) == 0
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        if yaml_parser_load_node_add(parser, ctx, index) == 0 {
                            return 0 as ::core::ffi::c_int;
                        }
                        if if ((*ctx).top.offset_from((*ctx).start) as ::core::ffi::c_long)
                            < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                &raw mut (*ctx).start as *mut *mut ::core::ffi::c_void,
                                &raw mut (*ctx).top as *mut *mut ::core::ffi::c_void,
                                &raw mut (*ctx).end as *mut *mut ::core::ffi::c_void,
                            ) != 0
                        {
                            let fresh6 = (*ctx).top;
                            (*ctx).top = (*ctx).top.offset(1);
                            *fresh6 = index;
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag as *mut ::core::ffi::c_void);
    yaml_free((*event).data.sequence_start.anchor as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_sequence_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*ctx).top.offset_from((*ctx).start) as ::core::ffi::c_long > 0 as ::core::ffi::c_long {
        } else {
            __assert_fail(
                b"((*ctx).top - (*ctx).start) > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                467 as ::core::ffi::c_uint,
                b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    index = *(*ctx).top.offset(-(1 as ::core::ffi::c_int as isize));
    '_c2rust_label_0: {
        if (*(*(*parser).document)
            .nodes
            .start
            .offset((index - 1 as ::core::ffi::c_int) as isize))
        .type_0 as ::core::ffi::c_uint
            == YAML_SEQUENCE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"parser->document->nodes.start[index-1].type == YAML_SEQUENCE_NODE\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                470 as ::core::ffi::c_uint,
                b"int yaml_parser_load_sequence_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*(*(*parser).document)
        .nodes
        .start
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .end_mark = (*event).end_mark;
    (*ctx).top = (*ctx).top.offset(-1);
    *(*ctx).top;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_mapping(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut node: yaml_node_t = yaml_node_t {
        type_0: YAML_NO_NODE,
        tag: ::core::ptr::null_mut::<yaml_char_t>(),
        data: C2RustUnnamed_16 {
            scalar: C2RustUnnamed_21 {
                value: ::core::ptr::null_mut::<yaml_char_t>(),
                length: 0,
                style: YAML_ANY_SCALAR_STYLE,
            },
        },
        start_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
        end_mark: yaml_mark_t {
            index: 0,
            line: 0,
            column: 0,
        },
    };
    let mut pairs: C2RustUnnamed_35 = C2RustUnnamed_35 {
        start: ::core::ptr::null_mut::<yaml_node_pair_t>(),
        end: ::core::ptr::null_mut::<yaml_node_pair_t>(),
        top: ::core::ptr::null_mut::<yaml_node_pair_t>(),
    };
    let mut index: ::core::ffi::c_int = 0;
    let mut tag: *mut yaml_char_t = (*event).data.mapping_start.tag;
    if !(if ((*(*parser).document)
        .nodes
        .top
        .offset_from((*(*parser).document).nodes.start) as ::core::ffi::c_long)
        < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as ::core::ffi::c_long
    {
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        if tag.is_null()
            || strcmp(
                tag as *mut ::core::ffi::c_char,
                b"!\0" as *const u8 as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            yaml_free(tag as *mut ::core::ffi::c_void);
            tag = yaml_strdup(YAML_DEFAULT_MAPPING_TAG.as_ptr() as *mut yaml_char_t);
            if tag.is_null() {
                current_block = 11123220269949747605;
            } else {
                current_block = 11875828834189669668;
            }
        } else {
            current_block = 11875828834189669668;
        }
        match current_block {
            11123220269949747605 => {}
            _ => {
                pairs.start = yaml_malloc(
                    (INITIAL_STACK_SIZE as size_t)
                        .wrapping_mul(::core::mem::size_of::<yaml_node_pair_t>() as size_t),
                ) as *mut yaml_node_pair_t;
                if !(if !pairs.start.is_null() {
                    pairs.top = pairs.start;
                    pairs.end = pairs.start.offset(INITIAL_STACK_SIZE as isize);
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } == 0)
                {
                    memset(
                        &raw mut node as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<yaml_node_t>() as size_t,
                    );
                    node.type_0 = YAML_MAPPING_NODE;
                    node.tag = tag;
                    node.start_mark = (*event).start_mark;
                    node.end_mark = (*event).end_mark;
                    node.data.mapping.pairs.start = pairs.start;
                    node.data.mapping.pairs.end = pairs.end;
                    node.data.mapping.pairs.top = pairs.start;
                    node.data.mapping.style = (*event).data.mapping_start.style;
                    if !(if (*(*parser).document).nodes.top != (*(*parser).document).nodes.end
                        || yaml_stack_extend(
                            &raw mut (*(*parser).document).nodes.start
                                as *mut *mut ::core::ffi::c_void,
                            &raw mut (*(*parser).document).nodes.top
                                as *mut *mut ::core::ffi::c_void,
                            &raw mut (*(*parser).document).nodes.end
                                as *mut *mut ::core::ffi::c_void,
                        ) != 0
                    {
                        let fresh0 = (*(*parser).document).nodes.top;
                        (*(*parser).document).nodes.top = (*(*parser).document).nodes.top.offset(1);
                        *fresh0 = node;
                        1 as ::core::ffi::c_int
                    } else {
                        (*parser).error = YAML_MEMORY_ERROR;
                        0 as ::core::ffi::c_int
                    } == 0)
                    {
                        index = (*(*parser).document)
                            .nodes
                            .top
                            .offset_from((*(*parser).document).nodes.start)
                            as ::core::ffi::c_long
                            as ::core::ffi::c_int;
                        if yaml_parser_register_anchor(
                            parser,
                            index,
                            (*event).data.mapping_start.anchor,
                        ) == 0
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        if yaml_parser_load_node_add(parser, ctx, index) == 0 {
                            return 0 as ::core::ffi::c_int;
                        }
                        if if ((*ctx).top.offset_from((*ctx).start) as ::core::ffi::c_long)
                            < (2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as ::core::ffi::c_long
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        if if (*ctx).top != (*ctx).end
                            || yaml_stack_extend(
                                &raw mut (*ctx).start as *mut *mut ::core::ffi::c_void,
                                &raw mut (*ctx).top as *mut *mut ::core::ffi::c_void,
                                &raw mut (*ctx).end as *mut *mut ::core::ffi::c_void,
                            ) != 0
                        {
                            let fresh1 = (*ctx).top;
                            (*ctx).top = (*ctx).top.offset(1);
                            *fresh1 = index;
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    yaml_free(tag as *mut ::core::ffi::c_void);
    yaml_free((*event).data.mapping_start.anchor as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_load_mapping_end(
    mut parser: *mut yaml_parser_t,
    mut event: *mut yaml_event_t,
    mut ctx: *mut loader_ctx,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if (*ctx).top.offset_from((*ctx).start) as ::core::ffi::c_long > 0 as ::core::ffi::c_long {
        } else {
            __assert_fail(
                b"((*ctx).top - (*ctx).start) > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                535 as ::core::ffi::c_uint,
                b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    index = *(*ctx).top.offset(-(1 as ::core::ffi::c_int as isize));
    '_c2rust_label_0: {
        if (*(*(*parser).document)
            .nodes
            .start
            .offset((index - 1 as ::core::ffi::c_int) as isize))
        .type_0 as ::core::ffi::c_uint
            == YAML_MAPPING_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"parser->document->nodes.start[index-1].type == YAML_MAPPING_NODE\0"
                    as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/loader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                538 as ::core::ffi::c_uint,
                b"int yaml_parser_load_mapping_end(yaml_parser_t *, yaml_event_t *, struct loader_ctx *)\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    };
    (*(*(*parser).document)
        .nodes
        .start
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .end_mark = (*event).end_mark;
    (*ctx).top = (*ctx).top.offset(-1);
    *(*ctx).top;
    return 1 as ::core::ffi::c_int;
}
