/*
 * test_suite.c — deterministic tests for libyaml (parser + emitter).
 * Outputs plain text; C and Rust outputs are diffed line-by-line.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#include <yaml.h>
#include "/home/leochanj/Desktop/libyaml/src/yaml_private.h"
#include "test_bridge.h"

/* ------------------------------------------------------------------ helpers */

static const char *event_type_name(yaml_event_type_t t)
{
    switch (t) {
    case YAML_NO_EVENT:             return "NO_EVENT";
    case YAML_STREAM_START_EVENT:   return "STREAM_START";
    case YAML_STREAM_END_EVENT:     return "STREAM_END";
    case YAML_DOCUMENT_START_EVENT: return "DOCUMENT_START";
    case YAML_DOCUMENT_END_EVENT:   return "DOCUMENT_END";
    case YAML_ALIAS_EVENT:          return "ALIAS";
    case YAML_SCALAR_EVENT:         return "SCALAR";
    case YAML_SEQUENCE_START_EVENT: return "SEQUENCE_START";
    case YAML_SEQUENCE_END_EVENT:   return "SEQUENCE_END";
    case YAML_MAPPING_START_EVENT:  return "MAPPING_START";
    case YAML_MAPPING_END_EVENT:    return "MAPPING_END";
    default:                        return "UNKNOWN";
    }
}

static const char *token_type_name(yaml_token_type_t t)
{
    switch (t) {
    case YAML_NO_TOKEN:                     return "NO_TOKEN";
    case YAML_STREAM_START_TOKEN:           return "STREAM_START";
    case YAML_STREAM_END_TOKEN:             return "STREAM_END";
    case YAML_VERSION_DIRECTIVE_TOKEN:      return "VERSION_DIRECTIVE";
    case YAML_TAG_DIRECTIVE_TOKEN:          return "TAG_DIRECTIVE";
    case YAML_DOCUMENT_START_TOKEN:         return "DOCUMENT_START";
    case YAML_DOCUMENT_END_TOKEN:           return "DOCUMENT_END";
    case YAML_BLOCK_SEQUENCE_START_TOKEN:   return "BLOCK_SEQUENCE_START";
    case YAML_BLOCK_MAPPING_START_TOKEN:    return "BLOCK_MAPPING_START";
    case YAML_BLOCK_END_TOKEN:              return "BLOCK_END";
    case YAML_FLOW_SEQUENCE_START_TOKEN:    return "FLOW_SEQUENCE_START";
    case YAML_FLOW_SEQUENCE_END_TOKEN:      return "FLOW_SEQUENCE_END";
    case YAML_FLOW_MAPPING_START_TOKEN:     return "FLOW_MAPPING_START";
    case YAML_FLOW_MAPPING_END_TOKEN:       return "FLOW_MAPPING_END";
    case YAML_BLOCK_ENTRY_TOKEN:            return "BLOCK_ENTRY";
    case YAML_FLOW_ENTRY_TOKEN:             return "FLOW_ENTRY";
    case YAML_KEY_TOKEN:                    return "KEY";
    case YAML_VALUE_TOKEN:                  return "VALUE";
    case YAML_ALIAS_TOKEN:                  return "ALIAS";
    case YAML_ANCHOR_TOKEN:                 return "ANCHOR";
    case YAML_TAG_TOKEN:                    return "TAG";
    case YAML_SCALAR_TOKEN:                 return "SCALAR";
    default:                                return "UNKNOWN";
    }
}

static const char *node_type_name(yaml_node_type_t t)
{
    switch (t) {
    case YAML_NO_NODE:       return "NO_NODE";
    case YAML_SCALAR_NODE:   return "SCALAR";
    case YAML_SEQUENCE_NODE: return "SEQUENCE";
    case YAML_MAPPING_NODE:  return "MAPPING";
    default:                 return "UNKNOWN";
    }
}

static const char *scalar_style_name(yaml_scalar_style_t s)
{
    switch (s) {
    case YAML_ANY_SCALAR_STYLE:           return "any";
    case YAML_PLAIN_SCALAR_STYLE:         return "plain";
    case YAML_SINGLE_QUOTED_SCALAR_STYLE: return "single_quoted";
    case YAML_DOUBLE_QUOTED_SCALAR_STYLE: return "double_quoted";
    case YAML_LITERAL_SCALAR_STYLE:       return "literal";
    case YAML_FOLDED_SCALAR_STYLE:        return "folded";
    default:                              return "unknown";
    }
}

/* Parse yaml_input and print one line per event. */
static void parse_and_print(const char *label, const char *yaml_input)
{
    yaml_parser_t parser;
    yaml_event_t  event;

    printf("=== parse: %s ===\n", label);

    if (!yaml_parser_initialize(&parser)) {
        printf("ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)yaml_input, strlen(yaml_input));

    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("ERROR: %s\n", parser.problem ? parser.problem : "unknown");
            break;
        }
        printf("event: %s", event_type_name(event.type));

        switch (event.type) {
        case YAML_STREAM_START_EVENT:
            printf(" encoding=%d", (int)event.data.stream_start.encoding);
            break;
        case YAML_DOCUMENT_START_EVENT:
            printf(" implicit=%d", event.data.document_start.implicit);
            if (event.data.document_start.version_directive) {
                yaml_version_directive_t *vd =
                    event.data.document_start.version_directive;
                printf(" version=%d.%d", vd->major, vd->minor);
            }
            break;
        case YAML_DOCUMENT_END_EVENT:
            printf(" implicit=%d", event.data.document_end.implicit);
            break;
        case YAML_ALIAS_EVENT:
            printf(" anchor=%s",
                event.data.alias.anchor
                    ? (const char *)event.data.alias.anchor : "(null)");
            break;
        case YAML_SCALAR_EVENT:
            printf(" value=%s",
                event.data.scalar.value
                    ? (const char *)event.data.scalar.value : "(null)");
            if (event.data.scalar.tag)
                printf(" tag=%s", (const char *)event.data.scalar.tag);
            if (event.data.scalar.anchor)
                printf(" anchor=%s", (const char *)event.data.scalar.anchor);
            printf(" style=%s",
                scalar_style_name(event.data.scalar.style));
            printf(" plain_implicit=%d quoted_implicit=%d",
                event.data.scalar.plain_implicit,
                event.data.scalar.quoted_implicit);
            break;
        case YAML_SEQUENCE_START_EVENT:
            if (event.data.sequence_start.tag)
                printf(" tag=%s",
                    (const char *)event.data.sequence_start.tag);
            if (event.data.sequence_start.anchor)
                printf(" anchor=%s",
                    (const char *)event.data.sequence_start.anchor);
            printf(" implicit=%d", event.data.sequence_start.implicit);
            break;
        case YAML_MAPPING_START_EVENT:
            if (event.data.mapping_start.tag)
                printf(" tag=%s",
                    (const char *)event.data.mapping_start.tag);
            if (event.data.mapping_start.anchor)
                printf(" anchor=%s",
                    (const char *)event.data.mapping_start.anchor);
            printf(" implicit=%d", event.data.mapping_start.implicit);
            break;
        default:
            break;
        }
        printf("\n");

        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        if (done) break;
    }

    yaml_parser_delete(&parser);
}

/* Scan yaml_input and print one line per token. */
static void scan_and_print(const char *label, const char *yaml_input)
{
    yaml_parser_t parser;
    yaml_token_t  token;

    printf("=== scan: %s ===\n", label);

    if (!yaml_parser_initialize(&parser)) {
        printf("ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)yaml_input, strlen(yaml_input));

    for (;;) {
        if (!yaml_parser_scan(&parser, &token)) {
            printf("ERROR: %s\n", parser.problem ? parser.problem : "unknown");
            break;
        }
        printf("token: %s", token_type_name(token.type));

        switch (token.type) {
        case YAML_SCALAR_TOKEN:
            printf(" value=%s style=%s",
                token.data.scalar.value
                    ? (const char *)token.data.scalar.value : "(null)",
                scalar_style_name(token.data.scalar.style));
            break;
        case YAML_ALIAS_TOKEN:
            printf(" value=%s",
                token.data.alias.value
                    ? (const char *)token.data.alias.value : "(null)");
            break;
        case YAML_ANCHOR_TOKEN:
            printf(" value=%s",
                token.data.anchor.value
                    ? (const char *)token.data.anchor.value : "(null)");
            break;
        case YAML_TAG_TOKEN:
            printf(" handle=%s suffix=%s",
                token.data.tag.handle
                    ? (const char *)token.data.tag.handle : "(null)",
                token.data.tag.suffix
                    ? (const char *)token.data.tag.suffix : "(null)");
            break;
        case YAML_VERSION_DIRECTIVE_TOKEN:
            printf(" major=%d minor=%d",
                token.data.version_directive.major,
                token.data.version_directive.minor);
            break;
        case YAML_TAG_DIRECTIVE_TOKEN:
            printf(" handle=%s prefix=%s",
                token.data.tag_directive.handle
                    ? (const char *)token.data.tag_directive.handle : "(null)",
                token.data.tag_directive.prefix
                    ? (const char *)token.data.tag_directive.prefix : "(null)");
            break;
        default:
            break;
        }
        printf("\n");

        int done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
        if (done) break;
    }

    yaml_parser_delete(&parser);
}

/* Load document and print all nodes. */
static void print_node(yaml_document_t *doc, int idx, int depth)
{
    yaml_node_t *node = yaml_document_get_node(doc, idx);
    if (!node) return;

    for (int i = 0; i < depth; i++) printf("  ");

    printf("node[%d] type=%s", idx, node_type_name(node->type));
    if (node->tag) printf(" tag=%s", (const char *)node->tag);

    switch (node->type) {
    case YAML_SCALAR_NODE:
        printf(" value=%s style=%s\n",
            node->data.scalar.value
                ? (const char *)node->data.scalar.value : "(null)",
            scalar_style_name(node->data.scalar.style));
        break;
    case YAML_SEQUENCE_NODE: {
        printf("\n");
        yaml_node_item_t *item;
        for (item = node->data.sequence.items.start;
             item < node->data.sequence.items.top; item++) {
            print_node(doc, *item, depth + 1);
        }
        break;
    }
    case YAML_MAPPING_NODE: {
        printf("\n");
        yaml_node_pair_t *pair;
        for (pair = node->data.mapping.pairs.start;
             pair < node->data.mapping.pairs.top; pair++) {
            for (int i = 0; i < depth + 1; i++) printf("  ");
            printf("key:\n");
            print_node(doc, pair->key, depth + 2);
            for (int i = 0; i < depth + 1; i++) printf("  ");
            printf("value:\n");
            print_node(doc, pair->value, depth + 2);
        }
        break;
    }
    default:
        printf("\n");
        break;
    }
}

static void load_and_print(const char *label, const char *yaml_input)
{
    yaml_parser_t   parser;
    yaml_document_t document;

    printf("=== load: %s ===\n", label);

    if (!yaml_parser_initialize(&parser)) {
        printf("ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)yaml_input, strlen(yaml_input));

    int doc_idx = 0;
    for (;;) {
        if (!yaml_parser_load(&parser, &document)) {
            printf("ERROR: %s\n", parser.problem ? parser.problem : "unknown");
            break;
        }
        yaml_node_t *root = yaml_document_get_root_node(&document);
        if (!root) {
            yaml_document_delete(&document);
            break;
        }
        printf("document[%d]:\n", doc_idx++);
        print_node(&document, 1, 1);
        yaml_document_delete(&document);
    }

    yaml_parser_delete(&parser);
}

/* Emit events and print the result. */
static void emit_and_print(const char *label)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[4096];
    size_t         written = 0;

    printf("=== emit: %s ===\n", label);

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* stream start */
    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* document start */
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* mapping start */
    yaml_mapping_start_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_MAP_TAG, 1, YAML_BLOCK_MAPPING_STYLE);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* key: "name" */
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"name", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* value: "libyaml" */
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"libyaml", 7, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* key: "version" */
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"version", 7, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* value: "0.2.5" */
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"0.2.5", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* mapping end */
    yaml_mapping_end_event_initialize(&event);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* document end */
    yaml_document_end_event_initialize(&event, 1);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    /* stream end */
    yaml_stream_end_event_initialize(&event);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    yaml_emitter_flush(&emitter);
    buf[written] = '\0';
    printf("%s\n", buf);
    yaml_emitter_delete(&emitter);
    return;

err:
    printf("ERROR: %s\n",
        emitter.problem ? emitter.problem : "unknown");
    yaml_emitter_delete(&emitter);
}

/* Emit a sequence of scalars. */
static void emit_sequence(const char *label)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[4096];
    size_t         written = 0;

    printf("=== emit: %s ===\n", label);

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    static const char *items[] = { "alpha", "beta", "gamma", "delta" };
    int n = 4;

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    yaml_sequence_start_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_SEQ_TAG, 1, YAML_BLOCK_SEQUENCE_STYLE);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    for (int i = 0; i < n; i++) {
        yaml_scalar_event_initialize(&event, NULL, NULL,
            (yaml_char_t *)items[i], (int)strlen(items[i]),
            1, 1, YAML_PLAIN_SCALAR_STYLE);
        if (!yaml_emitter_emit(&emitter, &event)) goto err;
    }

    yaml_sequence_end_event_initialize(&event);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    yaml_document_end_event_initialize(&event, 1);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    yaml_stream_end_event_initialize(&event);
    if (!yaml_emitter_emit(&emitter, &event)) goto err;

    yaml_emitter_flush(&emitter);
    buf[written] = '\0';
    printf("%s\n", buf);
    yaml_emitter_delete(&emitter);
    return;

err:
    printf("ERROR: %s\n", emitter.problem ? emitter.problem : "unknown");
    yaml_emitter_delete(&emitter);
}

/* Emit using the document API (yaml_emitter_dump). */
static void emit_document_api(const char *label)
{
    yaml_emitter_t  emitter;
    yaml_document_t document;
    unsigned char   buf[4096];
    size_t          written = 0;

    printf("=== emit_doc: %s ===\n", label);

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    if (!yaml_document_initialize(&document, NULL, NULL, NULL, 1, 1)) {
        printf("ERROR: document init failed\n");
        yaml_emitter_delete(&emitter);
        return;
    }

    int map = yaml_document_add_mapping(&document,
        (yaml_char_t *)YAML_MAP_TAG, YAML_BLOCK_MAPPING_STYLE);
    int key1 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"language", 8, YAML_PLAIN_SCALAR_STYLE);
    int val1 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"C", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&document, map, key1, val1);

    int key2 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"standard", 8, YAML_PLAIN_SCALAR_STYLE);
    int val2 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"C99", 3, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&document, map, key2, val2);

    int seq = yaml_document_add_sequence(&document,
        (yaml_char_t *)YAML_SEQ_TAG, YAML_BLOCK_SEQUENCE_STYLE);
    int s1 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"parse", 5, YAML_PLAIN_SCALAR_STYLE);
    int s2 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"emit", 4, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&document, seq, s1);
    yaml_document_append_sequence_item(&document, seq, s2);

    int key3 = yaml_document_add_scalar(&document,
        NULL, (yaml_char_t *)"features", 8, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&document, map, key3, seq);

    if (!yaml_emitter_open(&emitter)) {
        printf("ERROR: emitter open failed\n");
        yaml_document_delete(&document);
        yaml_emitter_delete(&emitter);
        return;
    }

    if (!yaml_emitter_dump(&emitter, &document)) {
        printf("ERROR: emitter dump failed\n");
        yaml_emitter_delete(&emitter);
        return;
    }

    yaml_emitter_close(&emitter);
    yaml_emitter_flush(&emitter);

    buf[written] = '\0';
    printf("%s\n", buf);
    yaml_emitter_delete(&emitter);
}

/* ---------------------------------------------------------------- test cases */

static void test_version(void)
{
    printf("=== version ===\n");
    printf("version_string: %s\n", yaml_get_version_string());

    int major, minor, patch;
    yaml_get_version(&major, &minor, &patch);
    printf("version: %d.%d.%d\n", major, minor, patch);
}

static void test_parse_simple_scalar(void)
{
    parse_and_print("simple_scalar", "hello\n");
}

static void test_parse_mapping(void)
{
    parse_and_print("mapping",
        "key1: value1\n"
        "key2: value2\n"
        "key3: 42\n");
}

static void test_parse_sequence(void)
{
    parse_and_print("sequence",
        "- item1\n"
        "- item2\n"
        "- item3\n");
}

static void test_parse_nested(void)
{
    parse_and_print("nested",
        "outer:\n"
        "  inner1: a\n"
        "  inner2: b\n"
        "list:\n"
        "  - x\n"
        "  - y\n");
}

static void test_parse_flow(void)
{
    parse_and_print("flow", "{a: 1, b: 2, c: [x, y, z]}\n");
}

static void test_parse_multiline_string(void)
{
    parse_and_print("multiline_literal",
        "text: |\n"
        "  line one\n"
        "  line two\n");
}

static void test_parse_folded_string(void)
{
    parse_and_print("folded_string",
        "text: >\n"
        "  folded\n"
        "  content\n");
}

static void test_parse_quoted_scalars(void)
{
    parse_and_print("quoted_scalars",
        "single: 'hello world'\n"
        "double: \"hello\\nworld\"\n");
}

static void test_parse_null_bool(void)
{
    parse_and_print("null_bool",
        "nothing: null\n"
        "flag_true: true\n"
        "flag_false: false\n");
}

static void test_parse_anchor_alias(void)
{
    parse_and_print("anchor_alias",
        "base: &anchor\n"
        "  x: 1\n"
        "  y: 2\n"
        "derived:\n"
        "  <<: *anchor\n"
        "  z: 3\n");
}

static void test_parse_multi_document(void)
{
    parse_and_print("multi_document",
        "---\n"
        "doc: one\n"
        "---\n"
        "doc: two\n"
        "...\n");
}

static void test_parse_version_directive(void)
{
    parse_and_print("version_directive",
        "%YAML 1.1\n"
        "---\n"
        "key: value\n");
}

static void test_parse_tag_directive(void)
{
    parse_and_print("tag_directive",
        "%TAG ! tag:example.com,2024:\n"
        "---\n"
        "!foo bar\n");
}

static void test_parse_empty_collections(void)
{
    parse_and_print("empty_mapping", "{}\n");
    parse_and_print("empty_sequence", "[]\n");
}

static void test_parse_numbers(void)
{
    parse_and_print("numbers",
        "integer: 42\n"
        "negative: -7\n"
        "float: 3.14\n"
        "sci: 1.5e10\n"
        "hex: 0xFF\n");
}

static void test_scan_simple(void)
{
    scan_and_print("scan_mapping",
        "a: 1\nb: 2\n");
}

static void test_scan_flow(void)
{
    scan_and_print("scan_flow", "[x, y, z]\n");
}

static void test_scan_tags(void)
{
    scan_and_print("scan_tags",
        "!!str hello\n");
}

static void test_scan_anchor_alias(void)
{
    scan_and_print("scan_anchor_alias",
        "- &a val\n"
        "- *a\n");
}

static void test_load_scalar(void)
{
    load_and_print("load_scalar", "hello\n");
}

static void test_load_mapping(void)
{
    load_and_print("load_mapping",
        "name: yaml\n"
        "type: library\n");
}

static void test_load_sequence(void)
{
    load_and_print("load_sequence",
        "- one\n"
        "- two\n"
        "- three\n");
}

static void test_load_nested(void)
{
    load_and_print("load_nested",
        "config:\n"
        "  host: localhost\n"
        "  port: 8080\n"
        "tags:\n"
        "  - alpha\n"
        "  - beta\n");
}

static void test_load_multi_document(void)
{
    load_and_print("load_multi_document",
        "---\nfirst: 1\n---\nsecond: 2\n...\n");
}

static void test_emit_mapping(void)
{
    emit_and_print("emit_mapping");
}

static void test_emit_sequence(void)
{
    emit_sequence("emit_sequence");
}

static void test_emit_document_api(void)
{
    emit_document_api("emit_document_api");
}

static void test_emit_quoted_scalar(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[1024];
    size_t         written = 0;

    printf("=== emit: quoted_scalar ===\n");

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &event);

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &event);

    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"hello: world", 12,
        0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &event);

    yaml_document_end_event_initialize(&event, 1);
    yaml_emitter_emit(&emitter, &event);

    yaml_stream_end_event_initialize(&event);
    yaml_emitter_emit(&emitter, &event);

    yaml_emitter_flush(&emitter);
    buf[written] = '\0';
    printf("%s\n", buf);
    yaml_emitter_delete(&emitter);
}

static void test_emit_canonical(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[2048];
    size_t         written = 0;

    printf("=== emit: canonical ===\n");

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_canonical(&emitter, 1);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &event);

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &event);

    yaml_mapping_start_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_MAP_TAG, 0, YAML_ANY_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &event);

    yaml_scalar_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_STR_TAG,
        (yaml_char_t *)"key", 3, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &event);

    yaml_scalar_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_STR_TAG,
        (yaml_char_t *)"value", 5, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &event);

    yaml_mapping_end_event_initialize(&event);
    yaml_emitter_emit(&emitter, &event);

    yaml_document_end_event_initialize(&event, 0);
    yaml_emitter_emit(&emitter, &event);

    yaml_stream_end_event_initialize(&event);
    yaml_emitter_emit(&emitter, &event);

    yaml_emitter_flush(&emitter);
    buf[written] = '\0';
    printf("%s\n", buf);
    yaml_emitter_delete(&emitter);
}

static void test_emit_flow_style(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[1024];
    size_t         written = 0;

    printf("=== emit: flow_style ===\n");

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &event);

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &event);

    yaml_sequence_start_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_SEQ_TAG, 1, YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &event);

    static const char *vals[] = {"1", "2", "3"};
    for (int i = 0; i < 3; i++) {
        yaml_scalar_event_initialize(&event, NULL, NULL,
            (yaml_char_t *)vals[i], 1,
            1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &event);
    }

    yaml_sequence_end_event_initialize(&event);
    yaml_emitter_emit(&emitter, &event);

    yaml_document_end_event_initialize(&event, 1);
    yaml_emitter_emit(&emitter, &event);

    yaml_stream_end_event_initialize(&event);
    yaml_emitter_emit(&emitter, &event);

    yaml_emitter_flush(&emitter);
    buf[written] = '\0';
    printf("%s\n", buf);
    yaml_emitter_delete(&emitter);
}

/* Test bridge_yaml_check_utf8 */
static void test_bridge_check_utf8(void)
{
    printf("=== bridge: check_utf8 ===\n");

    const char *valid_ascii = "hello world";
    int r1 = bridge_yaml_check_utf8(
        (const yaml_char_t *)valid_ascii, strlen(valid_ascii));
    printf("valid_ascii: %d\n", r1);

    const unsigned char valid_utf8[] = { 0xC3, 0xA9, 0x00 }; /* é */
    int r2 = bridge_yaml_check_utf8(valid_utf8, 2);
    printf("valid_utf8: %d\n", r2);

    const unsigned char invalid_utf8[] = { 0xFF, 0xFE, 0x00 };
    int r3 = bridge_yaml_check_utf8(invalid_utf8, 2);
    printf("invalid_utf8: %d\n", r3);
}

/* Test bridge_yaml_emitter_analyze_scalar */
static void test_bridge_analyze_scalar(void)
{
    printf("=== bridge: analyze_scalar ===\n");

    yaml_emitter_t emitter;
    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    unsigned char buf[512];
    size_t written = 0;
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);

    /* We need to emit stream/document/mapping-start first to set state */
    yaml_event_t event;
    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &event);
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &event);
    yaml_mapping_start_event_initialize(&event, NULL,
        (yaml_char_t *)YAML_MAP_TAG, 1, YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &event);

    const char *plain_val  = "hello";
    const char *special    = "key: value";
    const char *empty      = "";

    int r1 = bridge_yaml_emitter_analyze_scalar(
        &emitter, (yaml_char_t *)plain_val, strlen(plain_val));
    printf("plain: %d\n", r1);

    int r2 = bridge_yaml_emitter_analyze_scalar(
        &emitter, (yaml_char_t *)special, strlen(special));
    printf("special: %d\n", r2);

    int r3 = bridge_yaml_emitter_analyze_scalar(
        &emitter, (yaml_char_t *)empty, strlen(empty));
    printf("empty: %d\n", r3);

    yaml_emitter_delete(&emitter);
}

/* Test bridge_yaml_generate_anchor */
static void test_bridge_generate_anchor(void)
{
    printf("=== bridge: generate_anchor ===\n");

    yaml_emitter_t emitter;
    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init failed\n");
        return;
    }
    unsigned char buf[512];
    size_t written = 0;
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf), &written);

    for (int id = 1; id <= 5; id++) {
        yaml_char_t *anchor =
            bridge_yaml_emitter_generate_anchor(&emitter, id);
        printf("anchor[%d]: %s\n", id,
            anchor ? (const char *)anchor : "(null)");
        free(anchor);
    }

    yaml_emitter_delete(&emitter);
}

/* Parse-emit round-trip: parse input then emit. */
static void roundtrip(const char *label, const char *yaml_input)
{
    yaml_parser_t  parser;
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  out[8192];
    size_t         written = 0;

    printf("=== roundtrip: %s ===\n", label);

    if (!yaml_parser_initialize(&parser)) {
        printf("ERROR: parser init\n"); return;
    }
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)yaml_input, strlen(yaml_input));

    if (!yaml_emitter_initialize(&emitter)) {
        printf("ERROR: emitter init\n");
        yaml_parser_delete(&parser);
        return;
    }
    yaml_emitter_set_output_string(&emitter, out, sizeof(out), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("ERROR: parse: %s\n",
                parser.problem ? parser.problem : "unknown");
            break;
        }
        int done = (event.type == YAML_STREAM_END_EVENT);
        if (!yaml_emitter_emit(&emitter, &event)) {
            printf("ERROR: emit: %s\n",
                emitter.problem ? emitter.problem : "unknown");
            break;
        }
        if (done) break;
    }

    yaml_emitter_flush(&emitter);
    out[written] = '\0';
    printf("%s\n", out);

    yaml_parser_delete(&parser);
    yaml_emitter_delete(&emitter);
}

static void test_roundtrip_mapping(void)
{
    roundtrip("mapping",
        "name: test\nvalue: 42\nenabled: true\n");
}

static void test_roundtrip_sequence(void)
{
    roundtrip("sequence",
        "- apple\n- banana\n- cherry\n");
}

static void test_roundtrip_nested(void)
{
    roundtrip("nested",
        "server:\n"
        "  host: localhost\n"
        "  port: 9000\n"
        "clients:\n"
        "  - alice\n"
        "  - bob\n");
}

static void test_roundtrip_flow(void)
{
    roundtrip("flow",
        "{x: 1, y: 2, z: [a, b]}\n");
}

/* -------------------------------------------------------------------- main */

int main(void)
{
    /* version */
    test_version();

    /* parse (event level) */
    test_parse_simple_scalar();
    test_parse_mapping();
    test_parse_sequence();
    test_parse_nested();
    test_parse_flow();
    test_parse_multiline_string();
    test_parse_folded_string();
    test_parse_quoted_scalars();
    test_parse_null_bool();
    test_parse_anchor_alias();
    test_parse_multi_document();
    test_parse_version_directive();
    test_parse_tag_directive();
    test_parse_empty_collections();
    test_parse_numbers();

    /* scan (token level) */
    test_scan_simple();
    test_scan_flow();
    test_scan_tags();
    test_scan_anchor_alias();

    /* load (document/node level) */
    test_load_scalar();
    test_load_mapping();
    test_load_sequence();
    test_load_nested();
    test_load_multi_document();

    /* emit */
    test_emit_mapping();
    test_emit_sequence();
    test_emit_document_api();
    test_emit_quoted_scalar();
    test_emit_canonical();
    test_emit_flow_style();

    /* bridge helpers */
    test_bridge_check_utf8();
    test_bridge_analyze_scalar();
    test_bridge_generate_anchor();

    /* round-trips */
    test_roundtrip_mapping();
    test_roundtrip_sequence();
    test_roundtrip_nested();
    test_roundtrip_flow();

    return 0;
}
