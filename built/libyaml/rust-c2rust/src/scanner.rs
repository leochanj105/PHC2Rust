pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;
pub type _IO_wide_data = libc::c_void;

extern "C" {
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn yaml_token_delete(token: *mut yaml_token_t);
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn yaml_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn yaml_free(ptr: *mut ::core::ffi::c_void);
    fn yaml_parser_update_buffer(parser: *mut yaml_parser_t, length: size_t) -> ::core::ffi::c_int;
    fn yaml_string_extend(
        start: *mut *mut yaml_char_t,
        pointer: *mut *mut yaml_char_t,
        end: *mut *mut yaml_char_t,
    ) -> ::core::ffi::c_int;
    fn yaml_string_join(
        a_start: *mut *mut yaml_char_t,
        a_pointer: *mut *mut yaml_char_t,
        a_end: *mut *mut yaml_char_t,
        b_start: *mut *mut yaml_char_t,
        b_pointer: *mut *mut yaml_char_t,
        b_end: *mut *mut yaml_char_t,
    ) -> ::core::ffi::c_int;
    fn yaml_stack_extend(
        start: *mut *mut ::core::ffi::c_void,
        top: *mut *mut ::core::ffi::c_void,
        end: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn yaml_queue_extend(
        start: *mut *mut ::core::ffi::c_void,
        head: *mut *mut ::core::ffi::c_void,
        tail: *mut *mut ::core::ffi::c_void,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}
pub type ptrdiff_t = isize;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 54] = unsafe {
    ::core::mem::transmute::<[u8; 54], [::core::ffi::c_char; 54]>(
        *b"int yaml_parser_scan(yaml_parser_t *, yaml_token_t *)\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_scan(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !parser.is_null() {
        } else {
            __assert_fail(
                b"parser\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/scanner.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                745 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if !token.is_null() {
        } else {
            __assert_fail(
                b"token\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/scanner.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                746 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    memset(
        token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    if (*parser).stream_end_produced != 0 || (*parser).error as ::core::ffi::c_uint != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if (*parser).token_available == 0 {
        if yaml_parser_fetch_more_tokens(parser) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    let fresh0 = (*parser).tokens.head;
    (*parser).tokens.head = (*parser).tokens.head.offset(1);
    *token = *fresh0;
    (*parser).token_available = 0 as ::core::ffi::c_int;
    (*parser).tokens_parsed = (*parser).tokens_parsed.wrapping_add(1);
    if (*token).type_0 as ::core::ffi::c_uint
        == YAML_STREAM_END_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*parser).stream_end_produced = 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_set_scanner_error(
    mut parser: *mut yaml_parser_t,
    mut context: *const ::core::ffi::c_char,
    mut context_mark: yaml_mark_t,
    mut problem: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    (*parser).error = YAML_SCANNER_ERROR;
    (*parser).context = context;
    (*parser).context_mark = context_mark;
    (*parser).problem = problem;
    (*parser).problem_mark = (*parser).mark;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_parser_fetch_more_tokens(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut need_more_tokens: ::core::ffi::c_int = 0;
    loop {
        need_more_tokens = 0 as ::core::ffi::c_int;
        if (*parser).tokens.head == (*parser).tokens.tail {
            need_more_tokens = 1 as ::core::ffi::c_int;
        } else {
            let mut simple_key: *mut yaml_simple_key_t =
                ::core::ptr::null_mut::<yaml_simple_key_t>();
            if yaml_parser_stale_simple_keys(parser) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            simple_key = (*parser).simple_keys.start;
            while simple_key != (*parser).simple_keys.top {
                if (*simple_key).possible != 0
                    && (*simple_key).token_number == (*parser).tokens_parsed
                {
                    need_more_tokens = 1 as ::core::ffi::c_int;
                    break;
                } else {
                    simple_key = simple_key.offset(1);
                }
            }
        }
        if need_more_tokens == 0 {
            break;
        }
        if yaml_parser_fetch_next_token(parser) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*parser).token_available = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_next_token(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    if if (*parser).unread >= 1 as size_t {
        1 as ::core::ffi::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as size_t)
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*parser).stream_start_produced == 0 {
        return yaml_parser_fetch_stream_start(parser);
    }
    if yaml_parser_scan_to_next_token(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_stale_simple_keys(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_unroll_indent(parser, (*parser).mark.column as ptrdiff_t) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).unread >= 4 as size_t {
        1 as ::core::ffi::c_int
    } else {
        yaml_parser_update_buffer(parser, 4 as size_t)
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_stream_end(parser);
    }
    if (*parser).mark.column == 0 as size_t
        && *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '%' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_directive(parser);
    }
    if (*parser).mark.column == 0 as size_t
        && *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        && *(*parser)
            .buffer
            .pointer
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        && *(*parser)
            .buffer
            .pointer
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        && (*(*parser)
            .buffer
            .pointer
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
            || (*(*parser)
                .buffer
                .pointer
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_START_TOKEN);
    }
    if (*parser).mark.column == 0 as size_t
        && *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32 as yaml_char_t as ::core::ffi::c_int
        && *(*parser)
            .buffer
            .pointer
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32 as yaml_char_t as ::core::ffi::c_int
        && *(*parser)
            .buffer
            .pointer
            .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32 as yaml_char_t as ::core::ffi::c_int
        && (*(*parser)
            .buffer
            .pointer
            .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
            || (*(*parser)
                .buffer
                .pointer
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
    {
        return yaml_parser_fetch_document_indicator(parser, YAML_DOCUMENT_END_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '[' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_SEQUENCE_START_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '{' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_collection_start(parser, YAML_FLOW_MAPPING_START_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ']' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_SEQUENCE_END_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '}' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_collection_end(parser, YAML_FLOW_MAPPING_END_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ',' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_entry(parser);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        && (*(*parser)
            .buffer
            .pointer
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
            || (*(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
    {
        return yaml_parser_fetch_block_entry(parser);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '?' as i32 as yaml_char_t as ::core::ffi::c_int
        && ((*parser).flow_level != 0
            || (*(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                || (*(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)))
    {
        return yaml_parser_fetch_key(parser);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ':' as i32 as yaml_char_t as ::core::ffi::c_int
        && ((*parser).flow_level != 0
            || (*(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                || (*(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)))
    {
        return yaml_parser_fetch_value(parser);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '*' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_anchor(parser, YAML_ALIAS_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '&' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_anchor(parser, YAML_ANCHOR_TOKEN);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '!' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_tag(parser);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '|' as i32 as yaml_char_t as ::core::ffi::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 1 as ::core::ffi::c_int);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '>' as i32 as yaml_char_t as ::core::ffi::c_int
        && (*parser).flow_level == 0
    {
        return yaml_parser_fetch_block_scalar(parser, 0 as ::core::ffi::c_int);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_scalar(parser, 1 as ::core::ffi::c_int);
    }
    if *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '"' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        return yaml_parser_fetch_flow_scalar(parser, 0 as ::core::ffi::c_int);
    }
    if !(*(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
        || (*(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '?' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ':' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ',' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '[' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ']' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '{' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '}' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '#' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '&' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '*' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '!' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '|' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '>' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '"' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '%' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '@' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '`' as i32 as yaml_char_t as ::core::ffi::c_int)
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
            && !(*(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
        || (*parser).flow_level == 0
            && (*(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == ':' as i32 as yaml_char_t as ::core::ffi::c_int)
            && !(*(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                || (*(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset((1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
    {
        return yaml_parser_fetch_plain_scalar(parser);
    }
    return yaml_parser_set_scanner_error(
        parser,
        b"while scanning for the next token\0" as *const u8 as *const ::core::ffi::c_char,
        (*parser).mark,
        b"found character that cannot start any token\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn yaml_parser_stale_simple_keys(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut simple_key: *mut yaml_simple_key_t = ::core::ptr::null_mut::<yaml_simple_key_t>();
    simple_key = (*parser).simple_keys.start;
    while simple_key != (*parser).simple_keys.top {
        if (*simple_key).possible != 0
            && ((*simple_key).mark.line < (*parser).mark.line
                || (*simple_key).mark.index.wrapping_add(1024 as size_t) < (*parser).mark.index)
        {
            if (*simple_key).required != 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    b"while scanning a simple key\0" as *const u8 as *const ::core::ffi::c_char,
                    (*simple_key).mark,
                    b"could not find expected ':'\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            (*simple_key).possible = 0 as ::core::ffi::c_int;
        }
        simple_key = simple_key.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_save_simple_key(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut required: ::core::ffi::c_int = ((*parser).flow_level == 0
        && (*parser).indent as ptrdiff_t == (*parser).mark.column as ptrdiff_t)
        as ::core::ffi::c_int;
    if (*parser).simple_key_allowed != 0 {
        let mut simple_key: yaml_simple_key_t = yaml_simple_key_t {
            possible: 0,
            required: 0,
            token_number: 0,
            mark: yaml_mark_t {
                index: 0,
                line: 0,
                column: 0,
            },
        };
        simple_key.possible = 1 as ::core::ffi::c_int;
        simple_key.required = required;
        simple_key.token_number = (*parser)
            .tokens_parsed
            .wrapping_add((*parser).tokens.tail.offset_from((*parser).tokens.head)
                as ::core::ffi::c_long as size_t);
        simple_key.mark = (*parser).mark;
        if yaml_parser_remove_simple_key(parser) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        *(*parser)
            .simple_keys
            .top
            .offset(-(1 as ::core::ffi::c_int as isize)) = simple_key;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_remove_simple_key(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut simple_key: *mut yaml_simple_key_t = (*parser)
        .simple_keys
        .top
        .offset(-(1 as ::core::ffi::c_int as isize));
    if (*simple_key).possible != 0 {
        if (*simple_key).required != 0 {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a simple key\0" as *const u8 as *const ::core::ffi::c_char,
                (*simple_key).mark,
                b"could not find expected ':'\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    (*simple_key).possible = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_increase_flow_level(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut empty_simple_key: yaml_simple_key_t = yaml_simple_key_s {
        possible: 0 as ::core::ffi::c_int,
        required: 0 as ::core::ffi::c_int,
        token_number: 0 as size_t,
        mark: yaml_mark_s {
            index: 0 as size_t,
            line: 0 as size_t,
            column: 0 as size_t,
        },
    };
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            &raw mut (*parser).simple_keys.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).simple_keys.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).simple_keys.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh306 = (*parser).simple_keys.top;
        (*parser).simple_keys.top = (*parser).simple_keys.top.offset(1);
        *fresh306 = empty_simple_key;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*parser).flow_level == INT_MAX {
        (*parser).error = YAML_MEMORY_ERROR;
        return 0 as ::core::ffi::c_int;
    }
    (*parser).flow_level += 1;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_decrease_flow_level(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    if (*parser).flow_level != 0 {
        (*parser).flow_level -= 1;
        (*parser).simple_keys.top = (*parser).simple_keys.top.offset(-1);
        *(*parser).simple_keys.top;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_roll_indent(
    mut parser: *mut yaml_parser_t,
    mut column: ptrdiff_t,
    mut number: ptrdiff_t,
    mut type_0: yaml_token_type_t,
    mut mark: yaml_mark_t,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if ((*parser).indent as ptrdiff_t) < column {
        if if (*parser).indents.top != (*parser).indents.end
            || yaml_stack_extend(
                &raw mut (*parser).indents.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).indents.top as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).indents.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            let fresh299 = (*parser).indents.top;
            (*parser).indents.top = (*parser).indents.top.offset(1);
            *fresh299 = (*parser).indent;
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if column > INT_MAX as ptrdiff_t {
            (*parser).error = YAML_MEMORY_ERROR;
            return 0 as ::core::ffi::c_int;
        }
        (*parser).indent = column as ::core::ffi::c_int;
        memset(
            &raw mut token as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<yaml_token_t>() as size_t,
        );
        token.type_0 = type_0;
        token.start_mark = mark;
        token.end_mark = mark;
        if number == -(1 as ::core::ffi::c_int) as ptrdiff_t {
            if if (*parser).tokens.tail != (*parser).tokens.end
                || yaml_queue_extend(
                    &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
                    &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
                    &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
                    &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
                ) != 0
            {
                let fresh300 = (*parser).tokens.tail;
                (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
                *fresh300 = token;
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        } else if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            memmove(
                (*parser)
                    .tokens
                    .head
                    .offset((number as size_t).wrapping_sub((*parser).tokens_parsed) as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                (*parser)
                    .tokens
                    .head
                    .offset((number as size_t).wrapping_sub((*parser).tokens_parsed) as isize)
                    as *const ::core::ffi::c_void,
                ((*parser).tokens.tail.offset_from((*parser).tokens.head) as ::core::ffi::c_long
                    as size_t)
                    .wrapping_sub((number as size_t).wrapping_sub((*parser).tokens_parsed))
                    .wrapping_mul(::core::mem::size_of::<yaml_token_t>() as size_t),
            );
            *(*parser)
                .tokens
                .head
                .offset((number as size_t).wrapping_sub((*parser).tokens_parsed) as isize) = token;
            (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_unroll_indent(
    mut parser: *mut yaml_parser_t,
    mut column: ptrdiff_t,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level != 0 {
        return 1 as ::core::ffi::c_int;
    }
    while (*parser).indent as ptrdiff_t > column {
        memset(
            &raw mut token as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<yaml_token_t>() as size_t,
        );
        token.type_0 = YAML_BLOCK_END_TOKEN;
        token.start_mark = (*parser).mark;
        token.end_mark = (*parser).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            let fresh308 = (*parser).tokens.tail;
            (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
            *fresh308 = token;
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*parser).indents.top = (*parser).indents.top.offset(-1);
        (*parser).indent = *(*parser).indents.top;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_stream_start(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut simple_key: yaml_simple_key_t = yaml_simple_key_s {
        possible: 0 as ::core::ffi::c_int,
        required: 0 as ::core::ffi::c_int,
        token_number: 0 as size_t,
        mark: yaml_mark_s {
            index: 0 as size_t,
            line: 0 as size_t,
            column: 0 as size_t,
        },
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    (*parser).indent = -(1 as ::core::ffi::c_int);
    if if (*parser).simple_keys.top != (*parser).simple_keys.end
        || yaml_stack_extend(
            &raw mut (*parser).simple_keys.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).simple_keys.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).simple_keys.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh331 = (*parser).simple_keys.top;
        (*parser).simple_keys.top = (*parser).simple_keys.top.offset(1);
        *fresh331 = simple_key;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
    (*parser).stream_start_produced = 1 as ::core::ffi::c_int;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = YAML_STREAM_START_TOKEN;
    token.start_mark = (*parser).mark;
    token.end_mark = (*parser).mark;
    token.data.stream_start.encoding = (*parser).encoding;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh332 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh332 = token;
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
unsafe extern "C" fn yaml_parser_fetch_stream_end(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).mark.column != 0 as size_t {
        (*parser).mark.column = 0 as size_t;
        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
    }
    if yaml_parser_unroll_indent(parser, -(1 as ::core::ffi::c_int) as ptrdiff_t) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = YAML_STREAM_END_TOKEN;
    token.start_mark = (*parser).mark;
    token.end_mark = (*parser).mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh330 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh330 = token;
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
unsafe extern "C" fn yaml_parser_fetch_directive(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_unroll_indent(parser, -(1 as ::core::ffi::c_int) as ptrdiff_t) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    if yaml_parser_scan_directive(parser, &raw mut token) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh309 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh309 = token;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_token_delete(&raw mut token);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_document_indicator(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_unroll_indent(parser, -(1 as ::core::ffi::c_int) as ptrdiff_t) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh307 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh307 = token;
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
unsafe extern "C" fn yaml_parser_fetch_flow_collection_start(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_increase_flow_level(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh305 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh305 = token;
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
unsafe extern "C" fn yaml_parser_fetch_flow_collection_end(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_parser_decrease_flow_level(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = type_0;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh304 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh304 = token;
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
unsafe extern "C" fn yaml_parser_fetch_flow_entry(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = YAML_FLOW_ENTRY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh303 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh303 = token;
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
unsafe extern "C" fn yaml_parser_fetch_block_entry(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                ::core::ptr::null::<::core::ffi::c_char>(),
                (*parser).mark,
                b"block sequence entries are not allowed in this context\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as ptrdiff_t,
            -(1 as ::core::ffi::c_int) as ptrdiff_t,
            YAML_BLOCK_SEQUENCE_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = YAML_BLOCK_ENTRY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh302 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh302 = token;
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
unsafe extern "C" fn yaml_parser_fetch_key(mut parser: *mut yaml_parser_t) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if (*parser).flow_level == 0 {
        if (*parser).simple_key_allowed == 0 {
            return yaml_parser_set_scanner_error(
                parser,
                ::core::ptr::null::<::core::ffi::c_char>(),
                (*parser).mark,
                b"mapping keys are not allowed in this context\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if yaml_parser_roll_indent(
            parser,
            (*parser).mark.column as ptrdiff_t,
            -(1 as ::core::ffi::c_int) as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*parser).mark,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = ((*parser).flow_level == 0) as ::core::ffi::c_int;
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = YAML_KEY_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh301 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh301 = token;
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
unsafe extern "C" fn yaml_parser_fetch_value(mut parser: *mut yaml_parser_t) -> ::core::ffi::c_int {
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    let mut simple_key: *mut yaml_simple_key_t = (*parser)
        .simple_keys
        .top
        .offset(-(1 as ::core::ffi::c_int as isize));
    if (*simple_key).possible != 0 {
        memset(
            &raw mut token as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<yaml_token_t>() as size_t,
        );
        token.type_0 = YAML_KEY_TOKEN;
        token.start_mark = (*simple_key).mark;
        token.end_mark = (*simple_key).mark;
        if if (*parser).tokens.tail != (*parser).tokens.end
            || yaml_queue_extend(
                &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
                &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            memmove(
                (*parser)
                    .tokens
                    .head
                    .offset(
                        (*simple_key)
                            .token_number
                            .wrapping_sub((*parser).tokens_parsed) as isize,
                    )
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                (*parser).tokens.head.offset(
                    (*simple_key)
                        .token_number
                        .wrapping_sub((*parser).tokens_parsed) as isize,
                ) as *const ::core::ffi::c_void,
                ((*parser).tokens.tail.offset_from((*parser).tokens.head) as ::core::ffi::c_long
                    as size_t)
                    .wrapping_sub(
                        (*simple_key)
                            .token_number
                            .wrapping_sub((*parser).tokens_parsed),
                    )
                    .wrapping_mul(::core::mem::size_of::<yaml_token_t>() as size_t),
            );
            *(*parser).tokens.head.offset(
                (*simple_key)
                    .token_number
                    .wrapping_sub((*parser).tokens_parsed) as isize,
            ) = token;
            (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_parser_roll_indent(
            parser,
            (*simple_key).mark.column as ptrdiff_t,
            (*simple_key).token_number as ptrdiff_t,
            YAML_BLOCK_MAPPING_START_TOKEN,
            (*simple_key).mark,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*simple_key).possible = 0 as ::core::ffi::c_int;
        (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    } else {
        if (*parser).flow_level == 0 {
            if (*parser).simple_key_allowed == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    (*parser).mark,
                    b"mapping values are not allowed in this context\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if yaml_parser_roll_indent(
                parser,
                (*parser).mark.column as ptrdiff_t,
                -(1 as ::core::ffi::c_int) as ptrdiff_t,
                YAML_BLOCK_MAPPING_START_TOKEN,
                (*parser).mark,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        (*parser).simple_key_allowed = ((*parser).flow_level == 0) as ::core::ffi::c_int;
    }
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    end_mark = (*parser).mark;
    memset(
        &raw mut token as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_token_t>() as size_t,
    );
    token.type_0 = YAML_VALUE_TOKEN;
    token.start_mark = start_mark;
    token.end_mark = end_mark;
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh298 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh298 = token;
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
unsafe extern "C" fn yaml_parser_fetch_anchor(
    mut parser: *mut yaml_parser_t,
    mut type_0: yaml_token_type_t,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    if yaml_parser_scan_anchor(parser, &raw mut token, type_0) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh277 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh277 = token;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_token_delete(&raw mut token);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_tag(mut parser: *mut yaml_parser_t) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    if yaml_parser_scan_tag(parser, &raw mut token) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh195 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh195 = token;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_token_delete(&raw mut token);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_block_scalar(
    mut parser: *mut yaml_parser_t,
    mut literal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_remove_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
    if yaml_parser_scan_block_scalar(parser, &raw mut token, literal) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh155 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh155 = token;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_token_delete(&raw mut token);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_flow_scalar(
    mut parser: *mut yaml_parser_t,
    mut single: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    if yaml_parser_scan_flow_scalar(parser, &raw mut token, single) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh61 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh61 = token;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_token_delete(&raw mut token);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_fetch_plain_scalar(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    let mut token: yaml_token_t = yaml_token_t {
        type_0: YAML_NO_TOKEN,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_6 {
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
    if yaml_parser_save_simple_key(parser) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*parser).simple_key_allowed = 0 as ::core::ffi::c_int;
    if yaml_parser_scan_plain_scalar(parser, &raw mut token) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if if (*parser).tokens.tail != (*parser).tokens.end
        || yaml_queue_extend(
            &raw mut (*parser).tokens.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*parser).tokens.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh1 = (*parser).tokens.tail;
        (*parser).tokens.tail = (*parser).tokens.tail.offset(1);
        *fresh1 = token;
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_token_delete(&raw mut token);
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_to_next_token(
    mut parser: *mut yaml_parser_t,
) -> ::core::ffi::c_int {
    loop {
        if if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*parser).mark.column == 0 as size_t
            && (*(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -17i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -69i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -65i32 as yaml_char_t as ::core::ffi::c_int)
        {
            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
            (*parser).unread = (*parser).unread.wrapping_sub(1);
            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                4 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })
                        })
                    })
                }) as isize,
            );
        }
        if if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        while *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
            || ((*parser).flow_level != 0 || (*parser).simple_key_allowed == 0)
                && *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
            (*parser).unread = (*parser).unread.wrapping_sub(1);
            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                4 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })
                        })
                    })
                }) as isize,
            );
            if if (*parser).unread >= 1 as size_t {
                1 as ::core::ffi::c_int
            } else {
                yaml_parser_update_buffer(parser, 1 as size_t)
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '#' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            while !(*(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)
            {
                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                (*parser).unread = (*parser).unread.wrapping_sub(1);
                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    4 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                            })
                        })
                    }) as isize,
                );
                if if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        if !(*(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
        {
            break;
        }
        if if (*parser).unread >= 2 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 2 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            && *(*parser)
                .buffer
                .pointer
                .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            (*parser).mark.index = ((*parser).mark.index as ::core::ffi::c_ulong)
                .wrapping_add(2 as ::core::ffi::c_ulong)
                as size_t as size_t;
            (*parser).mark.column = 0 as size_t;
            (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
            (*parser).unread = ((*parser).unread as ::core::ffi::c_ulong)
                .wrapping_sub(2 as ::core::ffi::c_ulong) as size_t
                as size_t;
            (*parser).buffer.pointer = (*parser)
                .buffer
                .pointer
                .offset(2 as ::core::ffi::c_int as isize);
        } else {
            if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
            {
                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                (*parser).mark.column = 0 as size_t;
                (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                (*parser).unread = (*parser).unread.wrapping_sub(1);
                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    4 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                            })
                        })
                    }) as isize,
                );
            } else {
            };
        };
        if (*parser).flow_level == 0 {
            (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_directive(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut name: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    let mut major: ::core::ffi::c_int = 0;
    let mut minor: ::core::ffi::c_int = 0;
    let mut handle: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    let mut prefix: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    start_mark = (*parser).mark;
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    if !(yaml_parser_scan_directive_name(parser, start_mark, &raw mut name) == 0) {
        if strcmp(
            name as *mut ::core::ffi::c_char,
            b"YAML\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if yaml_parser_scan_version_directive_value(
                parser,
                start_mark,
                &raw mut major,
                &raw mut minor,
            ) == 0
            {
                current_block = 7130474914167427671;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<yaml_token_t>() as size_t,
                );
                (*token).type_0 = YAML_VERSION_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                (*token).data.version_directive.major = major;
                (*token).data.version_directive.minor = minor;
                current_block = 1841672684692190573;
            }
        } else if strcmp(
            name as *mut ::core::ffi::c_char,
            b"TAG\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if yaml_parser_scan_tag_directive_value(
                parser,
                start_mark,
                &raw mut handle,
                &raw mut prefix,
            ) == 0
            {
                current_block = 7130474914167427671;
            } else {
                end_mark = (*parser).mark;
                memset(
                    token as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<yaml_token_t>() as size_t,
                );
                (*token).type_0 = YAML_TAG_DIRECTIVE_TOKEN;
                (*token).start_mark = start_mark;
                (*token).end_mark = end_mark;
                (*token).data.tag_directive.handle = handle;
                (*token).data.tag_directive.prefix = prefix;
                current_block = 1841672684692190573;
            }
        } else {
            yaml_parser_set_scanner_error(
                parser,
                b"while scanning a directive\0" as *const u8 as *const ::core::ffi::c_char,
                start_mark,
                b"found unknown directive name\0" as *const u8 as *const ::core::ffi::c_char,
            );
            current_block = 7130474914167427671;
        }
        match current_block {
            7130474914167427671 => {}
            _ => {
                if !(if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0)
                {
                    loop {
                        if !(*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
                        {
                            current_block = 7149356873433890176;
                            break;
                        }
                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0xf8 as ::core::ffi::c_int
                                            == 0xf0 as ::core::ffi::c_int
                                        {
                                            4 as ::core::ffi::c_int
                                        } else {
                                            0 as ::core::ffi::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        );
                        if if (*parser).unread >= 1 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as size_t)
                        } == 0
                        {
                            current_block = 7130474914167427671;
                            break;
                        }
                    }
                    match current_block {
                        7130474914167427671 => {}
                        _ => {
                            if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '#' as i32 as yaml_char_t as ::core::ffi::c_int
                            {
                                loop {
                                    if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == -62i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -123i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -88i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -87i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\0' as i32 as yaml_char_t as ::core::ffi::c_int
                                    {
                                        current_block = 5601891728916014340;
                                        break;
                                    }
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    3 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        4 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                    if if (*parser).unread >= 1 as size_t {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        yaml_parser_update_buffer(parser, 1 as size_t)
                                    } == 0
                                    {
                                        current_block = 7130474914167427671;
                                        break;
                                    }
                                }
                            } else {
                                current_block = 5601891728916014340;
                            }
                            match current_block {
                                7130474914167427671 => {}
                                _ => {
                                    if !(*(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == -62i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -123i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -88i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == -87i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)
                                    {
                                        yaml_parser_set_scanner_error(
                                            parser,
                                            b"while scanning a directive\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                            start_mark,
                                            b"did not find expected comment or line break\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    } else {
                                        if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (0 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -123i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (0 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (0 as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -88i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (0 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (0 as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -87i32 as yaml_char_t as ::core::ffi::c_int
                                        {
                                            if if (*parser).unread >= 2 as size_t {
                                                1 as ::core::ffi::c_int
                                            } else {
                                                yaml_parser_update_buffer(parser, 2 as size_t)
                                            } == 0
                                            {
                                                current_block = 7130474914167427671;
                                            } else {
                                                if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\r' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(
                                                        (0 as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as ::core::ffi::c_int
                                                        == '\n' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                {
                                                    (*parser).mark.index = ((*parser).mark.index
                                                        as ::core::ffi::c_ulong)
                                                        .wrapping_add(2 as ::core::ffi::c_ulong)
                                                        as size_t
                                                        as size_t;
                                                    (*parser).mark.column = 0 as size_t;
                                                    (*parser).mark.line =
                                                        (*parser).mark.line.wrapping_add(1);
                                                    (*parser).unread = ((*parser).unread
                                                        as ::core::ffi::c_ulong)
                                                        .wrapping_sub(2 as ::core::ffi::c_ulong)
                                                        as size_t
                                                        as size_t;
                                                    (*parser).buffer.pointer = (*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(2 as ::core::ffi::c_int as isize);
                                                } else {
                                                    if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\r' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == '\n' as i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -62i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -123i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -30i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 2 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -88i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -30i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 2 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -87i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                    {
                                                        (*parser).mark.index =
                                                            (*parser).mark.index.wrapping_add(1);
                                                        (*parser).mark.column = 0 as size_t;
                                                        (*parser).mark.line =
                                                            (*parser).mark.line.wrapping_add(1);
                                                        (*parser).unread =
                                                            (*parser).unread.wrapping_sub(1);
                                                        (*parser).buffer.pointer = (*parser)
                                                            .buffer
                                                            .pointer
                                                            .offset(
                                                                (if *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                                    as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                                                                    == 0 as ::core::ffi::c_int
                                                                {
                                                                    1 as ::core::ffi::c_int
                                                                } else {
                                                                    (if *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                        as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                                                                        == 0xc0 as ::core::ffi::c_int
                                                                    {
                                                                        2 as ::core::ffi::c_int
                                                                    } else {
                                                                        (if *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                                                            == 0xe0 as ::core::ffi::c_int
                                                                        {
                                                                            3 as ::core::ffi::c_int
                                                                        } else {
                                                                            (if *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                                                                                == 0xf0 as ::core::ffi::c_int
                                                                            {
                                                                                4 as ::core::ffi::c_int
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            })
                                                                        })
                                                                    })
                                                                }) as isize,
                                                            );
                                                    } else {
                                                    };
                                                };
                                                current_block = 2232869372362427478;
                                            }
                                        } else {
                                            current_block = 2232869372362427478;
                                        }
                                        match current_block {
                                            7130474914167427671 => {}
                                            _ => {
                                                yaml_free(name as *mut ::core::ffi::c_void);
                                                return 1 as ::core::ffi::c_int;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(prefix as *mut ::core::ffi::c_void);
    yaml_free(handle as *mut ::core::ffi::c_void);
    yaml_free(name as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_directive_name(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut name: *mut *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        if !(if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0)
        {
            loop {
                if !(*(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '_' as i32
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '-' as i32)
                {
                    current_block = 6873731126896040597;
                    break;
                }
                if if if string.pointer.offset(5 as ::core::ffi::c_int as isize) < string.end
                    || yaml_string_extend(
                        &raw mut string.start,
                        &raw mut string.pointer,
                        &raw mut string.end,
                    ) != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } != 0
                {
                    if *(*parser).buffer.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh310 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                        let fresh311 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        *fresh311 = *fresh310;
                    } else {
                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh312 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh313 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh313 = *fresh312;
                            let fresh314 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh315 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh315 = *fresh314;
                        } else {
                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh316 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh317 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh317 = *fresh316;
                                let fresh318 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh319 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh319 = *fresh318;
                                let fresh320 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh321 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh321 = *fresh320;
                            } else {
                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh322 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh323 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh323 = *fresh322;
                                    let fresh324 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh325 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh325 = *fresh324;
                                    let fresh326 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh327 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh327 = *fresh326;
                                    let fresh328 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh329 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh329 = *fresh328;
                                } else {
                                };
                            };
                        };
                    };
                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0
                {
                    current_block = 17373271253665380711;
                    break;
                }
                if if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0
                {
                    current_block = 17373271253665380711;
                    break;
                }
            }
            match current_block {
                17373271253665380711 => {}
                _ => {
                    if string.start == string.pointer {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a directive\0" as *const u8
                                as *const ::core::ffi::c_char,
                            start_mark,
                            b"could not find expected directive name\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else if !(*(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                        || (*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -123i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -88i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -87i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a directive\0" as *const u8
                                as *const ::core::ffi::c_char,
                            start_mark,
                            b"found unexpected non-alphabetical character\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        *name = string.start;
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut ::core::ffi::c_void);
    string.end = ::core::ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_version_directive_value(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut major: *mut ::core::ffi::c_int,
    mut minor: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if if (*parser).unread >= 1 as size_t {
        1 as ::core::ffi::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as size_t)
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    while *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        || *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    })
                })
            }) as isize,
        );
        if if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if yaml_parser_scan_version_directive_number(parser, start_mark, major) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !(*(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '.' as i32 as yaml_char_t as ::core::ffi::c_int)
    {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const ::core::ffi::c_char,
            start_mark,
            b"did not find expected digit or '.' character\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
    (*parser).unread = (*parser).unread.wrapping_sub(1);
    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
        (if *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    })
                })
            })
        }) as isize,
    );
    if yaml_parser_scan_version_directive_number(parser, start_mark, minor) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
pub const MAX_NUMBER_LENGTH: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
unsafe extern "C" fn yaml_parser_scan_version_directive_number(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut number: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut value: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut length: size_t = 0 as size_t;
    if if (*parser).unread >= 1 as size_t {
        1 as ::core::ffi::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as size_t)
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    while *(*parser)
        .buffer
        .pointer
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
        && *(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        length = length.wrapping_add(1);
        if length > MAX_NUMBER_LENGTH as size_t {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a %YAML directive\0" as *const u8 as *const ::core::ffi::c_char,
                start_mark,
                b"found extremely long version number\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        value = value * 10 as ::core::ffi::c_int
            + (*(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - '0' as i32 as yaml_char_t as ::core::ffi::c_int);
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    })
                })
            }) as isize,
        );
        if if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if length == 0 {
        return yaml_parser_set_scanner_error(
            parser,
            b"while scanning a %YAML directive\0" as *const u8 as *const ::core::ffi::c_char,
            start_mark,
            b"did not find expected version number\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    *number = value;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_tag_directive_value(
    mut parser: *mut yaml_parser_t,
    mut start_mark: yaml_mark_t,
    mut handle: *mut *mut yaml_char_t,
    mut prefix: *mut *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut handle_value: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    let mut prefix_value: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    if if (*parser).unread >= 1 as size_t {
        1 as ::core::ffi::c_int
    } else {
        yaml_parser_update_buffer(parser, 1 as size_t)
    } == 0
    {
        current_block = 14529950745389461720;
    } else {
        current_block = 6239978542346980191;
    }
    '_error: loop {
        match current_block {
            14529950745389461720 => {
                yaml_free(handle_value as *mut ::core::ffi::c_void);
                yaml_free(prefix_value as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            }
            _ => {
                if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf8 as ::core::ffi::c_int
                                        == 0xf0 as ::core::ffi::c_int
                                    {
                                        4 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                })
                            })
                        }) as isize,
                    );
                    if if (*parser).unread >= 1 as size_t {
                        1 as ::core::ffi::c_int
                    } else {
                        yaml_parser_update_buffer(parser, 1 as size_t)
                    } == 0
                    {
                        current_block = 14529950745389461720;
                    } else {
                        current_block = 6239978542346980191;
                    }
                } else {
                    if yaml_parser_scan_tag_handle(
                        parser,
                        1 as ::core::ffi::c_int,
                        start_mark,
                        &raw mut handle_value,
                    ) == 0
                    {
                        current_block = 14529950745389461720;
                        continue;
                    }
                    if if (*parser).unread >= 1 as size_t {
                        1 as ::core::ffi::c_int
                    } else {
                        yaml_parser_update_buffer(parser, 1 as size_t)
                    } == 0
                    {
                        current_block = 14529950745389461720;
                        continue;
                    }
                    if !(*(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            b"while scanning a %TAG directive\0" as *const u8
                                as *const ::core::ffi::c_char,
                            start_mark,
                            b"did not find expected whitespace\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 14529950745389461720;
                    } else {
                        while *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0x80 as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xe0 as ::core::ffi::c_int
                                        == 0xc0 as ::core::ffi::c_int
                                    {
                                        2 as ::core::ffi::c_int
                                    } else {
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0xf0 as ::core::ffi::c_int
                                            == 0xe0 as ::core::ffi::c_int
                                        {
                                            3 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xf8 as ::core::ffi::c_int
                                                == 0xf0 as ::core::ffi::c_int
                                            {
                                                4 as ::core::ffi::c_int
                                            } else {
                                                0 as ::core::ffi::c_int
                                            })
                                        })
                                    })
                                }) as isize,
                            );
                            if if (*parser).unread >= 1 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as size_t)
                            } == 0
                            {
                                current_block = 14529950745389461720;
                                continue '_error;
                            }
                        }
                        if yaml_parser_scan_tag_uri(
                            parser,
                            1 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                            ::core::ptr::null_mut::<yaml_char_t>(),
                            start_mark,
                            &raw mut prefix_value,
                        ) == 0
                        {
                            current_block = 14529950745389461720;
                            continue;
                        }
                        if if (*parser).unread >= 1 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as size_t)
                        } == 0
                        {
                            current_block = 14529950745389461720;
                            continue;
                        }
                        if !(*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                            || (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a %TAG directive\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                start_mark,
                                b"did not find expected whitespace or line break\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 14529950745389461720;
                        } else {
                            *handle = handle_value;
                            *prefix = prefix_value;
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn yaml_parser_scan_anchor(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    mut type_0: yaml_token_type_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        start_mark = (*parser).mark;
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    })
                })
            }) as isize,
        );
        if !(if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0)
        {
            loop {
                if !(*(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '_' as i32
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '-' as i32)
                {
                    current_block = 17216689946888361452;
                    break;
                }
                if if if string.pointer.offset(5 as ::core::ffi::c_int as isize) < string.end
                    || yaml_string_extend(
                        &raw mut string.start,
                        &raw mut string.pointer,
                        &raw mut string.end,
                    ) != 0
                {
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } != 0
                {
                    if *(*parser).buffer.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh278 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                        let fresh279 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        *fresh279 = *fresh278;
                    } else {
                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh280 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh281 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh281 = *fresh280;
                            let fresh282 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh283 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh283 = *fresh282;
                        } else {
                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh284 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh285 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh285 = *fresh284;
                                let fresh286 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh287 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh287 = *fresh286;
                                let fresh288 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh289 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh289 = *fresh288;
                            } else {
                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh290 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh291 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh291 = *fresh290;
                                    let fresh292 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh293 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh293 = *fresh292;
                                    let fresh294 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh295 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh295 = *fresh294;
                                    let fresh296 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh297 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh297 = *fresh296;
                                } else {
                                };
                            };
                        };
                    };
                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                } == 0
                {
                    current_block = 14426145975659588934;
                    break;
                }
                if if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0
                {
                    current_block = 14426145975659588934;
                    break;
                }
                length += 1;
            }
            match current_block {
                14426145975659588934 => {}
                _ => {
                    end_mark = (*parser).mark;
                    if length == 0
                        || !(*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                            || (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '?' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ':' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ',' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ']' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '}' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '%' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '@' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '`' as i32 as yaml_char_t as ::core::ffi::c_int)
                    {
                        yaml_parser_set_scanner_error(
                            parser,
                            if type_0 as ::core::ffi::c_uint
                                == YAML_ANCHOR_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                b"while scanning an anchor\0" as *const u8
                                    as *const ::core::ffi::c_char
                            } else {
                                b"while scanning an alias\0" as *const u8
                                    as *const ::core::ffi::c_char
                            },
                            start_mark,
                            b"did not find expected alphabetic or numeric character\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    } else {
                        if type_0 as ::core::ffi::c_uint
                            == YAML_ANCHOR_TOKEN as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            memset(
                                token as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<yaml_token_t>() as size_t,
                            );
                            (*token).type_0 = YAML_ANCHOR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.anchor.value = string.start;
                        } else {
                            memset(
                                token as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<yaml_token_t>() as size_t,
                            );
                            (*token).type_0 = YAML_ALIAS_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.alias.value = string.start;
                        }
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut ::core::ffi::c_void);
    string.end = ::core::ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_tag(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut handle: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    let mut suffix: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    start_mark = (*parser).mark;
    if !(if (*parser).unread >= 2 as size_t {
        1 as ::core::ffi::c_int
    } else {
        yaml_parser_update_buffer(parser, 2 as size_t)
    } == 0)
    {
        if *(*parser)
            .buffer
            .pointer
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '<' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            handle = yaml_malloc(1 as size_t) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 9842263913179103545;
            } else {
                *handle.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as yaml_char_t;
                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                (*parser).unread = (*parser).unread.wrapping_sub(1);
                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    4 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                            })
                        })
                    }) as isize,
                );
                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                (*parser).unread = (*parser).unread.wrapping_sub(1);
                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    4 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                            })
                        })
                    }) as isize,
                );
                if yaml_parser_scan_tag_uri(
                    parser,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<yaml_char_t>(),
                    start_mark,
                    &raw mut suffix,
                ) == 0
                {
                    current_block = 9842263913179103545;
                } else if !(*(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '>' as i32 as yaml_char_t as ::core::ffi::c_int)
                {
                    yaml_parser_set_scanner_error(
                        parser,
                        b"while scanning a tag\0" as *const u8 as *const ::core::ffi::c_char,
                        start_mark,
                        b"did not find the expected '>'\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    current_block = 9842263913179103545;
                } else {
                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf8 as ::core::ffi::c_int
                                        == 0xf0 as ::core::ffi::c_int
                                    {
                                        4 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                })
                            })
                        }) as isize,
                    );
                    current_block = 10652014663920648156;
                }
            }
        } else if yaml_parser_scan_tag_handle(
            parser,
            0 as ::core::ffi::c_int,
            start_mark,
            &raw mut handle,
        ) == 0
        {
            current_block = 9842263913179103545;
        } else if *handle.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '!' as i32
            && *handle.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '\0' as i32
            && *handle.offset(
                strlen(handle as *mut ::core::ffi::c_char).wrapping_sub(1 as size_t) as isize,
            ) as ::core::ffi::c_int
                == '!' as i32
        {
            if yaml_parser_scan_tag_uri(
                parser,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                ::core::ptr::null_mut::<yaml_char_t>(),
                start_mark,
                &raw mut suffix,
            ) == 0
            {
                current_block = 9842263913179103545;
            } else {
                current_block = 10652014663920648156;
            }
        } else if yaml_parser_scan_tag_uri(
            parser,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            handle,
            start_mark,
            &raw mut suffix,
        ) == 0
        {
            current_block = 9842263913179103545;
        } else {
            yaml_free(handle as *mut ::core::ffi::c_void);
            handle = yaml_malloc(2 as size_t) as *mut yaml_char_t;
            if handle.is_null() {
                current_block = 9842263913179103545;
            } else {
                *handle.offset(0 as ::core::ffi::c_int as isize) = '!' as i32 as yaml_char_t;
                *handle.offset(1 as ::core::ffi::c_int as isize) = '\0' as i32 as yaml_char_t;
                if *suffix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32
                {
                    let mut tmp: *mut yaml_char_t = handle;
                    handle = suffix;
                    suffix = tmp;
                }
                current_block = 10652014663920648156;
            }
        }
        match current_block {
            9842263913179103545 => {}
            _ => {
                if !(if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0)
                {
                    if !(*(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                        || (*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -123i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -88i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser).buffer.pointer.offset(
                                    (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int
                                    == -87i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                    {
                        if (*parser).flow_level == 0
                            || !(*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ',' as i32 as yaml_char_t as ::core::ffi::c_int)
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a tag\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                start_mark,
                                b"did not find expected whitespace or line break\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 9842263913179103545;
                        } else {
                            current_block = 14648156034262866959;
                        }
                    } else {
                        current_block = 14648156034262866959;
                    }
                    match current_block {
                        9842263913179103545 => {}
                        _ => {
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<yaml_token_t>() as size_t,
                            );
                            (*token).type_0 = YAML_TAG_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.tag.handle = handle;
                            (*token).data.tag.suffix = suffix;
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free(handle as *mut ::core::ffi::c_void);
    yaml_free(suffix as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_tag_handle(
    mut parser: *mut yaml_parser_t,
    mut directive: ::core::ffi::c_int,
    mut start_mark: yaml_mark_t,
    mut handle: *mut *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        if !(if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0)
        {
            if !(*(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '!' as i32 as yaml_char_t as ::core::ffi::c_int)
            {
                yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while scanning a tag directive\0" as *const u8
                            as *const ::core::ffi::c_char
                    } else {
                        b"while scanning a tag\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    start_mark,
                    b"did not find expected '!'\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else if !(if if string.pointer.offset(5 as ::core::ffi::c_int as isize) < string.end
                || yaml_string_extend(
                    &raw mut string.start,
                    &raw mut string.pointer,
                    &raw mut string.end,
                ) != 0
            {
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } != 0
            {
                if *(*parser).buffer.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let fresh217 = (*parser).buffer.pointer;
                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                    let fresh218 = string.pointer;
                    string.pointer = string.pointer.offset(1);
                    *fresh218 = *fresh217;
                } else {
                    if *(*parser).buffer.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        let fresh219 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                        let fresh220 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        *fresh220 = *fresh219;
                        let fresh221 = (*parser).buffer.pointer;
                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                        let fresh222 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        *fresh222 = *fresh221;
                    } else {
                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            let fresh223 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh224 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh224 = *fresh223;
                            let fresh225 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh226 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh226 = *fresh225;
                            let fresh227 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh228 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            *fresh228 = *fresh227;
                        } else {
                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                let fresh229 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh230 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh230 = *fresh229;
                                let fresh231 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh232 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh232 = *fresh231;
                                let fresh233 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh234 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh234 = *fresh233;
                                let fresh235 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh236 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh236 = *fresh235;
                            } else {
                            };
                        };
                    };
                };
                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                (*parser).unread = (*parser).unread.wrapping_sub(1);
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            } == 0)
            {
                if !(if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0)
                {
                    loop {
                        if !(*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                            && *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '_' as i32
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '-' as i32)
                        {
                            current_block = 3640593987805443782;
                            break;
                        }
                        if if if string.pointer.offset(5 as ::core::ffi::c_int as isize)
                            < string.end
                            || yaml_string_extend(
                                &raw mut string.start,
                                &raw mut string.pointer,
                                &raw mut string.end,
                            ) != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } != 0
                        {
                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                let fresh237 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh238 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh238 = *fresh237;
                            } else {
                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    let fresh239 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh240 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh240 = *fresh239;
                                    let fresh241 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh242 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh242 = *fresh241;
                                } else {
                                    if *(*parser).buffer.pointer as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        let fresh243 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh244 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh244 = *fresh243;
                                        let fresh245 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh246 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh246 = *fresh245;
                                        let fresh247 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh248 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh248 = *fresh247;
                                    } else {
                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                            & 0xf8 as ::core::ffi::c_int
                                            == 0xf0 as ::core::ffi::c_int
                                        {
                                            let fresh249 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh250 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh250 = *fresh249;
                                            let fresh251 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh252 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh252 = *fresh251;
                                            let fresh253 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh254 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh254 = *fresh253;
                                            let fresh255 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh256 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh256 = *fresh255;
                                        } else {
                                        };
                                    };
                                };
                            };
                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            current_block = 12464373246871903111;
                            break;
                        }
                        if if (*parser).unread >= 1 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as size_t)
                        } == 0
                        {
                            current_block = 12464373246871903111;
                            break;
                        }
                    }
                    match current_block {
                        12464373246871903111 => {}
                        _ => {
                            if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '!' as i32 as yaml_char_t as ::core::ffi::c_int
                            {
                                if if if string.pointer.offset(5 as ::core::ffi::c_int as isize)
                                    < string.end
                                    || yaml_string_extend(
                                        &raw mut string.start,
                                        &raw mut string.pointer,
                                        &raw mut string.end,
                                    ) != 0
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as ::core::ffi::c_int
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as ::core::ffi::c_int
                                        & 0x80 as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                    {
                                        let fresh257 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh258 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh258 = *fresh257;
                                    } else {
                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                            & 0xe0 as ::core::ffi::c_int
                                            == 0xc0 as ::core::ffi::c_int
                                        {
                                            let fresh259 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh260 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh260 = *fresh259;
                                            let fresh261 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh262 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh262 = *fresh261;
                                        } else {
                                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                & 0xf0 as ::core::ffi::c_int
                                                == 0xe0 as ::core::ffi::c_int
                                            {
                                                let fresh263 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh264 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh264 = *fresh263;
                                                let fresh265 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh266 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh266 = *fresh265;
                                                let fresh267 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh268 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh268 = *fresh267;
                                            } else {
                                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                    & 0xf8 as ::core::ffi::c_int
                                                    == 0xf0 as ::core::ffi::c_int
                                                {
                                                    let fresh269 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh270 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh270 = *fresh269;
                                                    let fresh271 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh272 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh272 = *fresh271;
                                                    let fresh273 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh274 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh274 = *fresh273;
                                                    let fresh275 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh276 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh276 = *fresh275;
                                                } else {
                                                };
                                            };
                                        };
                                    };
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                } == 0
                                {
                                    current_block = 12464373246871903111;
                                } else {
                                    current_block = 17860125682698302841;
                                }
                            } else if directive != 0
                                && !(*string.start.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '!' as i32
                                    && *string.start.offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\0' as i32)
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while parsing a tag directive\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    start_mark,
                                    b"did not find expected '!'\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                current_block = 12464373246871903111;
                            } else {
                                current_block = 17860125682698302841;
                            }
                            match current_block {
                                12464373246871903111 => {}
                                _ => {
                                    *handle = string.start;
                                    return 1 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut ::core::ffi::c_void);
    string.end = ::core::ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_tag_uri(
    mut parser: *mut yaml_parser_t,
    mut uri_char: ::core::ffi::c_int,
    mut directive: ::core::ffi::c_int,
    mut head: *mut yaml_char_t,
    mut start_mark: yaml_mark_t,
    mut uri: *mut *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut length: size_t = if !head.is_null() {
        strlen(head as *mut ::core::ffi::c_char)
    } else {
        0 as size_t
    };
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        current_block = 835382562062889664;
    } else {
        current_block = 6239978542346980191;
    }
    '_error: loop {
        match current_block {
            835382562062889664 => {
                yaml_free(string.start as *mut ::core::ffi::c_void);
                string.end = ::core::ptr::null_mut::<yaml_char_t>();
                string.pointer = string.end;
                string.start = string.pointer;
                return 0 as ::core::ffi::c_int;
            }
            _ => {
                if string.end.offset_from(string.start) as ::core::ffi::c_long as size_t <= length {
                    if !(yaml_string_extend(
                        &raw mut string.start,
                        &raw mut string.pointer,
                        &raw mut string.end,
                    ) == 0)
                    {
                        current_block = 6239978542346980191;
                        continue;
                    }
                    (*parser).error = YAML_MEMORY_ERROR;
                    current_block = 835382562062889664;
                } else {
                    if length > 1 as size_t {
                        memcpy(
                            string.start as *mut ::core::ffi::c_void,
                            head.offset(1 as ::core::ffi::c_int as isize)
                                as *const ::core::ffi::c_void,
                            length.wrapping_sub(1 as size_t),
                        );
                        string.pointer = string
                            .pointer
                            .offset(length.wrapping_sub(1 as size_t) as isize);
                    }
                    if if (*parser).unread >= 1 as size_t {
                        1 as ::core::ffi::c_int
                    } else {
                        yaml_parser_update_buffer(parser, 1 as size_t)
                    } == 0
                    {
                        current_block = 835382562062889664;
                        continue;
                    }
                    while *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                            && *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                            && *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '_' as i32
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '-' as i32
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ';' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '/' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '?' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ':' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '@' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '&' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '=' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '+' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '$' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '.' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '%' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '!' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '~' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '*' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '(' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ')' as i32 as yaml_char_t as ::core::ffi::c_int
                        || uri_char != 0
                            && (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ',' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '[' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ']' as i32 as yaml_char_t as ::core::ffi::c_int)
                    {
                        if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '%' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            if if string.pointer.offset(5 as ::core::ffi::c_int as isize)
                                < string.end
                                || yaml_string_extend(
                                    &raw mut string.start,
                                    &raw mut string.pointer,
                                    &raw mut string.end,
                                ) != 0
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (*parser).error = YAML_MEMORY_ERROR;
                                0 as ::core::ffi::c_int
                            } == 0
                            {
                                current_block = 835382562062889664;
                                continue '_error;
                            }
                            if yaml_parser_scan_uri_escapes(
                                parser,
                                directive,
                                start_mark,
                                &raw mut string,
                            ) == 0
                            {
                                current_block = 835382562062889664;
                                continue '_error;
                            }
                        } else if if if string.pointer.offset(5 as ::core::ffi::c_int as isize)
                            < string.end
                            || yaml_string_extend(
                                &raw mut string.start,
                                &raw mut string.pointer,
                                &raw mut string.end,
                            ) != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } != 0
                        {
                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                let fresh196 = (*parser).buffer.pointer;
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                let fresh197 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                *fresh197 = *fresh196;
                            } else {
                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    let fresh198 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh199 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh199 = *fresh198;
                                    let fresh200 = (*parser).buffer.pointer;
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                                    let fresh201 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh201 = *fresh200;
                                } else {
                                    if *(*parser).buffer.pointer as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        let fresh202 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh203 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh203 = *fresh202;
                                        let fresh204 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh205 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh205 = *fresh204;
                                        let fresh206 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh207 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh207 = *fresh206;
                                    } else {
                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                            & 0xf8 as ::core::ffi::c_int
                                            == 0xf0 as ::core::ffi::c_int
                                        {
                                            let fresh208 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh209 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh209 = *fresh208;
                                            let fresh210 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh211 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh211 = *fresh210;
                                            let fresh212 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh213 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh213 = *fresh212;
                                            let fresh214 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh215 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh215 = *fresh214;
                                        } else {
                                        };
                                    };
                                };
                            };
                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            current_block = 835382562062889664;
                            continue '_error;
                        }
                        length = length.wrapping_add(1);
                        if if (*parser).unread >= 1 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as size_t)
                        } == 0
                        {
                            current_block = 835382562062889664;
                            continue '_error;
                        }
                    }
                    if length == 0 {
                        if if string.pointer.offset(5 as ::core::ffi::c_int as isize) < string.end
                            || yaml_string_extend(
                                &raw mut string.start,
                                &raw mut string.pointer,
                                &raw mut string.end,
                            ) != 0
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (*parser).error = YAML_MEMORY_ERROR;
                            0 as ::core::ffi::c_int
                        } == 0
                        {
                            current_block = 835382562062889664;
                            continue;
                        }
                        yaml_parser_set_scanner_error(
                            parser,
                            if directive != 0 {
                                b"while parsing a %TAG directive\0" as *const u8
                                    as *const ::core::ffi::c_char
                            } else {
                                b"while parsing a tag\0" as *const u8 as *const ::core::ffi::c_char
                            },
                            start_mark,
                            b"did not find expected tag URI\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        current_block = 835382562062889664;
                    } else {
                        *uri = string.start;
                        return 1 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
}
unsafe extern "C" fn yaml_parser_scan_uri_escapes(
    mut parser: *mut yaml_parser_t,
    mut directive: ::core::ffi::c_int,
    mut start_mark: yaml_mark_t,
    mut string: *mut yaml_string_t,
) -> ::core::ffi::c_int {
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut octet: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
        if if (*parser).unread >= 3 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 3 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '%' as i32 as yaml_char_t as ::core::ffi::c_int
            && (*(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 'F' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 'f' as i32 as yaml_char_t as ::core::ffi::c_int)
            && (*(*parser)
                .buffer
                .pointer
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 'F' as i32 as yaml_char_t as ::core::ffi::c_int
                || *(*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 'f' as i32 as yaml_char_t as ::core::ffi::c_int))
        {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"while parsing a tag\0" as *const u8 as *const ::core::ffi::c_char
                },
                start_mark,
                b"did not find URI escaped octet\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        octet = (((if *(*parser)
            .buffer
            .pointer
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
            && *(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= 'F' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            *(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                + 10 as ::core::ffi::c_int
        } else {
            (if *(*parser)
                .buffer
                .pointer
                .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= 'f' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int
            } else {
                *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - '0' as i32 as yaml_char_t as ::core::ffi::c_int
            })
        }) << 4 as ::core::ffi::c_int)
            + (if *(*parser)
                .buffer
                .pointer
                .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    <= 'F' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                *(*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                    + 10 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                    && *(*parser)
                        .buffer
                        .pointer
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        <= 'f' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    *(*parser)
                        .buffer
                        .pointer
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        - 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int
                } else {
                    *(*parser)
                        .buffer
                        .pointer
                        .offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        - '0' as i32 as yaml_char_t as ::core::ffi::c_int
                })
            })) as ::core::ffi::c_uchar;
        if width == 0 {
            width = if octet as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
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
            };
            if width == 0 {
                return yaml_parser_set_scanner_error(
                    parser,
                    if directive != 0 {
                        b"while parsing a %TAG directive\0" as *const u8
                            as *const ::core::ffi::c_char
                    } else {
                        b"while parsing a tag\0" as *const u8 as *const ::core::ffi::c_char
                    },
                    start_mark,
                    b"found an incorrect leading UTF-8 octet\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        } else if octet as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
            != 0x80 as ::core::ffi::c_int
        {
            return yaml_parser_set_scanner_error(
                parser,
                if directive != 0 {
                    b"while parsing a %TAG directive\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"while parsing a tag\0" as *const u8 as *const ::core::ffi::c_char
                },
                start_mark,
                b"found an incorrect trailing UTF-8 octet\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        let fresh216 = (*string).pointer;
        (*string).pointer = (*string).pointer.offset(1);
        *fresh216 = octet as yaml_char_t;
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    })
                })
            }) as isize,
        );
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    })
                })
            }) as isize,
        );
        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
        (*parser).unread = (*parser).unread.wrapping_sub(1);
        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
            (if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf8 as ::core::ffi::c_int
                            == 0xf0 as ::core::ffi::c_int
                        {
                            4 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    })
                })
            }) as isize,
        );
        width -= 1;
        if !(width != 0) {
            break;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_block_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    mut literal: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_break: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut trailing_breaks: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut chomping: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut increment: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut indent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut leading_blank: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut trailing_blank: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
        if !(if !leading_break.start.is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = leading_break
                .start
                .offset(16 as ::core::ffi::c_int as isize);
            memset(
                leading_break.start as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
            if !(if !trailing_breaks.start.is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = trailing_breaks
                    .start
                    .offset(16 as ::core::ffi::c_int as isize);
                memset(
                    trailing_breaks.start as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    16 as size_t,
                );
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } == 0)
            {
                start_mark = (*parser).mark;
                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                (*parser).unread = (*parser).unread.wrapping_sub(1);
                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    4 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                })
                            })
                        })
                    }) as isize,
                );
                if !(if (*parser).unread >= 1 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    yaml_parser_update_buffer(parser, 1 as size_t)
                } == 0)
                {
                    if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '+' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                    {
                        chomping = if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '+' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            -(1 as ::core::ffi::c_int)
                        };
                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                        (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0xf8 as ::core::ffi::c_int
                                            == 0xf0 as ::core::ffi::c_int
                                        {
                                            4 as ::core::ffi::c_int
                                        } else {
                                            0 as ::core::ffi::c_int
                                        })
                                    })
                                })
                            }) as isize,
                        );
                        if if (*parser).unread >= 1 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as size_t)
                        } == 0
                        {
                            current_block = 12773981749305220097;
                        } else if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                            && *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '0' as i32 as yaml_char_t as ::core::ffi::c_int
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while scanning a block scalar\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    start_mark,
                                    b"found an indentation indicator equal to 0\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                current_block = 12773981749305220097;
                            } else {
                                increment = *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    - '0' as i32 as yaml_char_t as ::core::ffi::c_int;
                                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0x80 as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0xe0 as ::core::ffi::c_int
                                            == 0xc0 as ::core::ffi::c_int
                                        {
                                            2 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xf0 as ::core::ffi::c_int
                                                == 0xe0 as ::core::ffi::c_int
                                            {
                                                3 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf8 as ::core::ffi::c_int
                                                    == 0xf0 as ::core::ffi::c_int
                                                {
                                                    4 as ::core::ffi::c_int
                                                } else {
                                                    0 as ::core::ffi::c_int
                                                })
                                            })
                                        })
                                    }) as isize,
                                );
                                current_block = 6669252993407410313;
                            }
                        } else {
                            current_block = 6669252993407410313;
                        }
                    } else if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
                    {
                        if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '0' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a block scalar\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                start_mark,
                                b"found an indentation indicator equal to 0\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 12773981749305220097;
                        } else {
                            increment = *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - '0' as i32 as yaml_char_t as ::core::ffi::c_int;
                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0x80 as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xe0 as ::core::ffi::c_int
                                        == 0xc0 as ::core::ffi::c_int
                                    {
                                        2 as ::core::ffi::c_int
                                    } else {
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0xf0 as ::core::ffi::c_int
                                            == 0xe0 as ::core::ffi::c_int
                                        {
                                            3 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xf8 as ::core::ffi::c_int
                                                == 0xf0 as ::core::ffi::c_int
                                            {
                                                4 as ::core::ffi::c_int
                                            } else {
                                                0 as ::core::ffi::c_int
                                            })
                                        })
                                    })
                                }) as isize,
                            );
                            if if (*parser).unread >= 1 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as size_t)
                            } == 0
                            {
                                current_block = 12773981749305220097;
                            } else {
                                if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '+' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                {
                                    chomping = if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '+' as i32 as yaml_char_t as ::core::ffi::c_int
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        -(1 as ::core::ffi::c_int)
                                    };
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    3 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        4 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                }
                                current_block = 6669252993407410313;
                            }
                        }
                    } else {
                        current_block = 6669252993407410313;
                    }
                    match current_block {
                        12773981749305220097 => {}
                        _ => {
                            if !(if (*parser).unread >= 1 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as size_t)
                            } == 0)
                            {
                                loop {
                                    if !(*(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
                                    {
                                        current_block = 11932355480408055363;
                                        break;
                                    }
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    3 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        4 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                    if if (*parser).unread >= 1 as size_t {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        yaml_parser_update_buffer(parser, 1 as size_t)
                                    } == 0
                                    {
                                        current_block = 12773981749305220097;
                                        break;
                                    }
                                }
                                match current_block {
                                    12773981749305220097 => {}
                                    _ => {
                                        if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '#' as i32 as yaml_char_t as ::core::ffi::c_int
                                        {
                                            loop {
                                                if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\r' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\n' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -62i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -123i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -30i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -128i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -88i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -30i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -128i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -87i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\0' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                {
                                                    current_block = 9520865839495247062;
                                                    break;
                                                }
                                                (*parser).mark.index =
                                                    (*parser).mark.index.wrapping_add(1);
                                                (*parser).mark.column =
                                                    (*parser).mark.column.wrapping_add(1);
                                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(
                                                        (if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            & 0x80 as ::core::ffi::c_int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            1 as ::core::ffi::c_int
                                                        } else {
                                                            (if *(*parser).buffer.pointer.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                & 0xe0 as ::core::ffi::c_int
                                                                == 0xc0 as ::core::ffi::c_int
                                                            {
                                                                2 as ::core::ffi::c_int
                                                            } else {
                                                                (if *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(
                                                                        0 as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                    as ::core::ffi::c_int
                                                                    & 0xf0 as ::core::ffi::c_int
                                                                    == 0xe0 as ::core::ffi::c_int
                                                                {
                                                                    3 as ::core::ffi::c_int
                                                                } else {
                                                                    (if *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(
                                                                            0 as ::core::ffi::c_int
                                                                                as isize,
                                                                        )
                                                                        as ::core::ffi::c_int
                                                                        & 0xf8 as ::core::ffi::c_int
                                                                        == 0xf0
                                                                            as ::core::ffi::c_int
                                                                    {
                                                                        4 as ::core::ffi::c_int
                                                                    } else {
                                                                        0 as ::core::ffi::c_int
                                                                    })
                                                                })
                                                            })
                                                        })
                                                            as isize,
                                                    );
                                                if if (*parser).unread >= 1 as size_t {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    yaml_parser_update_buffer(parser, 1 as size_t)
                                                } == 0
                                                {
                                                    current_block = 12773981749305220097;
                                                    break;
                                                }
                                            }
                                        } else {
                                            current_block = 9520865839495247062;
                                        }
                                        match current_block {
                                            12773981749305220097 => {}
                                            _ => {
                                                if !(*(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\r' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\n' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -62i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -123i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -30i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -128i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -88i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -30i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 1 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -128i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            (0 as ::core::ffi::c_int
                                                                + 2 as ::core::ffi::c_int)
                                                                as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -87i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\0' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int)
                                                {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while scanning a block scalar\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        start_mark,
                                                        b"did not find expected comment or line break\0"
                                                            as *const u8 as *const ::core::ffi::c_char,
                                                    );
                                                } else {
                                                    if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\r' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == '\n' as i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -62i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -123i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -30i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 2 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -88i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -30i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 1 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                (0 as ::core::ffi::c_int
                                                                    + 2 as ::core::ffi::c_int)
                                                                    as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -87i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                    {
                                                        if if (*parser).unread >= 2 as size_t {
                                                            1 as ::core::ffi::c_int
                                                        } else {
                                                            yaml_parser_update_buffer(
                                                                parser,
                                                                2 as size_t,
                                                            )
                                                        } == 0
                                                        {
                                                            current_block = 12773981749305220097;
                                                        } else {
                                                            if *(*parser).buffer.pointer.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == '\r' as i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                                && *(*parser).buffer.pointer.offset(
                                                                    (0 as ::core::ffi::c_int
                                                                        + 1 as ::core::ffi::c_int)
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    == '\n' as i32 as yaml_char_t
                                                                        as ::core::ffi::c_int
                                                            {
                                                                (*parser).mark.index = ((*parser)
                                                                    .mark
                                                                    .index
                                                                    as ::core::ffi::c_ulong)
                                                                    .wrapping_add(
                                                                        2 as ::core::ffi::c_ulong,
                                                                    )
                                                                    as size_t
                                                                    as size_t;
                                                                (*parser).mark.column = 0 as size_t;
                                                                (*parser).mark.line = (*parser)
                                                                    .mark
                                                                    .line
                                                                    .wrapping_add(1);
                                                                (*parser).unread = ((*parser).unread
                                                                    as ::core::ffi::c_ulong)
                                                                    .wrapping_sub(
                                                                        2 as ::core::ffi::c_ulong,
                                                                    )
                                                                    as size_t
                                                                    as size_t;
                                                                (*parser).buffer.pointer =
                                                                    (*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(
                                                                            2 as ::core::ffi::c_int
                                                                                as isize,
                                                                        );
                                                            } else {
                                                                if *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                                    as ::core::ffi::c_int
                                                                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                    || *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                        as ::core::ffi::c_int
                                                                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                    || *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                        as ::core::ffi::c_int
                                                                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                                        && *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(
                                                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                            ) as ::core::ffi::c_int
                                                                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                                                                    || *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                        as ::core::ffi::c_int
                                                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                                        && *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(
                                                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                            ) as ::core::ffi::c_int
                                                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                                        && *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(
                                                                                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                                                            ) as ::core::ffi::c_int
                                                                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                                                                    || *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                        as ::core::ffi::c_int
                                                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                                        && *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(
                                                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                            ) as ::core::ffi::c_int
                                                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                                        && *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(
                                                                                (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                                                            ) as ::core::ffi::c_int
                                                                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                                                                {
                                                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                                                    (*parser).mark.column = 0 as size_t;
                                                                    (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                                                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                                    (*parser).buffer.pointer = (*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(
                                                                            (if *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                                                                                == 0 as ::core::ffi::c_int
                                                                            {
                                                                                1 as ::core::ffi::c_int
                                                                            } else {
                                                                                (if *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                                                    as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                                                                                    == 0xc0 as ::core::ffi::c_int
                                                                                {
                                                                                    2 as ::core::ffi::c_int
                                                                                } else {
                                                                                    (if *(*parser)
                                                                                        .buffer
                                                                                        .pointer
                                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                                        as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                                                                        == 0xe0 as ::core::ffi::c_int
                                                                                    {
                                                                                        3 as ::core::ffi::c_int
                                                                                    } else {
                                                                                        (if *(*parser)
                                                                                            .buffer
                                                                                            .pointer
                                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                                            as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                                                                                            == 0xf0 as ::core::ffi::c_int
                                                                                        {
                                                                                            4 as ::core::ffi::c_int
                                                                                        } else {
                                                                                            0 as ::core::ffi::c_int
                                                                                        })
                                                                                    })
                                                                                })
                                                                            }) as isize,
                                                                        );
                                                                } else {};
                                                            };
                                                            current_block = 11743904203796629665;
                                                        }
                                                    } else {
                                                        current_block = 11743904203796629665;
                                                    }
                                                    match current_block {
                                                        12773981749305220097 => {}
                                                        _ => {
                                                            end_mark = (*parser).mark;
                                                            if increment != 0 {
                                                                indent = if (*parser).indent
                                                                    >= 0 as ::core::ffi::c_int
                                                                {
                                                                    (*parser).indent + increment
                                                                } else {
                                                                    increment
                                                                };
                                                            }
                                                            if !(yaml_parser_scan_block_scalar_breaks(
                                                                parser,
                                                                &raw mut indent,
                                                                &raw mut trailing_breaks,
                                                                start_mark,
                                                                &raw mut end_mark,
                                                            ) == 0)
                                                            {
                                                                if !(if (*parser).unread >= 1 as size_t {
                                                                    1 as ::core::ffi::c_int
                                                                } else {
                                                                    yaml_parser_update_buffer(parser, 1 as size_t)
                                                                } == 0)
                                                                {
                                                                    's_226: loop {
                                                                        if !((*parser).mark.column as ::core::ffi::c_int == indent
                                                                            && !(*(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                                                                        {
                                                                            current_block = 5807581744382915773;
                                                                            break;
                                                                        }
                                                                        trailing_blank = (*(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int
                                                                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
                                                                            as ::core::ffi::c_int;
                                                                        if literal == 0
                                                                            && *leading_break.start as ::core::ffi::c_int == '\n' as i32
                                                                            && leading_blank == 0 && trailing_blank == 0
                                                                        {
                                                                            if *trailing_breaks.start as ::core::ffi::c_int
                                                                                == '\0' as i32
                                                                            {
                                                                                if if string
                                                                                    .pointer
                                                                                    .offset(5 as ::core::ffi::c_int as isize) < string.end
                                                                                    || yaml_string_extend(
                                                                                        &raw mut string.start,
                                                                                        &raw mut string.pointer,
                                                                                        &raw mut string.end,
                                                                                    ) != 0
                                                                                {
                                                                                    1 as ::core::ffi::c_int
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0 as ::core::ffi::c_int
                                                                                } == 0
                                                                                {
                                                                                    current_block = 12773981749305220097;
                                                                                    break;
                                                                                }
                                                                                let fresh156 = string.pointer;
                                                                                string.pointer = string.pointer.offset(1);
                                                                                *fresh156 = ' ' as i32 as yaml_char_t;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut ::core::ffi::c_void,
                                                                                0 as ::core::ffi::c_int,
                                                                                leading_break.end.offset_from(leading_break.start)
                                                                                    as ::core::ffi::c_long as size_t,
                                                                            );
                                                                        } else {
                                                                            if if yaml_string_join(
                                                                                &raw mut string.start,
                                                                                &raw mut string.pointer,
                                                                                &raw mut string.end,
                                                                                &raw mut leading_break.start,
                                                                                &raw mut leading_break.pointer,
                                                                                &raw mut leading_break.end,
                                                                            ) != 0
                                                                            {
                                                                                leading_break.pointer = leading_break.start;
                                                                                1 as ::core::ffi::c_int
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0 as ::core::ffi::c_int
                                                                            } == 0
                                                                            {
                                                                                current_block = 12773981749305220097;
                                                                                break;
                                                                            }
                                                                            leading_break.pointer = leading_break.start;
                                                                            memset(
                                                                                leading_break.start as *mut ::core::ffi::c_void,
                                                                                0 as ::core::ffi::c_int,
                                                                                leading_break.end.offset_from(leading_break.start)
                                                                                    as ::core::ffi::c_long as size_t,
                                                                            );
                                                                        }
                                                                        if if yaml_string_join(
                                                                            &raw mut string.start,
                                                                            &raw mut string.pointer,
                                                                            &raw mut string.end,
                                                                            &raw mut trailing_breaks.start,
                                                                            &raw mut trailing_breaks.pointer,
                                                                            &raw mut trailing_breaks.end,
                                                                        ) != 0
                                                                        {
                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                            1 as ::core::ffi::c_int
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0 as ::core::ffi::c_int
                                                                        } == 0
                                                                        {
                                                                            current_block = 12773981749305220097;
                                                                            break;
                                                                        }
                                                                        trailing_breaks.pointer = trailing_breaks.start;
                                                                        memset(
                                                                            trailing_breaks.start as *mut ::core::ffi::c_void,
                                                                            0 as ::core::ffi::c_int,
                                                                            trailing_breaks.end.offset_from(trailing_breaks.start)
                                                                                as ::core::ffi::c_long as size_t,
                                                                        );
                                                                        leading_blank = (*(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int
                                                                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
                                                                            as ::core::ffi::c_int;
                                                                        while !(*(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int
                                                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                                                && *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(
                                                                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                    ) as ::core::ffi::c_int
                                                                                    == -123i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                                                && *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(
                                                                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                    ) as ::core::ffi::c_int
                                                                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                                                && *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(
                                                                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                                                                    ) as ::core::ffi::c_int
                                                                                    == -88i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                                                && *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(
                                                                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
                                                                                    ) as ::core::ffi::c_int
                                                                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                                                && *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(
                                                                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
                                                                                    ) as ::core::ffi::c_int
                                                                                    == -87i32 as yaml_char_t as ::core::ffi::c_int
                                                                            || *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == '\0' as i32 as yaml_char_t as ::core::ffi::c_int)
                                                                        {
                                                                            if if if string
                                                                                .pointer
                                                                                .offset(5 as ::core::ffi::c_int as isize) < string.end
                                                                                || yaml_string_extend(
                                                                                    &raw mut string.start,
                                                                                    &raw mut string.pointer,
                                                                                    &raw mut string.end,
                                                                                ) != 0
                                                                            {
                                                                                1 as ::core::ffi::c_int
                                                                            } else {
                                                                                (*parser).error = YAML_MEMORY_ERROR;
                                                                                0 as ::core::ffi::c_int
                                                                            } != 0
                                                                            {
                                                                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                                                    & 0x80 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                                                                                {
                                                                                    let fresh157 = (*parser).buffer.pointer;
                                                                                    (*parser).buffer.pointer = (*parser)
                                                                                        .buffer
                                                                                        .pointer
                                                                                        .offset(1);
                                                                                    let fresh158 = string.pointer;
                                                                                    string.pointer = string.pointer.offset(1);
                                                                                    *fresh158 = *fresh157;
                                                                                } else {
                                                                                    if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                                                        & 0xe0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
                                                                                    {
                                                                                        let fresh159 = (*parser).buffer.pointer;
                                                                                        (*parser).buffer.pointer = (*parser)
                                                                                            .buffer
                                                                                            .pointer
                                                                                            .offset(1);
                                                                                        let fresh160 = string.pointer;
                                                                                        string.pointer = string.pointer.offset(1);
                                                                                        *fresh160 = *fresh159;
                                                                                        let fresh161 = (*parser).buffer.pointer;
                                                                                        (*parser).buffer.pointer = (*parser)
                                                                                            .buffer
                                                                                            .pointer
                                                                                            .offset(1);
                                                                                        let fresh162 = string.pointer;
                                                                                        string.pointer = string.pointer.offset(1);
                                                                                        *fresh162 = *fresh161;
                                                                                    } else {
                                                                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                                                            & 0xf0 as ::core::ffi::c_int == 0xe0 as ::core::ffi::c_int
                                                                                        {
                                                                                            let fresh163 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = (*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1);
                                                                                            let fresh164 = string.pointer;
                                                                                            string.pointer = string.pointer.offset(1);
                                                                                            *fresh164 = *fresh163;
                                                                                            let fresh165 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = (*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1);
                                                                                            let fresh166 = string.pointer;
                                                                                            string.pointer = string.pointer.offset(1);
                                                                                            *fresh166 = *fresh165;
                                                                                            let fresh167 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = (*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1);
                                                                                            let fresh168 = string.pointer;
                                                                                            string.pointer = string.pointer.offset(1);
                                                                                            *fresh168 = *fresh167;
                                                                                        } else {
                                                                                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                                                                & 0xf8 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int
                                                                                            {
                                                                                                let fresh169 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = (*parser)
                                                                                                    .buffer
                                                                                                    .pointer
                                                                                                    .offset(1);
                                                                                                let fresh170 = string.pointer;
                                                                                                string.pointer = string.pointer.offset(1);
                                                                                                *fresh170 = *fresh169;
                                                                                                let fresh171 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = (*parser)
                                                                                                    .buffer
                                                                                                    .pointer
                                                                                                    .offset(1);
                                                                                                let fresh172 = string.pointer;
                                                                                                string.pointer = string.pointer.offset(1);
                                                                                                *fresh172 = *fresh171;
                                                                                                let fresh173 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = (*parser)
                                                                                                    .buffer
                                                                                                    .pointer
                                                                                                    .offset(1);
                                                                                                let fresh174 = string.pointer;
                                                                                                string.pointer = string.pointer.offset(1);
                                                                                                *fresh174 = *fresh173;
                                                                                                let fresh175 = (*parser).buffer.pointer;
                                                                                                (*parser).buffer.pointer = (*parser)
                                                                                                    .buffer
                                                                                                    .pointer
                                                                                                    .offset(1);
                                                                                                let fresh176 = string.pointer;
                                                                                                string.pointer = string.pointer.offset(1);
                                                                                                *fresh176 = *fresh175;
                                                                                            } else {};
                                                                                        };
                                                                                    };
                                                                                };
                                                                                (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                                                                (*parser).mark.column = (*parser)
                                                                                    .mark
                                                                                    .column
                                                                                    .wrapping_add(1);
                                                                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                                                1 as ::core::ffi::c_int
                                                                            } else {
                                                                                0 as ::core::ffi::c_int
                                                                            } == 0
                                                                            {
                                                                                current_block = 12773981749305220097;
                                                                                break 's_226;
                                                                            }
                                                                            if if (*parser).unread >= 1 as size_t {
                                                                                1 as ::core::ffi::c_int
                                                                            } else {
                                                                                yaml_parser_update_buffer(parser, 1 as size_t)
                                                                            } == 0
                                                                            {
                                                                                current_block = 12773981749305220097;
                                                                                break 's_226;
                                                                            }
                                                                        }
                                                                        if if (*parser).unread >= 2 as size_t {
                                                                            1 as ::core::ffi::c_int
                                                                        } else {
                                                                            yaml_parser_update_buffer(parser, 2 as size_t)
                                                                        } == 0
                                                                        {
                                                                            current_block = 12773981749305220097;
                                                                            break;
                                                                        }
                                                                        if if if leading_break
                                                                            .pointer
                                                                            .offset(5 as ::core::ffi::c_int as isize)
                                                                            < leading_break.end
                                                                            || yaml_string_extend(
                                                                                &raw mut leading_break.start,
                                                                                &raw mut leading_break.pointer,
                                                                                &raw mut leading_break.end,
                                                                            ) != 0
                                                                        {
                                                                            1 as ::core::ffi::c_int
                                                                        } else {
                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                            0 as ::core::ffi::c_int
                                                                        } != 0
                                                                        {
                                                                            if *(*parser)
                                                                                .buffer
                                                                                .pointer
                                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                                as ::core::ffi::c_int
                                                                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                                && *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                                                    as ::core::ffi::c_int
                                                                                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                            {
                                                                                let fresh177 = leading_break.pointer;
                                                                                leading_break.pointer = leading_break.pointer.offset(1);
                                                                                *fresh177 = '\n' as i32 as yaml_char_t;
                                                                                (*parser).buffer.pointer = (*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(2 as ::core::ffi::c_int as isize);
                                                                                (*parser).mark.index = ((*parser).mark.index
                                                                                    as ::core::ffi::c_ulong)
                                                                                    .wrapping_add(2 as ::core::ffi::c_ulong) as size_t
                                                                                    as size_t;
                                                                                (*parser).mark.column = 0 as size_t;
                                                                                (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                                                                                (*parser).unread = ((*parser).unread
                                                                                    as ::core::ffi::c_ulong)
                                                                                    .wrapping_sub(2 as ::core::ffi::c_ulong) as size_t
                                                                                    as size_t;
                                                                            } else {
                                                                                if *(*parser)
                                                                                    .buffer
                                                                                    .pointer
                                                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                                                    as ::core::ffi::c_int
                                                                                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                                    || *(*parser)
                                                                                        .buffer
                                                                                        .pointer
                                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                                        as ::core::ffi::c_int
                                                                                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                                                                {
                                                                                    let fresh178 = leading_break.pointer;
                                                                                    leading_break.pointer = leading_break.pointer.offset(1);
                                                                                    *fresh178 = '\n' as i32 as yaml_char_t;
                                                                                    (*parser).buffer.pointer = (*parser)
                                                                                        .buffer
                                                                                        .pointer
                                                                                        .offset(1);
                                                                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                                                                    (*parser).mark.column = 0 as size_t;
                                                                                    (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                                                                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                                                } else {
                                                                                    if *(*parser)
                                                                                        .buffer
                                                                                        .pointer
                                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                                        as ::core::ffi::c_int
                                                                                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                                                        && *(*parser)
                                                                                            .buffer
                                                                                            .pointer
                                                                                            .offset(1 as ::core::ffi::c_int as isize)
                                                                                            as ::core::ffi::c_int
                                                                                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                                                                                    {
                                                                                        let fresh179 = leading_break.pointer;
                                                                                        leading_break.pointer = leading_break.pointer.offset(1);
                                                                                        *fresh179 = '\n' as i32 as yaml_char_t;
                                                                                        (*parser).buffer.pointer = (*parser)
                                                                                            .buffer
                                                                                            .pointer
                                                                                            .offset(2 as ::core::ffi::c_int as isize);
                                                                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                                                                        (*parser).mark.column = 0 as size_t;
                                                                                        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                                                                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                                                    } else {
                                                                                        if *(*parser)
                                                                                            .buffer
                                                                                            .pointer
                                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                                            as ::core::ffi::c_int
                                                                                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                                                            && *(*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1 as ::core::ffi::c_int as isize)
                                                                                                as ::core::ffi::c_int
                                                                                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                                                            && (*(*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(2 as ::core::ffi::c_int as isize)
                                                                                                as ::core::ffi::c_int
                                                                                                == -88i32 as yaml_char_t as ::core::ffi::c_int
                                                                                                || *(*parser)
                                                                                                    .buffer
                                                                                                    .pointer
                                                                                                    .offset(2 as ::core::ffi::c_int as isize)
                                                                                                    as ::core::ffi::c_int
                                                                                                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
                                                                                        {
                                                                                            let fresh180 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = (*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1);
                                                                                            let fresh181 = leading_break.pointer;
                                                                                            leading_break.pointer = leading_break.pointer.offset(1);
                                                                                            *fresh181 = *fresh180;
                                                                                            let fresh182 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = (*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1);
                                                                                            let fresh183 = leading_break.pointer;
                                                                                            leading_break.pointer = leading_break.pointer.offset(1);
                                                                                            *fresh183 = *fresh182;
                                                                                            let fresh184 = (*parser).buffer.pointer;
                                                                                            (*parser).buffer.pointer = (*parser)
                                                                                                .buffer
                                                                                                .pointer
                                                                                                .offset(1);
                                                                                            let fresh185 = leading_break.pointer;
                                                                                            leading_break.pointer = leading_break.pointer.offset(1);
                                                                                            *fresh185 = *fresh184;
                                                                                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                                                                            (*parser).mark.column = 0 as size_t;
                                                                                            (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                                                                                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                                                        } else {};
                                                                                    };
                                                                                };
                                                                            };
                                                                            1 as ::core::ffi::c_int
                                                                        } else {
                                                                            0 as ::core::ffi::c_int
                                                                        } == 0
                                                                        {
                                                                            current_block = 12773981749305220097;
                                                                            break;
                                                                        }
                                                                        if yaml_parser_scan_block_scalar_breaks(
                                                                            parser,
                                                                            &raw mut indent,
                                                                            &raw mut trailing_breaks,
                                                                            start_mark,
                                                                            &raw mut end_mark,
                                                                        ) == 0
                                                                        {
                                                                            current_block = 12773981749305220097;
                                                                            break;
                                                                        }
                                                                    }
                                                                    match current_block {
                                                                        12773981749305220097 => {}
                                                                        _ => {
                                                                            if chomping != -(1 as ::core::ffi::c_int) {
                                                                                if if yaml_string_join(
                                                                                    &raw mut string.start,
                                                                                    &raw mut string.pointer,
                                                                                    &raw mut string.end,
                                                                                    &raw mut leading_break.start,
                                                                                    &raw mut leading_break.pointer,
                                                                                    &raw mut leading_break.end,
                                                                                ) != 0
                                                                                {
                                                                                    leading_break.pointer = leading_break.start;
                                                                                    1 as ::core::ffi::c_int
                                                                                } else {
                                                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                                                    0 as ::core::ffi::c_int
                                                                                } == 0
                                                                                {
                                                                                    current_block = 12773981749305220097;
                                                                                } else {
                                                                                    current_block = 981995395831942902;
                                                                                }
                                                                            } else {
                                                                                current_block = 981995395831942902;
                                                                            }
                                                                            match current_block {
                                                                                12773981749305220097 => {}
                                                                                _ => {
                                                                                    if chomping == 1 as ::core::ffi::c_int {
                                                                                        if if yaml_string_join(
                                                                                            &raw mut string.start,
                                                                                            &raw mut string.pointer,
                                                                                            &raw mut string.end,
                                                                                            &raw mut trailing_breaks.start,
                                                                                            &raw mut trailing_breaks.pointer,
                                                                                            &raw mut trailing_breaks.end,
                                                                                        ) != 0
                                                                                        {
                                                                                            trailing_breaks.pointer = trailing_breaks.start;
                                                                                            1 as ::core::ffi::c_int
                                                                                        } else {
                                                                                            (*parser).error = YAML_MEMORY_ERROR;
                                                                                            0 as ::core::ffi::c_int
                                                                                        } == 0
                                                                                        {
                                                                                            current_block = 12773981749305220097;
                                                                                        } else {
                                                                                            current_block = 16779030619667747692;
                                                                                        }
                                                                                    } else {
                                                                                        current_block = 16779030619667747692;
                                                                                    }
                                                                                    match current_block {
                                                                                        12773981749305220097 => {}
                                                                                        _ => {
                                                                                            memset(
                                                                                                token as *mut ::core::ffi::c_void,
                                                                                                0 as ::core::ffi::c_int,
                                                                                                ::core::mem::size_of::<yaml_token_t>() as size_t,
                                                                                            );
                                                                                            (*token).type_0 = YAML_SCALAR_TOKEN;
                                                                                            (*token).start_mark = start_mark;
                                                                                            (*token).end_mark = end_mark;
                                                                                            (*token).data.scalar.value = string.start;
                                                                                            (*token).data.scalar.length = string
                                                                                                .pointer
                                                                                                .offset_from(string.start) as ::core::ffi::c_long as size_t;
                                                                                            (*token).data.scalar.style = (if literal != 0 {
                                                                                                YAML_LITERAL_SCALAR_STYLE as ::core::ffi::c_int
                                                                                            } else {
                                                                                                YAML_FOLDED_SCALAR_STYLE as ::core::ffi::c_int
                                                                                            }) as yaml_scalar_style_t;
                                                                                            yaml_free(leading_break.start as *mut ::core::ffi::c_void);
                                                                                            leading_break.end = ::core::ptr::null_mut::<yaml_char_t>();
                                                                                            leading_break.pointer = leading_break.end;
                                                                                            leading_break.start = leading_break.pointer;
                                                                                            yaml_free(
                                                                                                trailing_breaks.start as *mut ::core::ffi::c_void,
                                                                                            );
                                                                                            trailing_breaks.end = ::core::ptr::null_mut::<
                                                                                                yaml_char_t,
                                                                                            >();
                                                                                            trailing_breaks.pointer = trailing_breaks.end;
                                                                                            trailing_breaks.start = trailing_breaks.pointer;
                                                                                            return 1 as ::core::ffi::c_int;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut ::core::ffi::c_void);
    string.end = ::core::ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut ::core::ffi::c_void);
    leading_break.end = ::core::ptr::null_mut::<yaml_char_t>();
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut ::core::ffi::c_void);
    trailing_breaks.end = ::core::ptr::null_mut::<yaml_char_t>();
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_block_scalar_breaks(
    mut parser: *mut yaml_parser_t,
    mut indent: *mut ::core::ffi::c_int,
    mut breaks: *mut yaml_string_t,
    mut start_mark: yaml_mark_t,
    mut end_mark: *mut yaml_mark_t,
) -> ::core::ffi::c_int {
    let mut max_indent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *end_mark = (*parser).mark;
    loop {
        if if (*parser).unread >= 1 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 1 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        while (*indent == 0 || ((*parser).mark.column as ::core::ffi::c_int) < *indent)
            && *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
            (*parser).unread = (*parser).unread.wrapping_sub(1);
            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                (if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                4 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            })
                        })
                    })
                }) as isize,
            );
            if if (*parser).unread >= 1 as size_t {
                1 as ::core::ffi::c_int
            } else {
                yaml_parser_update_buffer(parser, 1 as size_t)
            } == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*parser).mark.column as ::core::ffi::c_int > max_indent {
            max_indent = (*parser).mark.column as ::core::ffi::c_int;
        }
        if (*indent == 0 || ((*parser).mark.column as ::core::ffi::c_int) < *indent)
            && *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            return yaml_parser_set_scanner_error(
                parser,
                b"while scanning a block scalar\0" as *const u8 as *const ::core::ffi::c_char,
                start_mark,
                b"found a tab character where an indentation space is expected\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        if !(*(*parser)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
        {
            break;
        }
        if if (*parser).unread >= 2 as size_t {
            1 as ::core::ffi::c_int
        } else {
            yaml_parser_update_buffer(parser, 2 as size_t)
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if if if (*breaks).pointer.offset(5 as ::core::ffi::c_int as isize) < (*breaks).end
            || yaml_string_extend(
                &raw mut (*breaks).start,
                &raw mut (*breaks).pointer,
                &raw mut (*breaks).end,
            ) != 0
        {
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } != 0
        {
            if *(*parser)
                .buffer
                .pointer
                .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                && *(*parser)
                    .buffer
                    .pointer
                    .offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                let fresh186 = (*breaks).pointer;
                (*breaks).pointer = (*breaks).pointer.offset(1);
                *fresh186 = '\n' as i32 as yaml_char_t;
                (*parser).buffer.pointer = (*parser)
                    .buffer
                    .pointer
                    .offset(2 as ::core::ffi::c_int as isize);
                (*parser).mark.index = ((*parser).mark.index as ::core::ffi::c_ulong)
                    .wrapping_add(2 as ::core::ffi::c_ulong)
                    as size_t as size_t;
                (*parser).mark.column = 0 as size_t;
                (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                (*parser).unread = ((*parser).unread as ::core::ffi::c_ulong)
                    .wrapping_sub(2 as ::core::ffi::c_ulong)
                    as size_t as size_t;
            } else {
                if *(*parser)
                    .buffer
                    .pointer
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    let fresh187 = (*breaks).pointer;
                    (*breaks).pointer = (*breaks).pointer.offset(1);
                    *fresh187 = '\n' as i32 as yaml_char_t;
                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                    (*parser).mark.column = 0 as size_t;
                    (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                } else {
                    if *(*parser)
                        .buffer
                        .pointer
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                        && *(*parser)
                            .buffer
                            .pointer
                            .offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                    {
                        let fresh188 = (*breaks).pointer;
                        (*breaks).pointer = (*breaks).pointer.offset(1);
                        *fresh188 = '\n' as i32 as yaml_char_t;
                        (*parser).buffer.pointer = (*parser)
                            .buffer
                            .pointer
                            .offset(2 as ::core::ffi::c_int as isize);
                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                        (*parser).mark.column = 0 as size_t;
                        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                    } else {
                        if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                            && *(*parser)
                                .buffer
                                .pointer
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                            && (*(*parser)
                                .buffer
                                .pointer
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == -88i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
                        {
                            let fresh189 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh190 = (*breaks).pointer;
                            (*breaks).pointer = (*breaks).pointer.offset(1);
                            *fresh190 = *fresh189;
                            let fresh191 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh192 = (*breaks).pointer;
                            (*breaks).pointer = (*breaks).pointer.offset(1);
                            *fresh192 = *fresh191;
                            let fresh193 = (*parser).buffer.pointer;
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(1);
                            let fresh194 = (*breaks).pointer;
                            (*breaks).pointer = (*breaks).pointer.offset(1);
                            *fresh194 = *fresh193;
                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                            (*parser).mark.column = 0 as size_t;
                            (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                        } else {
                        };
                    };
                };
            };
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        *end_mark = (*parser).mark;
    }
    if *indent == 0 {
        *indent = max_indent;
        if *indent < (*parser).indent + 1 as ::core::ffi::c_int {
            *indent = (*parser).indent + 1 as ::core::ffi::c_int;
        }
        if *indent < 1 as ::core::ffi::c_int {
            *indent = 1 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_flow_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
    mut single: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_break: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut trailing_breaks: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut whitespaces: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_blanks: ::core::ffi::c_int = 0;
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
        if !(if !leading_break.start.is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = leading_break
                .start
                .offset(16 as ::core::ffi::c_int as isize);
            memset(
                leading_break.start as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
            if !(if !trailing_breaks.start.is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = trailing_breaks
                    .start
                    .offset(16 as ::core::ffi::c_int as isize);
                memset(
                    trailing_breaks.start as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    16 as size_t,
                );
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } == 0)
            {
                whitespaces.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
                if !(if !whitespaces.start.is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = whitespaces.start.offset(16 as ::core::ffi::c_int as isize);
                    memset(
                        whitespaces.start as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        16 as size_t,
                    );
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } == 0)
                {
                    start_mark = (*parser).mark;
                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                        (if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf8 as ::core::ffi::c_int
                                        == 0xf0 as ::core::ffi::c_int
                                    {
                                        4 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    })
                                })
                            })
                        }) as isize,
                    );
                    's_44: loop {
                        if if (*parser).unread >= 4 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 4 as size_t)
                        } == 0
                        {
                            current_block = 13911338632193099610;
                            break;
                        }
                        if (*parser).mark.column == 0 as size_t
                            && (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '.' as i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '.' as i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '.' as i32 as yaml_char_t as ::core::ffi::c_int)
                            && (*(*parser)
                                .buffer
                                .pointer
                                .offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                || (*(*parser)
                                    .buffer
                                    .pointer
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a quoted scalar\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                start_mark,
                                b"found unexpected document indicator\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 13911338632193099610;
                            break;
                        } else if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '\0' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            yaml_parser_set_scanner_error(
                                parser,
                                b"while scanning a quoted scalar\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                start_mark,
                                b"found unexpected end of stream\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            current_block = 13911338632193099610;
                            break;
                        } else {
                            if if (*parser).unread >= 2 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 2 as size_t)
                            } == 0
                            {
                                current_block = 13911338632193099610;
                                break;
                            }
                            leading_blanks = 0 as ::core::ffi::c_int;
                            while !(*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                || (*(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                            {
                                if single != 0
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
                                {
                                    if if string.pointer.offset(5 as ::core::ffi::c_int as isize)
                                        < string.end
                                        || yaml_string_extend(
                                            &raw mut string.start,
                                            &raw mut string.pointer,
                                            &raw mut string.end,
                                        ) != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 13911338632193099610;
                                        break 's_44;
                                    }
                                    let fresh62 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    *fresh62 = '\'' as i32 as yaml_char_t;
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    3 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        4 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    3 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        4 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                } else {
                                    if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                            as yaml_char_t
                                            as ::core::ffi::c_int
                                    {
                                        break;
                                    }
                                    if single == 0
                                        && *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\\' as i32 as yaml_char_t as ::core::ffi::c_int
                                        && (*(*parser)
                                            .buffer
                                            .pointer
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -123i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -88i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
                                    {
                                        if if (*parser).unread >= 3 as size_t {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            yaml_parser_update_buffer(parser, 3 as size_t)
                                        } == 0
                                        {
                                            current_block = 13911338632193099610;
                                            break 's_44;
                                        }
                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                        (*parser).mark.column =
                                            (*parser).mark.column.wrapping_add(1);
                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0x80 as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                1 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xe0 as ::core::ffi::c_int
                                                    == 0xc0 as ::core::ffi::c_int
                                                {
                                                    2 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0xe0 as ::core::ffi::c_int
                                                    {
                                                        3 as ::core::ffi::c_int
                                                    } else {
                                                        (if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            & 0xf8 as ::core::ffi::c_int
                                                            == 0xf0 as ::core::ffi::c_int
                                                        {
                                                            4 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                        if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser).buffer.pointer.offset(
                                                (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                    as isize,
                                            )
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                        {
                                            (*parser).mark.index = ((*parser).mark.index
                                                as ::core::ffi::c_ulong)
                                                .wrapping_add(2 as ::core::ffi::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0 as size_t;
                                            (*parser).mark.line =
                                                (*parser).mark.line.wrapping_add(1);
                                            (*parser).unread = ((*parser).unread
                                                as ::core::ffi::c_ulong)
                                                .wrapping_sub(2 as ::core::ffi::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).buffer.pointer = (*parser)
                                                .buffer
                                                .pointer
                                                .offset(2 as ::core::ffi::c_int as isize);
                                        } else {
                                            if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                || *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                || *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(
                                                        (0 as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as ::core::ffi::c_int
                                                        == -123i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                || *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(
                                                        (0 as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as ::core::ffi::c_int
                                                        == -128i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(
                                                        (0 as ::core::ffi::c_int
                                                            + 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as ::core::ffi::c_int
                                                        == -88i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                || *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(
                                                        (0 as ::core::ffi::c_int
                                                            + 1 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as ::core::ffi::c_int
                                                        == -128i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(
                                                        (0 as ::core::ffi::c_int
                                                            + 2 as ::core::ffi::c_int)
                                                            as isize,
                                                    )
                                                        as ::core::ffi::c_int
                                                        == -87i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                            {
                                                (*parser).mark.index =
                                                    (*parser).mark.index.wrapping_add(1);
                                                (*parser).mark.column = 0 as size_t;
                                                (*parser).mark.line =
                                                    (*parser).mark.line.wrapping_add(1);
                                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(
                                                        (if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            & 0x80 as ::core::ffi::c_int
                                                            == 0 as ::core::ffi::c_int
                                                        {
                                                            1 as ::core::ffi::c_int
                                                        } else {
                                                            (if *(*parser).buffer.pointer.offset(
                                                                0 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                & 0xe0 as ::core::ffi::c_int
                                                                == 0xc0 as ::core::ffi::c_int
                                                            {
                                                                2 as ::core::ffi::c_int
                                                            } else {
                                                                (if *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(
                                                                        0 as ::core::ffi::c_int
                                                                            as isize,
                                                                    )
                                                                    as ::core::ffi::c_int
                                                                    & 0xf0 as ::core::ffi::c_int
                                                                    == 0xe0 as ::core::ffi::c_int
                                                                {
                                                                    3 as ::core::ffi::c_int
                                                                } else {
                                                                    (if *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(
                                                                            0 as ::core::ffi::c_int
                                                                                as isize,
                                                                        )
                                                                        as ::core::ffi::c_int
                                                                        & 0xf8 as ::core::ffi::c_int
                                                                        == 0xf0
                                                                            as ::core::ffi::c_int
                                                                    {
                                                                        4 as ::core::ffi::c_int
                                                                    } else {
                                                                        0 as ::core::ffi::c_int
                                                                    })
                                                                })
                                                            })
                                                        })
                                                            as isize,
                                                    );
                                            } else {
                                            };
                                        };
                                        leading_blanks = 1 as ::core::ffi::c_int;
                                        break;
                                    } else if single == 0
                                        && *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\\' as i32 as yaml_char_t as ::core::ffi::c_int
                                    {
                                        let mut code_length: size_t = 0 as size_t;
                                        if if string
                                            .pointer
                                            .offset(5 as ::core::ffi::c_int as isize)
                                            < string.end
                                            || yaml_string_extend(
                                                &raw mut string.start,
                                                &raw mut string.pointer,
                                                &raw mut string.end,
                                            ) != 0
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as ::core::ffi::c_int
                                        } == 0
                                        {
                                            current_block = 13911338632193099610;
                                            break 's_44;
                                        }
                                        match *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                        {
                                            48 => {
                                                let fresh63 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh63 = '\0' as i32 as yaml_char_t;
                                            }
                                            97 => {
                                                let fresh64 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh64 = '\u{7}' as i32 as yaml_char_t;
                                            }
                                            98 => {
                                                let fresh65 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh65 = '\u{8}' as i32 as yaml_char_t;
                                            }
                                            116 | 9 => {
                                                let fresh66 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh66 = '\t' as i32 as yaml_char_t;
                                            }
                                            110 => {
                                                let fresh67 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh67 = '\n' as i32 as yaml_char_t;
                                            }
                                            118 => {
                                                let fresh68 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh68 = '\u{b}' as i32 as yaml_char_t;
                                            }
                                            102 => {
                                                let fresh69 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh69 = '\u{c}' as i32 as yaml_char_t;
                                            }
                                            114 => {
                                                let fresh70 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh70 = '\r' as i32 as yaml_char_t;
                                            }
                                            101 => {
                                                let fresh71 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh71 = '\u{1b}' as i32 as yaml_char_t;
                                            }
                                            32 => {
                                                let fresh72 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh72 = ' ' as i32 as yaml_char_t;
                                            }
                                            34 => {
                                                let fresh73 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh73 = '"' as i32 as yaml_char_t;
                                            }
                                            47 => {
                                                let fresh74 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh74 = '/' as i32 as yaml_char_t;
                                            }
                                            92 => {
                                                let fresh75 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh75 = '\\' as i32 as yaml_char_t;
                                            }
                                            78 => {
                                                let fresh76 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh76 = -62i32 as yaml_char_t;
                                                let fresh77 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh77 = -123i32 as yaml_char_t;
                                            }
                                            95 => {
                                                let fresh78 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh78 = -62i32 as yaml_char_t;
                                                let fresh79 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh79 = -96i32 as yaml_char_t;
                                            }
                                            76 => {
                                                let fresh80 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh80 = -30i32 as yaml_char_t;
                                                let fresh81 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh81 = -128i32 as yaml_char_t;
                                                let fresh82 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh82 = -88i32 as yaml_char_t;
                                            }
                                            80 => {
                                                let fresh83 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh83 = -30i32 as yaml_char_t;
                                                let fresh84 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh84 = -128i32 as yaml_char_t;
                                                let fresh85 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh85 = -87i32 as yaml_char_t;
                                            }
                                            120 => {
                                                code_length = 2 as size_t;
                                            }
                                            117 => {
                                                code_length = 4 as size_t;
                                            }
                                            85 => {
                                                code_length = 8 as size_t;
                                            }
                                            _ => {
                                                yaml_parser_set_scanner_error(
                                                    parser,
                                                    b"while parsing a quoted scalar\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    start_mark,
                                                    b"found unknown escape character\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                current_block = 13911338632193099610;
                                                break 's_44;
                                            }
                                        }
                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                        (*parser).mark.column =
                                            (*parser).mark.column.wrapping_add(1);
                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0x80 as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                1 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xe0 as ::core::ffi::c_int
                                                    == 0xc0 as ::core::ffi::c_int
                                                {
                                                    2 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0xe0 as ::core::ffi::c_int
                                                    {
                                                        3 as ::core::ffi::c_int
                                                    } else {
                                                        (if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            & 0xf8 as ::core::ffi::c_int
                                                            == 0xf0 as ::core::ffi::c_int
                                                        {
                                                            4 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                        (*parser).mark.column =
                                            (*parser).mark.column.wrapping_add(1);
                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0x80 as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                1 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xe0 as ::core::ffi::c_int
                                                    == 0xc0 as ::core::ffi::c_int
                                                {
                                                    2 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0xe0 as ::core::ffi::c_int
                                                    {
                                                        3 as ::core::ffi::c_int
                                                    } else {
                                                        (if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            & 0xf8 as ::core::ffi::c_int
                                                            == 0xf0 as ::core::ffi::c_int
                                                        {
                                                            4 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                        if code_length != 0 {
                                            let mut value: ::core::ffi::c_uint =
                                                0 as ::core::ffi::c_uint;
                                            let mut k: size_t = 0;
                                            if if (*parser).unread >= code_length {
                                                1 as ::core::ffi::c_int
                                            } else {
                                                yaml_parser_update_buffer(parser, code_length)
                                            } == 0
                                            {
                                                current_block = 13911338632193099610;
                                                break 's_44;
                                            }
                                            k = 0 as size_t;
                                            while k < code_length {
                                                if !(*(*parser).buffer.pointer.offset(k as isize)
                                                    as ::core::ffi::c_int
                                                    >= '0' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                    && *(*parser).buffer.pointer.offset(k as isize)
                                                        as ::core::ffi::c_int
                                                        <= '9' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                    || *(*parser).buffer.pointer.offset(k as isize)
                                                        as ::core::ffi::c_int
                                                        >= 'A' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser)
                                                            .buffer
                                                            .pointer
                                                            .offset(k as isize)
                                                            as ::core::ffi::c_int
                                                            <= 'F' as i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    || *(*parser).buffer.pointer.offset(k as isize)
                                                        as ::core::ffi::c_int
                                                        >= 'a' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser)
                                                            .buffer
                                                            .pointer
                                                            .offset(k as isize)
                                                            as ::core::ffi::c_int
                                                            <= 'f' as i32 as yaml_char_t
                                                                as ::core::ffi::c_int)
                                                {
                                                    yaml_parser_set_scanner_error(
                                                        parser,
                                                        b"while parsing a quoted scalar\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        start_mark,
                                                        b"did not find expected hexdecimal number\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    current_block = 13911338632193099610;
                                                    break 's_44;
                                                } else {
                                                    value = (value << 4 as ::core::ffi::c_int)
                                                        .wrapping_add(
                                                            (if *(*parser)
                                                                .buffer
                                                                .pointer
                                                                .offset(k as isize)
                                                                as ::core::ffi::c_int
                                                                >= 'A' as i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                                && *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(k as isize)
                                                                    as ::core::ffi::c_int
                                                                    <= 'F' as i32 as yaml_char_t
                                                                        as ::core::ffi::c_int
                                                            {
                                                                *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(k as isize)
                                                                    as ::core::ffi::c_int
                                                                    - 'A' as i32 as yaml_char_t
                                                                        as ::core::ffi::c_int
                                                                    + 10 as ::core::ffi::c_int
                                                            } else {
                                                                (if *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(k as isize)
                                                                    as ::core::ffi::c_int
                                                                    >= 'a' as i32 as yaml_char_t
                                                                        as ::core::ffi::c_int
                                                                    && *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(k as isize)
                                                                        as ::core::ffi::c_int
                                                                        <= 'f' as i32 as yaml_char_t
                                                                            as ::core::ffi::c_int
                                                                {
                                                                    *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(k as isize)
                                                                        as ::core::ffi::c_int
                                                                        - 'a' as i32 as yaml_char_t
                                                                            as ::core::ffi::c_int
                                                                        + 10 as ::core::ffi::c_int
                                                                } else {
                                                                    *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(k as isize)
                                                                        as ::core::ffi::c_int
                                                                        - '0' as i32 as yaml_char_t
                                                                            as ::core::ffi::c_int
                                                                })
                                                            })
                                                                as ::core::ffi::c_uint,
                                                        );
                                                    k = k.wrapping_add(1);
                                                }
                                            }
                                            if value >= 0xd800 as ::core::ffi::c_uint
                                                && value <= 0xdfff as ::core::ffi::c_uint
                                                || value
                                                    > 0x10ffff as ::core::ffi::c_int
                                                        as ::core::ffi::c_uint
                                            {
                                                yaml_parser_set_scanner_error(
                                                    parser,
                                                    b"while parsing a quoted scalar\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    start_mark,
                                                    b"found invalid Unicode character escape code\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                current_block = 13911338632193099610;
                                                break 's_44;
                                            } else {
                                                if value <= 0x7f as ::core::ffi::c_uint {
                                                    let fresh86 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh86 = value as yaml_char_t;
                                                } else if value <= 0x7ff as ::core::ffi::c_uint {
                                                    let fresh87 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh87 = (0xc0 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value >> 6 as ::core::ffi::c_int,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh88 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh88 = (0x80 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value & 0x3f as ::core::ffi::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                } else if value <= 0xffff as ::core::ffi::c_uint {
                                                    let fresh89 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh89 = (0xe0 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value >> 12 as ::core::ffi::c_int,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh90 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh90 = (0x80 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value >> 6 as ::core::ffi::c_int
                                                                & 0x3f as ::core::ffi::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh91 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh91 = (0x80 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value & 0x3f as ::core::ffi::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                } else {
                                                    let fresh92 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh92 = (0xf0 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value >> 18 as ::core::ffi::c_int,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh93 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh93 = (0x80 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value >> 12 as ::core::ffi::c_int
                                                                & 0x3f as ::core::ffi::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh94 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh94 = (0x80 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value >> 6 as ::core::ffi::c_int
                                                                & 0x3f as ::core::ffi::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                    let fresh95 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh95 = (0x80 as ::core::ffi::c_uint)
                                                        .wrapping_add(
                                                            value & 0x3f as ::core::ffi::c_uint,
                                                        )
                                                        as yaml_char_t;
                                                }
                                                k = 0 as size_t;
                                                while k < code_length {
                                                    (*parser).mark.index =
                                                        (*parser).mark.index.wrapping_add(1);
                                                    (*parser).mark.column =
                                                        (*parser).mark.column.wrapping_add(1);
                                                    (*parser).unread =
                                                        (*parser).unread.wrapping_sub(1);
                                                    (*parser).buffer.pointer = (*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(
                                                            (if *(*parser)
                                                                .buffer
                                                                .pointer
                                                                .offset(0 as ::core::ffi::c_int as isize)
                                                                as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                                                                == 0 as ::core::ffi::c_int
                                                            {
                                                                1 as ::core::ffi::c_int
                                                            } else {
                                                                (if *(*parser)
                                                                    .buffer
                                                                    .pointer
                                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                                    as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                                                                    == 0xc0 as ::core::ffi::c_int
                                                                {
                                                                    2 as ::core::ffi::c_int
                                                                } else {
                                                                    (if *(*parser)
                                                                        .buffer
                                                                        .pointer
                                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                                        as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                                                        == 0xe0 as ::core::ffi::c_int
                                                                    {
                                                                        3 as ::core::ffi::c_int
                                                                    } else {
                                                                        (if *(*parser)
                                                                            .buffer
                                                                            .pointer
                                                                            .offset(0 as ::core::ffi::c_int as isize)
                                                                            as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                                                                            == 0xf0 as ::core::ffi::c_int
                                                                        {
                                                                            4 as ::core::ffi::c_int
                                                                        } else {
                                                                            0 as ::core::ffi::c_int
                                                                        })
                                                                    })
                                                                })
                                                            }) as isize,
                                                        );
                                                    k = k.wrapping_add(1);
                                                }
                                            }
                                        }
                                    } else if if if string
                                        .pointer
                                        .offset(5 as ::core::ffi::c_int as isize)
                                        < string.end
                                        || yaml_string_extend(
                                            &raw mut string.start,
                                            &raw mut string.pointer,
                                            &raw mut string.end,
                                        ) != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            let fresh96 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh97 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh97 = *fresh96;
                                        } else {
                                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                let fresh98 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh99 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh99 = *fresh98;
                                                let fresh100 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh101 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh101 = *fresh100;
                                            } else {
                                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    let fresh102 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh103 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh103 = *fresh102;
                                                    let fresh104 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh105 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh105 = *fresh104;
                                                    let fresh106 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh107 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh107 = *fresh106;
                                                } else {
                                                    if *(*parser).buffer.pointer
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        let fresh108 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh109 = string.pointer;
                                                        string.pointer = string.pointer.offset(1);
                                                        *fresh109 = *fresh108;
                                                        let fresh110 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh111 = string.pointer;
                                                        string.pointer = string.pointer.offset(1);
                                                        *fresh111 = *fresh110;
                                                        let fresh112 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh113 = string.pointer;
                                                        string.pointer = string.pointer.offset(1);
                                                        *fresh113 = *fresh112;
                                                        let fresh114 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh115 = string.pointer;
                                                        string.pointer = string.pointer.offset(1);
                                                        *fresh115 = *fresh114;
                                                    } else {
                                                    };
                                                };
                                            };
                                        };
                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                        (*parser).mark.column =
                                            (*parser).mark.column.wrapping_add(1);
                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 13911338632193099610;
                                        break 's_44;
                                    }
                                }
                                if if (*parser).unread >= 2 as size_t {
                                    1 as ::core::ffi::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as size_t)
                                } == 0
                                {
                                    current_block = 13911338632193099610;
                                    break 's_44;
                                }
                            }
                            if if (*parser).unread >= 1 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as size_t)
                            } == 0
                            {
                                current_block = 13911338632193099610;
                                break;
                            }
                            if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == (if single != 0 { '\'' as i32 } else { '"' as i32 })
                                    as yaml_char_t
                                    as ::core::ffi::c_int
                            {
                                current_block = 10468276026569382870;
                                break;
                            }
                            if if (*parser).unread >= 1 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as size_t)
                            } == 0
                            {
                                current_block = 13911338632193099610;
                                break;
                            }
                            while *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                || (*(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -87i32 as yaml_char_t as ::core::ffi::c_int)
                            {
                                if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                {
                                    if leading_blanks == 0 {
                                        if if if whitespaces
                                            .pointer
                                            .offset(5 as ::core::ffi::c_int as isize)
                                            < whitespaces.end
                                            || yaml_string_extend(
                                                &raw mut whitespaces.start,
                                                &raw mut whitespaces.pointer,
                                                &raw mut whitespaces.end,
                                            ) != 0
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as ::core::ffi::c_int
                                        } != 0
                                        {
                                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                & 0x80 as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                let fresh116 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh117 = whitespaces.pointer;
                                                whitespaces.pointer = whitespaces.pointer.offset(1);
                                                *fresh117 = *fresh116;
                                            } else {
                                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                    & 0xe0 as ::core::ffi::c_int
                                                    == 0xc0 as ::core::ffi::c_int
                                                {
                                                    let fresh118 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh119 = whitespaces.pointer;
                                                    whitespaces.pointer =
                                                        whitespaces.pointer.offset(1);
                                                    *fresh119 = *fresh118;
                                                    let fresh120 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh121 = whitespaces.pointer;
                                                    whitespaces.pointer =
                                                        whitespaces.pointer.offset(1);
                                                    *fresh121 = *fresh120;
                                                } else {
                                                    if *(*parser).buffer.pointer
                                                        as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0xe0 as ::core::ffi::c_int
                                                    {
                                                        let fresh122 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh123 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh123 = *fresh122;
                                                        let fresh124 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh125 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh125 = *fresh124;
                                                        let fresh126 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh127 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh127 = *fresh126;
                                                    } else {
                                                        if *(*parser).buffer.pointer
                                                            as ::core::ffi::c_int
                                                            & 0xf8 as ::core::ffi::c_int
                                                            == 0xf0 as ::core::ffi::c_int
                                                        {
                                                            let fresh128 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh129 = whitespaces.pointer;
                                                            whitespaces.pointer =
                                                                whitespaces.pointer.offset(1);
                                                            *fresh129 = *fresh128;
                                                            let fresh130 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh131 = whitespaces.pointer;
                                                            whitespaces.pointer =
                                                                whitespaces.pointer.offset(1);
                                                            *fresh131 = *fresh130;
                                                            let fresh132 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh133 = whitespaces.pointer;
                                                            whitespaces.pointer =
                                                                whitespaces.pointer.offset(1);
                                                            *fresh133 = *fresh132;
                                                            let fresh134 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh135 = whitespaces.pointer;
                                                            whitespaces.pointer =
                                                                whitespaces.pointer.offset(1);
                                                            *fresh135 = *fresh134;
                                                        } else {
                                                        };
                                                    };
                                                };
                                            };
                                            (*parser).mark.index =
                                                (*parser).mark.index.wrapping_add(1);
                                            (*parser).mark.column =
                                                (*parser).mark.column.wrapping_add(1);
                                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                                            1 as ::core::ffi::c_int
                                        } else {
                                            0 as ::core::ffi::c_int
                                        } == 0
                                        {
                                            current_block = 13911338632193099610;
                                            break 's_44;
                                        }
                                    } else {
                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                        (*parser).mark.column =
                                            (*parser).mark.column.wrapping_add(1);
                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0x80 as ::core::ffi::c_int
                                                == 0 as ::core::ffi::c_int
                                            {
                                                1 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xe0 as ::core::ffi::c_int
                                                    == 0xc0 as ::core::ffi::c_int
                                                {
                                                    2 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf0 as ::core::ffi::c_int
                                                        == 0xe0 as ::core::ffi::c_int
                                                    {
                                                        3 as ::core::ffi::c_int
                                                    } else {
                                                        (if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            & 0xf8 as ::core::ffi::c_int
                                                            == 0xf0 as ::core::ffi::c_int
                                                        {
                                                            4 as ::core::ffi::c_int
                                                        } else {
                                                            0 as ::core::ffi::c_int
                                                        })
                                                    })
                                                })
                                            }) as isize,
                                        );
                                    }
                                } else {
                                    if if (*parser).unread >= 2 as size_t {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        yaml_parser_update_buffer(parser, 2 as size_t)
                                    } == 0
                                    {
                                        current_block = 13911338632193099610;
                                        break 's_44;
                                    }
                                    if leading_blanks == 0 {
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            whitespaces.end.offset_from(whitespaces.start)
                                                as ::core::ffi::c_long
                                                as size_t,
                                        );
                                        if if if leading_break
                                            .pointer
                                            .offset(5 as ::core::ffi::c_int as isize)
                                            < leading_break.end
                                            || yaml_string_extend(
                                                &raw mut leading_break.start,
                                                &raw mut leading_break.pointer,
                                                &raw mut leading_break.end,
                                            ) != 0
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as ::core::ffi::c_int
                                        } != 0
                                        {
                                            if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                            {
                                                let fresh136 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.offset(1);
                                                *fresh136 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer = (*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(2 as ::core::ffi::c_int as isize);
                                                (*parser).mark.index = ((*parser).mark.index
                                                    as ::core::ffi::c_ulong)
                                                    .wrapping_add(2 as ::core::ffi::c_ulong)
                                                    as size_t
                                                    as size_t;
                                                (*parser).mark.column = 0 as size_t;
                                                (*parser).mark.line =
                                                    (*parser).mark.line.wrapping_add(1);
                                                (*parser).unread = ((*parser).unread
                                                    as ::core::ffi::c_ulong)
                                                    .wrapping_sub(2 as ::core::ffi::c_ulong)
                                                    as size_t
                                                    as size_t;
                                            } else {
                                                if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\r' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                    || *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == '\n' as i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                {
                                                    let fresh137 = leading_break.pointer;
                                                    leading_break.pointer =
                                                        leading_break.pointer.offset(1);
                                                    *fresh137 = '\n' as i32 as yaml_char_t;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    (*parser).mark.index =
                                                        (*parser).mark.index.wrapping_add(1);
                                                    (*parser).mark.column = 0 as size_t;
                                                    (*parser).mark.line =
                                                        (*parser).mark.line.wrapping_add(1);
                                                    (*parser).unread =
                                                        (*parser).unread.wrapping_sub(1);
                                                } else {
                                                    if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -62i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            1 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -123i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                    {
                                                        let fresh138 = leading_break.pointer;
                                                        leading_break.pointer =
                                                            leading_break.pointer.offset(1);
                                                        *fresh138 = '\n' as i32 as yaml_char_t;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(
                                                                2 as ::core::ffi::c_int as isize,
                                                            );
                                                        (*parser).mark.index =
                                                            (*parser).mark.index.wrapping_add(1);
                                                        (*parser).mark.column = 0 as size_t;
                                                        (*parser).mark.line =
                                                            (*parser).mark.line.wrapping_add(1);
                                                        (*parser).unread =
                                                            (*parser).unread.wrapping_sub(1);
                                                    } else {
                                                        if *(*parser).buffer.pointer.offset(
                                                            0 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -30i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            && *(*parser).buffer.pointer.offset(
                                                                1 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -128i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                            && (*(*parser).buffer.pointer.offset(
                                                                2 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -88i32 as yaml_char_t
                                                                    as ::core::ffi::c_int
                                                                || *(*parser).buffer.pointer.offset(
                                                                    2 as ::core::ffi::c_int
                                                                        as isize,
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    == -87i32 as yaml_char_t
                                                                        as ::core::ffi::c_int)
                                                        {
                                                            let fresh139 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh140 = leading_break.pointer;
                                                            leading_break.pointer =
                                                                leading_break.pointer.offset(1);
                                                            *fresh140 = *fresh139;
                                                            let fresh141 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh142 = leading_break.pointer;
                                                            leading_break.pointer =
                                                                leading_break.pointer.offset(1);
                                                            *fresh142 = *fresh141;
                                                            let fresh143 = (*parser).buffer.pointer;
                                                            (*parser).buffer.pointer =
                                                                (*parser).buffer.pointer.offset(1);
                                                            let fresh144 = leading_break.pointer;
                                                            leading_break.pointer =
                                                                leading_break.pointer.offset(1);
                                                            *fresh144 = *fresh143;
                                                            (*parser).mark.index = (*parser)
                                                                .mark
                                                                .index
                                                                .wrapping_add(1);
                                                            (*parser).mark.column = 0 as size_t;
                                                            (*parser).mark.line =
                                                                (*parser).mark.line.wrapping_add(1);
                                                            (*parser).unread =
                                                                (*parser).unread.wrapping_sub(1);
                                                        } else {
                                                        };
                                                    };
                                                };
                                            };
                                            1 as ::core::ffi::c_int
                                        } else {
                                            0 as ::core::ffi::c_int
                                        } == 0
                                        {
                                            current_block = 13911338632193099610;
                                            break 's_44;
                                        }
                                        leading_blanks = 1 as ::core::ffi::c_int;
                                    } else if if if trailing_breaks
                                        .pointer
                                        .offset(5 as ::core::ffi::c_int as isize)
                                        < trailing_breaks.end
                                        || yaml_string_extend(
                                            &raw mut trailing_breaks.start,
                                            &raw mut trailing_breaks.pointer,
                                            &raw mut trailing_breaks.end,
                                        ) != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } != 0
                                    {
                                        if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                        {
                                            let fresh145 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.offset(1);
                                            *fresh145 = '\n' as i32 as yaml_char_t;
                                            (*parser).buffer.pointer = (*parser)
                                                .buffer
                                                .pointer
                                                .offset(2 as ::core::ffi::c_int as isize);
                                            (*parser).mark.index = ((*parser).mark.index
                                                as ::core::ffi::c_ulong)
                                                .wrapping_add(2 as ::core::ffi::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0 as size_t;
                                            (*parser).mark.line =
                                                (*parser).mark.line.wrapping_add(1);
                                            (*parser).unread = ((*parser).unread
                                                as ::core::ffi::c_ulong)
                                                .wrapping_sub(2 as ::core::ffi::c_ulong)
                                                as size_t
                                                as size_t;
                                        } else {
                                            if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                || *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                            {
                                                let fresh146 = trailing_breaks.pointer;
                                                trailing_breaks.pointer =
                                                    trailing_breaks.pointer.offset(1);
                                                *fresh146 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                (*parser).mark.index =
                                                    (*parser).mark.index.wrapping_add(1);
                                                (*parser).mark.column = 0 as size_t;
                                                (*parser).mark.line =
                                                    (*parser).mark.line.wrapping_add(1);
                                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                            } else {
                                                if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                    && *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -123i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                {
                                                    let fresh147 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer =
                                                        trailing_breaks.pointer.offset(1);
                                                    *fresh147 = '\n' as i32 as yaml_char_t;
                                                    (*parser).buffer.pointer = (*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(2 as ::core::ffi::c_int as isize);
                                                    (*parser).mark.index =
                                                        (*parser).mark.index.wrapping_add(1);
                                                    (*parser).mark.column = 0 as size_t;
                                                    (*parser).mark.line =
                                                        (*parser).mark.line.wrapping_add(1);
                                                    (*parser).unread =
                                                        (*parser).unread.wrapping_sub(1);
                                                } else {
                                                    if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -30i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            1 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -128i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        && (*(*parser).buffer.pointer.offset(
                                                            2 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -88i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            || *(*parser).buffer.pointer.offset(
                                                                2 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -87i32 as yaml_char_t
                                                                    as ::core::ffi::c_int)
                                                    {
                                                        let fresh148 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh149 = trailing_breaks.pointer;
                                                        trailing_breaks.pointer =
                                                            trailing_breaks.pointer.offset(1);
                                                        *fresh149 = *fresh148;
                                                        let fresh150 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh151 = trailing_breaks.pointer;
                                                        trailing_breaks.pointer =
                                                            trailing_breaks.pointer.offset(1);
                                                        *fresh151 = *fresh150;
                                                        let fresh152 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh153 = trailing_breaks.pointer;
                                                        trailing_breaks.pointer =
                                                            trailing_breaks.pointer.offset(1);
                                                        *fresh153 = *fresh152;
                                                        (*parser).mark.index =
                                                            (*parser).mark.index.wrapping_add(1);
                                                        (*parser).mark.column = 0 as size_t;
                                                        (*parser).mark.line =
                                                            (*parser).mark.line.wrapping_add(1);
                                                        (*parser).unread =
                                                            (*parser).unread.wrapping_sub(1);
                                                    } else {
                                                    };
                                                };
                                            };
                                        };
                                        1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 13911338632193099610;
                                        break 's_44;
                                    }
                                }
                                if if (*parser).unread >= 1 as size_t {
                                    1 as ::core::ffi::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 1 as size_t)
                                } == 0
                                {
                                    current_block = 13911338632193099610;
                                    break 's_44;
                                }
                            }
                            if leading_blanks != 0 {
                                if *leading_break.start.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32
                                {
                                    if *trailing_breaks
                                        .start
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\0' as i32
                                    {
                                        if if string
                                            .pointer
                                            .offset(5 as ::core::ffi::c_int as isize)
                                            < string.end
                                            || yaml_string_extend(
                                                &raw mut string.start,
                                                &raw mut string.pointer,
                                                &raw mut string.end,
                                            ) != 0
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as ::core::ffi::c_int
                                        } == 0
                                        {
                                            current_block = 13911338632193099610;
                                            break;
                                        }
                                        let fresh154 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh154 = ' ' as i32 as yaml_char_t;
                                    } else {
                                        if if yaml_string_join(
                                            &raw mut string.start,
                                            &raw mut string.pointer,
                                            &raw mut string.end,
                                            &raw mut trailing_breaks.start,
                                            &raw mut trailing_breaks.pointer,
                                            &raw mut trailing_breaks.end,
                                        ) != 0
                                        {
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as ::core::ffi::c_int
                                        } == 0
                                        {
                                            current_block = 13911338632193099610;
                                            break;
                                        }
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        memset(
                                            trailing_breaks.start as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            trailing_breaks.end.offset_from(trailing_breaks.start)
                                                as ::core::ffi::c_long
                                                as size_t,
                                        );
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        leading_break.end.offset_from(leading_break.start)
                                            as ::core::ffi::c_long
                                            as size_t,
                                    );
                                } else {
                                    if if yaml_string_join(
                                        &raw mut string.start,
                                        &raw mut string.pointer,
                                        &raw mut string.end,
                                        &raw mut leading_break.start,
                                        &raw mut leading_break.pointer,
                                        &raw mut leading_break.end,
                                    ) != 0
                                    {
                                        leading_break.pointer = leading_break.start;
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 13911338632193099610;
                                        break;
                                    }
                                    if if yaml_string_join(
                                        &raw mut string.start,
                                        &raw mut string.pointer,
                                        &raw mut string.end,
                                        &raw mut trailing_breaks.start,
                                        &raw mut trailing_breaks.pointer,
                                        &raw mut trailing_breaks.end,
                                    ) != 0
                                    {
                                        trailing_breaks.pointer = trailing_breaks.start;
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 13911338632193099610;
                                        break;
                                    }
                                    leading_break.pointer = leading_break.start;
                                    memset(
                                        leading_break.start as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        leading_break.end.offset_from(leading_break.start)
                                            as ::core::ffi::c_long
                                            as size_t,
                                    );
                                    trailing_breaks.pointer = trailing_breaks.start;
                                    memset(
                                        trailing_breaks.start as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        trailing_breaks.end.offset_from(trailing_breaks.start)
                                            as ::core::ffi::c_long
                                            as size_t,
                                    );
                                }
                            } else {
                                if if yaml_string_join(
                                    &raw mut string.start,
                                    &raw mut string.pointer,
                                    &raw mut string.end,
                                    &raw mut whitespaces.start,
                                    &raw mut whitespaces.pointer,
                                    &raw mut whitespaces.end,
                                ) != 0
                                {
                                    whitespaces.pointer = whitespaces.start;
                                    1 as ::core::ffi::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as ::core::ffi::c_int
                                } == 0
                                {
                                    current_block = 13911338632193099610;
                                    break;
                                }
                                whitespaces.pointer = whitespaces.start;
                                memset(
                                    whitespaces.start as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    whitespaces.end.offset_from(whitespaces.start)
                                        as ::core::ffi::c_long
                                        as size_t,
                                );
                            }
                        }
                    }
                    match current_block {
                        13911338632193099610 => {}
                        _ => {
                            (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                            (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                            (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                (if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0x80 as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    (if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xe0 as ::core::ffi::c_int
                                        == 0xc0 as ::core::ffi::c_int
                                    {
                                        2 as ::core::ffi::c_int
                                    } else {
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0xf0 as ::core::ffi::c_int
                                            == 0xe0 as ::core::ffi::c_int
                                        {
                                            3 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xf8 as ::core::ffi::c_int
                                                == 0xf0 as ::core::ffi::c_int
                                            {
                                                4 as ::core::ffi::c_int
                                            } else {
                                                0 as ::core::ffi::c_int
                                            })
                                        })
                                    })
                                }) as isize,
                            );
                            end_mark = (*parser).mark;
                            memset(
                                token as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<yaml_token_t>() as size_t,
                            );
                            (*token).type_0 = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.scalar.value = string.start;
                            (*token).data.scalar.length = string.pointer.offset_from(string.start)
                                as ::core::ffi::c_long
                                as size_t;
                            (*token).data.scalar.style = (if single != 0 {
                                YAML_SINGLE_QUOTED_SCALAR_STYLE as ::core::ffi::c_int
                            } else {
                                YAML_DOUBLE_QUOTED_SCALAR_STYLE as ::core::ffi::c_int
                            })
                                as yaml_scalar_style_t;
                            yaml_free(leading_break.start as *mut ::core::ffi::c_void);
                            leading_break.end = ::core::ptr::null_mut::<yaml_char_t>();
                            leading_break.pointer = leading_break.end;
                            leading_break.start = leading_break.pointer;
                            yaml_free(trailing_breaks.start as *mut ::core::ffi::c_void);
                            trailing_breaks.end = ::core::ptr::null_mut::<yaml_char_t>();
                            trailing_breaks.pointer = trailing_breaks.end;
                            trailing_breaks.start = trailing_breaks.pointer;
                            yaml_free(whitespaces.start as *mut ::core::ffi::c_void);
                            whitespaces.end = ::core::ptr::null_mut::<yaml_char_t>();
                            whitespaces.pointer = whitespaces.end;
                            whitespaces.start = whitespaces.pointer;
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut ::core::ffi::c_void);
    string.end = ::core::ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut ::core::ffi::c_void);
    leading_break.end = ::core::ptr::null_mut::<yaml_char_t>();
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut ::core::ffi::c_void);
    trailing_breaks.end = ::core::ptr::null_mut::<yaml_char_t>();
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    yaml_free(whitespaces.start as *mut ::core::ffi::c_void);
    whitespaces.end = ::core::ptr::null_mut::<yaml_char_t>();
    whitespaces.pointer = whitespaces.end;
    whitespaces.start = whitespaces.pointer;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_parser_scan_plain_scalar(
    mut parser: *mut yaml_parser_t,
    mut token: *mut yaml_token_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut start_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut end_mark: yaml_mark_t = yaml_mark_t {
        index: 0,
        line: 0,
        column: 0,
    };
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_break: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut trailing_breaks: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut whitespaces: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut leading_blanks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut indent: ::core::ffi::c_int = (*parser).indent + 1 as ::core::ffi::c_int;
    string.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if !(if !string.start.is_null() {
        string.pointer = string.start;
        string.end = string.start.offset(16 as ::core::ffi::c_int as isize);
        memset(
            string.start as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            16 as size_t,
        );
        1 as ::core::ffi::c_int
    } else {
        (*parser).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        leading_break.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
        if !(if !leading_break.start.is_null() {
            leading_break.pointer = leading_break.start;
            leading_break.end = leading_break
                .start
                .offset(16 as ::core::ffi::c_int as isize);
            memset(
                leading_break.start as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                16 as size_t,
            );
            1 as ::core::ffi::c_int
        } else {
            (*parser).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0)
        {
            trailing_breaks.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
            if !(if !trailing_breaks.start.is_null() {
                trailing_breaks.pointer = trailing_breaks.start;
                trailing_breaks.end = trailing_breaks
                    .start
                    .offset(16 as ::core::ffi::c_int as isize);
                memset(
                    trailing_breaks.start as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    16 as size_t,
                );
                1 as ::core::ffi::c_int
            } else {
                (*parser).error = YAML_MEMORY_ERROR;
                0 as ::core::ffi::c_int
            } == 0)
            {
                whitespaces.start = yaml_malloc(16 as size_t) as *mut yaml_char_t;
                if !(if !whitespaces.start.is_null() {
                    whitespaces.pointer = whitespaces.start;
                    whitespaces.end = whitespaces.start.offset(16 as ::core::ffi::c_int as isize);
                    memset(
                        whitespaces.start as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        16 as size_t,
                    );
                    1 as ::core::ffi::c_int
                } else {
                    (*parser).error = YAML_MEMORY_ERROR;
                    0 as ::core::ffi::c_int
                } == 0)
                {
                    end_mark = (*parser).mark;
                    start_mark = end_mark;
                    's_43: loop {
                        if if (*parser).unread >= 4 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 4 as size_t)
                        } == 0
                        {
                            current_block = 6276756041929655903;
                            break;
                        }
                        if (*parser).mark.column == 0 as size_t
                            && (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '.' as i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '.' as i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(2 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '.' as i32 as yaml_char_t as ::core::ffi::c_int)
                            && (*(*parser)
                                .buffer
                                .pointer
                                .offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                || (*(*parser)
                                    .buffer
                                    .pointer
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser).buffer.pointer.offset(
                                            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                                as isize,
                                        )
                                            as ::core::ffi::c_int
                                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(3 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                        if *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == '#' as i32 as yaml_char_t as ::core::ffi::c_int
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                        while !(*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                            || (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                        {
                            if (*parser).flow_level != 0
                                && *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ':' as i32 as yaml_char_t as ::core::ffi::c_int
                                && (*(*parser)
                                    .buffer
                                    .pointer
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ',' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '?' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '[' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == ']' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '{' as i32 as yaml_char_t as ::core::ffi::c_int
                                    || *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '}' as i32 as yaml_char_t as ::core::ffi::c_int)
                            {
                                yaml_parser_set_scanner_error(
                                    parser,
                                    b"while scanning a plain scalar\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    start_mark,
                                    b"found unexpected ':'\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                current_block = 6276756041929655903;
                                break 's_43;
                            } else {
                                if *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == ':' as i32 as yaml_char_t as ::core::ffi::c_int
                                    && (*(*parser)
                                        .buffer
                                        .pointer
                                        .offset(1 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                        || (*(*parser)
                                            .buffer
                                            .pointer
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t
                                                    as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -123i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -128i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -88i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 1 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -128i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                                && *(*parser).buffer.pointer.offset(
                                                    (1 as ::core::ffi::c_int
                                                        + 2 as ::core::ffi::c_int)
                                                        as isize,
                                                )
                                                    as ::core::ffi::c_int
                                                    == -87i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\0' as i32 as yaml_char_t
                                                    as ::core::ffi::c_int))
                                    || (*parser).flow_level != 0
                                        && (*(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == ',' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '[' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == ']' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '{' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '}' as i32 as yaml_char_t as ::core::ffi::c_int)
                                {
                                    break;
                                }
                                if leading_blanks != 0 || whitespaces.start != whitespaces.pointer {
                                    if leading_blanks != 0 {
                                        if *leading_break
                                            .start
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\n' as i32
                                        {
                                            if *trailing_breaks
                                                .start
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\0' as i32
                                            {
                                                if if string
                                                    .pointer
                                                    .offset(5 as ::core::ffi::c_int as isize)
                                                    < string.end
                                                    || yaml_string_extend(
                                                        &raw mut string.start,
                                                        &raw mut string.pointer,
                                                        &raw mut string.end,
                                                    ) != 0
                                                {
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0 as ::core::ffi::c_int
                                                } == 0
                                                {
                                                    current_block = 6276756041929655903;
                                                    break 's_43;
                                                }
                                                let fresh2 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh2 = ' ' as i32 as yaml_char_t;
                                            } else {
                                                if if yaml_string_join(
                                                    &raw mut string.start,
                                                    &raw mut string.pointer,
                                                    &raw mut string.end,
                                                    &raw mut trailing_breaks.start,
                                                    &raw mut trailing_breaks.pointer,
                                                    &raw mut trailing_breaks.end,
                                                ) != 0
                                                {
                                                    trailing_breaks.pointer = trailing_breaks.start;
                                                    1 as ::core::ffi::c_int
                                                } else {
                                                    (*parser).error = YAML_MEMORY_ERROR;
                                                    0 as ::core::ffi::c_int
                                                } == 0
                                                {
                                                    current_block = 6276756041929655903;
                                                    break 's_43;
                                                }
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                memset(
                                                    trailing_breaks.start
                                                        as *mut ::core::ffi::c_void,
                                                    0 as ::core::ffi::c_int,
                                                    trailing_breaks
                                                        .end
                                                        .offset_from(trailing_breaks.start)
                                                        as ::core::ffi::c_long
                                                        as size_t,
                                                );
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut ::core::ffi::c_void,
                                                0 as ::core::ffi::c_int,
                                                leading_break.end.offset_from(leading_break.start)
                                                    as ::core::ffi::c_long
                                                    as size_t,
                                            );
                                        } else {
                                            if if yaml_string_join(
                                                &raw mut string.start,
                                                &raw mut string.pointer,
                                                &raw mut string.end,
                                                &raw mut leading_break.start,
                                                &raw mut leading_break.pointer,
                                                &raw mut leading_break.end,
                                            ) != 0
                                            {
                                                leading_break.pointer = leading_break.start;
                                                1 as ::core::ffi::c_int
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0 as ::core::ffi::c_int
                                            } == 0
                                            {
                                                current_block = 6276756041929655903;
                                                break 's_43;
                                            }
                                            if if yaml_string_join(
                                                &raw mut string.start,
                                                &raw mut string.pointer,
                                                &raw mut string.end,
                                                &raw mut trailing_breaks.start,
                                                &raw mut trailing_breaks.pointer,
                                                &raw mut trailing_breaks.end,
                                            ) != 0
                                            {
                                                trailing_breaks.pointer = trailing_breaks.start;
                                                1 as ::core::ffi::c_int
                                            } else {
                                                (*parser).error = YAML_MEMORY_ERROR;
                                                0 as ::core::ffi::c_int
                                            } == 0
                                            {
                                                current_block = 6276756041929655903;
                                                break 's_43;
                                            }
                                            leading_break.pointer = leading_break.start;
                                            memset(
                                                leading_break.start as *mut ::core::ffi::c_void,
                                                0 as ::core::ffi::c_int,
                                                leading_break.end.offset_from(leading_break.start)
                                                    as ::core::ffi::c_long
                                                    as size_t,
                                            );
                                            trailing_breaks.pointer = trailing_breaks.start;
                                            memset(
                                                trailing_breaks.start as *mut ::core::ffi::c_void,
                                                0 as ::core::ffi::c_int,
                                                trailing_breaks
                                                    .end
                                                    .offset_from(trailing_breaks.start)
                                                    as ::core::ffi::c_long
                                                    as size_t,
                                            );
                                        }
                                        leading_blanks = 0 as ::core::ffi::c_int;
                                    } else {
                                        if if yaml_string_join(
                                            &raw mut string.start,
                                            &raw mut string.pointer,
                                            &raw mut string.end,
                                            &raw mut whitespaces.start,
                                            &raw mut whitespaces.pointer,
                                            &raw mut whitespaces.end,
                                        ) != 0
                                        {
                                            whitespaces.pointer = whitespaces.start;
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (*parser).error = YAML_MEMORY_ERROR;
                                            0 as ::core::ffi::c_int
                                        } == 0
                                        {
                                            current_block = 6276756041929655903;
                                            break 's_43;
                                        }
                                        whitespaces.pointer = whitespaces.start;
                                        memset(
                                            whitespaces.start as *mut ::core::ffi::c_void,
                                            0 as ::core::ffi::c_int,
                                            whitespaces.end.offset_from(whitespaces.start)
                                                as ::core::ffi::c_long
                                                as size_t,
                                        );
                                    }
                                }
                                if if if string.pointer.offset(5 as ::core::ffi::c_int as isize)
                                    < string.end
                                    || yaml_string_extend(
                                        &raw mut string.start,
                                        &raw mut string.pointer,
                                        &raw mut string.end,
                                    ) != 0
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as ::core::ffi::c_int
                                } != 0
                                {
                                    if *(*parser).buffer.pointer as ::core::ffi::c_int
                                        & 0x80 as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                    {
                                        let fresh3 = (*parser).buffer.pointer;
                                        (*parser).buffer.pointer =
                                            (*parser).buffer.pointer.offset(1);
                                        let fresh4 = string.pointer;
                                        string.pointer = string.pointer.offset(1);
                                        *fresh4 = *fresh3;
                                    } else {
                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                            & 0xe0 as ::core::ffi::c_int
                                            == 0xc0 as ::core::ffi::c_int
                                        {
                                            let fresh5 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh6 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh6 = *fresh5;
                                            let fresh7 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh8 = string.pointer;
                                            string.pointer = string.pointer.offset(1);
                                            *fresh8 = *fresh7;
                                        } else {
                                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                & 0xf0 as ::core::ffi::c_int
                                                == 0xe0 as ::core::ffi::c_int
                                            {
                                                let fresh9 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh10 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh10 = *fresh9;
                                                let fresh11 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh12 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh12 = *fresh11;
                                                let fresh13 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh14 = string.pointer;
                                                string.pointer = string.pointer.offset(1);
                                                *fresh14 = *fresh13;
                                            } else {
                                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                    & 0xf8 as ::core::ffi::c_int
                                                    == 0xf0 as ::core::ffi::c_int
                                                {
                                                    let fresh15 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh16 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh16 = *fresh15;
                                                    let fresh17 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh18 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh18 = *fresh17;
                                                    let fresh19 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh20 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh20 = *fresh19;
                                                    let fresh21 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh22 = string.pointer;
                                                    string.pointer = string.pointer.offset(1);
                                                    *fresh22 = *fresh21;
                                                } else {
                                                };
                                            };
                                        };
                                    };
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                } == 0
                                {
                                    current_block = 6276756041929655903;
                                    break 's_43;
                                }
                                end_mark = (*parser).mark;
                                if if (*parser).unread >= 2 as size_t {
                                    1 as ::core::ffi::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as size_t)
                                } == 0
                                {
                                    current_block = 6276756041929655903;
                                    break 's_43;
                                }
                            }
                        }
                        if !(*(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                            || (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -87i32 as yaml_char_t as ::core::ffi::c_int))
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                        if if (*parser).unread >= 1 as size_t {
                            1 as ::core::ffi::c_int
                        } else {
                            yaml_parser_update_buffer(parser, 1 as size_t)
                        } == 0
                        {
                            current_block = 6276756041929655903;
                            break;
                        }
                        while *(*parser)
                            .buffer
                            .pointer
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                            || *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                            || (*(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                                    && *(*parser).buffer.pointer.offset(
                                        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        == -87i32 as yaml_char_t as ::core::ffi::c_int)
                        {
                            if *(*parser)
                                .buffer
                                .pointer
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                                || *(*parser)
                                    .buffer
                                    .pointer
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                            {
                                if leading_blanks != 0
                                    && ((*parser).mark.column as ::core::ffi::c_int) < indent
                                    && *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                                {
                                    yaml_parser_set_scanner_error(
                                        parser,
                                        b"while scanning a plain scalar\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        start_mark,
                                        b"found a tab character that violates indentation\0"
                                            as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block = 6276756041929655903;
                                    break 's_43;
                                } else if leading_blanks == 0 {
                                    if if if whitespaces
                                        .pointer
                                        .offset(5 as ::core::ffi::c_int as isize)
                                        < whitespaces.end
                                        || yaml_string_extend(
                                            &raw mut whitespaces.start,
                                            &raw mut whitespaces.pointer,
                                            &raw mut whitespaces.end,
                                        ) != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } != 0
                                    {
                                        if *(*parser).buffer.pointer as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            let fresh23 = (*parser).buffer.pointer;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            let fresh24 = whitespaces.pointer;
                                            whitespaces.pointer = whitespaces.pointer.offset(1);
                                            *fresh24 = *fresh23;
                                        } else {
                                            if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                let fresh25 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh26 = whitespaces.pointer;
                                                whitespaces.pointer = whitespaces.pointer.offset(1);
                                                *fresh26 = *fresh25;
                                                let fresh27 = (*parser).buffer.pointer;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                let fresh28 = whitespaces.pointer;
                                                whitespaces.pointer = whitespaces.pointer.offset(1);
                                                *fresh28 = *fresh27;
                                            } else {
                                                if *(*parser).buffer.pointer as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    let fresh29 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh30 = whitespaces.pointer;
                                                    whitespaces.pointer =
                                                        whitespaces.pointer.offset(1);
                                                    *fresh30 = *fresh29;
                                                    let fresh31 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh32 = whitespaces.pointer;
                                                    whitespaces.pointer =
                                                        whitespaces.pointer.offset(1);
                                                    *fresh32 = *fresh31;
                                                    let fresh33 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh34 = whitespaces.pointer;
                                                    whitespaces.pointer =
                                                        whitespaces.pointer.offset(1);
                                                    *fresh34 = *fresh33;
                                                } else {
                                                    if *(*parser).buffer.pointer
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        let fresh35 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh36 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh36 = *fresh35;
                                                        let fresh37 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh38 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh38 = *fresh37;
                                                        let fresh39 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh40 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh40 = *fresh39;
                                                        let fresh41 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh42 = whitespaces.pointer;
                                                        whitespaces.pointer =
                                                            whitespaces.pointer.offset(1);
                                                        *fresh42 = *fresh41;
                                                    } else {
                                                    };
                                                };
                                            };
                                        };
                                        (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                        (*parser).mark.column =
                                            (*parser).mark.column.wrapping_add(1);
                                        (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 6276756041929655903;
                                        break 's_43;
                                    }
                                } else {
                                    (*parser).mark.index = (*parser).mark.index.wrapping_add(1);
                                    (*parser).mark.column = (*parser).mark.column.wrapping_add(1);
                                    (*parser).unread = (*parser).unread.wrapping_sub(1);
                                    (*parser).buffer.pointer = (*parser).buffer.pointer.offset(
                                        (if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & 0x80 as ::core::ffi::c_int
                                            == 0 as ::core::ffi::c_int
                                        {
                                            1 as ::core::ffi::c_int
                                        } else {
                                            (if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                & 0xe0 as ::core::ffi::c_int
                                                == 0xc0 as ::core::ffi::c_int
                                            {
                                                2 as ::core::ffi::c_int
                                            } else {
                                                (if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    & 0xf0 as ::core::ffi::c_int
                                                    == 0xe0 as ::core::ffi::c_int
                                                {
                                                    3 as ::core::ffi::c_int
                                                } else {
                                                    (if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        & 0xf8 as ::core::ffi::c_int
                                                        == 0xf0 as ::core::ffi::c_int
                                                    {
                                                        4 as ::core::ffi::c_int
                                                    } else {
                                                        0 as ::core::ffi::c_int
                                                    })
                                                })
                                            })
                                        }) as isize,
                                    );
                                }
                            } else {
                                if if (*parser).unread >= 2 as size_t {
                                    1 as ::core::ffi::c_int
                                } else {
                                    yaml_parser_update_buffer(parser, 2 as size_t)
                                } == 0
                                {
                                    current_block = 6276756041929655903;
                                    break 's_43;
                                }
                                if leading_blanks == 0 {
                                    whitespaces.pointer = whitespaces.start;
                                    memset(
                                        whitespaces.start as *mut ::core::ffi::c_void,
                                        0 as ::core::ffi::c_int,
                                        whitespaces.end.offset_from(whitespaces.start)
                                            as ::core::ffi::c_long
                                            as size_t,
                                    );
                                    if if if leading_break
                                        .pointer
                                        .offset(5 as ::core::ffi::c_int as isize)
                                        < leading_break.end
                                        || yaml_string_extend(
                                            &raw mut leading_break.start,
                                            &raw mut leading_break.pointer,
                                            &raw mut leading_break.end,
                                        ) != 0
                                    {
                                        1 as ::core::ffi::c_int
                                    } else {
                                        (*parser).error = YAML_MEMORY_ERROR;
                                        0 as ::core::ffi::c_int
                                    } != 0
                                    {
                                        if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            && *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(1 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                        {
                                            let fresh43 = leading_break.pointer;
                                            leading_break.pointer = leading_break.pointer.offset(1);
                                            *fresh43 = '\n' as i32 as yaml_char_t;
                                            (*parser).buffer.pointer = (*parser)
                                                .buffer
                                                .pointer
                                                .offset(2 as ::core::ffi::c_int as isize);
                                            (*parser).mark.index = ((*parser).mark.index
                                                as ::core::ffi::c_ulong)
                                                .wrapping_add(2 as ::core::ffi::c_ulong)
                                                as size_t
                                                as size_t;
                                            (*parser).mark.column = 0 as size_t;
                                            (*parser).mark.line =
                                                (*parser).mark.line.wrapping_add(1);
                                            (*parser).unread = ((*parser).unread
                                                as ::core::ffi::c_ulong)
                                                .wrapping_sub(2 as ::core::ffi::c_ulong)
                                                as size_t
                                                as size_t;
                                        } else {
                                            if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                                || *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == '\n' as i32 as yaml_char_t
                                                        as ::core::ffi::c_int
                                            {
                                                let fresh44 = leading_break.pointer;
                                                leading_break.pointer =
                                                    leading_break.pointer.offset(1);
                                                *fresh44 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer =
                                                    (*parser).buffer.pointer.offset(1);
                                                (*parser).mark.index =
                                                    (*parser).mark.index.wrapping_add(1);
                                                (*parser).mark.column = 0 as size_t;
                                                (*parser).mark.line =
                                                    (*parser).mark.line.wrapping_add(1);
                                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                            } else {
                                                if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                    && *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -123i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                {
                                                    let fresh45 = leading_break.pointer;
                                                    leading_break.pointer =
                                                        leading_break.pointer.offset(1);
                                                    *fresh45 = '\n' as i32 as yaml_char_t;
                                                    (*parser).buffer.pointer = (*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(2 as ::core::ffi::c_int as isize);
                                                    (*parser).mark.index =
                                                        (*parser).mark.index.wrapping_add(1);
                                                    (*parser).mark.column = 0 as size_t;
                                                    (*parser).mark.line =
                                                        (*parser).mark.line.wrapping_add(1);
                                                    (*parser).unread =
                                                        (*parser).unread.wrapping_sub(1);
                                                } else {
                                                    if *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(0 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -30i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        && *(*parser).buffer.pointer.offset(
                                                            1 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -128i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                        && (*(*parser).buffer.pointer.offset(
                                                            2 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -88i32 as yaml_char_t
                                                                as ::core::ffi::c_int
                                                            || *(*parser).buffer.pointer.offset(
                                                                2 as ::core::ffi::c_int as isize,
                                                            )
                                                                as ::core::ffi::c_int
                                                                == -87i32 as yaml_char_t
                                                                    as ::core::ffi::c_int)
                                                    {
                                                        let fresh46 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh47 = leading_break.pointer;
                                                        leading_break.pointer =
                                                            leading_break.pointer.offset(1);
                                                        *fresh47 = *fresh46;
                                                        let fresh48 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh49 = leading_break.pointer;
                                                        leading_break.pointer =
                                                            leading_break.pointer.offset(1);
                                                        *fresh49 = *fresh48;
                                                        let fresh50 = (*parser).buffer.pointer;
                                                        (*parser).buffer.pointer =
                                                            (*parser).buffer.pointer.offset(1);
                                                        let fresh51 = leading_break.pointer;
                                                        leading_break.pointer =
                                                            leading_break.pointer.offset(1);
                                                        *fresh51 = *fresh50;
                                                        (*parser).mark.index =
                                                            (*parser).mark.index.wrapping_add(1);
                                                        (*parser).mark.column = 0 as size_t;
                                                        (*parser).mark.line =
                                                            (*parser).mark.line.wrapping_add(1);
                                                        (*parser).unread =
                                                            (*parser).unread.wrapping_sub(1);
                                                    } else {
                                                    };
                                                };
                                            };
                                        };
                                        1 as ::core::ffi::c_int
                                    } else {
                                        0 as ::core::ffi::c_int
                                    } == 0
                                    {
                                        current_block = 6276756041929655903;
                                        break 's_43;
                                    }
                                    leading_blanks = 1 as ::core::ffi::c_int;
                                } else if if if trailing_breaks
                                    .pointer
                                    .offset(5 as ::core::ffi::c_int as isize)
                                    < trailing_breaks.end
                                    || yaml_string_extend(
                                        &raw mut trailing_breaks.start,
                                        &raw mut trailing_breaks.pointer,
                                        &raw mut trailing_breaks.end,
                                    ) != 0
                                {
                                    1 as ::core::ffi::c_int
                                } else {
                                    (*parser).error = YAML_MEMORY_ERROR;
                                    0 as ::core::ffi::c_int
                                } != 0
                                {
                                    if *(*parser)
                                        .buffer
                                        .pointer
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                        && *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                    {
                                        let fresh52 = trailing_breaks.pointer;
                                        trailing_breaks.pointer = trailing_breaks.pointer.offset(1);
                                        *fresh52 = '\n' as i32 as yaml_char_t;
                                        (*parser).buffer.pointer = (*parser)
                                            .buffer
                                            .pointer
                                            .offset(2 as ::core::ffi::c_int as isize);
                                        (*parser).mark.index = ((*parser).mark.index
                                            as ::core::ffi::c_ulong)
                                            .wrapping_add(2 as ::core::ffi::c_ulong)
                                            as size_t
                                            as size_t;
                                        (*parser).mark.column = 0 as size_t;
                                        (*parser).mark.line = (*parser).mark.line.wrapping_add(1);
                                        (*parser).unread = ((*parser).unread
                                            as ::core::ffi::c_ulong)
                                            .wrapping_sub(2 as ::core::ffi::c_ulong)
                                            as size_t
                                            as size_t;
                                    } else {
                                        if *(*parser)
                                            .buffer
                                            .pointer
                                            .offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                                            || *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                                        {
                                            let fresh53 = trailing_breaks.pointer;
                                            trailing_breaks.pointer =
                                                trailing_breaks.pointer.offset(1);
                                            *fresh53 = '\n' as i32 as yaml_char_t;
                                            (*parser).buffer.pointer =
                                                (*parser).buffer.pointer.offset(1);
                                            (*parser).mark.index =
                                                (*parser).mark.index.wrapping_add(1);
                                            (*parser).mark.column = 0 as size_t;
                                            (*parser).mark.line =
                                                (*parser).mark.line.wrapping_add(1);
                                            (*parser).unread = (*parser).unread.wrapping_sub(1);
                                        } else {
                                            if *(*parser)
                                                .buffer
                                                .pointer
                                                .offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int
                                                == -62i32 as yaml_char_t as ::core::ffi::c_int
                                                && *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(1 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -123i32 as yaml_char_t as ::core::ffi::c_int
                                            {
                                                let fresh54 = trailing_breaks.pointer;
                                                trailing_breaks.pointer =
                                                    trailing_breaks.pointer.offset(1);
                                                *fresh54 = '\n' as i32 as yaml_char_t;
                                                (*parser).buffer.pointer = (*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(2 as ::core::ffi::c_int as isize);
                                                (*parser).mark.index =
                                                    (*parser).mark.index.wrapping_add(1);
                                                (*parser).mark.column = 0 as size_t;
                                                (*parser).mark.line =
                                                    (*parser).mark.line.wrapping_add(1);
                                                (*parser).unread = (*parser).unread.wrapping_sub(1);
                                            } else {
                                                if *(*parser)
                                                    .buffer
                                                    .pointer
                                                    .offset(0 as ::core::ffi::c_int as isize)
                                                    as ::core::ffi::c_int
                                                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                                                    && *(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(1 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -128i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                    && (*(*parser)
                                                        .buffer
                                                        .pointer
                                                        .offset(2 as ::core::ffi::c_int as isize)
                                                        as ::core::ffi::c_int
                                                        == -88i32 as yaml_char_t
                                                            as ::core::ffi::c_int
                                                        || *(*parser).buffer.pointer.offset(
                                                            2 as ::core::ffi::c_int as isize,
                                                        )
                                                            as ::core::ffi::c_int
                                                            == -87i32 as yaml_char_t
                                                                as ::core::ffi::c_int)
                                                {
                                                    let fresh55 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh56 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer =
                                                        trailing_breaks.pointer.offset(1);
                                                    *fresh56 = *fresh55;
                                                    let fresh57 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh58 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer =
                                                        trailing_breaks.pointer.offset(1);
                                                    *fresh58 = *fresh57;
                                                    let fresh59 = (*parser).buffer.pointer;
                                                    (*parser).buffer.pointer =
                                                        (*parser).buffer.pointer.offset(1);
                                                    let fresh60 = trailing_breaks.pointer;
                                                    trailing_breaks.pointer =
                                                        trailing_breaks.pointer.offset(1);
                                                    *fresh60 = *fresh59;
                                                    (*parser).mark.index =
                                                        (*parser).mark.index.wrapping_add(1);
                                                    (*parser).mark.column = 0 as size_t;
                                                    (*parser).mark.line =
                                                        (*parser).mark.line.wrapping_add(1);
                                                    (*parser).unread =
                                                        (*parser).unread.wrapping_sub(1);
                                                } else {
                                                };
                                            };
                                        };
                                    };
                                    1 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                } == 0
                                {
                                    current_block = 6276756041929655903;
                                    break 's_43;
                                }
                            }
                            if if (*parser).unread >= 1 as size_t {
                                1 as ::core::ffi::c_int
                            } else {
                                yaml_parser_update_buffer(parser, 1 as size_t)
                            } == 0
                            {
                                current_block = 6276756041929655903;
                                break 's_43;
                            }
                        }
                        if (*parser).flow_level == 0
                            && ((*parser).mark.column as ::core::ffi::c_int) < indent
                        {
                            current_block = 16415152177862271243;
                            break;
                        }
                    }
                    match current_block {
                        6276756041929655903 => {}
                        _ => {
                            memset(
                                token as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<yaml_token_t>() as size_t,
                            );
                            (*token).type_0 = YAML_SCALAR_TOKEN;
                            (*token).start_mark = start_mark;
                            (*token).end_mark = end_mark;
                            (*token).data.scalar.value = string.start;
                            (*token).data.scalar.length = string.pointer.offset_from(string.start)
                                as ::core::ffi::c_long
                                as size_t;
                            (*token).data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
                            if leading_blanks != 0 {
                                (*parser).simple_key_allowed = 1 as ::core::ffi::c_int;
                            }
                            yaml_free(leading_break.start as *mut ::core::ffi::c_void);
                            leading_break.end = ::core::ptr::null_mut::<yaml_char_t>();
                            leading_break.pointer = leading_break.end;
                            leading_break.start = leading_break.pointer;
                            yaml_free(trailing_breaks.start as *mut ::core::ffi::c_void);
                            trailing_breaks.end = ::core::ptr::null_mut::<yaml_char_t>();
                            trailing_breaks.pointer = trailing_breaks.end;
                            trailing_breaks.start = trailing_breaks.pointer;
                            yaml_free(whitespaces.start as *mut ::core::ffi::c_void);
                            whitespaces.end = ::core::ptr::null_mut::<yaml_char_t>();
                            whitespaces.pointer = whitespaces.end;
                            whitespaces.start = whitespaces.pointer;
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    yaml_free(string.start as *mut ::core::ffi::c_void);
    string.end = ::core::ptr::null_mut::<yaml_char_t>();
    string.pointer = string.end;
    string.start = string.pointer;
    yaml_free(leading_break.start as *mut ::core::ffi::c_void);
    leading_break.end = ::core::ptr::null_mut::<yaml_char_t>();
    leading_break.pointer = leading_break.end;
    leading_break.start = leading_break.pointer;
    yaml_free(trailing_breaks.start as *mut ::core::ffi::c_void);
    trailing_breaks.end = ::core::ptr::null_mut::<yaml_char_t>();
    trailing_breaks.pointer = trailing_breaks.end;
    trailing_breaks.start = trailing_breaks.pointer;
    yaml_free(whitespaces.start as *mut ::core::ffi::c_void);
    whitespaces.end = ::core::ptr::null_mut::<yaml_char_t>();
    whitespaces.pointer = whitespaces.end;
    whitespaces.start = whitespaces.pointer;
    return 0 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
