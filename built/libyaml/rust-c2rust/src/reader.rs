pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;
pub type _IO_wide_data = libc::c_void;

extern "C" {
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
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
    pub data: C2RustUnnamed_7,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub scalar: C2RustUnnamed_12,
    pub sequence: C2RustUnnamed_10,
    pub mapping: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub pairs: C2RustUnnamed_9,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub struct C2RustUnnamed_10 {
    pub items: C2RustUnnamed_11,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_14,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_13,
    pub start_implicit: ::core::ffi::c_int,
    pub end_implicit: ::core::ffi::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
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
    pub input: C2RustUnnamed_24,
    pub eof: ::core::ffi::c_int,
    pub buffer: C2RustUnnamed_23,
    pub unread: size_t,
    pub raw_buffer: C2RustUnnamed_22,
    pub encoding: yaml_encoding_t,
    pub offset: size_t,
    pub mark: yaml_mark_t,
    pub stream_start_produced: ::core::ffi::c_int,
    pub stream_end_produced: ::core::ffi::c_int,
    pub flow_level: ::core::ffi::c_int,
    pub tokens: C2RustUnnamed_21,
    pub tokens_parsed: size_t,
    pub token_available: ::core::ffi::c_int,
    pub indents: C2RustUnnamed_20,
    pub indent: ::core::ffi::c_int,
    pub simple_key_allowed: ::core::ffi::c_int,
    pub simple_keys: C2RustUnnamed_19,
    pub states: C2RustUnnamed_18,
    pub state: yaml_parser_state_t,
    pub marks: C2RustUnnamed_17,
    pub tag_directives: C2RustUnnamed_16,
    pub aliases: C2RustUnnamed_15,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub start: *mut yaml_alias_data_t,
    pub end: *mut yaml_alias_data_t,
    pub top: *mut yaml_alias_data_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub start: *mut yaml_mark_t,
    pub end: *mut yaml_mark_t,
    pub top: *mut yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub start: *mut yaml_parser_state_t,
    pub end: *mut yaml_parser_state_t,
    pub top: *mut yaml_parser_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub start: *mut yaml_simple_key_t,
    pub end: *mut yaml_simple_key_t,
    pub top: *mut yaml_simple_key_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub start: *mut ::core::ffi::c_int,
    pub end: *mut ::core::ffi::c_int,
    pub top: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub start: *mut yaml_token_t,
    pub end: *mut yaml_token_t,
    pub head: *mut yaml_token_t,
    pub tail: *mut yaml_token_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut ::core::ffi::c_uchar,
    pub end: *mut ::core::ffi::c_uchar,
    pub pointer: *mut ::core::ffi::c_uchar,
    pub last: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub string: C2RustUnnamed_25,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_25 {
    pub start: *const ::core::ffi::c_uchar,
    pub end: *const ::core::ffi::c_uchar,
    pub current: *const ::core::ffi::c_uchar,
}
pub type yaml_parser_t = yaml_parser_s;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 55] = unsafe {
    ::core::mem::transmute::<[u8; 55], [::core::ffi::c_char; 55]>(
        *b"int yaml_parser_update_buffer(yaml_parser_t *, size_t)\0",
    )
};
pub const MAX_FILE_SIZE: size_t = (!(0 as ::core::ffi::c_int as size_t)).wrapping_div(2 as size_t);
unsafe extern "C" fn yaml_parser_set_reader_error(
    mut parser: *mut yaml_parser_t,
    mut problem: *const ::core::ffi::c_char,
    mut offset: size_t,
    mut value: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*parser).error = YAML_READER_ERROR;
    (*parser).problem = problem;
    (*parser).problem_offset = offset;
    (*parser).problem_value = value;
    return 0 as ::core::ffi::c_int;
}
pub const BOM_UTF8: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"\xEF\xBB\xBF\0") };
pub const BOM_UTF16LE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\xFF\xFE\0") };
pub const BOM_UTF16BE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\xFE\xFF\0") };
unsafe extern "C" fn yaml_parser_determine_encoding(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    while (*parser).eof == 0
        && ((*parser)
            .raw_buffer
            .last
            .offset_from((*parser).raw_buffer.pointer) as ::core::ffi::c_long)
            < 3 as ::core::ffi::c_long
    {
        if yaml_parser_update_raw_buffer(parser) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*parser)
        .raw_buffer
        .last
        .offset_from((*parser).raw_buffer.pointer) as ::core::ffi::c_long
        >= 2 as ::core::ffi::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const ::core::ffi::c_void,
            BOM_UTF16LE.as_ptr() as *const ::core::ffi::c_void,
            2 as size_t,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16LE_ENCODING;
        (*parser).raw_buffer.pointer = (*parser)
            .raw_buffer
            .pointer
            .offset(2 as ::core::ffi::c_int as isize);
        (*parser).offset = ((*parser).offset as ::core::ffi::c_ulong)
            .wrapping_add(2 as ::core::ffi::c_ulong) as size_t as size_t;
    } else if (*parser)
        .raw_buffer
        .last
        .offset_from((*parser).raw_buffer.pointer) as ::core::ffi::c_long
        >= 2 as ::core::ffi::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const ::core::ffi::c_void,
            BOM_UTF16BE.as_ptr() as *const ::core::ffi::c_void,
            2 as size_t,
        ) == 0
    {
        (*parser).encoding = YAML_UTF16BE_ENCODING;
        (*parser).raw_buffer.pointer = (*parser)
            .raw_buffer
            .pointer
            .offset(2 as ::core::ffi::c_int as isize);
        (*parser).offset = ((*parser).offset as ::core::ffi::c_ulong)
            .wrapping_add(2 as ::core::ffi::c_ulong) as size_t as size_t;
    } else if (*parser)
        .raw_buffer
        .last
        .offset_from((*parser).raw_buffer.pointer) as ::core::ffi::c_long
        >= 3 as ::core::ffi::c_long
        && memcmp(
            (*parser).raw_buffer.pointer as *const ::core::ffi::c_void,
            BOM_UTF8.as_ptr() as *const ::core::ffi::c_void,
            3 as size_t,
        ) == 0
    {
        (*parser).encoding = YAML_UTF8_ENCODING;
        (*parser).raw_buffer.pointer = (*parser)
            .raw_buffer
            .pointer
            .offset(3 as ::core::ffi::c_int as isize);
        (*parser).offset = ((*parser).offset as ::core::ffi::c_ulong)
            .wrapping_add(3 as ::core::ffi::c_ulong) as size_t as size_t;
    } else {
        (*parser).encoding = YAML_UTF8_ENCODING;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_update_raw_buffer(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut size_read: size_t = 0 as size_t;
    if (*parser).raw_buffer.start == (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.last == (*parser).raw_buffer.end
    {
        return 1 as ::core::ffi::c_int;
    }
    if (*parser).eof != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if (*parser).raw_buffer.start < (*parser).raw_buffer.pointer
        && (*parser).raw_buffer.pointer < (*parser).raw_buffer.last
    {
        memmove(
            (*parser).raw_buffer.start as *mut ::core::ffi::c_void,
            (*parser).raw_buffer.pointer as *const ::core::ffi::c_void,
            (*parser)
                .raw_buffer
                .last
                .offset_from((*parser).raw_buffer.pointer) as ::core::ffi::c_long
                as size_t,
        );
    }
    (*parser).raw_buffer.last = (*parser).raw_buffer.last.offset(
        -((*parser)
            .raw_buffer
            .pointer
            .offset_from((*parser).raw_buffer.start) as ::core::ffi::c_long as isize),
    );
    (*parser).raw_buffer.pointer = (*parser).raw_buffer.start;
    if (*parser).read_handler.expect("non-null function pointer")(
        (*parser).read_handler_data,
        (*parser).raw_buffer.last,
        (*parser)
            .raw_buffer
            .end
            .offset_from((*parser).raw_buffer.last) as ::core::ffi::c_long as size_t,
        &raw mut size_read,
    ) == 0
    {
        return yaml_parser_set_reader_error(
            parser,
            b"input error\0" as *const u8 as *const ::core::ffi::c_char,
            (*parser).offset,
            -(1 as ::core::ffi::c_int),
        );
    }
    (*parser).raw_buffer.last = (*parser).raw_buffer.last.offset(size_read as isize);
    if size_read == 0 {
        (*parser).eof = 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_update_buffer(
    mut parser: *mut yaml_parser_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut first: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    '_c2rust_label: {
        if (*parser).read_handler.is_some() {
        } else {
            __assert_fail(
                b"parser->read_handler\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/reader.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                146 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    if (*parser).eof != 0 && (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
        return 1 as ::core::ffi::c_int;
    }
    if (*parser).unread >= length {
        return 1 as ::core::ffi::c_int;
    }
    if (*parser).encoding as u64 == 0 {
        if yaml_parser_determine_encoding(parser) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*parser).buffer.start < (*parser).buffer.pointer
        && (*parser).buffer.pointer < (*parser).buffer.last
    {
        let mut size: size_t = (*parser).buffer.last.offset_from((*parser).buffer.pointer)
            as ::core::ffi::c_long as size_t;
        memmove(
            (*parser).buffer.start as *mut ::core::ffi::c_void,
            (*parser).buffer.pointer as *const ::core::ffi::c_void,
            size,
        );
        (*parser).buffer.pointer = (*parser).buffer.start;
        (*parser).buffer.last = (*parser).buffer.start.offset(size as isize);
    } else if (*parser).buffer.pointer == (*parser).buffer.last {
        (*parser).buffer.pointer = (*parser).buffer.start;
        (*parser).buffer.last = (*parser).buffer.start;
    }
    while (*parser).unread < length {
        if first == 0 || (*parser).raw_buffer.pointer == (*parser).raw_buffer.last {
            if yaml_parser_update_raw_buffer(parser) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        first = 0 as ::core::ffi::c_int;
        while (*parser).raw_buffer.pointer != (*parser).raw_buffer.last {
            let mut value: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut value2: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut incomplete: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut octet: ::core::ffi::c_uchar = 0;
            let mut width: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            let mut low: ::core::ffi::c_int = 0;
            let mut high: ::core::ffi::c_int = 0;
            let mut k: size_t = 0;
            let mut raw_unread: size_t = (*parser)
                .raw_buffer
                .last
                .offset_from((*parser).raw_buffer.pointer)
                as ::core::ffi::c_long as size_t;
            match (*parser).encoding as ::core::ffi::c_uint {
                1 => {
                    octet = *(*parser)
                        .raw_buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize);
                    width = (if octet as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else if octet as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else if octet as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else if octet as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint;
                    if width == 0 {
                        return yaml_parser_set_reader_error(
                            parser,
                            b"invalid leading UTF-8 octet\0" as *const u8
                                as *const ::core::ffi::c_char,
                            (*parser).offset,
                            octet as ::core::ffi::c_int,
                        );
                    }
                    if width as size_t > raw_unread {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-8 octet sequence\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*parser).offset,
                                -(1 as ::core::ffi::c_int),
                            );
                        }
                        incomplete = 1 as ::core::ffi::c_int;
                    } else {
                        value = (if octet as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            octet as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int
                        } else if octet as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            octet as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int
                        } else if octet as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            octet as ::core::ffi::c_int & 0xf as ::core::ffi::c_int
                        } else if octet as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            octet as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint;
                        k = 1 as size_t;
                        while k < width as size_t {
                            octet = *(*parser).raw_buffer.pointer.offset(k as isize);
                            if octet as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                                != 0x80 as ::core::ffi::c_int
                            {
                                return yaml_parser_set_reader_error(
                                    parser,
                                    b"invalid trailing UTF-8 octet\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*parser).offset.wrapping_add(k),
                                    octet as ::core::ffi::c_int,
                                );
                            }
                            value = (value << 6 as ::core::ffi::c_int).wrapping_add(
                                (octet as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                            );
                            k = k.wrapping_add(1);
                        }
                        if !(width == 1 as ::core::ffi::c_uint
                            || width == 2 as ::core::ffi::c_uint
                                && value >= 0x80 as ::core::ffi::c_uint
                            || width == 3 as ::core::ffi::c_uint
                                && value >= 0x800 as ::core::ffi::c_uint
                            || width == 4 as ::core::ffi::c_uint
                                && value >= 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid length of a UTF-8 sequence\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*parser).offset,
                                -(1 as ::core::ffi::c_int),
                            );
                        }
                        if value >= 0xd800 as ::core::ffi::c_uint
                            && value <= 0xdfff as ::core::ffi::c_uint
                            || value > 0x10ffff as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"invalid Unicode character\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*parser).offset,
                                value as ::core::ffi::c_int,
                            );
                        }
                    }
                }
                2 | 3 => {
                    low = if (*parser).encoding as ::core::ffi::c_uint
                        == YAML_UTF16LE_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                    high = if (*parser).encoding as ::core::ffi::c_uint
                        == YAML_UTF16LE_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    if raw_unread < 2 as size_t {
                        if (*parser).eof != 0 {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"incomplete UTF-16 character\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*parser).offset,
                                -(1 as ::core::ffi::c_int),
                            );
                        }
                        incomplete = 1 as ::core::ffi::c_int;
                    } else {
                        value = (*(*parser).raw_buffer.pointer.offset(low as isize)
                            as ::core::ffi::c_int
                            + ((*(*parser).raw_buffer.pointer.offset(high as isize)
                                as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int))
                            as ::core::ffi::c_uint;
                        if value & 0xfc00 as ::core::ffi::c_uint == 0xdc00 as ::core::ffi::c_uint {
                            return yaml_parser_set_reader_error(
                                parser,
                                b"unexpected low surrogate area\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*parser).offset,
                                value as ::core::ffi::c_int,
                            );
                        }
                        if value & 0xfc00 as ::core::ffi::c_uint == 0xd800 as ::core::ffi::c_uint {
                            width = 4 as ::core::ffi::c_uint;
                            if raw_unread < 4 as size_t {
                                if (*parser).eof != 0 {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"incomplete UTF-16 surrogate pair\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        (*parser).offset,
                                        -(1 as ::core::ffi::c_int),
                                    );
                                }
                                incomplete = 1 as ::core::ffi::c_int;
                            } else {
                                value2 = (*(*parser)
                                    .raw_buffer
                                    .pointer
                                    .offset((low + 2 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + ((*(*parser)
                                        .raw_buffer
                                        .pointer
                                        .offset((high + 2 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int)
                                        << 8 as ::core::ffi::c_int))
                                    as ::core::ffi::c_uint;
                                if value2 & 0xfc00 as ::core::ffi::c_uint
                                    != 0xdc00 as ::core::ffi::c_uint
                                {
                                    return yaml_parser_set_reader_error(
                                        parser,
                                        b"expected low surrogate area\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        (*parser).offset.wrapping_add(2 as size_t),
                                        value2 as ::core::ffi::c_int,
                                    );
                                }
                                value = (0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint)
                                    .wrapping_add(
                                        (value & 0x3ff as ::core::ffi::c_uint)
                                            << 10 as ::core::ffi::c_int,
                                    )
                                    .wrapping_add(value2 & 0x3ff as ::core::ffi::c_uint);
                            }
                        } else {
                            width = 2 as ::core::ffi::c_uint;
                        }
                    }
                }
                _ => {
                    '_c2rust_label_0: {};
                }
            }
            if incomplete != 0 {
                break;
            }
            if !(value == 0x9 as ::core::ffi::c_uint
                || value == 0xa as ::core::ffi::c_uint
                || value == 0xd as ::core::ffi::c_uint
                || value >= 0x20 as ::core::ffi::c_uint && value <= 0x7e as ::core::ffi::c_uint
                || value == 0x85 as ::core::ffi::c_uint
                || value >= 0xa0 as ::core::ffi::c_uint && value <= 0xd7ff as ::core::ffi::c_uint
                || value >= 0xe000 as ::core::ffi::c_uint && value <= 0xfffd as ::core::ffi::c_uint
                || value >= 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint
                    && value <= 0x10ffff as ::core::ffi::c_int as ::core::ffi::c_uint)
            {
                return yaml_parser_set_reader_error(
                    parser,
                    b"control characters are not allowed\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*parser).offset,
                    value as ::core::ffi::c_int,
                );
            }
            (*parser).raw_buffer.pointer = (*parser).raw_buffer.pointer.offset(width as isize);
            (*parser).offset = ((*parser).offset as ::core::ffi::c_ulong)
                .wrapping_add(width as ::core::ffi::c_ulong)
                as size_t as size_t;
            if value <= 0x7f as ::core::ffi::c_uint {
                let fresh0 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh0 = value as yaml_char_t;
            } else if value <= 0x7ff as ::core::ffi::c_uint {
                let fresh1 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh1 = (0xc0 as ::core::ffi::c_uint)
                    .wrapping_add(value >> 6 as ::core::ffi::c_int)
                    as yaml_char_t;
                let fresh2 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh2 = (0x80 as ::core::ffi::c_uint)
                    .wrapping_add(value & 0x3f as ::core::ffi::c_uint)
                    as yaml_char_t;
            } else if value <= 0xffff as ::core::ffi::c_uint {
                let fresh3 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh3 = (0xe0 as ::core::ffi::c_uint)
                    .wrapping_add(value >> 12 as ::core::ffi::c_int)
                    as yaml_char_t;
                let fresh4 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh4 = (0x80 as ::core::ffi::c_uint)
                    .wrapping_add(value >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                    as yaml_char_t;
                let fresh5 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh5 = (0x80 as ::core::ffi::c_uint)
                    .wrapping_add(value & 0x3f as ::core::ffi::c_uint)
                    as yaml_char_t;
            } else {
                let fresh6 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh6 = (0xf0 as ::core::ffi::c_uint)
                    .wrapping_add(value >> 18 as ::core::ffi::c_int)
                    as yaml_char_t;
                let fresh7 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh7 = (0x80 as ::core::ffi::c_uint)
                    .wrapping_add(value >> 12 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                    as yaml_char_t;
                let fresh8 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh8 = (0x80 as ::core::ffi::c_uint)
                    .wrapping_add(value >> 6 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_uint)
                    as yaml_char_t;
                let fresh9 = (*parser).buffer.last;
                (*parser).buffer.last = (*parser).buffer.last.offset(1);
                *fresh9 = (0x80 as ::core::ffi::c_uint)
                    .wrapping_add(value & 0x3f as ::core::ffi::c_uint)
                    as yaml_char_t;
            }
            (*parser).unread = (*parser).unread.wrapping_add(1);
        }
        if (*parser).eof != 0 {
            let fresh10 = (*parser).buffer.last;
            (*parser).buffer.last = (*parser).buffer.last.offset(1);
            *fresh10 = '\0' as i32 as yaml_char_t;
            (*parser).unread = (*parser).unread.wrapping_add(1);
            return 1 as ::core::ffi::c_int;
        }
    }
    if (*parser).offset >= MAX_FILE_SIZE {
        return yaml_parser_set_reader_error(
            parser,
            b"input is too long\0" as *const u8 as *const ::core::ffi::c_char,
            (*parser).offset,
            -(1 as ::core::ffi::c_int),
        );
    }
    return 1 as ::core::ffi::c_int;
}
