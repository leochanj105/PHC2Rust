pub type _IO_codecvt = libc::c_void;
pub type _IO_marker = libc::c_void;
pub type _IO_wide_data = libc::c_void;

extern "C" {
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn yaml_event_delete(event: *mut yaml_event_t);
    fn yaml_emitter_flush(emitter: *mut yaml_emitter_t) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn yaml_free(ptr: *mut ::core::ffi::c_void);
    fn yaml_strdup(_: *const yaml_char_t) -> *mut yaml_char_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yaml_string_t {
    pub start: *mut yaml_char_t,
    pub end: *mut yaml_char_t,
    pub pointer: *mut yaml_char_t,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
unsafe extern "C" fn yaml_emitter_set_emitter_error(
    mut emitter: *mut yaml_emitter_t,
    mut problem: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    (*emitter).error = YAML_EMITTER_ERROR;
    (*emitter).problem = problem;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn yaml_emitter_emit(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if if (*emitter).events.tail != (*emitter).events.end
        || yaml_queue_extend(
            &raw mut (*emitter).events.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).events.head as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).events.tail as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).events.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh0 = (*emitter).events.tail;
        (*emitter).events.tail = (*emitter).events.tail.offset(1);
        *fresh0 = *event;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        yaml_event_delete(event);
        return 0 as ::core::ffi::c_int;
    }
    while yaml_emitter_need_more_events(emitter) == 0 {
        if yaml_emitter_analyze_event(emitter, (*emitter).events.head) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_state_machine(emitter, (*emitter).events.head) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        let fresh1 = (*emitter).events.head;
        (*emitter).events.head = (*emitter).events.head.offset(1);
        yaml_event_delete(fresh1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_need_more_events(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    let mut level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut accumulate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut event: *mut yaml_event_t = ::core::ptr::null_mut::<yaml_event_t>();
    if (*emitter).events.head == (*emitter).events.tail {
        return 1 as ::core::ffi::c_int;
    }
    match (*(*emitter).events.head).type_0 as ::core::ffi::c_uint {
        3 => {
            accumulate = 1 as ::core::ffi::c_int;
        }
        7 => {
            accumulate = 2 as ::core::ffi::c_int;
        }
        9 => {
            accumulate = 3 as ::core::ffi::c_int;
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    if (*emitter).events.tail.offset_from((*emitter).events.head) as ::core::ffi::c_long
        > accumulate as ::core::ffi::c_long
    {
        return 0 as ::core::ffi::c_int;
    }
    event = (*emitter).events.head;
    while event != (*emitter).events.tail {
        match (*event).type_0 as ::core::ffi::c_uint {
            1 | 3 | 7 | 9 => {
                level += 1 as ::core::ffi::c_int;
            }
            2 | 4 | 8 | 10 => {
                level -= 1 as ::core::ffi::c_int;
            }
            _ => {}
        }
        if level == 0 {
            return 0 as ::core::ffi::c_int;
        }
        event = event.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_append_tag_directive(
    mut emitter: *mut yaml_emitter_t,
    mut value: yaml_tag_directive_t,
    mut allow_duplicates: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut tag_directive: *mut yaml_tag_directive_t =
        ::core::ptr::null_mut::<yaml_tag_directive_t>();
    let mut copy: yaml_tag_directive_t = yaml_tag_directive_s {
        handle: ::core::ptr::null_mut::<yaml_char_t>(),
        prefix: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        if strcmp(
            value.handle as *mut ::core::ffi::c_char,
            (*tag_directive).handle as *mut ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if allow_duplicates != 0 {
                return 1 as ::core::ffi::c_int;
            }
            return yaml_emitter_set_emitter_error(
                emitter,
                b"duplicate %TAG directive\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        tag_directive = tag_directive.offset(1);
    }
    copy.handle = yaml_strdup(value.handle);
    copy.prefix = yaml_strdup(value.prefix);
    if copy.handle.is_null() || copy.prefix.is_null() {
        (*emitter).error = YAML_MEMORY_ERROR;
    } else if !(if (*emitter).tag_directives.top != (*emitter).tag_directives.end
        || yaml_stack_extend(
            &raw mut (*emitter).tag_directives.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).tag_directives.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).tag_directives.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh405 = (*emitter).tag_directives.top;
        (*emitter).tag_directives.top = (*emitter).tag_directives.top.offset(1);
        *fresh405 = copy;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0)
    {
        return 1 as ::core::ffi::c_int;
    }
    yaml_free(copy.handle as *mut ::core::ffi::c_void);
    yaml_free(copy.prefix as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_increase_indent(
    mut emitter: *mut yaml_emitter_t,
    mut flow: ::core::ffi::c_int,
    mut indentless: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if if (*emitter).indents.top != (*emitter).indents.end
        || yaml_stack_extend(
            &raw mut (*emitter).indents.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).indents.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).indents.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh395 = (*emitter).indents.top;
        (*emitter).indents.top = (*emitter).indents.top.offset(1);
        *fresh395 = (*emitter).indent;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*emitter).indent < 0 as ::core::ffi::c_int {
        (*emitter).indent = if flow != 0 {
            (*emitter).best_indent
        } else {
            0 as ::core::ffi::c_int
        };
    } else if indentless == 0 {
        (*emitter).indent += (*emitter).best_indent;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_state_machine(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    match (*emitter).state as ::core::ffi::c_uint {
        0 => return yaml_emitter_emit_stream_start(emitter, event),
        1 => {
            return yaml_emitter_emit_document_start(emitter, event, 1 as ::core::ffi::c_int);
        }
        2 => {
            return yaml_emitter_emit_document_start(emitter, event, 0 as ::core::ffi::c_int);
        }
        3 => return yaml_emitter_emit_document_content(emitter, event),
        4 => return yaml_emitter_emit_document_end(emitter, event),
        5 => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 1 as ::core::ffi::c_int);
        }
        6 => {
            return yaml_emitter_emit_flow_sequence_item(emitter, event, 0 as ::core::ffi::c_int);
        }
        7 => {
            return yaml_emitter_emit_flow_mapping_key(emitter, event, 1 as ::core::ffi::c_int);
        }
        8 => {
            return yaml_emitter_emit_flow_mapping_key(emitter, event, 0 as ::core::ffi::c_int);
        }
        9 => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 1 as ::core::ffi::c_int);
        }
        10 => {
            return yaml_emitter_emit_flow_mapping_value(emitter, event, 0 as ::core::ffi::c_int);
        }
        11 => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 1 as ::core::ffi::c_int);
        }
        12 => {
            return yaml_emitter_emit_block_sequence_item(emitter, event, 0 as ::core::ffi::c_int);
        }
        13 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 1 as ::core::ffi::c_int);
        }
        14 => {
            return yaml_emitter_emit_block_mapping_key(emitter, event, 0 as ::core::ffi::c_int);
        }
        15 => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 1 as ::core::ffi::c_int);
        }
        16 => {
            return yaml_emitter_emit_block_mapping_value(emitter, event, 0 as ::core::ffi::c_int);
        }
        17 => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected nothing after STREAM-END\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        _ => {
            '_c2rust_label: {};
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_stream_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    (*emitter).open_ended = 0 as ::core::ffi::c_int;
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_STREAM_START_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = (*event).data.stream_start.encoding;
        }
        if (*emitter).encoding as u64 == 0 {
            (*emitter).encoding = YAML_UTF8_ENCODING;
        }
        if (*emitter).best_indent < 2 as ::core::ffi::c_int
            || (*emitter).best_indent > 9 as ::core::ffi::c_int
        {
            (*emitter).best_indent = 2 as ::core::ffi::c_int;
        }
        if (*emitter).best_width >= 0 as ::core::ffi::c_int
            && (*emitter).best_width <= (*emitter).best_indent * 2 as ::core::ffi::c_int
        {
            (*emitter).best_width = 80 as ::core::ffi::c_int;
        }
        if (*emitter).best_width < 0 as ::core::ffi::c_int {
            (*emitter).best_width = INT_MAX;
        }
        if (*emitter).line_break as u64 == 0 {
            (*emitter).line_break = YAML_LN_BREAK;
        }
        (*emitter).indent = -(1 as ::core::ffi::c_int);
        (*emitter).line = 0 as ::core::ffi::c_int;
        (*emitter).column = 0 as ::core::ffi::c_int;
        (*emitter).whitespace = 1 as ::core::ffi::c_int;
        (*emitter).indention = 1 as ::core::ffi::c_int;
        if (*emitter).encoding as ::core::ffi::c_uint
            != YAML_UTF8_ENCODING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if yaml_emitter_write_bom(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        (*emitter).state = YAML_EMIT_FIRST_DOCUMENT_START_STATE;
        return 1 as ::core::ffi::c_int;
    }
    return yaml_emitter_set_emitter_error(
        emitter,
        b"expected STREAM-START\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn yaml_emitter_emit_document_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_DOCUMENT_START_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut default_tag_directives: [yaml_tag_directive_t; 3] = [
            yaml_tag_directive_s {
                handle: b"!\0" as *const u8 as *const ::core::ffi::c_char as *mut yaml_char_t,
                prefix: b"!\0" as *const u8 as *const ::core::ffi::c_char as *mut yaml_char_t,
            },
            yaml_tag_directive_s {
                handle: b"!!\0" as *const u8 as *const ::core::ffi::c_char as *mut yaml_char_t,
                prefix: b"tag:yaml.org,2002:\0" as *const u8 as *const ::core::ffi::c_char
                    as *mut yaml_char_t,
            },
            yaml_tag_directive_s {
                handle: ::core::ptr::null_mut::<yaml_char_t>(),
                prefix: ::core::ptr::null_mut::<yaml_char_t>(),
            },
        ];
        let mut tag_directive: *mut yaml_tag_directive_t =
            ::core::ptr::null_mut::<yaml_tag_directive_t>();
        let mut implicit: ::core::ffi::c_int = 0;
        if !(*event).data.document_start.version_directive.is_null() {
            if yaml_emitter_analyze_version_directive(
                emitter,
                *(*event).data.document_start.version_directive,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
        tag_directive = (*event).data.document_start.tag_directives.start;
        while tag_directive != (*event).data.document_start.tag_directives.end {
            if yaml_emitter_analyze_tag_directive(emitter, *tag_directive) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 0 as ::core::ffi::c_int)
                == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            tag_directive = tag_directive.offset(1);
        }
        tag_directive = &raw mut default_tag_directives as *mut yaml_tag_directive_t;
        while !(*tag_directive).handle.is_null() {
            if yaml_emitter_append_tag_directive(emitter, *tag_directive, 1 as ::core::ffi::c_int)
                == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            tag_directive = tag_directive.offset(1);
        }
        implicit = (*event).data.document_start.implicit;
        if first == 0 || (*emitter).canonical != 0 {
            implicit = 0 as ::core::ffi::c_int;
        }
        if (!(*event).data.document_start.version_directive.is_null()
            || (*event).data.document_start.tag_directives.start
                != (*event).data.document_start.tag_directives.end)
            && (*emitter).open_ended != 0
        {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        (*emitter).open_ended = 0 as ::core::ffi::c_int;
        if !(*event).data.document_start.version_directive.is_null() {
            implicit = 0 as ::core::ffi::c_int;
            if yaml_emitter_write_indicator(
                emitter,
                b"%YAML\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*(*event).data.document_start.version_directive).minor == 1 as ::core::ffi::c_int {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"1.1\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            } else if yaml_emitter_write_indicator(
                emitter,
                b"1.2\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if (*event).data.document_start.tag_directives.start
            != (*event).data.document_start.tag_directives.end
        {
            implicit = 0 as ::core::ffi::c_int;
            tag_directive = (*event).data.document_start.tag_directives.start;
            while tag_directive != (*event).data.document_start.tag_directives.end {
                if yaml_emitter_write_indicator(
                    emitter,
                    b"%TAG\0" as *const u8 as *const ::core::ffi::c_char,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                if yaml_emitter_write_tag_handle(
                    emitter,
                    (*tag_directive).handle,
                    strlen((*tag_directive).handle as *mut ::core::ffi::c_char),
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                if yaml_emitter_write_tag_content(
                    emitter,
                    (*tag_directive).prefix,
                    strlen((*tag_directive).prefix as *mut ::core::ffi::c_char),
                    1 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                tag_directive = tag_directive.offset(1);
            }
        }
        if yaml_emitter_check_empty_document(emitter) != 0 {
            implicit = 0 as ::core::ffi::c_int;
        }
        if implicit == 0 {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            if yaml_emitter_write_indicator(
                emitter,
                b"---\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if (*emitter).canonical != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
        (*emitter).state = YAML_EMIT_DOCUMENT_CONTENT_STATE;
        (*emitter).open_ended = 0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    } else if (*event).type_0 as ::core::ffi::c_uint
        == YAML_STREAM_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*emitter).open_ended == 2 as ::core::ffi::c_int {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).open_ended = 0 as ::core::ffi::c_int;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*emitter).state = YAML_EMIT_END_STATE;
        return 1 as ::core::ffi::c_int;
    }
    return yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-START or STREAM-END\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn yaml_emitter_emit_document_content(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh404 = (*emitter).states.top;
        (*emitter).states.top = (*emitter).states.top.offset(1);
        *fresh404 = YAML_EMIT_DOCUMENT_END_STATE;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_document_end(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_DOCUMENT_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*event).data.document_end.implicit == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b"...\0" as *const u8 as *const ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).open_ended = 0 as ::core::ffi::c_int;
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        } else if (*emitter).open_ended == 0 {
            (*emitter).open_ended = 1 as ::core::ffi::c_int;
        }
        if yaml_emitter_flush(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        (*emitter).state = YAML_EMIT_DOCUMENT_START_STATE;
        while !((*emitter).tag_directives.start == (*emitter).tag_directives.top) {
            (*emitter).tag_directives.top = (*emitter).tag_directives.top.offset(-1);
            let mut tag_directive: yaml_tag_directive_t = *(*emitter).tag_directives.top;
            yaml_free(tag_directive.handle as *mut ::core::ffi::c_void);
            yaml_free(tag_directive.prefix as *mut ::core::ffi::c_void);
        }
        return 1 as ::core::ffi::c_int;
    }
    return yaml_emitter_set_emitter_error(
        emitter,
        b"expected DOCUMENT-END\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn yaml_emitter_emit_flow_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"[\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_increase_indent(emitter, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*emitter).flow_level += 1;
    }
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_SEQUENCE_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*emitter).flow_level -= 1;
        (*emitter).indents.top = (*emitter).indents.top.offset(-1);
        (*emitter).indent = *(*emitter).indents.top;
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"]\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*emitter).states.top = (*emitter).states.top.offset(-1);
        (*emitter).state = *(*emitter).states.top;
        return 1 as ::core::ffi::c_int;
    }
    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh403 = (*emitter).states.top;
        (*emitter).states.top = (*emitter).states.top.offset(1);
        *fresh403 = YAML_EMIT_FLOW_SEQUENCE_ITEM_STATE;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_flow_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if first != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b"{\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_increase_indent(emitter, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*emitter).flow_level += 1;
    }
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_MAPPING_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*emitter).flow_level -= 1;
        (*emitter).indents.top = (*emitter).indents.top.offset(-1);
        (*emitter).indent = *(*emitter).indents.top;
        if (*emitter).canonical != 0 && first == 0 {
            if yaml_emitter_write_indicator(
                emitter,
                b",\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b"}\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        (*emitter).states.top = (*emitter).states.top.offset(-1);
        (*emitter).state = *(*emitter).states.top;
        return 1 as ::core::ffi::c_int;
    }
    if first == 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b",\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*emitter).canonical == 0 && yaml_emitter_check_simple_key(emitter) != 0 {
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            let fresh401 = (*emitter).states.top;
            (*emitter).states.top = (*emitter).states.top.offset(1);
            *fresh401 = YAML_EMIT_FLOW_MAPPING_SIMPLE_VALUE_STATE;
            1 as ::core::ffi::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            let fresh402 = (*emitter).states.top;
            (*emitter).states.top = (*emitter).states.top.offset(1);
            *fresh402 = YAML_EMIT_FLOW_MAPPING_VALUE_STATE;
            1 as ::core::ffi::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn yaml_emitter_emit_flow_mapping_value(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut simple: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        if (*emitter).canonical != 0 || (*emitter).column > (*emitter).best_width {
            if yaml_emitter_write_indent(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh400 = (*emitter).states.top;
        (*emitter).states.top = (*emitter).states.top.offset(1);
        *fresh400 = YAML_EMIT_FLOW_MAPPING_KEY_STATE;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_block_sequence_item(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(
            emitter,
            0 as ::core::ffi::c_int,
            ((*emitter).mapping_context != 0 && (*emitter).indention == 0) as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_SEQUENCE_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*emitter).indents.top = (*emitter).indents.top.offset(-1);
        (*emitter).indent = *(*emitter).indents.top;
        (*emitter).states.top = (*emitter).states.top.offset(-1);
        (*emitter).state = *(*emitter).states.top;
        return 1 as ::core::ffi::c_int;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh399 = (*emitter).states.top;
        (*emitter).states.top = (*emitter).states.top.offset(1);
        *fresh399 = YAML_EMIT_BLOCK_SEQUENCE_ITEM_STATE;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_block_mapping_key(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut first: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if first != 0 {
        if yaml_emitter_increase_indent(emitter, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int)
            == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*event).type_0 as ::core::ffi::c_uint
        == YAML_MAPPING_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*emitter).indents.top = (*emitter).indents.top.offset(-1);
        (*emitter).indent = *(*emitter).indents.top;
        (*emitter).states.top = (*emitter).states.top.offset(-1);
        (*emitter).state = *(*emitter).states.top;
        return 1 as ::core::ffi::c_int;
    }
    if yaml_emitter_write_indent(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_check_simple_key(emitter) != 0 {
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            let fresh397 = (*emitter).states.top;
            (*emitter).states.top = (*emitter).states.top.offset(1);
            *fresh397 = YAML_EMIT_BLOCK_MAPPING_SIMPLE_VALUE_STATE;
            1 as ::core::ffi::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"?\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if if (*emitter).states.top != (*emitter).states.end
            || yaml_stack_extend(
                &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
                &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
            ) != 0
        {
            let fresh398 = (*emitter).states.top;
            (*emitter).states.top = (*emitter).states.top.offset(1);
            *fresh398 = YAML_EMIT_BLOCK_MAPPING_VALUE_STATE;
            1 as ::core::ffi::c_int
        } else {
            (*emitter).error = YAML_MEMORY_ERROR;
            0 as ::core::ffi::c_int
        } == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        return yaml_emitter_emit_node(
            emitter,
            event,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn yaml_emitter_emit_block_mapping_value(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut simple: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if simple != 0 {
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    if if (*emitter).states.top != (*emitter).states.end
        || yaml_stack_extend(
            &raw mut (*emitter).states.start as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.top as *mut *mut ::core::ffi::c_void,
            &raw mut (*emitter).states.end as *mut *mut ::core::ffi::c_void,
        ) != 0
    {
        let fresh2 = (*emitter).states.top;
        (*emitter).states.top = (*emitter).states.top.offset(1);
        *fresh2 = YAML_EMIT_BLOCK_MAPPING_KEY_STATE;
        1 as ::core::ffi::c_int
    } else {
        (*emitter).error = YAML_MEMORY_ERROR;
        0 as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return yaml_emitter_emit_node(
        emitter,
        event,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn yaml_emitter_emit_node(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
    mut root: ::core::ffi::c_int,
    mut sequence: ::core::ffi::c_int,
    mut mapping: ::core::ffi::c_int,
    mut simple_key: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*emitter).root_context = root;
    (*emitter).sequence_context = sequence;
    (*emitter).mapping_context = mapping;
    (*emitter).simple_key_context = simple_key;
    match (*event).type_0 as ::core::ffi::c_uint {
        5 => return yaml_emitter_emit_alias(emitter, event),
        6 => return yaml_emitter_emit_scalar(emitter, event),
        7 => return yaml_emitter_emit_sequence_start(emitter, event),
        9 => return yaml_emitter_emit_mapping_start(emitter, event),
        _ => {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"expected SCALAR, SEQUENCE-START, MAPPING-START, or ALIAS\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    };
}
unsafe extern "C" fn yaml_emitter_emit_alias(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*emitter).simple_key_context != 0 {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh396 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh396 = ' ' as i32 as yaml_char_t;
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).states.top = (*emitter).states.top.offset(-1);
    (*emitter).state = *(*emitter).states.top;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if yaml_emitter_select_scalar_style(emitter, event) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_increase_indent(emitter, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_process_scalar(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).indents.top = (*emitter).indents.top.offset(-1);
    (*emitter).indent = *(*emitter).indents.top;
    (*emitter).states.top = (*emitter).states.top.offset(-1);
    (*emitter).state = *(*emitter).states.top;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_sequence_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*emitter).flow_level != 0
        || (*emitter).canonical != 0
        || (*event).data.sequence_start.style as ::core::ffi::c_uint
            == YAML_FLOW_SEQUENCE_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
        || yaml_emitter_check_empty_sequence(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_SEQUENCE_FIRST_ITEM_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_SEQUENCE_FIRST_ITEM_STATE;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_emit_mapping_start(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    if yaml_emitter_process_anchor(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_process_tag(emitter) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*emitter).flow_level != 0
        || (*emitter).canonical != 0
        || (*event).data.mapping_start.style as ::core::ffi::c_uint
            == YAML_FLOW_MAPPING_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
        || yaml_emitter_check_empty_mapping(emitter) != 0
    {
        (*emitter).state = YAML_EMIT_FLOW_MAPPING_FIRST_KEY_STATE;
    } else {
        (*emitter).state = YAML_EMIT_BLOCK_MAPPING_FIRST_KEY_STATE;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_check_empty_document(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_check_empty_sequence(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    if ((*emitter).events.tail.offset_from((*emitter).events.head) as ::core::ffi::c_long)
        < 2 as ::core::ffi::c_long
    {
        return 0 as ::core::ffi::c_int;
    }
    return ((*(*emitter)
        .events
        .head
        .offset(0 as ::core::ffi::c_int as isize))
    .type_0 as ::core::ffi::c_uint
        == YAML_SEQUENCE_START_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*emitter)
            .events
            .head
            .offset(1 as ::core::ffi::c_int as isize))
        .type_0 as ::core::ffi::c_uint
            == YAML_SEQUENCE_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_check_empty_mapping(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    if ((*emitter).events.tail.offset_from((*emitter).events.head) as ::core::ffi::c_long)
        < 2 as ::core::ffi::c_long
    {
        return 0 as ::core::ffi::c_int;
    }
    return ((*(*emitter)
        .events
        .head
        .offset(0 as ::core::ffi::c_int as isize))
    .type_0 as ::core::ffi::c_uint
        == YAML_MAPPING_START_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*emitter)
            .events
            .head
            .offset(1 as ::core::ffi::c_int as isize))
        .type_0 as ::core::ffi::c_uint
            == YAML_MAPPING_END_EVENT as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_check_simple_key(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    let mut event: *mut yaml_event_t = (*emitter).events.head;
    let mut length: size_t = 0 as size_t;
    match (*event).type_0 as ::core::ffi::c_uint {
        5 => {
            length = (length as ::core::ffi::c_ulong)
                .wrapping_add((*emitter).anchor_data.anchor_length as ::core::ffi::c_ulong)
                as size_t as size_t;
        }
        6 => {
            if (*emitter).scalar_data.multiline != 0 {
                return 0 as ::core::ffi::c_int;
            }
            length = (length as ::core::ffi::c_ulong).wrapping_add(
                (*emitter)
                    .anchor_data
                    .anchor_length
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length)
                    .wrapping_add((*emitter).scalar_data.length)
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        }
        7 => {
            if yaml_emitter_check_empty_sequence(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            length = (length as ::core::ffi::c_ulong).wrapping_add(
                (*emitter)
                    .anchor_data
                    .anchor_length
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length)
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        }
        9 => {
            if yaml_emitter_check_empty_mapping(emitter) == 0 {
                return 0 as ::core::ffi::c_int;
            }
            length = (length as ::core::ffi::c_ulong).wrapping_add(
                (*emitter)
                    .anchor_data
                    .anchor_length
                    .wrapping_add((*emitter).tag_data.handle_length)
                    .wrapping_add((*emitter).tag_data.suffix_length)
                    as ::core::ffi::c_ulong,
            ) as size_t as size_t;
        }
        _ => return 0 as ::core::ffi::c_int,
    }
    if length > 128 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_select_scalar_style(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    let mut style: yaml_scalar_style_t = (*event).data.scalar.style;
    let mut no_tag: ::core::ffi::c_int = ((*emitter).tag_data.handle.is_null()
        && (*emitter).tag_data.suffix.is_null())
        as ::core::ffi::c_int;
    if no_tag != 0
        && (*event).data.scalar.plain_implicit == 0
        && (*event).data.scalar.quoted_implicit == 0
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"neither tag nor implicit flags are specified\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    if style as ::core::ffi::c_uint
        == YAML_ANY_SCALAR_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        style = YAML_PLAIN_SCALAR_STYLE;
    }
    if (*emitter).canonical != 0 {
        style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
    }
    if (*emitter).simple_key_context != 0 && (*emitter).scalar_data.multiline != 0 {
        style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
    }
    if style as ::core::ffi::c_uint
        == YAML_PLAIN_SCALAR_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*emitter).flow_level != 0 && (*emitter).scalar_data.flow_plain_allowed == 0
            || (*emitter).flow_level == 0 && (*emitter).scalar_data.block_plain_allowed == 0
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if (*emitter).scalar_data.length == 0
            && ((*emitter).flow_level != 0 || (*emitter).simple_key_context != 0)
        {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
        if no_tag != 0 && (*event).data.scalar.plain_implicit == 0 {
            style = YAML_SINGLE_QUOTED_SCALAR_STYLE;
        }
    }
    if style as ::core::ffi::c_uint
        == YAML_SINGLE_QUOTED_SCALAR_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*emitter).scalar_data.single_quoted_allowed == 0 {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }
    if style as ::core::ffi::c_uint
        == YAML_LITERAL_SCALAR_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
        || style as ::core::ffi::c_uint
            == YAML_FOLDED_SCALAR_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*emitter).scalar_data.block_allowed == 0
            || (*emitter).flow_level != 0
            || (*emitter).simple_key_context != 0
        {
            style = YAML_DOUBLE_QUOTED_SCALAR_STYLE;
        }
    }
    if no_tag != 0
        && (*event).data.scalar.quoted_implicit == 0
        && style as ::core::ffi::c_uint
            != YAML_PLAIN_SCALAR_STYLE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*emitter).tag_data.handle =
            b"!\0" as *const u8 as *const ::core::ffi::c_char as *mut yaml_char_t;
        (*emitter).tag_data.handle_length = 1 as size_t;
    }
    (*emitter).scalar_data.style = style;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_process_anchor(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    if (*emitter).anchor_data.anchor.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if yaml_emitter_write_indicator(
        emitter,
        if (*emitter).anchor_data.alias != 0 {
            b"*\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"&\0" as *const u8 as *const ::core::ffi::c_char
        },
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return yaml_emitter_write_anchor(
        emitter,
        (*emitter).anchor_data.anchor,
        (*emitter).anchor_data.anchor_length,
    );
}
unsafe extern "C" fn yaml_emitter_process_tag(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    if (*emitter).tag_data.handle.is_null() && (*emitter).tag_data.suffix.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    if !(*emitter).tag_data.handle.is_null() {
        if yaml_emitter_write_tag_handle(
            emitter,
            (*emitter).tag_data.handle,
            (*emitter).tag_data.handle_length,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if !(*emitter).tag_data.suffix.is_null() {
            if yaml_emitter_write_tag_content(
                emitter,
                (*emitter).tag_data.suffix,
                (*emitter).tag_data.suffix_length,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
        }
    } else {
        if yaml_emitter_write_indicator(
            emitter,
            b"!<\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_write_tag_content(
            emitter,
            (*emitter).tag_data.suffix,
            (*emitter).tag_data.suffix_length,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if yaml_emitter_write_indicator(
            emitter,
            b">\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_process_scalar(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    match (*emitter).scalar_data.style as ::core::ffi::c_uint {
        1 => {
            return yaml_emitter_write_plain_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                ((*emitter).simple_key_context == 0) as ::core::ffi::c_int,
            );
        }
        2 => {
            return yaml_emitter_write_single_quoted_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                ((*emitter).simple_key_context == 0) as ::core::ffi::c_int,
            );
        }
        3 => {
            return yaml_emitter_write_double_quoted_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
                ((*emitter).simple_key_context == 0) as ::core::ffi::c_int,
            );
        }
        4 => {
            return yaml_emitter_write_literal_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
            );
        }
        5 => {
            return yaml_emitter_write_folded_scalar(
                emitter,
                (*emitter).scalar_data.value,
                (*emitter).scalar_data.length,
            );
        }
        _ => {
            '_c2rust_label: {};
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_version_directive(
    mut emitter: *mut yaml_emitter_t,
    mut version_directive: yaml_version_directive_t,
) -> ::core::ffi::c_int {
    if version_directive.major != 1 as ::core::ffi::c_int
        || version_directive.minor != 1 as ::core::ffi::c_int
            && version_directive.minor != 2 as ::core::ffi::c_int
    {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"incompatible %YAML directive\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_tag_directive(
    mut emitter: *mut yaml_emitter_t,
    mut tag_directive: yaml_tag_directive_t,
) -> ::core::ffi::c_int {
    let mut handle: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut prefix: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut handle_length: size_t = 0;
    let mut prefix_length: size_t = 0;
    handle_length = strlen(tag_directive.handle as *mut ::core::ffi::c_char);
    prefix_length = strlen(tag_directive.prefix as *mut ::core::ffi::c_char);
    handle.start = tag_directive.handle;
    handle.end = tag_directive.handle.offset(handle_length as isize);
    handle.pointer = tag_directive.handle;
    prefix.start = tag_directive.prefix;
    prefix.end = tag_directive.prefix.offset(prefix_length as isize);
    prefix.pointer = tag_directive.prefix;
    if handle.start == handle.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must not be empty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *handle.start.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must start with '!'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if *handle.end.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int != '!' as i32 {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag handle must end with '!'\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    handle.pointer = handle.pointer.offset(1);
    while handle.pointer < handle.end.offset(-(1 as ::core::ffi::c_int as isize)) {
        if !(*handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
            && *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
            || *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                && *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
            || *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                && *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
            || *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '_' as i32
            || *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32)
        {
            return yaml_emitter_set_emitter_error(
                emitter,
                b"tag handle must contain alphanumerical characters only\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        handle.pointer = handle.pointer.offset(
            (if *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else if *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else if *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else if *handle.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        );
    }
    if prefix.start == prefix.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag prefix must not be empty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut anchor: *mut yaml_char_t,
    mut alias: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut anchor_length: size_t = 0;
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    anchor_length = strlen(anchor as *mut ::core::ffi::c_char);
    string.start = anchor;
    string.end = anchor.offset(anchor_length as isize);
    string.pointer = anchor;
    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            if alias != 0 {
                b"alias value must not be empty\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"anchor value must not be empty\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    while string.pointer != string.end {
        if !(*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
            && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '_' as i32
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32)
        {
            return yaml_emitter_set_emitter_error(
                emitter,
                if alias != 0 {
                    b"alias value must contain alphanumerical characters only\0" as *const u8
                        as *const ::core::ffi::c_char
                } else {
                    b"anchor value must contain alphanumerical characters only\0" as *const u8
                        as *const ::core::ffi::c_char
                },
            );
        }
        string.pointer = string.pointer.offset(
            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        );
    }
    (*emitter).anchor_data.anchor = string.start;
    (*emitter).anchor_data.anchor_length =
        string.end.offset_from(string.start) as ::core::ffi::c_long as size_t;
    (*emitter).anchor_data.alias = alias;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_tag(
    mut emitter: *mut yaml_emitter_t,
    mut tag: *mut yaml_char_t,
) -> ::core::ffi::c_int {
    let mut tag_length: size_t = 0;
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut tag_directive: *mut yaml_tag_directive_t =
        ::core::ptr::null_mut::<yaml_tag_directive_t>();
    tag_length = strlen(tag as *mut ::core::ffi::c_char);
    string.start = tag;
    string.end = tag.offset(tag_length as isize);
    string.pointer = tag;
    if string.start == string.end {
        return yaml_emitter_set_emitter_error(
            emitter,
            b"tag value must not be empty\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    tag_directive = (*emitter).tag_directives.start;
    while tag_directive != (*emitter).tag_directives.top {
        let mut prefix_length: size_t = strlen((*tag_directive).prefix as *mut ::core::ffi::c_char);
        if prefix_length < string.end.offset_from(string.start) as ::core::ffi::c_long as size_t
            && strncmp(
                (*tag_directive).prefix as *mut ::core::ffi::c_char,
                string.start as *mut ::core::ffi::c_char,
                prefix_length,
            ) == 0 as ::core::ffi::c_int
        {
            (*emitter).tag_data.handle = (*tag_directive).handle;
            (*emitter).tag_data.handle_length =
                strlen((*tag_directive).handle as *mut ::core::ffi::c_char);
            (*emitter).tag_data.suffix = string.start.offset(prefix_length as isize);
            (*emitter).tag_data.suffix_length = (string.end.offset_from(string.start)
                as ::core::ffi::c_long as size_t)
                .wrapping_sub(prefix_length);
            return 1 as ::core::ffi::c_int;
        }
        tag_directive = tag_directive.offset(1);
    }
    (*emitter).tag_data.suffix = string.start;
    (*emitter).tag_data.suffix_length =
        string.end.offset_from(string.start) as ::core::ffi::c_long as size_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut block_indicators: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut flow_indicators: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut line_breaks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut special_characters: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut leading_space: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut leading_break: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut trailing_space: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut trailing_break: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut break_space: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut space_break: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut preceded_by_whitespace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut followed_by_whitespace: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut previous_space: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut previous_break: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    (*emitter).scalar_data.value = value;
    (*emitter).scalar_data.length = length;
    if string.start == string.end {
        (*emitter).scalar_data.multiline = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.flow_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_plain_allowed = 1 as ::core::ffi::c_int;
        (*emitter).scalar_data.single_quoted_allowed = 1 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_allowed = 0 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        && *string.pointer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        && *string.pointer.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32 as yaml_char_t as ::core::ffi::c_int
        || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '.' as i32 as yaml_char_t as ::core::ffi::c_int
            && *string.pointer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32 as yaml_char_t as ::core::ffi::c_int
            && *string.pointer.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32 as yaml_char_t as ::core::ffi::c_int
    {
        block_indicators = 1 as ::core::ffi::c_int;
        flow_indicators = 1 as ::core::ffi::c_int;
    }
    preceded_by_whitespace = 1 as ::core::ffi::c_int;
    followed_by_whitespace = (*string.pointer.offset(
        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else {
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xf0 as ::core::ffi::c_int
                    == 0xe0 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
    ) as ::core::ffi::c_int
        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        || *string.pointer.offset(
            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
        ) as ::core::ffi::c_int
            == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
        || (*string.pointer.offset(
            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xe0 as ::core::ffi::c_int
                    == 0xc0 as ::core::ffi::c_int
                {
                    2 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
        ) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(
                    ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    }) + 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(
                    ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    }) + 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(
                    ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    }) + 2 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(
                    ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    }) + 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(
                    ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    }) + 2 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) as ::core::ffi::c_int
                == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    while string.pointer != string.end {
        if string.start == string.pointer {
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '#' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ',' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '[' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ']' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '{' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '}' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '&' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '*' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '!' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '|' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '>' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '"' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '%' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '@' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '`' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                flow_indicators = 1 as ::core::ffi::c_int;
                block_indicators = 1 as ::core::ffi::c_int;
            }
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ':' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                flow_indicators = 1 as ::core::ffi::c_int;
                if followed_by_whitespace != 0 {
                    block_indicators = 1 as ::core::ffi::c_int;
                }
            }
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32 as yaml_char_t as ::core::ffi::c_int
                && followed_by_whitespace != 0
            {
                flow_indicators = 1 as ::core::ffi::c_int;
                block_indicators = 1 as ::core::ffi::c_int;
            }
        } else {
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ',' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '?' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '[' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ']' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '{' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '}' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                flow_indicators = 1 as ::core::ffi::c_int;
            }
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ':' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                flow_indicators = 1 as ::core::ffi::c_int;
                if followed_by_whitespace != 0 {
                    block_indicators = 1 as ::core::ffi::c_int;
                }
            }
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '#' as i32 as yaml_char_t as ::core::ffi::c_int
                && preceded_by_whitespace != 0
            {
                flow_indicators = 1 as ::core::ffi::c_int;
                block_indicators = 1 as ::core::ffi::c_int;
            }
        }
        if !(*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xa as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 0x20 as ::core::ffi::c_int
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 0x7e as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xc2 as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    >= 0xa0 as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                > 0xc2 as ::core::ffi::c_int
                && (*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    < 0xed as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xed as ::core::ffi::c_int
                && (*string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int)
                    < 0xa0 as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xee as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xef as ::core::ffi::c_int
                && !(*string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == 0xbb as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == 0xbf as ::core::ffi::c_int)
                && !(*string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == 0xbf as ::core::ffi::c_int
                    && (*string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == 0xbe as ::core::ffi::c_int
                        || *string
                            .pointer
                            .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == 0xbf as ::core::ffi::c_int)))
            || !(*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '\u{7f}' as i32 as yaml_char_t as ::core::ffi::c_int)
                && (*emitter).unicode == 0
        {
            special_characters = 1 as ::core::ffi::c_int;
        }
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
        {
            line_breaks = 1 as ::core::ffi::c_int;
        }
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            if string.start == string.pointer {
                leading_space = 1 as ::core::ffi::c_int;
            }
            if string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) == string.end
            {
                trailing_space = 1 as ::core::ffi::c_int;
            }
            if previous_break != 0 {
                break_space = 1 as ::core::ffi::c_int;
            }
            previous_space = 1 as ::core::ffi::c_int;
            previous_break = 0 as ::core::ffi::c_int;
        } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
        {
            if string.start == string.pointer {
                leading_break = 1 as ::core::ffi::c_int;
            }
            if string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) == string.end
            {
                trailing_break = 1 as ::core::ffi::c_int;
            }
            if previous_space != 0 {
                space_break = 1 as ::core::ffi::c_int;
            }
            previous_space = 0 as ::core::ffi::c_int;
            previous_break = 1 as ::core::ffi::c_int;
        } else {
            previous_space = 0 as ::core::ffi::c_int;
            previous_break = 0 as ::core::ffi::c_int;
        }
        preceded_by_whitespace = (*string.pointer.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
            || (*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
            as ::core::ffi::c_int;
        string.pointer = string.pointer.offset(
            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as isize,
        );
        if string.pointer != string.end {
            followed_by_whitespace = (*string.pointer.offset(
                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    1 as ::core::ffi::c_int
                } else {
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            3 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
            ) as ::core::ffi::c_int
                == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                ) as ::core::ffi::c_int
                    == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                || (*string.pointer.offset(
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else {
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                3 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                ) as ::core::ffi::c_int
                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    ) as ::core::ffi::c_int
                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    ) as ::core::ffi::c_int
                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                        && *string.pointer.offset(
                            ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                            }) + 1 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    ) as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *string.pointer.offset(
                            ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                            }) + 1 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *string.pointer.offset(
                            ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                            }) + 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    ) as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *string.pointer.offset(
                            ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                            }) + 1 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *string.pointer.offset(
                            ((if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0x80 as ::core::ffi::c_int
                                == 0 as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xe0 as ::core::ffi::c_int
                                    == 0xc0 as ::core::ffi::c_int
                                {
                                    2 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int
                                        & 0xf0 as ::core::ffi::c_int
                                        == 0xe0 as ::core::ffi::c_int
                                    {
                                        3 as ::core::ffi::c_int
                                    } else {
                                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                            }) + 2 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(
                        (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            & 0x80 as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            1 as ::core::ffi::c_int
                        } else {
                            (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                & 0xe0 as ::core::ffi::c_int
                                == 0xc0 as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    & 0xf0 as ::core::ffi::c_int
                                    == 0xe0 as ::core::ffi::c_int
                                {
                                    3 as ::core::ffi::c_int
                                } else {
                                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
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
                    ) as ::core::ffi::c_int
                        == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                as ::core::ffi::c_int;
        }
    }
    (*emitter).scalar_data.multiline = line_breaks;
    (*emitter).scalar_data.flow_plain_allowed = 1 as ::core::ffi::c_int;
    (*emitter).scalar_data.block_plain_allowed = 1 as ::core::ffi::c_int;
    (*emitter).scalar_data.single_quoted_allowed = 1 as ::core::ffi::c_int;
    (*emitter).scalar_data.block_allowed = 1 as ::core::ffi::c_int;
    if leading_space != 0 || leading_break != 0 || trailing_space != 0 || trailing_break != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as ::core::ffi::c_int;
    }
    if trailing_space != 0 {
        (*emitter).scalar_data.block_allowed = 0 as ::core::ffi::c_int;
    }
    if break_space != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.single_quoted_allowed = 0 as ::core::ffi::c_int;
    }
    if space_break != 0 || special_characters != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.single_quoted_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_allowed = 0 as ::core::ffi::c_int;
    }
    if line_breaks != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as ::core::ffi::c_int;
        (*emitter).scalar_data.block_plain_allowed = 0 as ::core::ffi::c_int;
    }
    if flow_indicators != 0 {
        (*emitter).scalar_data.flow_plain_allowed = 0 as ::core::ffi::c_int;
    }
    if block_indicators != 0 {
        (*emitter).scalar_data.block_plain_allowed = 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_analyze_event(
    mut emitter: *mut yaml_emitter_t,
    mut event: *mut yaml_event_t,
) -> ::core::ffi::c_int {
    (*emitter).anchor_data.anchor = ::core::ptr::null_mut::<yaml_char_t>();
    (*emitter).anchor_data.anchor_length = 0 as size_t;
    (*emitter).tag_data.handle = ::core::ptr::null_mut::<yaml_char_t>();
    (*emitter).tag_data.handle_length = 0 as size_t;
    (*emitter).tag_data.suffix = ::core::ptr::null_mut::<yaml_char_t>();
    (*emitter).tag_data.suffix_length = 0 as size_t;
    (*emitter).scalar_data.value = ::core::ptr::null_mut::<yaml_char_t>();
    (*emitter).scalar_data.length = 0 as size_t;
    match (*event).type_0 as ::core::ffi::c_uint {
        5 => {
            if yaml_emitter_analyze_anchor(
                emitter,
                (*event).data.alias.anchor,
                1 as ::core::ffi::c_int,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        6 => {
            if !(*event).data.scalar.anchor.is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.scalar.anchor,
                    0 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(*event).data.scalar.tag.is_null()
                && ((*emitter).canonical != 0
                    || (*event).data.scalar.plain_implicit == 0
                        && (*event).data.scalar.quoted_implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.scalar.tag) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if yaml_emitter_analyze_scalar(
                emitter,
                (*event).data.scalar.value,
                (*event).data.scalar.length,
            ) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            return 1 as ::core::ffi::c_int;
        }
        7 => {
            if !(*event).data.sequence_start.anchor.is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.sequence_start.anchor,
                    0 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(*event).data.sequence_start.tag.is_null()
                && ((*emitter).canonical != 0 || (*event).data.sequence_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.sequence_start.tag) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            return 1 as ::core::ffi::c_int;
        }
        9 => {
            if !(*event).data.mapping_start.anchor.is_null() {
                if yaml_emitter_analyze_anchor(
                    emitter,
                    (*event).data.mapping_start.anchor,
                    0 as ::core::ffi::c_int,
                ) == 0
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(*event).data.mapping_start.tag.is_null()
                && ((*emitter).canonical != 0 || (*event).data.mapping_start.implicit == 0)
            {
                if yaml_emitter_analyze_tag(emitter, (*event).data.mapping_start.tag) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            return 1 as ::core::ffi::c_int;
        }
        _ => return 1 as ::core::ffi::c_int,
    };
}
unsafe extern "C" fn yaml_emitter_write_bom(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    if !((*emitter)
        .buffer
        .pointer
        .offset(5 as ::core::ffi::c_int as isize)
        < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
    {
        return 0 as ::core::ffi::c_int;
    }
    let fresh406 = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
    *fresh406 = -17i32 as yaml_char_t;
    let fresh407 = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
    *fresh407 = -69i32 as yaml_char_t;
    let fresh408 = (*emitter).buffer.pointer;
    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
    *fresh408 = -65i32 as yaml_char_t;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_indent(
    mut emitter: *mut yaml_emitter_t,
) -> ::core::ffi::c_int {
    let mut indent: ::core::ffi::c_int = if (*emitter).indent >= 0 as ::core::ffi::c_int {
        (*emitter).indent
    } else {
        0 as ::core::ffi::c_int
    };
    if (*emitter).indention == 0
        || (*emitter).column > indent
        || (*emitter).column == indent && (*emitter).whitespace == 0
    {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if (*emitter).line_break as ::core::ffi::c_uint
                    == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh143 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh143 = '\r' as i32 as yaml_char_t;
                } else {
                    if (*emitter).line_break as ::core::ffi::c_uint
                        == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let fresh144 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh144 = '\n' as i32 as yaml_char_t;
                    } else {
                        if (*emitter).line_break as ::core::ffi::c_uint
                            == YAML_CRLN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let fresh145 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh145 = '\r' as i32 as yaml_char_t;
                            let fresh146 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh146 = '\n' as i32 as yaml_char_t;
                        } else {
                        };
                    };
                };
                (*emitter).column = 0 as ::core::ffi::c_int;
                (*emitter).line += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    while (*emitter).column < indent {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh147 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh147 = ' ' as i32 as yaml_char_t;
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).whitespace = 1 as ::core::ffi::c_int;
    (*emitter).indention = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_indicator(
    mut emitter: *mut yaml_emitter_t,
    mut indicator: *const ::core::ffi::c_char,
    mut need_whitespace: ::core::ffi::c_int,
    mut is_whitespace: ::core::ffi::c_int,
    mut is_indention: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut indicator_length: size_t = 0;
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    indicator_length = strlen(indicator);
    string.start = indicator as *mut yaml_char_t;
    string.end = (indicator as *mut yaml_char_t).offset(indicator_length as isize);
    string.pointer = indicator as *mut yaml_char_t;
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh3 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh3 = ' ' as i32 as yaml_char_t;
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    while string.pointer != string.end {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let fresh4 = string.pointer;
                    string.pointer = string.pointer.offset(1);
                    let fresh5 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh5 = *fresh4;
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        let fresh6 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh7 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh7 = *fresh6;
                        let fresh8 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh9 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh9 = *fresh8;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            let fresh10 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh11 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh11 = *fresh10;
                            let fresh12 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh13 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh13 = *fresh12;
                            let fresh14 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh15 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh15 = *fresh14;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                let fresh16 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh17 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh17 = *fresh16;
                                let fresh18 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh19 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh19 = *fresh18;
                                let fresh20 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh21 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh21 = *fresh20;
                                let fresh22 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh23 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh23 = *fresh22;
                            } else {
                            };
                        };
                    };
                };
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).whitespace = is_whitespace;
    (*emitter).indention = ((*emitter).indention != 0 && is_indention != 0) as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_anchor(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    while string.pointer != string.end {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let fresh71 = string.pointer;
                    string.pointer = string.pointer.offset(1);
                    let fresh72 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh72 = *fresh71;
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        let fresh73 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh74 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh74 = *fresh73;
                        let fresh75 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh76 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh76 = *fresh75;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            let fresh77 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh78 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh78 = *fresh77;
                            let fresh79 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh80 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh80 = *fresh79;
                            let fresh81 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh82 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh82 = *fresh81;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                let fresh83 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh84 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh84 = *fresh83;
                                let fresh85 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh86 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh86 = *fresh85;
                                let fresh87 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh88 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh88 = *fresh87;
                                let fresh89 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh90 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh90 = *fresh89;
                            } else {
                            };
                        };
                    };
                };
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).whitespace = 0 as ::core::ffi::c_int;
    (*emitter).indention = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_tag_handle(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if (*emitter).whitespace == 0 {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh50 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh50 = ' ' as i32 as yaml_char_t;
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    while string.pointer != string.end {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    let fresh51 = string.pointer;
                    string.pointer = string.pointer.offset(1);
                    let fresh52 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh52 = *fresh51;
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        let fresh53 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh54 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh54 = *fresh53;
                        let fresh55 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh56 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh56 = *fresh55;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                            == 0xe0 as ::core::ffi::c_int
                        {
                            let fresh57 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh58 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh58 = *fresh57;
                            let fresh59 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh60 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh60 = *fresh59;
                            let fresh61 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh62 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh62 = *fresh61;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf8 as ::core::ffi::c_int
                                == 0xf0 as ::core::ffi::c_int
                            {
                                let fresh63 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh64 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh64 = *fresh63;
                                let fresh65 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh66 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh66 = *fresh65;
                                let fresh67 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh68 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh68 = *fresh67;
                                let fresh69 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh70 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh70 = *fresh69;
                            } else {
                            };
                        };
                    };
                };
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).whitespace = 0 as ::core::ffi::c_int;
    (*emitter).indention = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_tag_content(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut need_whitespace: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if need_whitespace != 0 && (*emitter).whitespace == 0 {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh24 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh24 = ' ' as i32 as yaml_char_t;
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    while string.pointer != string.end {
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >= '0' as i32 as yaml_char_t as ::core::ffi::c_int
            && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                <= '9' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'A' as i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'Z' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 'a' as i32 as yaml_char_t as ::core::ffi::c_int
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 'z' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '_' as i32
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '-' as i32
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ';' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '/' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '?' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ':' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '@' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '&' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '=' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '+' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '$' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ',' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '_' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '.' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '~' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '*' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '(' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ')' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '[' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == ']' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh25 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh26 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh26 = *fresh25;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh27 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh28 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh28 = *fresh27;
                            let fresh29 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh30 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh30 = *fresh29;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh31 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh32 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh32 = *fresh31;
                                let fresh33 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh34 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh34 = *fresh33;
                                let fresh35 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh36 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh36 = *fresh35;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh37 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh38 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh38 = *fresh37;
                                    let fresh39 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh40 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh40 = *fresh39;
                                    let fresh41 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh42 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh42 = *fresh41;
                                    let fresh43 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh44 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh44 = *fresh43;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
        } else {
            let mut width: ::core::ffi::c_int = if *string
                .pointer
                .offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xe0 as ::core::ffi::c_int
                == 0xc0 as ::core::ffi::c_int
            {
                2 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int
                == 0xe0 as ::core::ffi::c_int
            {
                3 as ::core::ffi::c_int
            } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf8 as ::core::ffi::c_int
                == 0xf0 as ::core::ffi::c_int
            {
                4 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            let mut value_0: ::core::ffi::c_uint = 0;
            loop {
                let fresh45 = width;
                width = width - 1;
                if !(fresh45 != 0) {
                    break;
                }
                let fresh46 = string.pointer;
                string.pointer = string.pointer.offset(1);
                value_0 = *fresh46 as ::core::ffi::c_uint;
                if !(((*emitter)
                    .buffer
                    .pointer
                    .offset(5 as ::core::ffi::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh47 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh47 = '%' as i32 as yaml_char_t;
                        (*emitter).column += 1;
                        1 as ::core::ffi::c_int != 0
                    })
                {
                    return 0 as ::core::ffi::c_int;
                }
                if !(((*emitter)
                    .buffer
                    .pointer
                    .offset(5 as ::core::ffi::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh48 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh48 = (value_0 >> 4 as ::core::ffi::c_int).wrapping_add(
                            (if (value_0 >> 4 as ::core::ffi::c_int) < 10 as ::core::ffi::c_uint {
                                '0' as i32
                            } else {
                                'A' as i32 - 10 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        ) as yaml_char_t;
                        (*emitter).column += 1;
                        1 as ::core::ffi::c_int != 0
                    })
                {
                    return 0 as ::core::ffi::c_int;
                }
                if !(((*emitter)
                    .buffer
                    .pointer
                    .offset(5 as ::core::ffi::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh49 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh49 = (value_0 & 0xf as ::core::ffi::c_uint).wrapping_add(
                            (if (value_0 & 0xf as ::core::ffi::c_uint) < 10 as ::core::ffi::c_uint {
                                '0' as i32
                            } else {
                                'A' as i32 - 10 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint,
                        ) as yaml_char_t;
                        (*emitter).column += 1;
                        1 as ::core::ffi::c_int != 0
                    })
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    (*emitter).whitespace = 0 as ::core::ffi::c_int;
    (*emitter).indention = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_plain_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut allow_breaks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut spaces: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut breaks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if (*emitter).whitespace == 0 && (length != 0 || (*emitter).flow_level != 0) {
        if !(((*emitter)
            .buffer
            .pointer
            .offset(5 as ::core::ffi::c_int as isize)
            < (*emitter).buffer.end
            || yaml_emitter_flush(emitter) != 0)
            && {
                let fresh326 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh326 = ' ' as i32 as yaml_char_t;
                (*emitter).column += 1;
                1 as ::core::ffi::c_int != 0
            })
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    while string.pointer != string.end {
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && !(*string.pointer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                string.pointer = string.pointer.offset(
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as isize,
                );
            } else if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh327 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh328 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh328 = *fresh327;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh329 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh330 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh330 = *fresh329;
                            let fresh331 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh332 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh332 = *fresh331;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh333 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh334 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh334 = *fresh333;
                                let fresh335 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh336 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh336 = *fresh335;
                                let fresh337 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh338 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh338 = *fresh337;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh339 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh340 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh340 = *fresh339;
                                    let fresh341 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh342 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh342 = *fresh341;
                                    let fresh343 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh344 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh344 = *fresh343;
                                    let fresh345 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh346 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh346 = *fresh345;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            spaces = 1 as ::core::ffi::c_int;
        } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
        {
            if breaks == 0
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                if !(((*emitter)
                    .buffer
                    .pointer
                    .offset(5 as ::core::ffi::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as ::core::ffi::c_uint
                            == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let fresh347 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh347 = '\r' as i32 as yaml_char_t;
                        } else {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh348 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh348 = '\n' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_CRLN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh349 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh349 = '\r' as i32 as yaml_char_t;
                                    let fresh350 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh350 = '\n' as i32 as yaml_char_t;
                                } else {
                                };
                            };
                        };
                        (*emitter).column = 0 as ::core::ffi::c_int;
                        (*emitter).line += 1;
                        1 as ::core::ffi::c_int != 0
                    })
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    (((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh351 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh351 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh352 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh352 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as ::core::ffi::c_uint
                                        == YAML_CRLN_BREAK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        let fresh353 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh353 = '\r' as i32 as yaml_char_t;
                                        let fresh354 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh354 = '\n' as i32 as yaml_char_t;
                                    } else {
                                    };
                                };
                            };
                            (*emitter).column = 0 as ::core::ffi::c_int;
                            (*emitter).line += 1;
                            1 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    string.pointer = string.pointer.offset(1);
                    1 as ::core::ffi::c_int
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh355 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh356 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh356 = *fresh355;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh357 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh358 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh358 = *fresh357;
                            let fresh359 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh360 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh360 = *fresh359;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh361 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh362 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh362 = *fresh361;
                                let fresh363 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh364 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh364 = *fresh363;
                                let fresh365 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh366 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh366 = *fresh365;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh367 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh368 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh368 = *fresh367;
                                    let fresh369 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh370 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh370 = *fresh369;
                                    let fresh371 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh372 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh372 = *fresh371;
                                    let fresh373 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh374 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh374 = *fresh373;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column = 0 as ::core::ffi::c_int;
                    (*emitter).line += 1;
                    1 as ::core::ffi::c_int
                }) != 0)
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 1 as ::core::ffi::c_int;
            breaks = 1 as ::core::ffi::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh375 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh376 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh376 = *fresh375;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh377 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh378 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh378 = *fresh377;
                            let fresh379 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh380 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh380 = *fresh379;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh381 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh382 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh382 = *fresh381;
                                let fresh383 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh384 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh384 = *fresh383;
                                let fresh385 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh386 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh386 = *fresh385;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh387 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh388 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh388 = *fresh387;
                                    let fresh389 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh390 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh390 = *fresh389;
                                    let fresh391 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh392 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh392 = *fresh391;
                                    let fresh393 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh394 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh394 = *fresh393;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 0 as ::core::ffi::c_int;
            spaces = 0 as ::core::ffi::c_int;
            breaks = 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).whitespace = 0 as ::core::ffi::c_int;
    (*emitter).indention = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_single_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut allow_breaks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut spaces: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut breaks: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b"'\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    while string.pointer != string.end {
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.offset(-(1 as ::core::ffi::c_int as isize))
                && !(*string.pointer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int)
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                string.pointer = string.pointer.offset(
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as isize,
                );
            } else if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh257 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh258 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh258 = *fresh257;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh259 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh260 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh260 = *fresh259;
                            let fresh261 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh262 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh262 = *fresh261;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh263 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh264 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh264 = *fresh263;
                                let fresh265 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh266 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh266 = *fresh265;
                                let fresh267 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh268 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh268 = *fresh267;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh269 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh270 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh270 = *fresh269;
                                    let fresh271 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh272 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh272 = *fresh271;
                                    let fresh273 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh274 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh274 = *fresh273;
                                    let fresh275 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh276 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh276 = *fresh275;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            spaces = 1 as ::core::ffi::c_int;
        } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
        {
            if breaks == 0
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                if !(((*emitter)
                    .buffer
                    .pointer
                    .offset(5 as ::core::ffi::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        if (*emitter).line_break as ::core::ffi::c_uint
                            == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let fresh277 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh277 = '\r' as i32 as yaml_char_t;
                        } else {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh278 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh278 = '\n' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_CRLN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh279 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh279 = '\r' as i32 as yaml_char_t;
                                    let fresh280 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh280 = '\n' as i32 as yaml_char_t;
                                } else {
                                };
                            };
                        };
                        (*emitter).column = 0 as ::core::ffi::c_int;
                        (*emitter).line += 1;
                        1 as ::core::ffi::c_int != 0
                    })
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    (((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh281 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh281 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh282 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh282 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as ::core::ffi::c_uint
                                        == YAML_CRLN_BREAK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        let fresh283 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh283 = '\r' as i32 as yaml_char_t;
                                        let fresh284 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh284 = '\n' as i32 as yaml_char_t;
                                    } else {
                                    };
                                };
                            };
                            (*emitter).column = 0 as ::core::ffi::c_int;
                            (*emitter).line += 1;
                            1 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    string.pointer = string.pointer.offset(1);
                    1 as ::core::ffi::c_int
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh285 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh286 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh286 = *fresh285;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh287 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh288 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh288 = *fresh287;
                            let fresh289 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh290 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh290 = *fresh289;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh291 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh292 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh292 = *fresh291;
                                let fresh293 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh294 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh294 = *fresh293;
                                let fresh295 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh296 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh296 = *fresh295;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh297 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh298 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh298 = *fresh297;
                                    let fresh299 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh300 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh300 = *fresh299;
                                    let fresh301 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh302 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh302 = *fresh301;
                                    let fresh303 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh304 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh304 = *fresh303;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column = 0 as ::core::ffi::c_int;
                    (*emitter).line += 1;
                    1 as ::core::ffi::c_int
                }) != 0)
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 1 as ::core::ffi::c_int;
            breaks = 1 as ::core::ffi::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\'' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                if !(((*emitter)
                    .buffer
                    .pointer
                    .offset(5 as ::core::ffi::c_int as isize)
                    < (*emitter).buffer.end
                    || yaml_emitter_flush(emitter) != 0)
                    && {
                        let fresh305 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh305 = '\'' as i32 as yaml_char_t;
                        (*emitter).column += 1;
                        1 as ::core::ffi::c_int != 0
                    })
                {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh306 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh307 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh307 = *fresh306;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh308 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh309 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh309 = *fresh308;
                            let fresh310 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh311 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh311 = *fresh310;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh312 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh313 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh313 = *fresh312;
                                let fresh314 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh315 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh315 = *fresh314;
                                let fresh316 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh317 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh317 = *fresh316;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh318 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh319 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh319 = *fresh318;
                                    let fresh320 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh321 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh321 = *fresh320;
                                    let fresh322 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh323 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh323 = *fresh322;
                                    let fresh324 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh325 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh325 = *fresh324;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 0 as ::core::ffi::c_int;
            spaces = 0 as ::core::ffi::c_int;
            breaks = 0 as ::core::ffi::c_int;
        }
    }
    if breaks != 0 {
        if yaml_emitter_write_indent(emitter) == 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"'\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).whitespace = 0 as ::core::ffi::c_int;
    (*emitter).indention = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_double_quoted_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
    mut allow_breaks: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut spaces: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    while string.pointer != string.end {
        if !(*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0xa as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >= 0x20 as ::core::ffi::c_int
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= 0x7e as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xc2 as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    >= 0xa0 as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                > 0xc2 as ::core::ffi::c_int
                && (*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                    < 0xed as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xed as ::core::ffi::c_int
                && (*string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int)
                    < 0xa0 as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xee as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0xef as ::core::ffi::c_int
                && !(*string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == 0xbb as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == 0xbf as ::core::ffi::c_int)
                && !(*string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == 0xbf as ::core::ffi::c_int
                    && (*string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == 0xbe as ::core::ffi::c_int
                        || *string
                            .pointer
                            .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == 0xbf as ::core::ffi::c_int)))
            || (*emitter).unicode == 0
                && !(*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    <= '\u{7f}' as i32 as yaml_char_t as ::core::ffi::c_int)
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -17i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -69i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -65i32 as yaml_char_t as ::core::ffi::c_int
            || (*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int)
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '"' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\\' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            let mut octet: ::core::ffi::c_uchar = 0;
            let mut width: ::core::ffi::c_uint = 0;
            let mut value_0: ::core::ffi::c_uint = 0;
            let mut k: ::core::ffi::c_int = 0;
            octet =
                *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uchar;
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
            value_0 = (if octet as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
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
            k = 1 as ::core::ffi::c_int;
            while k < width as ::core::ffi::c_int {
                octet = *string.pointer.offset(k as isize) as ::core::ffi::c_uchar;
                value_0 = (value_0 << 6 as ::core::ffi::c_int).wrapping_add(
                    (octet as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int)
                        as ::core::ffi::c_uint,
                );
                k += 1;
            }
            string.pointer = string.pointer.offset(width as isize);
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    let fresh196 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh196 = '\\' as i32 as yaml_char_t;
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            match value_0 {
                0 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh197 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh197 = '0' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                7 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh198 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh198 = 'a' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                8 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh199 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh199 = 'b' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                9 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh200 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh200 = 't' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                10 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh201 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh201 = 'n' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                11 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh202 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh202 = 'v' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                12 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh203 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh203 = 'f' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                13 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh204 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh204 = 'r' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                27 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh205 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh205 = 'e' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                34 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh206 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh206 = '"' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                92 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh207 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh207 = '\\' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                133 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh208 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh208 = 'N' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                160 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh209 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh209 = '_' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                8232 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh210 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh210 = 'L' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                8233 => {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh211 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh211 = 'P' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                _ => {
                    if value_0 <= 0xff as ::core::ffi::c_uint {
                        if !(((*emitter)
                            .buffer
                            .pointer
                            .offset(5 as ::core::ffi::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh212 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh212 = 'x' as i32 as yaml_char_t;
                                (*emitter).column += 1;
                                1 as ::core::ffi::c_int != 0
                            })
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        width = 2 as ::core::ffi::c_uint;
                    } else if value_0 <= 0xffff as ::core::ffi::c_uint {
                        if !(((*emitter)
                            .buffer
                            .pointer
                            .offset(5 as ::core::ffi::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh213 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh213 = 'u' as i32 as yaml_char_t;
                                (*emitter).column += 1;
                                1 as ::core::ffi::c_int != 0
                            })
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        width = 4 as ::core::ffi::c_uint;
                    } else {
                        if !(((*emitter)
                            .buffer
                            .pointer
                            .offset(5 as ::core::ffi::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh214 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh214 = 'U' as i32 as yaml_char_t;
                                (*emitter).column += 1;
                                1 as ::core::ffi::c_int != 0
                            })
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        width = 8 as ::core::ffi::c_uint;
                    }
                    k = width
                        .wrapping_sub(1 as ::core::ffi::c_uint)
                        .wrapping_mul(4 as ::core::ffi::c_uint)
                        as ::core::ffi::c_int;
                    while k >= 0 as ::core::ffi::c_int {
                        let mut digit: ::core::ffi::c_int =
                            (value_0 >> k & 0xf as ::core::ffi::c_uint) as ::core::ffi::c_int;
                        if !(((*emitter)
                            .buffer
                            .pointer
                            .offset(5 as ::core::ffi::c_int as isize)
                            < (*emitter).buffer.end
                            || yaml_emitter_flush(emitter) != 0)
                            && {
                                let fresh215 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh215 = (digit
                                    + (if digit < 10 as ::core::ffi::c_int {
                                        '0' as i32
                                    } else {
                                        'A' as i32 - 10 as ::core::ffi::c_int
                                    })) as yaml_char_t;
                                (*emitter).column += 1;
                                1 as ::core::ffi::c_int != 0
                            })
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        k -= 4 as ::core::ffi::c_int;
                    }
                }
            }
            spaces = 0 as ::core::ffi::c_int;
        } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        {
            if allow_breaks != 0
                && spaces == 0
                && (*emitter).column > (*emitter).best_width
                && string.pointer != string.start
                && string.pointer != string.end.offset(-(1 as ::core::ffi::c_int as isize))
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                if *string.pointer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            let fresh216 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh216 = '\\' as i32 as yaml_char_t;
                            (*emitter).column += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
                string.pointer = string.pointer.offset(
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as isize,
                );
            } else if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh217 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh218 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh218 = *fresh217;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh219 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh220 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh220 = *fresh219;
                            let fresh221 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh222 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh222 = *fresh221;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh223 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh224 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh224 = *fresh223;
                                let fresh225 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh226 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh226 = *fresh225;
                                let fresh227 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh228 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh228 = *fresh227;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh229 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh230 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh230 = *fresh229;
                                    let fresh231 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh232 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh232 = *fresh231;
                                    let fresh233 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh234 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh234 = *fresh233;
                                    let fresh235 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh236 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh236 = *fresh235;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            spaces = 1 as ::core::ffi::c_int;
        } else {
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh237 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh238 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh238 = *fresh237;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh239 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh240 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh240 = *fresh239;
                            let fresh241 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh242 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh242 = *fresh241;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh243 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh244 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh244 = *fresh243;
                                let fresh245 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh246 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh246 = *fresh245;
                                let fresh247 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh248 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh248 = *fresh247;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh249 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh250 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh250 = *fresh249;
                                    let fresh251 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh252 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh252 = *fresh251;
                                    let fresh253 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh254 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh254 = *fresh253;
                                    let fresh255 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh256 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh256 = *fresh255;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            spaces = 0 as ::core::ffi::c_int;
        }
    }
    if yaml_emitter_write_indicator(
        emitter,
        b"\"\0" as *const u8 as *const ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).whitespace = 0 as ::core::ffi::c_int;
    (*emitter).indention = 0 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_block_scalar_hints(
    mut emitter: *mut yaml_emitter_t,
    mut string: yaml_string_t,
) -> ::core::ffi::c_int {
    let mut indent_hint: [::core::ffi::c_char; 2] = [0; 2];
    let mut chomp_hint: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
        || (*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
    {
        indent_hint[0 as ::core::ffi::c_int as usize] = ('0' as i32
            + (*emitter).best_indent as ::core::ffi::c_char as ::core::ffi::c_int)
            as ::core::ffi::c_char;
        indent_hint[1 as ::core::ffi::c_int as usize] = '\0' as i32 as ::core::ffi::c_char;
        if yaml_emitter_write_indicator(
            emitter,
            &raw mut indent_hint as *mut ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    (*emitter).open_ended = 0 as ::core::ffi::c_int;
    string.pointer = string.end;
    if string.start == string.pointer {
        chomp_hint = b"-\0" as *const u8 as *const ::core::ffi::c_char;
    } else {
        loop {
            string.pointer = string.pointer.offset(-1);
            if !(*string.pointer as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int)
            {
                break;
            }
        }
        if !(*string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int)
        {
            chomp_hint = b"-\0" as *const u8 as *const ::core::ffi::c_char;
        } else if string.start == string.pointer {
            chomp_hint = b"+\0" as *const u8 as *const ::core::ffi::c_char;
            (*emitter).open_ended = 2 as ::core::ffi::c_int;
        } else {
            loop {
                string.pointer = string.pointer.offset(-1);
                if !(*string.pointer as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    == 0x80 as ::core::ffi::c_int)
                {
                    break;
                }
            }
            if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -62i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -123i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -88i32 as yaml_char_t as ::core::ffi::c_int
                || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == -30i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -128i32 as yaml_char_t as ::core::ffi::c_int
                    && *string
                        .pointer
                        .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        == -87i32 as yaml_char_t as ::core::ffi::c_int
            {
                chomp_hint = b"+\0" as *const u8 as *const ::core::ffi::c_char;
                (*emitter).open_ended = 2 as ::core::ffi::c_int;
            }
        }
    }
    if !chomp_hint.is_null() {
        if yaml_emitter_write_indicator(
            emitter,
            chomp_hint,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_literal_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut breaks: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b"|\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !(((*emitter)
        .buffer
        .pointer
        .offset(5 as ::core::ffi::c_int as isize)
        < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as ::core::ffi::c_uint
                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let fresh148 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh148 = '\r' as i32 as yaml_char_t;
            } else {
                if (*emitter).line_break as ::core::ffi::c_uint
                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh149 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh149 = '\n' as i32 as yaml_char_t;
                } else {
                    if (*emitter).line_break as ::core::ffi::c_uint
                        == YAML_CRLN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let fresh150 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh150 = '\r' as i32 as yaml_char_t;
                        let fresh151 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh151 = '\n' as i32 as yaml_char_t;
                    } else {
                    };
                };
            };
            (*emitter).column = 0 as ::core::ffi::c_int;
            (*emitter).line += 1;
            1 as ::core::ffi::c_int != 0
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).indention = 1 as ::core::ffi::c_int;
    (*emitter).whitespace = 1 as ::core::ffi::c_int;
    while string.pointer != string.end {
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
        {
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    (((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh152 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh152 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh153 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh153 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as ::core::ffi::c_uint
                                        == YAML_CRLN_BREAK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        let fresh154 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh154 = '\r' as i32 as yaml_char_t;
                                        let fresh155 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh155 = '\n' as i32 as yaml_char_t;
                                    } else {
                                    };
                                };
                            };
                            (*emitter).column = 0 as ::core::ffi::c_int;
                            (*emitter).line += 1;
                            1 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    string.pointer = string.pointer.offset(1);
                    1 as ::core::ffi::c_int
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh156 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh157 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh157 = *fresh156;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh158 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh159 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh159 = *fresh158;
                            let fresh160 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh161 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh161 = *fresh160;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh162 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh163 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh163 = *fresh162;
                                let fresh164 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh165 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh165 = *fresh164;
                                let fresh166 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh167 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh167 = *fresh166;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh168 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh169 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh169 = *fresh168;
                                    let fresh170 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh171 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh171 = *fresh170;
                                    let fresh172 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh173 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh173 = *fresh172;
                                    let fresh174 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh175 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh175 = *fresh174;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column = 0 as ::core::ffi::c_int;
                    (*emitter).line += 1;
                    1 as ::core::ffi::c_int
                }) != 0)
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 1 as ::core::ffi::c_int;
            breaks = 1 as ::core::ffi::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
            }
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh176 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh177 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh177 = *fresh176;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh178 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh179 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh179 = *fresh178;
                            let fresh180 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh181 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh181 = *fresh180;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh182 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh183 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh183 = *fresh182;
                                let fresh184 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh185 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh185 = *fresh184;
                                let fresh186 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh187 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh187 = *fresh186;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh188 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh189 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh189 = *fresh188;
                                    let fresh190 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh191 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh191 = *fresh190;
                                    let fresh192 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh193 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh193 = *fresh192;
                                    let fresh194 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh195 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh195 = *fresh194;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 0 as ::core::ffi::c_int;
            breaks = 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn yaml_emitter_write_folded_scalar(
    mut emitter: *mut yaml_emitter_t,
    mut value: *mut yaml_char_t,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut string: yaml_string_t = yaml_string_t {
        start: ::core::ptr::null_mut::<yaml_char_t>(),
        end: ::core::ptr::null_mut::<yaml_char_t>(),
        pointer: ::core::ptr::null_mut::<yaml_char_t>(),
    };
    let mut breaks: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut leading_spaces: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    string.start = value;
    string.end = value.offset(length as isize);
    string.pointer = value;
    if yaml_emitter_write_indicator(
        emitter,
        b">\0" as *const u8 as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if yaml_emitter_write_block_scalar_hints(emitter, string) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if !(((*emitter)
        .buffer
        .pointer
        .offset(5 as ::core::ffi::c_int as isize)
        < (*emitter).buffer.end
        || yaml_emitter_flush(emitter) != 0)
        && {
            if (*emitter).line_break as ::core::ffi::c_uint
                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let fresh91 = (*emitter).buffer.pointer;
                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                *fresh91 = '\r' as i32 as yaml_char_t;
            } else {
                if (*emitter).line_break as ::core::ffi::c_uint
                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    let fresh92 = (*emitter).buffer.pointer;
                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                    *fresh92 = '\n' as i32 as yaml_char_t;
                } else {
                    if (*emitter).line_break as ::core::ffi::c_uint
                        == YAML_CRLN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let fresh93 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh93 = '\r' as i32 as yaml_char_t;
                        let fresh94 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh94 = '\n' as i32 as yaml_char_t;
                    } else {
                    };
                };
            };
            (*emitter).column = 0 as ::core::ffi::c_int;
            (*emitter).line += 1;
            1 as ::core::ffi::c_int != 0
        })
    {
        return 0 as ::core::ffi::c_int;
    }
    (*emitter).indention = 1 as ::core::ffi::c_int;
    (*emitter).whitespace = 1 as ::core::ffi::c_int;
    while string.pointer != string.end {
        if *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -62i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -123i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -88i32 as yaml_char_t as ::core::ffi::c_int
            || *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == -30i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -128i32 as yaml_char_t as ::core::ffi::c_int
                && *string
                    .pointer
                    .offset((0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    == -87i32 as yaml_char_t as ::core::ffi::c_int
        {
            if breaks == 0
                && leading_spaces == 0
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
            {
                let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while *string.pointer.offset(k as isize) as ::core::ffi::c_int
                    == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        == -62i32 as yaml_char_t as ::core::ffi::c_int
                        && *string
                            .pointer
                            .offset((k + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -123i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *string
                            .pointer
                            .offset((k + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *string
                            .pointer
                            .offset((k + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -88i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        == -30i32 as yaml_char_t as ::core::ffi::c_int
                        && *string
                            .pointer
                            .offset((k + 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -128i32 as yaml_char_t as ::core::ffi::c_int
                        && *string
                            .pointer
                            .offset((k + 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            == -87i32 as yaml_char_t as ::core::ffi::c_int
                {
                    k += if *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else if *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else if *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else if *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                }
                if !(*string.pointer.offset(k as isize) as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                        == '\t' as i32 as yaml_char_t as ::core::ffi::c_int
                    || (*string.pointer.offset(k as isize) as ::core::ffi::c_int
                        == '\r' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                            == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                        || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                            == -62i32 as yaml_char_t as ::core::ffi::c_int
                            && *string
                                .pointer
                                .offset((k + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == -123i32 as yaml_char_t as ::core::ffi::c_int
                        || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                            && *string
                                .pointer
                                .offset((k + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                            && *string
                                .pointer
                                .offset((k + 2 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == -88i32 as yaml_char_t as ::core::ffi::c_int
                        || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                            == -30i32 as yaml_char_t as ::core::ffi::c_int
                            && *string
                                .pointer
                                .offset((k + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == -128i32 as yaml_char_t as ::core::ffi::c_int
                            && *string
                                .pointer
                                .offset((k + 2 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int
                                == -87i32 as yaml_char_t as ::core::ffi::c_int
                        || *string.pointer.offset(k as isize) as ::core::ffi::c_int
                            == '\0' as i32 as yaml_char_t as ::core::ffi::c_int))
                {
                    if !(((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh95 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh95 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh96 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh96 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as ::core::ffi::c_uint
                                        == YAML_CRLN_BREAK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        let fresh97 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh97 = '\r' as i32 as yaml_char_t;
                                        let fresh98 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh98 = '\n' as i32 as yaml_char_t;
                                    } else {
                                    };
                                };
                            };
                            (*emitter).column = 0 as ::core::ffi::c_int;
                            (*emitter).line += 1;
                            1 as ::core::ffi::c_int != 0
                        })
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
            if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == '\n' as i32 as yaml_char_t as ::core::ffi::c_int
                {
                    (((*emitter)
                        .buffer
                        .pointer
                        .offset(5 as ::core::ffi::c_int as isize)
                        < (*emitter).buffer.end
                        || yaml_emitter_flush(emitter) != 0)
                        && {
                            if (*emitter).line_break as ::core::ffi::c_uint
                                == YAML_CR_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                let fresh99 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh99 = '\r' as i32 as yaml_char_t;
                            } else {
                                if (*emitter).line_break as ::core::ffi::c_uint
                                    == YAML_LN_BREAK as ::core::ffi::c_int as ::core::ffi::c_uint
                                {
                                    let fresh100 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh100 = '\n' as i32 as yaml_char_t;
                                } else {
                                    if (*emitter).line_break as ::core::ffi::c_uint
                                        == YAML_CRLN_BREAK as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        let fresh101 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh101 = '\r' as i32 as yaml_char_t;
                                        let fresh102 = (*emitter).buffer.pointer;
                                        (*emitter).buffer.pointer =
                                            (*emitter).buffer.pointer.offset(1);
                                        *fresh102 = '\n' as i32 as yaml_char_t;
                                    } else {
                                    };
                                };
                            };
                            (*emitter).column = 0 as ::core::ffi::c_int;
                            (*emitter).line += 1;
                            1 as ::core::ffi::c_int != 0
                        }) as ::core::ffi::c_int;
                    string.pointer = string.pointer.offset(1);
                    1 as ::core::ffi::c_int
                } else {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh103 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh104 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh104 = *fresh103;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh105 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh106 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh106 = *fresh105;
                            let fresh107 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh108 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh108 = *fresh107;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh109 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh110 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh110 = *fresh109;
                                let fresh111 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh112 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh112 = *fresh111;
                                let fresh113 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh114 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh114 = *fresh113;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh115 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh116 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh116 = *fresh115;
                                    let fresh117 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh118 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh118 = *fresh117;
                                    let fresh119 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh120 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh120 = *fresh119;
                                    let fresh121 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh122 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh122 = *fresh121;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column = 0 as ::core::ffi::c_int;
                    (*emitter).line += 1;
                    1 as ::core::ffi::c_int
                }) != 0)
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 1 as ::core::ffi::c_int;
            breaks = 1 as ::core::ffi::c_int;
        } else {
            if breaks != 0 {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                leading_spaces = (*string.pointer.offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                    || *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        == '\t' as i32 as yaml_char_t as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            }
            if breaks == 0
                && *string.pointer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int
                && !(*string.pointer.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    == ' ' as i32 as yaml_char_t as ::core::ffi::c_int)
                && (*emitter).column > (*emitter).best_width
            {
                if yaml_emitter_write_indent(emitter) == 0 {
                    return 0 as ::core::ffi::c_int;
                }
                string.pointer = string.pointer.offset(
                    (if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        1 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xe0 as ::core::ffi::c_int
                        == 0xc0 as ::core::ffi::c_int
                    {
                        2 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf0 as ::core::ffi::c_int
                        == 0xe0 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else if *string.pointer.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & 0xf8 as ::core::ffi::c_int
                        == 0xf0 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as isize,
                );
            } else if !(((*emitter)
                .buffer
                .pointer
                .offset(5 as ::core::ffi::c_int as isize)
                < (*emitter).buffer.end
                || yaml_emitter_flush(emitter) != 0)
                && {
                    if *string.pointer as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        let fresh123 = string.pointer;
                        string.pointer = string.pointer.offset(1);
                        let fresh124 = (*emitter).buffer.pointer;
                        (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                        *fresh124 = *fresh123;
                    } else {
                        if *string.pointer as ::core::ffi::c_int & 0xe0 as ::core::ffi::c_int
                            == 0xc0 as ::core::ffi::c_int
                        {
                            let fresh125 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh126 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh126 = *fresh125;
                            let fresh127 = string.pointer;
                            string.pointer = string.pointer.offset(1);
                            let fresh128 = (*emitter).buffer.pointer;
                            (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                            *fresh128 = *fresh127;
                        } else {
                            if *string.pointer as ::core::ffi::c_int & 0xf0 as ::core::ffi::c_int
                                == 0xe0 as ::core::ffi::c_int
                            {
                                let fresh129 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh130 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh130 = *fresh129;
                                let fresh131 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh132 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh132 = *fresh131;
                                let fresh133 = string.pointer;
                                string.pointer = string.pointer.offset(1);
                                let fresh134 = (*emitter).buffer.pointer;
                                (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                *fresh134 = *fresh133;
                            } else {
                                if *string.pointer as ::core::ffi::c_int
                                    & 0xf8 as ::core::ffi::c_int
                                    == 0xf0 as ::core::ffi::c_int
                                {
                                    let fresh135 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh136 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh136 = *fresh135;
                                    let fresh137 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh138 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh138 = *fresh137;
                                    let fresh139 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh140 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh140 = *fresh139;
                                    let fresh141 = string.pointer;
                                    string.pointer = string.pointer.offset(1);
                                    let fresh142 = (*emitter).buffer.pointer;
                                    (*emitter).buffer.pointer = (*emitter).buffer.pointer.offset(1);
                                    *fresh142 = *fresh141;
                                } else {
                                };
                            };
                        };
                    };
                    (*emitter).column += 1;
                    1 as ::core::ffi::c_int != 0
                })
            {
                return 0 as ::core::ffi::c_int;
            }
            (*emitter).indention = 0 as ::core::ffi::c_int;
            breaks = 0 as ::core::ffi::c_int;
        }
    }
    return 1 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
