pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;
pub type _IO_wide_data = libc::c_void;

extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
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
    fn yaml_emitter_emit(
        emitter: *mut yaml_emitter_t,
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
pub type yaml_break_e = ::core::ffi::c_uint;
pub const YAML_CRLN_BREAK: yaml_break_e = 3;
pub const YAML_LN_BREAK: yaml_break_e = 2;
pub const YAML_CR_BREAK: yaml_break_e = 1;
pub const YAML_ANY_BREAK: yaml_break_e = 0;
pub type yaml_break_t = yaml_break_e;
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
    pub data: C2RustUnnamed,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub stream_start: C2RustUnnamed_7,
    pub document_start: C2RustUnnamed_5,
    pub document_end: C2RustUnnamed_4,
    pub alias: C2RustUnnamed_3,
    pub scalar: C2RustUnnamed_2,
    pub sequence_start: C2RustUnnamed_1,
    pub mapping_start: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: ::core::ffi::c_int,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub anchor: *mut yaml_char_t,
    pub tag: *mut yaml_char_t,
    pub implicit: ::core::ffi::c_int,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct C2RustUnnamed_3 {
    pub anchor: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub implicit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_6,
    pub implicit: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
    pub data: C2RustUnnamed_8,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub scalar: C2RustUnnamed_13,
    pub sequence: C2RustUnnamed_11,
    pub mapping: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub pairs: C2RustUnnamed_10,
    pub style: yaml_mapping_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct C2RustUnnamed_11 {
    pub items: C2RustUnnamed_12,
    pub style: yaml_sequence_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub start: *mut yaml_node_item_t,
    pub end: *mut yaml_node_item_t,
    pub top: *mut yaml_node_item_t,
}
pub type yaml_node_item_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub style: yaml_scalar_style_t,
}
pub type yaml_node_t = yaml_node_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_document_s {
    pub nodes: C2RustUnnamed_15,
    pub version_directive: *mut yaml_version_directive_t,
    pub tag_directives: C2RustUnnamed_14,
    pub start_implicit: ::core::ffi::c_int,
    pub end_implicit: ::core::ffi::c_int,
    pub start_mark: yaml_mark_t,
    pub end_mark: yaml_mark_t,
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
    pub start: *mut yaml_node_t,
    pub end: *mut yaml_node_t,
    pub top: *mut yaml_node_t,
}
pub type yaml_document_t = yaml_document_s;
pub type yaml_write_handler_t = unsafe extern "C" fn(
    *mut ::core::ffi::c_void,
    *mut ::core::ffi::c_uchar,
    size_t,
) -> ::core::ffi::c_int;
pub type yaml_emitter_state_e = ::core::ffi::c_uint;
pub const YAML_EMIT_END_STATE: yaml_emitter_state_e = 17;
pub const YAML_EMIT_BLOCK_MAPPING_VALUE_STATE: yaml_emitter_state_e = 16;
pub const YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 15;
pub const YAML_EMIT_BLOCK_MAPPING_KEY_STATE: yaml_emitter_state_e = 14;
pub const YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 13;
pub const YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 12;
pub const YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 11;
pub const YAML_EMIT_FLOW_MAPPING_VALUE_STATE: yaml_emitter_state_e = 10;
pub const YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE: yaml_emitter_state_e = 9;
pub const YAML_EMIT_FLOW_MAPPING_KEY_STATE: yaml_emitter_state_e = 8;
pub const YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE: yaml_emitter_state_e = 7;
pub const YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE: yaml_emitter_state_e = 6;
pub const YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE: yaml_emitter_state_e = 5;
pub const YAML_EMIT_DOCUMENT_END_STATE: yaml_emitter_state_e = 4;
pub const YAML_EMIT_DOCUMENT_CONTENT_STATE: yaml_emitter_state_e = 3;
pub const YAML_EMIT_DOCUMENT_START_STATE: yaml_emitter_state_e = 2;
pub const YAML_EMIT_FIRST_DOCUMENT_START_STATE: yaml_emitter_state_e = 1;
pub const YAML_EMIT_STREAM_START_STATE: yaml_emitter_state_e = 0;
pub type yaml_emitter_state_t = yaml_emitter_state_e;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_anchors_s {
    pub references: ::core::ffi::c_int,
    pub anchor: ::core::ffi::c_int,
    pub serialized: ::core::ffi::c_int,
}
pub type yaml_anchors_t = yaml_anchors_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_emitter_s {
    pub error: yaml_error_type_t,
    pub problem: *const ::core::ffi::c_char,
    pub write_handler: Option<yaml_write_handler_t>,
    pub write_handler_data: *mut ::core::ffi::c_void,
    pub output: C2RustUnnamed_25,
    pub buffer: C2RustUnnamed_24,
    pub raw_buffer: C2RustUnnamed_23,
    pub encoding: yaml_encoding_t,
    pub canonical: ::core::ffi::c_int,
    pub best_indent: ::core::ffi::c_int,
    pub best_width: ::core::ffi::c_int,
    pub unicode: ::core::ffi::c_int,
    pub line_break: yaml_break_t,
    pub states: C2RustUnnamed_22,
    pub state: yaml_emitter_state_t,
    pub events: C2RustUnnamed_21,
    pub indents: C2RustUnnamed_20,
    pub tag_directives: C2RustUnnamed_19,
    pub indent: ::core::ffi::c_int,
    pub flow_level: ::core::ffi::c_int,
    pub root_context: ::core::ffi::c_int,
    pub sequence_context: ::core::ffi::c_int,
    pub mapping_context: ::core::ffi::c_int,
    pub simple_key_context: ::core::ffi::c_int,
    pub line: ::core::ffi::c_int,
    pub column: ::core::ffi::c_int,
    pub whitespace: ::core::ffi::c_int,
    pub indention: ::core::ffi::c_int,
    pub open_ended: ::core::ffi::c_int,
    pub anchor_data: C2RustUnnamed_18,
    pub tag_data: C2RustUnnamed_17,
    pub scalar_data: C2RustUnnamed_16,
    pub opened: ::core::ffi::c_int,
    pub closed: ::core::ffi::c_int,
    pub anchors: *mut yaml_anchors_t,
    pub last_anchor_id: ::core::ffi::c_int,
    pub document: *mut yaml_document_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub value: *mut yaml_char_t,
    pub length: size_t,
    pub multiline: ::core::ffi::c_int,
    pub flow_plain_allowed: ::core::ffi::c_int,
    pub block_plain_allowed: ::core::ffi::c_int,
    pub single_quoted_allowed: ::core::ffi::c_int,
    pub block_allowed: ::core::ffi::c_int,
    pub style: yaml_scalar_style_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub handle: *mut yaml_char_t,
    pub handle_length: size_t,
    pub suffix: *mut yaml_char_t,
    pub suffix_length: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub anchor: *mut yaml_char_t,
    pub anchor_length: size_t,
    pub alias: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub start: *mut yaml_tag_directive_t,
    pub end: *mut yaml_tag_directive_t,
    pub top: *mut yaml_tag_directive_t,
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
    pub start: *mut yaml_event_t,
    pub end: *mut yaml_event_t,
    pub head: *mut yaml_event_t,
    pub tail: *mut yaml_event_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub start: *mut yaml_emitter_state_t,
    pub end: *mut yaml_emitter_state_t,
    pub top: *mut yaml_emitter_state_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub start: *mut ::core::ffi::c_uchar,
    pub end: *mut ::core::ffi::c_uchar,
    pub pointer: *mut ::core::ffi::c_uchar,
    pub last: *mut ::core::ffi::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
    pub last: *mut yaml_char_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub string: C2RustUnnamed_26,
    pub file: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub buffer: *mut ::core::ffi::c_uchar,
    pub size: size_t,
    pub size_written: *mut size_t,
}
pub type yaml_emitter_t = yaml_emitter_s;
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
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_open(mut emitter: *mut yaml_emitter_t) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    '_c2rust_label: {
        if !emitter.is_null() {
        } else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_uint,
                b"int yaml_emitter_open(yaml_emitter_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*emitter).opened == 0 {
        } else {
            __assert_fail(
                b"!emitter->opened\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                68 as ::core::ffi::c_uint,
                b"int yaml_emitter_open(yaml_emitter_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_STREAM_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.stream_start.encoding = YAML_ANY_ENCODING;
    if yaml_emitter_emit(emitter, &raw mut event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).opened = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_close(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    '_c2rust_label: {
        if !emitter.is_null() {
        } else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                91 as ::core::ffi::c_uint,
                b"int yaml_emitter_close(yaml_emitter_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*emitter).opened != 0 {
        } else {
            __assert_fail(
                b"emitter->opened\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                92 as ::core::ffi::c_uint,
                b"int yaml_emitter_close(yaml_emitter_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*emitter).closed != 0 {
        return 1 as ::core::ffi::c_int;
    }
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_STREAM_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &raw mut event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).closed = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_dump(
    mut emitter: *mut yaml_emitter_t,
    mut document: *mut yaml_document_t,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    '_c2rust_label: {
        if !emitter.is_null() {
        } else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                117 as ::core::ffi::c_uint,
                b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !document.is_null() {
        } else {
            __assert_fail(
                b"document\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                118 as ::core::ffi::c_uint,
                b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*emitter).document = document;
    if (*emitter).opened == 0 {
        if yaml_emitter_open(emitter) == 0 {
            current_block = 16368559313306159759;
        } else {
            current_block = 7502529970979898288;
        }
    } else {
        current_block = 7502529970979898288;
    }
    match current_block {
        7502529970979898288 => {
            if (*document).nodes.start == (*document).nodes.top {
                if !(yaml_emitter_close(emitter) == 0) {
                    yaml_emitter_delete_document_and_anchors(emitter);
                    return 1 as ::core::ffi::c_int;
                }
            } else {
                '_c2rust_label_1: {
                    if (*emitter).opened != 0 {
                    } else {
                        __assert_fail(
                            b"emitter->opened\0" as *const u8 as *const ::core::ffi::c_char,
                            b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            132 as ::core::ffi::c_uint,
                            b"int yaml_emitter_dump(yaml_emitter_t *, yaml_document_t *)\0"
                                as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                (*emitter).anchors = yaml_malloc(
                    (::core::mem::size_of::<yaml_anchors_t>() as size_t)
                        .wrapping_mul((*document).nodes.top.offset_from((*document).nodes.start)
                            as ::core::ffi::c_long as size_t),
                ) as *mut yaml_anchors_t;
                if !(*emitter).anchors.is_null() {
                    memset(
                        (*emitter).anchors as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        (::core::mem::size_of::<yaml_anchors_t>() as size_t).wrapping_mul(
                            (*document).nodes.top.offset_from((*document).nodes.start)
                                as ::core::ffi::c_long as size_t,
                        ),
                    );
                    memset(
                        &raw mut event as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        ::core::mem::size_of::<yaml_event_t>() as size_t,
                    );
                    event.type_0 = YAML_DOCUMENT_START_EVENT;
                    event.start_mark = mark;
                    event.end_mark = mark;
                    event.data.document_start.version_directive = (*document).version_directive;
                    event.data.document_start.tag_directives.start =
                        (*document).tag_directives.start;
                    event.data.document_start.tag_directives.end = (*document).tag_directives.end;
                    event.data.document_start.implicit = (*document).start_implicit;
                    if !(yaml_emitter_emit(emitter, &raw mut event) == 0) {
                        yaml_emitter_anchor_node(emitter, 1 as ::core::ffi::c_int);
                        if !(yaml_emitter_dump_node(emitter, 1 as ::core::ffi::c_int) == 0) {
                            memset(
                                &raw mut event as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                ::core::mem::size_of::<yaml_event_t>() as size_t,
                            );
                            event.type_0 = YAML_DOCUMENT_END_EVENT;
                            event.start_mark = mark;
                            event.end_mark = mark;
                            event.data.document_end.implicit = (*document).end_implicit;
                            if !(yaml_emitter_emit(emitter, &raw mut event) == 0) {
                                yaml_emitter_delete_document_and_anchors(emitter);
                                return 1 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    yaml_emitter_delete_document_and_anchors(emitter);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_delete_document_and_anchors(mut emitter: *mut yaml_emitter_t) {
    let mut index: ::core::ffi::c_int = 0;
    if (*emitter).anchors.is_null() {
        yaml_document_delete((*emitter).document);
        (*emitter).document = ::core::ptr::null_mut::<yaml_document_t>();
        return;
    }
    index = 0 as ::core::ffi::c_int;
    while (*(*emitter).document).nodes.start.offset(index as isize)
        < (*(*emitter).document).nodes.top
    {
        let mut node: yaml_node_t = *(*(*emitter).document).nodes.start.offset(index as isize);
        if (*(*emitter).anchors.offset(index as isize)).serialized == 0 {
            yaml_free(node.tag as *mut ::core::ffi::c_void);
            if node.type_0 as ::core::ffi::c_uint
                == YAML_SCALAR_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                yaml_free(node.data.scalar.value as *mut ::core::ffi::c_void);
            }
        }
        if node.type_0 as ::core::ffi::c_uint
            == YAML_SEQUENCE_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            yaml_free(node.data.sequence.items.start as *mut ::core::ffi::c_void);
            node.data.sequence.items.end = ::core::ptr::null_mut::<yaml_node_item_t>();
            node.data.sequence.items.top = node.data.sequence.items.end;
            node.data.sequence.items.start = node.data.sequence.items.top;
        }
        if node.type_0 as ::core::ffi::c_uint
            == YAML_MAPPING_NODE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            yaml_free(node.data.mapping.pairs.start as *mut ::core::ffi::c_void);
            node.data.mapping.pairs.end = ::core::ptr::null_mut::<yaml_node_pair_t>();
            node.data.mapping.pairs.top = node.data.mapping.pairs.end;
            node.data.mapping.pairs.start = node.data.mapping.pairs.top;
        }
        index += 1;
    }
    yaml_free((*(*emitter).document).nodes.start as *mut ::core::ffi::c_void);
    (*(*emitter).document).nodes.end = ::core::ptr::null_mut::<yaml_node_t>();
    (*(*emitter).document).nodes.top = (*(*emitter).document).nodes.end;
    (*(*emitter).document).nodes.start = (*(*emitter).document).nodes.top;
    yaml_free((*emitter).anchors as *mut ::core::ffi::c_void);
    (*emitter).anchors = ::core::ptr::null_mut::<yaml_anchors_t>();
    (*emitter).last_anchor_id = 0 as ::core::ffi::c_int;
    (*emitter).document = ::core::ptr::null_mut::<yaml_document_t>();
}
unsafe extern "C" fn yaml_emitter_anchor_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: ::core::ffi::c_int,
) {
    let mut node: *mut yaml_node_t = (*(*emitter).document)
        .nodes
        .start
        .offset(index as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut item: *mut yaml_node_item_t = ::core::ptr::null_mut::<yaml_node_item_t>();
    let mut pair: *mut yaml_node_pair_t = ::core::ptr::null_mut::<yaml_node_pair_t>();
    let ref mut fresh0 = (*(*emitter)
        .anchors
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .references;
    *fresh0 += 1;
    if (*(*emitter)
        .anchors
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .references
        == 1 as ::core::ffi::c_int
    {
        match (*node).type_0 as ::core::ffi::c_uint {
            2 => {
                item = (*node).data.sequence.items.start;
                while item < (*node).data.sequence.items.top {
                    yaml_emitter_anchor_node(emitter, *item);
                    item = item.offset(1);
                }
            }
            3 => {
                pair = (*node).data.mapping.pairs.start;
                while pair < (*node).data.mapping.pairs.top {
                    yaml_emitter_anchor_node(emitter, (*pair).key);
                    yaml_emitter_anchor_node(emitter, (*pair).value);
                    pair = pair.offset(1);
                }
            }
            _ => {}
        }
    } else if (*(*emitter)
        .anchors
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .references
        == 2 as ::core::ffi::c_int
    {
        (*emitter).last_anchor_id += 1;
        (*(*emitter)
            .anchors
            .offset((index - 1 as ::core::ffi::c_int) as isize))
        .anchor = (*emitter).last_anchor_id;
    }
}
pub const ANCHOR_TEMPLATE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"id%03d\0") };
unsafe extern "C" fn yaml_emitter_generate_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut anchor_id: ::core::ffi::c_int,
) -> *mut yaml_char_t {
    let mut anchor: *mut yaml_char_t = yaml_malloc(16 as size_t) as *mut yaml_char_t;
    if anchor.is_null() {
        return ::core::ptr::null_mut::<yaml_char_t>();
    }
    sprintf(
        anchor as *mut ::core::ffi::c_char,
        ANCHOR_TEMPLATE.as_ptr(),
        anchor_id,
    );
    return anchor;
}
unsafe extern "C" fn yaml_emitter_dump_node(
    mut emitter: *mut yaml_emitter_t,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut node: *mut yaml_node_t = (*(*emitter).document)
        .nodes
        .start
        .offset(index as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut anchor_id: ::core::ffi::c_int = (*(*emitter)
        .anchors
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .anchor;
    let mut anchor: *mut yaml_char_t = ::core::ptr::null_mut::<yaml_char_t>();
    if anchor_id != 0 {
        anchor = yaml_emitter_generate_anchor(emitter, anchor_id);
        if anchor.is_null() {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*(*emitter)
        .anchors
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .serialized
        != 0
    {
        return yaml_emitter_dump_alias(emitter, anchor);
    }
    (*(*emitter)
        .anchors
        .offset((index - 1 as ::core::ffi::c_int) as isize))
    .serialized = 1 as ::core::ffi::c_int;
    match (*node).type_0 as ::core::ffi::c_uint {
        1 => return yaml_emitter_dump_scalar(emitter, node, anchor),
        2 => return yaml_emitter_dump_sequence(emitter, node, anchor),
        3 => return yaml_emitter_dump_mapping(emitter, node, anchor),
        _ => {
            '_c2rust_label: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"/home/leochanj/Desktop/libyaml/src/dumper.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    289 as ::core::ffi::c_uint,
                    b"int yaml_emitter_dump_node(yaml_emitter_t *, int)\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            };
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_dump_alias(
    mut emitter: *mut yaml_emitter_t,
    mut anchor: *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_ALIAS_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.alias.anchor = anchor;
    return yaml_emitter_emit(emitter, &raw mut event);
}
unsafe extern "C" fn yaml_emitter_dump_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    let mut plain_implicit: ::core::ffi::c_int = (strcmp(
        (*node).tag as *mut ::core::ffi::c_char,
        YAML_DEFAULT_SCALAR_TAG.as_ptr(),
    ) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut quoted_implicit: ::core::ffi::c_int = (strcmp(
        (*node).tag as *mut ::core::ffi::c_char,
        YAML_DEFAULT_SCALAR_TAG.as_ptr(),
    ) == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_SCALAR_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.scalar.anchor = anchor;
    event.data.scalar.tag = (*node).tag;
    event.data.scalar.value = (*node).data.scalar.value;
    event.data.scalar.length = (*node).data.scalar.length;
    event.data.scalar.plain_implicit = plain_implicit;
    event.data.scalar.quoted_implicit = quoted_implicit;
    event.data.scalar.style = (*node).data.scalar.style;
    return yaml_emitter_emit(emitter, &raw mut event);
}
unsafe extern "C" fn yaml_emitter_dump_sequence(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    let mut implicit: ::core::ffi::c_int = (strcmp(
        (*node).tag as *mut ::core::ffi::c_char,
        YAML_DEFAULT_SEQUENCE_TAG.as_ptr(),
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut item: *mut yaml_node_item_t = ::core::ptr::null_mut::<yaml_node_item_t>();
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_SEQUENCE_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.sequence_start.anchor = anchor;
    event.data.sequence_start.tag = (*node).tag;
    event.data.sequence_start.implicit = implicit;
    event.data.sequence_start.style = (*node).data.sequence.style;
    if yaml_emitter_emit(emitter, &raw mut event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    item = (*node).data.sequence.items.start;
    while item < (*node).data.sequence.items.top {
        if yaml_emitter_dump_node(emitter, *item) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        item = item.offset(1);
    }
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_SEQUENCE_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &raw mut event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_dump_mapping(
    mut emitter: *mut yaml_emitter_t,
    mut node: *mut yaml_node_t,
    mut anchor: *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut event: yaml_event_t = yaml_event_t {
        type_0: YAML_NO_EVENT,
        data: C2RustUnnamed {
            stream_start: C2RustUnnamed_7 {
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
    let mut mark: yaml_mark_t = yaml_mark_s {
        index: 0 as size_t,
        line: 0 as size_t,
        column: 0 as size_t,
    };
    let mut implicit: ::core::ffi::c_int = (strcmp(
        (*node).tag as *mut ::core::ffi::c_char,
        YAML_DEFAULT_MAPPING_TAG.as_ptr(),
    ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut pair: *mut yaml_node_pair_t = ::core::ptr::null_mut::<yaml_node_pair_t>();
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_MAPPING_START_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    event.data.mapping_start.anchor = anchor;
    event.data.mapping_start.tag = (*node).tag;
    event.data.mapping_start.implicit = implicit;
    event.data.mapping_start.style = (*node).data.mapping.style;
    if yaml_emitter_emit(emitter, &raw mut event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    pair = (*node).data.mapping.pairs.start;
    while pair < (*node).data.mapping.pairs.top {
        if yaml_emitter_dump_node(emitter, (*pair).key) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_dump_node(emitter, (*pair).value) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        pair = pair.offset(1);
    }
    memset(
        &raw mut event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<yaml_event_t>() as size_t,
    );
    event.type_0 = YAML_MAPPING_END_EVENT;
    event.start_mark = mark;
    event.end_mark = mark;
    if yaml_emitter_emit(emitter, &raw mut event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
