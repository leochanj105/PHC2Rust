pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;
pub type _IO_wide_data = libc::c_void;

extern "C" {
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
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 41] = unsafe {
    ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
        *b"int yaml_emitter_flush(yaml_emitter_t *)\0",
    )
};
unsafe extern "C" fn yaml_emitter_set_writer_error(
    mut emitter: *mut yaml_emitter_t,
    mut problem: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    (*emitter).error = YAML_WRITER_ERROR;
    (*emitter).problem = problem;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_flush(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    let mut low: ::core::ffi::c_int = 0;
    let mut high: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !emitter.is_null() {
        } else {
            __assert_fail(
                b"emitter\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/writer.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                36 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if (*emitter).write_handler.is_some() {
        } else {
            __assert_fail(
                b"emitter->write_handler\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/writer.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                37 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_1: {
        if (*emitter).encoding as u64 != 0 {
        } else {
            __assert_fail(
                b"emitter->encoding\0" as *const u8 as *const ::core::ffi::c_char,
                b"/home/leochanj/Desktop/libyaml/src/writer.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                38 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    (*emitter).buffer.last = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.start;
    if (*emitter).buffer.start == (*emitter).buffer.last {
        return 1 as ::core::ffi::c_int;
    }
    if (*emitter).encoding as ::core::ffi::c_uint
        == YAML_UTF8_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*emitter).write_handler.expect("non-null function pointer")(
            (*emitter).write_handler_data,
            (*emitter).buffer.start as *mut ::core::ffi::c_uchar,
            (*emitter).buffer.last.offset_from((*emitter).buffer.start) as ::core::ffi::c_long
                as size_t,
        ) != 0
        {
            (*emitter).buffer.last = (*emitter).buffer.start;
            (*emitter).buffer.pointer = (*emitter).buffer.start;
            return 1 as ::core::ffi::c_int;
        } else {
            return yaml_emitter_set_writer_error(
                emitter,
                b"write error\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    low = if (*emitter).encoding as ::core::ffi::c_uint
        == YAML_UTF16LE_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    high = if (*emitter).encoding as ::core::ffi::c_uint
        == YAML_UTF16LE_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    while (*emitter).buffer.pointer != (*emitter).buffer.last {
        let mut octet: ::core::ffi::c_uchar = 0;
        let mut width: ::core::ffi::c_uint = 0;
        let mut value: ::core::ffi::c_uint = 0;
        let mut k: size_t = 0;
        octet = *(*emitter)
            .buffer
            .pointer
            .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar;
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
            octet = *(*emitter).buffer.pointer.offset(k as isize) as ::core::ffi::c_uchar;
            value = (value << 6 as ::core::ffi::c_int).wrapping_add(
                (octet as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
            k = k.wrapping_add(1);
        }
        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(width as isize);
        if value < 0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint {
            *(*emitter).raw_buffer.last.offset(high as isize) =
                (value >> 8 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *(*emitter).raw_buffer.last.offset(low as isize) =
                (value & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
            (*emitter).raw_buffer.last = (*emitter)
                .raw_buffer
                .last
                .offset(2 as ::core::ffi::c_int as isize);
        } else {
            value = value.wrapping_sub(0x10000 as ::core::ffi::c_int as ::core::ffi::c_uint);
            *(*emitter).raw_buffer.last.offset(high as isize) = (0xd8 as ::core::ffi::c_uint)
                .wrapping_add(value >> 18 as ::core::ffi::c_int)
                as ::core::ffi::c_uchar;
            *(*emitter).raw_buffer.last.offset(low as isize) = (value >> 10 as ::core::ffi::c_int
                & 0xff as ::core::ffi::c_uint)
                as ::core::ffi::c_uchar;
            *(*emitter)
                .raw_buffer
                .last
                .offset((high + 2 as ::core::ffi::c_int) as isize) = (0xdc as ::core::ffi::c_uint)
                .wrapping_add(value >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint)
                as ::core::ffi::c_uchar;
            *(*emitter)
                .raw_buffer
                .last
                .offset((low + 2 as ::core::ffi::c_int) as isize) =
                (value & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
            (*emitter).raw_buffer.last = (*emitter)
                .raw_buffer
                .last
                .offset(4 as ::core::ffi::c_int as isize);
        }
    }
    if (*emitter).write_handler.expect("non-null function pointer")(
        (*emitter).write_handler_data,
        (*emitter).raw_buffer.start,
        (*emitter)
            .raw_buffer
            .last
            .offset_from((*emitter).raw_buffer.start) as ::core::ffi::c_long as size_t,
    ) != 0
    {
        (*emitter).buffer.last = (*emitter).buffer.start;
        (*emitter).buffer.pointer = (*emitter).buffer.start;
        (*emitter).raw_buffer.last = (*emitter).raw_buffer.start;
        (*emitter).raw_buffer.pointer = (*emitter).raw_buffer.start;
        return 1 as ::core::ffi::c_int;
    } else {
        return yaml_emitter_set_writer_error(
            emitter,
            b"write error\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
