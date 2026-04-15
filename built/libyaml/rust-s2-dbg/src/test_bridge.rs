//! Test bridge wrappers — calls each private libyaml implementation
//! in the transpiled Rust crate via crate::function().

#![allow(warnings)]

use super::*;



use libc::{c_char, c_int, c_uchar, c_void, size_t};

// ── api.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_string_read_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
    size_read: *mut size_t,
) -> c_int {
    crate::yaml_string_read_handler(data, buffer, size, size_read)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_file_read_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
    size_read: *mut size_t,
) -> c_int {
    crate::yaml_file_read_handler(data, buffer, size, size_read)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_string_write_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
) -> c_int {
    crate::yaml_string_write_handler(data, buffer, size)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_file_write_handler(
    data: *mut c_void,
    buffer: *mut c_uchar,
    size: size_t,
) -> c_int {
    crate::yaml_file_write_handler(data, buffer, size)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_check_utf8(
    start: *const yaml_char_t,
    length: size_t,
) -> c_int {
    crate::yaml_check_utf8(start, length)
}

// ── dumper.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_delete_document_and_anchors(
    emitter: *mut yaml_emitter_t,
) {
    crate::yaml_emitter_delete_document_and_anchors(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_anchor_node(
    emitter: *mut yaml_emitter_t,
    index: c_int,
) {
    crate::yaml_emitter_anchor_node(emitter, index)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_generate_anchor(
    emitter: *mut yaml_emitter_t,
    anchor_id: c_int,
) -> *mut yaml_char_t {
    crate::yaml_emitter_generate_anchor(emitter, anchor_id)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_dump_node(
    emitter: *mut yaml_emitter_t,
    index: c_int,
) -> c_int {
    crate::yaml_emitter_dump_node(emitter, index)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_dump_alias(
    emitter: *mut yaml_emitter_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    crate::yaml_emitter_dump_alias(emitter, anchor)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_dump_scalar(
    emitter: *mut yaml_emitter_t,
    node: *mut yaml_node_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    crate::yaml_emitter_dump_scalar(emitter, node, anchor)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_dump_sequence(
    emitter: *mut yaml_emitter_t,
    node: *mut yaml_node_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    crate::yaml_emitter_dump_sequence(emitter, node, anchor)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_dump_mapping(
    emitter: *mut yaml_emitter_t,
    node: *mut yaml_node_t,
    anchor: *mut yaml_char_t,
) -> c_int {
    crate::yaml_emitter_dump_mapping(emitter, node, anchor)
}

// ── emitter.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_set_emitter_error(
    emitter: *mut yaml_emitter_t,
    problem: *const c_char,
) -> c_int {
    crate::yaml_emitter_set_emitter_error(emitter, problem)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_need_more_events(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_need_more_events(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_append_tag_directive(
    emitter: *mut yaml_emitter_t,
    value: yaml_tag_directive_t,
    allow_duplicates: c_int,
) -> c_int {
    crate::yaml_emitter_append_tag_directive(emitter, value, allow_duplicates)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_increase_indent(
    emitter: *mut yaml_emitter_t,
    flow: c_int,
    indentless: c_int,
) -> c_int {
    crate::yaml_emitter_increase_indent(emitter, flow, indentless)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_state_machine(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_state_machine(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_stream_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_stream_start(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_document_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_emitter_emit_document_start(emitter, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_document_content(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_document_content(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_document_end(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_document_end(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_flow_sequence_item(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_emitter_emit_flow_sequence_item(emitter, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_flow_mapping_key(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_emitter_emit_flow_mapping_key(emitter, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_flow_mapping_value(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    simple: c_int,
) -> c_int {
    crate::yaml_emitter_emit_flow_mapping_value(emitter, event, simple)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_block_sequence_item(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_emitter_emit_block_sequence_item(emitter, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_block_mapping_key(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_emitter_emit_block_mapping_key(emitter, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_block_mapping_value(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    simple: c_int,
) -> c_int {
    crate::yaml_emitter_emit_block_mapping_value(emitter, event, simple)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_node(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
    root: c_int,
    sequence: c_int,
    mapping: c_int,
    simple_key: c_int,
) -> c_int {
    crate::yaml_emitter_emit_node(emitter, event, root, sequence, mapping, simple_key)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_alias(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_alias(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_scalar(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_scalar(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_sequence_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_sequence_start(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_emit_mapping_start(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_emit_mapping_start(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_check_empty_document(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_check_empty_document(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_check_empty_sequence(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_check_empty_sequence(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_check_empty_mapping(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_check_empty_mapping(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_check_simple_key(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_check_simple_key(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_select_scalar_style(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_select_scalar_style(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_process_anchor(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_process_anchor(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_process_tag(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_process_tag(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_process_scalar(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_process_scalar(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_analyze_version_directive(
    emitter: *mut yaml_emitter_t,
    version_directive: yaml_version_directive_t,
) -> c_int {
    crate::yaml_emitter_analyze_version_directive(emitter, version_directive)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_analyze_tag_directive(
    emitter: *mut yaml_emitter_t,
    tag_directive: yaml_tag_directive_t,
) -> c_int {
    crate::yaml_emitter_analyze_tag_directive(emitter, tag_directive)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_analyze_anchor(
    emitter: *mut yaml_emitter_t,
    anchor: *mut yaml_char_t,
    alias: c_int,
) -> c_int {
    crate::yaml_emitter_analyze_anchor(emitter, anchor, alias)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_analyze_tag(
    emitter: *mut yaml_emitter_t,
    tag: *mut yaml_char_t,
) -> c_int {
    crate::yaml_emitter_analyze_tag(emitter, tag)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_analyze_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    crate::yaml_emitter_analyze_scalar(emitter, value, length)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_analyze_event(
    emitter: *mut yaml_emitter_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_emitter_analyze_event(emitter, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_bom(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_write_bom(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_indent(
    emitter: *mut yaml_emitter_t,
) -> c_int {
    crate::yaml_emitter_write_indent(emitter)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_indicator(
    emitter: *mut yaml_emitter_t,
    indicator: *const c_char,
    need_whitespace: c_int,
    is_whitespace: c_int,
    is_indention: c_int,
) -> c_int {
    crate::yaml_emitter_write_indicator(
        emitter,
        indicator,
        need_whitespace,
        is_whitespace,
        is_indention,
    )
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_anchor(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    crate::yaml_emitter_write_anchor(emitter, value, length)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_tag_handle(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    crate::yaml_emitter_write_tag_handle(emitter, value, length)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_tag_content(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    need_whitespace: c_int,
) -> c_int {
    crate::yaml_emitter_write_tag_content(emitter, value, length, need_whitespace)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_plain_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: c_int,
) -> c_int {
    crate::yaml_emitter_write_plain_scalar(emitter, value, length, allow_breaks)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_single_quoted_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: c_int,
) -> c_int {
    crate::yaml_emitter_write_single_quoted_scalar(emitter, value, length, allow_breaks)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_double_quoted_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
    allow_breaks: c_int,
) -> c_int {
    crate::yaml_emitter_write_double_quoted_scalar(emitter, value, length, allow_breaks)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_block_scalar_hints(
    emitter: *mut yaml_emitter_t,
    string: yaml_string_t,
) -> c_int {
    crate::yaml_emitter_write_block_scalar_hints(emitter, string)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_literal_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    crate::yaml_emitter_write_literal_scalar(emitter, value, length)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_write_folded_scalar(
    emitter: *mut yaml_emitter_t,
    value: *mut yaml_char_t,
    length: size_t,
) -> c_int {
    crate::yaml_emitter_write_folded_scalar(emitter, value, length)
}

// ── loader.c ──

#[no_mangle]

#[no_mangle]

#[no_mangle]

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_document(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_load_document(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_nodes(
    parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_nodes(parser, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_register_anchor(
    parser: *mut yaml_parser_t,
    index: c_int,
    anchor: *mut yaml_char_t,
) -> c_int {
    crate::yaml_parser_register_anchor(parser, index, anchor)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_node_add(
    parser: *mut yaml_parser_t,
    ctx: *mut loader_ctx,
    index: c_int,
) -> c_int {
    crate::yaml_parser_load_node_add(parser, ctx, index)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_alias(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_alias(parser, event, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_scalar(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_scalar(parser, event, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_sequence(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_sequence(parser, event, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_sequence_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_sequence_end(parser, event, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_mapping(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_mapping(parser, event, ctx)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_load_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    ctx: *mut loader_ctx,
) -> c_int {
    crate::yaml_parser_load_mapping_end(parser, event, ctx)
}

// ── parser.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_parser_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_set_parser_error(parser, problem, problem_mark)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_parser_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_set_parser_error_context(
        parser,
        context,
        context_mark,
        problem,
        problem_mark,
    )
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_maximum_level_reached(
    parser: *mut yaml_parser_t,
    context_mark: yaml_mark_t,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_maximum_level_reached(parser, context_mark, problem_mark)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_state_machine(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_state_machine(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_stream_start(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_stream_start(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_document_start(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    implicit: c_int,
) -> c_int {
    crate::yaml_parser_parse_document_start(parser, event, implicit)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_document_content(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_document_content(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_document_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_document_end(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_node(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    block: c_int,
    indentless_sequence: c_int,
) -> c_int {
    crate::yaml_parser_parse_node(parser, event, block, indentless_sequence)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_block_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_parser_parse_block_sequence_entry(parser, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_indentless_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_indentless_sequence_entry(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_block_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_parser_parse_block_mapping_key(parser, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_block_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_block_mapping_value(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_flow_sequence_entry(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_parser_parse_flow_sequence_entry(parser, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_flow_sequence_entry_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_flow_sequence_entry_mapping_key(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_flow_sequence_entry_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_flow_sequence_entry_mapping_value(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_flow_sequence_entry_mapping_end(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
) -> c_int {
    crate::yaml_parser_parse_flow_sequence_entry_mapping_end(parser, event)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_flow_mapping_key(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    first: c_int,
) -> c_int {
    crate::yaml_parser_parse_flow_mapping_key(parser, event, first)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_parse_flow_mapping_value(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    empty: c_int,
) -> c_int {
    crate::yaml_parser_parse_flow_mapping_value(parser, event, empty)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_process_empty_scalar(
    parser: *mut yaml_parser_t,
    event: *mut yaml_event_t,
    mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_process_empty_scalar(parser, event, mark)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_process_directives(
    parser: *mut yaml_parser_t,
    version_directive_ref: *mut *mut yaml_version_directive_t,
    tag_directives_start_ref: *mut *mut yaml_tag_directive_t,
    tag_directives_end_ref: *mut *mut yaml_tag_directive_t,
) -> c_int {
    crate::yaml_parser_process_directives(
        parser,
        version_directive_ref,
        tag_directives_start_ref,
        tag_directives_end_ref,
    )
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_append_tag_directive(
    parser: *mut yaml_parser_t,
    value: yaml_tag_directive_t,
    allow_duplicates: c_int,
    mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_append_tag_directive(parser, value, allow_duplicates, mark)
}

// ── reader.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_reader_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    offset: size_t,
    value: c_int,
) -> c_int {
    crate::yaml_parser_set_reader_error(parser, problem, offset, value)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_determine_encoding(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_determine_encoding(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_update_raw_buffer(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_update_raw_buffer(parser)
}

// ── scanner.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_scanner_error(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
) -> c_int {
    crate::yaml_parser_set_scanner_error(parser, context, context_mark, problem)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_next_token(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_next_token(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_stale_simple_keys(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_stale_simple_keys(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_save_simple_key(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_save_simple_key(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_remove_simple_key(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_remove_simple_key(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_increase_flow_level(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_increase_flow_level(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_decrease_flow_level(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_decrease_flow_level(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_roll_indent(
    parser: *mut yaml_parser_t,
    column: isize,
    number: isize,
    type_: yaml_token_type_t,
    mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_roll_indent(parser, column, number, type_, mark)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_unroll_indent(
    parser: *mut yaml_parser_t,
    column: isize,
) -> c_int {
    crate::yaml_parser_unroll_indent(parser, column)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_stream_start(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_stream_start(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_stream_end(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_stream_end(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_directive(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_directive(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_document_indicator(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    crate::yaml_parser_fetch_document_indicator(parser, type_)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_flow_collection_start(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    crate::yaml_parser_fetch_flow_collection_start(parser, type_)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_flow_collection_end(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    crate::yaml_parser_fetch_flow_collection_end(parser, type_)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_flow_entry(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_flow_entry(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_block_entry(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_block_entry(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_key(parser: *mut yaml_parser_t) -> c_int {
    crate::yaml_parser_fetch_key(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_value(parser: *mut yaml_parser_t) -> c_int {
    crate::yaml_parser_fetch_value(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_anchor(
    parser: *mut yaml_parser_t,
    type_: yaml_token_type_t,
) -> c_int {
    crate::yaml_parser_fetch_anchor(parser, type_)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_tag(parser: *mut yaml_parser_t) -> c_int {
    crate::yaml_parser_fetch_tag(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_block_scalar(
    parser: *mut yaml_parser_t,
    literal: c_int,
) -> c_int {
    crate::yaml_parser_fetch_block_scalar(parser, literal)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_flow_scalar(
    parser: *mut yaml_parser_t,
    single: c_int,
) -> c_int {
    crate::yaml_parser_fetch_flow_scalar(parser, single)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_fetch_plain_scalar(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_fetch_plain_scalar(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_to_next_token(
    parser: *mut yaml_parser_t,
) -> c_int {
    crate::yaml_parser_scan_to_next_token(parser)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_directive(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    crate::yaml_parser_scan_directive(parser, token)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_directive_name(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    name: *mut *mut yaml_char_t,
) -> c_int {
    crate::yaml_parser_scan_directive_name(parser, start_mark, name)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_version_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    major: *mut c_int,
    minor: *mut c_int,
) -> c_int {
    crate::yaml_parser_scan_version_directive_value(parser, start_mark, major, minor)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_version_directive_number(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    number: *mut c_int,
) -> c_int {
    crate::yaml_parser_scan_version_directive_number(parser, start_mark, number)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_tag_directive_value(
    parser: *mut yaml_parser_t,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
    prefix: *mut *mut yaml_char_t,
) -> c_int {
    crate::yaml_parser_scan_tag_directive_value(parser, start_mark, handle, prefix)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_anchor(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    type_: yaml_token_type_t,
) -> c_int {
    crate::yaml_parser_scan_anchor(parser, token, type_)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_tag(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    crate::yaml_parser_scan_tag(parser, token)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_tag_handle(
    parser: *mut yaml_parser_t,
    directive: c_int,
    start_mark: yaml_mark_t,
    handle: *mut *mut yaml_char_t,
) -> c_int {
    crate::yaml_parser_scan_tag_handle(parser, directive, start_mark, handle)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_tag_uri(
    parser: *mut yaml_parser_t,
    uri_char: c_int,
    directive: c_int,
    head: *mut yaml_char_t,
    start_mark: yaml_mark_t,
    uri: *mut *mut yaml_char_t,
) -> c_int {
    crate::yaml_parser_scan_tag_uri(parser, uri_char, directive, head, start_mark, uri)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_uri_escapes(
    parser: *mut yaml_parser_t,
    directive: c_int,
    start_mark: yaml_mark_t,
    string: *mut yaml_string_t,
) -> c_int {
    crate::yaml_parser_scan_uri_escapes(parser, directive, start_mark, string)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_block_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    literal: c_int,
) -> c_int {
    crate::yaml_parser_scan_block_scalar(parser, token, literal)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_block_scalar_breaks(
    parser: *mut yaml_parser_t,
    indent: *mut c_int,
    breaks: *mut yaml_string_t,
    start_mark: yaml_mark_t,
    end_mark: *mut yaml_mark_t,
) -> c_int {
    crate::yaml_parser_scan_block_scalar_breaks(
        parser,
        indent,
        breaks,
        start_mark,
        end_mark,
    )
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_flow_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
    single: c_int,
) -> c_int {
    crate::yaml_parser_scan_flow_scalar(parser, token, single)
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_scan_plain_scalar(
    parser: *mut yaml_parser_t,
    token: *mut yaml_token_t,
) -> c_int {
    crate::yaml_parser_scan_plain_scalar(parser, token)
}

// ── writer.c ──

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_emitter_set_writer_error(
    emitter: *mut yaml_emitter_t,
    problem: *const c_char,
) -> c_int {
    crate::yaml_emitter_set_writer_error(emitter, problem)
}

// ── stubs for functions Sonnet did not implement ──
// These return 0 / do nothing — the Rust transpile is missing these functions.
// The C side has real implementations, so any test calling these will show a diff.

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_composer_error(
    parser: *mut yaml_parser_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_loader_set_composer_error(parser, problem, problem_mark) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_set_composer_error_context(
    parser: *mut yaml_parser_t,
    context: *const c_char,
    context_mark: yaml_mark_t,
    problem: *const c_char,
    problem_mark: yaml_mark_t,
) -> c_int {
    crate::yaml_parser_loader_set_composer_error_context(parser, context, context_mark, problem, problem_mark) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn bridge_yaml_parser_delete_aliases(
    _parser: *mut yaml_parser_t,
) {
}
