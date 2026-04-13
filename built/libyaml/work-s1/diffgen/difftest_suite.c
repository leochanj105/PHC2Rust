/*
 * test_suite.c — libyaml deterministic test suite
 * Exercises parser (event and document APIs) and emitter.
 * Output is plain text; C and Rust builds can be diffed line-by-line.
 */

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#ifdef HAVE_CONFIG_H
#include "config.h"
#endif
#include <yaml.h>
/* Include yaml_private.h to get yaml_string_t and loader_ctx used in test_bridge.h */
#include "yaml_private.h"
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

static const char *scalar_style_name(yaml_scalar_style_t s)
{
    switch (s) {
    case YAML_ANY_SCALAR_STYLE:           return "any";
    case YAML_PLAIN_SCALAR_STYLE:         return "plain";
    case YAML_SINGLE_QUOTED_SCALAR_STYLE: return "single";
    case YAML_DOUBLE_QUOTED_SCALAR_STYLE: return "double";
    case YAML_LITERAL_SCALAR_STYLE:       return "literal";
    case YAML_FOLDED_SCALAR_STYLE:        return "folded";
    default:                              return "unknown";
    }
}

static const char *sequence_style_name(yaml_sequence_style_t s)
{
    switch (s) {
    case YAML_ANY_SEQUENCE_STYLE:   return "any";
    case YAML_BLOCK_SEQUENCE_STYLE: return "block";
    case YAML_FLOW_SEQUENCE_STYLE:  return "flow";
    default:                        return "unknown";
    }
}

static const char *mapping_style_name(yaml_mapping_style_t s)
{
    switch (s) {
    case YAML_ANY_MAPPING_STYLE:   return "any";
    case YAML_BLOCK_MAPPING_STYLE: return "block";
    case YAML_FLOW_MAPPING_STYLE:  return "flow";
    default:                       return "unknown";
    }
}

static const char *encoding_name(yaml_encoding_t e)
{
    switch (e) {
    case YAML_ANY_ENCODING:     return "any";
    case YAML_UTF8_ENCODING:    return "utf8";
    case YAML_UTF16LE_ENCODING: return "utf16le";
    case YAML_UTF16BE_ENCODING: return "utf16be";
    default:                    return "unknown";
    }
}

/* Print one event's details. */
static void print_event(const yaml_event_t *e)
{
    printf("  event: %s", event_type_name(e->type));
    switch (e->type) {
    case YAML_STREAM_START_EVENT:
        printf(" encoding=%s", encoding_name(e->data.stream_start.encoding));
        break;
    case YAML_DOCUMENT_START_EVENT:
        if (e->data.document_start.version_directive)
            printf(" version=%d.%d",
                   e->data.document_start.version_directive->major,
                   e->data.document_start.version_directive->minor);
        printf(" implicit=%d", e->data.document_start.implicit);
        break;
    case YAML_DOCUMENT_END_EVENT:
        printf(" implicit=%d", e->data.document_end.implicit);
        break;
    case YAML_ALIAS_EVENT:
        printf(" anchor=%s", e->data.alias.anchor
               ? (char *)e->data.alias.anchor : "(null)");
        break;
    case YAML_SCALAR_EVENT:
        if (e->data.scalar.anchor)
            printf(" anchor=%s", (char *)e->data.scalar.anchor);
        if (e->data.scalar.tag)
            printf(" tag=%s", (char *)e->data.scalar.tag);
        printf(" value=[%.*s] style=%s plain_implicit=%d quoted_implicit=%d",
               (int)e->data.scalar.length,
               (char *)e->data.scalar.value,
               scalar_style_name(e->data.scalar.style),
               e->data.scalar.plain_implicit,
               e->data.scalar.quoted_implicit);
        break;
    case YAML_SEQUENCE_START_EVENT:
        if (e->data.sequence_start.anchor)
            printf(" anchor=%s", (char *)e->data.sequence_start.anchor);
        if (e->data.sequence_start.tag)
            printf(" tag=%s", (char *)e->data.sequence_start.tag);
        printf(" implicit=%d style=%s",
               e->data.sequence_start.implicit,
               sequence_style_name(e->data.sequence_start.style));
        break;
    case YAML_MAPPING_START_EVENT:
        if (e->data.mapping_start.anchor)
            printf(" anchor=%s", (char *)e->data.mapping_start.anchor);
        if (e->data.mapping_start.tag)
            printf(" tag=%s", (char *)e->data.mapping_start.tag);
        printf(" implicit=%d style=%s",
               e->data.mapping_start.implicit,
               mapping_style_name(e->data.mapping_start.style));
        break;
    default:
        break;
    }
    printf("\n");
}

/* Parse input string and print all events. */
static void parse_and_print_events(const char *label, const char *input)
{
    yaml_parser_t parser;
    yaml_event_t  event;

    printf("=== parse_events: %s ===\n", label);
    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)input,
                                 strlen(input));
    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("  PARSE_ERROR: %s\n",
                   parser.problem ? parser.problem : "(unknown)");
            yaml_parser_delete(&parser);
            return;
        }
        print_event(&event);
        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        if (done) break;
    }
    yaml_parser_delete(&parser);
}

/* Emit events produced by a parser into a buffer and print the buffer. */
static void parse_and_emit(const char *label, const char *input)
{
    yaml_parser_t  parser;
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[65536];
    size_t         written = 0;

    printf("=== parse_emit: %s ===\n", label);

    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)input,
                                 strlen(input));

    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR: emitter init failed\n");
        yaml_parser_delete(&parser);
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_unicode(&emitter, 1);

    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("  PARSE_ERROR: %s\n",
                   parser.problem ? parser.problem : "(unknown)");
            break;
        }
        int done = (event.type == YAML_STREAM_END_EVENT);
        if (!yaml_emitter_emit(&emitter, &event)) {
            printf("  EMIT_ERROR: %s\n",
                   emitter.problem ? emitter.problem : "(unknown)");
            break;
        }
        if (done) break;
    }
    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);
    yaml_parser_delete(&parser);

    printf("%s", (char *)buf);
    if (written > 0 && buf[written - 1] != '\n')
        printf("\n");
}

/* ------------------------------------------------------------------ tests */

static void test_version(void)
{
    int major, minor, patch;
    printf("=== test_version ===\n");
    printf("version_string=%s\n", yaml_get_version_string());
    yaml_get_version(&major, &minor, &patch);
    printf("version=%d.%d.%d\n", major, minor, patch);
}

static void test_parse_scalar(void)
{
    parse_and_print_events("scalar_plain",    "hello\n");
    parse_and_print_events("scalar_quoted",   "\"world\"\n");
    parse_and_print_events("scalar_single",   "'foo bar'\n");
    parse_and_print_events("scalar_null",     "null\n");
    parse_and_print_events("scalar_bool_t",   "true\n");
    parse_and_print_events("scalar_bool_f",   "false\n");
    parse_and_print_events("scalar_int",      "42\n");
    parse_and_print_events("scalar_float",    "3.14\n");
    parse_and_print_events("scalar_empty",    "\"\"\n");
}

static void test_parse_sequence(void)
{
    parse_and_print_events("seq_block",
        "- alpha\n"
        "- beta\n"
        "- gamma\n");
    parse_and_print_events("seq_flow",
        "[1, 2, 3]\n");
    parse_and_print_events("seq_nested",
        "- - a\n"
        "  - b\n"
        "- - c\n"
        "  - d\n");
    parse_and_print_events("seq_empty_block", "[]\n");
}

static void test_parse_mapping(void)
{
    parse_and_print_events("map_block",
        "key: value\n"
        "foo: bar\n");
    parse_and_print_events("map_flow",
        "{a: 1, b: 2}\n");
    parse_and_print_events("map_nested",
        "outer:\n"
        "  inner: deep\n"
        "  x: y\n");
    parse_and_print_events("map_empty_flow", "{}\n");
}

static void test_parse_complex(void)
{
    parse_and_print_events("complex_mixed",
        "name: Alice\n"
        "scores:\n"
        "  - 10\n"
        "  - 20\n"
        "  - 30\n"
        "active: true\n");

    parse_and_print_events("complex_literal_scalar",
        "text: |\n"
        "  line one\n"
        "  line two\n");

    parse_and_print_events("complex_folded_scalar",
        "text: >\n"
        "  folded\n"
        "  text\n");

    parse_and_print_events("complex_multiline_str",
        "key: \"multi\\nline\"\n");

    parse_and_print_events("complex_seq_of_maps",
        "- name: Bob\n"
        "  age: 30\n"
        "- name: Carol\n"
        "  age: 25\n");
}

static void test_parse_anchors(void)
{
    parse_and_print_events("anchor_basic",
        "base: &anchor\n"
        "  x: 1\n"
        "  y: 2\n"
        "derived: *anchor\n");

    parse_and_print_events("anchor_scalar",
        "a: &s hello\n"
        "b: *s\n");

    parse_and_print_events("anchor_seq",
        "list: &l\n"
        "  - 1\n"
        "  - 2\n"
        "copy: *l\n");
}

static void test_parse_tags(void)
{
    parse_and_print_events("tag_explicit_str",
        "!!str hello\n");
    parse_and_print_events("tag_explicit_int",
        "!!int 42\n");
    parse_and_print_events("tag_explicit_bool",
        "!!bool true\n");
    parse_and_print_events("tag_explicit_null",
        "!!null ~\n");
    parse_and_print_events("tag_seq",
        "!!seq\n"
        "- a\n"
        "- b\n");
    parse_and_print_events("tag_map",
        "!!map\n"
        "key: val\n");
}

static void test_parse_directive(void)
{
    parse_and_print_events("directive_version",
        "%YAML 1.1\n"
        "---\n"
        "foo: bar\n");

    parse_and_print_events("directive_tag",
        "%TAG ! tag:example.com,2000:app/\n"
        "---\n"
        "!type value\n");
}

static void test_parse_multidoc(void)
{
    parse_and_print_events("multidoc",
        "---\n"
        "doc: one\n"
        "---\n"
        "doc: two\n"
        "...\n");
}

static void test_parse_error_handling(void)
{
    yaml_parser_t parser;
    yaml_event_t  event;
    const char *bad = "key: [unclosed\n";

    printf("=== parse_error_handling ===\n");
    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)bad,
                                 strlen(bad));
    int ok = 1;
    while (ok) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("  caught_error=%s\n",
                   parser.problem ? parser.problem : "(unknown)");
            ok = 0;
        } else {
            if (event.type == YAML_STREAM_END_EVENT) {
                yaml_event_delete(&event);
                break;
            }
            yaml_event_delete(&event);
        }
    }
    yaml_parser_delete(&parser);
}

/* --- token scanner test --- */
static void test_scan_tokens(void)
{
    yaml_parser_t parser;
    yaml_token_t  token;
    const char *input = "key: value\n";

    printf("=== test_scan_tokens ===\n");
    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)input,
                                 strlen(input));
    for (;;) {
        if (!yaml_parser_scan(&parser, &token)) {
            printf("  SCAN_ERROR\n");
            break;
        }
        printf("  token_type=%d\n", token.type);
        int done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
        if (done) break;
    }
    yaml_parser_delete(&parser);
}

/* --- document API test --- */
static void print_node(yaml_document_t *doc, int node_id, int depth)
{
    yaml_node_t *node = yaml_document_get_node(doc, node_id);
    if (!node) return;

    char indent[64];
    int i;
    if (depth > 30) depth = 30;
    for (i = 0; i < depth * 2 && i < 62; i++) indent[i] = ' ';
    indent[i] = '\0';

    switch (node->type) {
    case YAML_SCALAR_NODE:
        printf("%sscalar tag=%s value=[%.*s]\n",
               indent,
               node->tag ? (char *)node->tag : "(null)",
               (int)node->data.scalar.length,
               (char *)node->data.scalar.value);
        break;
    case YAML_SEQUENCE_NODE: {
        printf("%ssequence tag=%s\n", indent,
               node->tag ? (char *)node->tag : "(null)");
        yaml_node_item_t *item;
        for (item = node->data.sequence.items.start;
             item < node->data.sequence.items.top; item++) {
            print_node(doc, *item, depth + 1);
        }
        break;
    }
    case YAML_MAPPING_NODE: {
        printf("%smapping tag=%s\n", indent,
               node->tag ? (char *)node->tag : "(null)");
        yaml_node_pair_t *pair;
        for (pair = node->data.mapping.pairs.start;
             pair < node->data.mapping.pairs.top; pair++) {
            printf("%s  key:\n", indent);
            print_node(doc, pair->key, depth + 2);
            printf("%s  val:\n", indent);
            print_node(doc, pair->value, depth + 2);
        }
        break;
    }
    default:
        printf("%s(empty)\n", indent);
        break;
    }
}

static void load_and_print_document(const char *label, const char *input)
{
    yaml_parser_t   parser;
    yaml_document_t doc;

    printf("=== load_document: %s ===\n", label);
    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR: parser init failed\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)input,
                                 strlen(input));
    int doc_count = 0;
    for (;;) {
        if (!yaml_parser_load(&parser, &doc)) {
            printf("  LOAD_ERROR: %s\n",
                   parser.problem ? parser.problem : "(unknown)");
            break;
        }
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        if (!root) {
            yaml_document_delete(&doc);
            break;
        }
        printf("  document %d:\n", ++doc_count);
        print_node(&doc, 1, 2);
        yaml_document_delete(&doc);
    }
    yaml_parser_delete(&parser);
}

static void test_load_document(void)
{
    load_and_print_document("scalar",     "hello\n");
    load_and_print_document("map",        "a: 1\nb: 2\n");
    load_and_print_document("seq",        "- x\n- y\n- z\n");
    load_and_print_document("nested",
        "person:\n"
        "  name: Dave\n"
        "  hobbies:\n"
        "    - coding\n"
        "    - hiking\n");
    load_and_print_document("multidoc",
        "---\nfirst: doc\n---\nsecond: doc\n");
}

/* --- document creation API --- */
static void test_document_create_api(void)
{
    yaml_document_t doc;
    yaml_emitter_t  emitter;
    unsigned char   buf[16384];
    size_t          written = 0;

    printf("=== test_document_create_api ===\n");

    /* Build: {name: Eve, scores: [100, 200]} */
    if (!yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1)) {
        printf("  ERROR: document init failed\n");
        return;
    }

    int mapping = yaml_document_add_mapping(&doc,
                      (yaml_char_t *)YAML_MAP_TAG,
                      YAML_BLOCK_MAPPING_STYLE);

    int key_name = yaml_document_add_scalar(&doc,
                      (yaml_char_t *)YAML_STR_TAG,
                      (yaml_char_t *)"name", 4,
                      YAML_PLAIN_SCALAR_STYLE);
    int val_name = yaml_document_add_scalar(&doc,
                      (yaml_char_t *)YAML_STR_TAG,
                      (yaml_char_t *)"Eve", 3,
                      YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, mapping, key_name, val_name);

    int key_scores = yaml_document_add_scalar(&doc,
                        (yaml_char_t *)YAML_STR_TAG,
                        (yaml_char_t *)"scores", 6,
                        YAML_PLAIN_SCALAR_STYLE);
    int seq = yaml_document_add_sequence(&doc,
                  (yaml_char_t *)YAML_SEQ_TAG,
                  YAML_BLOCK_SEQUENCE_STYLE);
    yaml_document_append_mapping_pair(&doc, mapping, key_scores, seq);

    int s100 = yaml_document_add_scalar(&doc,
                   (yaml_char_t *)YAML_INT_TAG,
                   (yaml_char_t *)"100", 3,
                   YAML_PLAIN_SCALAR_STYLE);
    int s200 = yaml_document_add_scalar(&doc,
                   (yaml_char_t *)YAML_INT_TAG,
                   (yaml_char_t *)"200", 3,
                   YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s100);
    yaml_document_append_sequence_item(&doc, seq, s200);

    /* Dump via emitter */
    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR: emitter init failed\n");
        yaml_document_delete(&doc);
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_unicode(&emitter, 1);

    if (!yaml_emitter_open(&emitter)) {
        printf("  OPEN_ERROR: %s\n", emitter.problem ? emitter.problem : "(unknown)");
    } else if (!yaml_emitter_dump(&emitter, &doc)) {
        printf("  DUMP_ERROR: %s\n", emitter.problem ? emitter.problem : "(unknown)");
    } else if (!yaml_emitter_close(&emitter)) {
        printf("  CLOSE_ERROR: %s\n", emitter.problem ? emitter.problem : "(unknown)");
    } else {
        yaml_emitter_flush(&emitter);
        printf("%s", (char *)buf);
        if (written > 0 && buf[written - 1] != '\n')
            printf("\n");
    }
    yaml_emitter_delete(&emitter);
    /* Note: yaml_emitter_dump takes ownership of doc, no need to delete */
}

/* --- emitter event API --- */
static void test_emit_events(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[16384];
    size_t         written = 0;

    printf("=== test_emit_events ===\n");

    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR: emitter init failed\n");
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_unicode(&emitter, 1);

#define EMIT(init_call) \
    do { \
        if (!(init_call)) { printf("  INIT_ERROR\n"); goto done; } \
        if (!yaml_emitter_emit(&emitter, &event)) { \
            printf("  EMIT_ERROR: %s\n", emitter.problem ? emitter.problem : "(unknown)"); \
            goto done; \
        } \
    } while (0)

    EMIT(yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING));
    EMIT(yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1));
    EMIT(yaml_mapping_start_event_initialize(&event, NULL, NULL, 1,
                                              YAML_BLOCK_MAPPING_STYLE));

    EMIT(yaml_scalar_event_initialize(&event, NULL, NULL,
                                      (yaml_char_t *)"city", 4,
                                      1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMIT(yaml_scalar_event_initialize(&event, NULL, NULL,
                                      (yaml_char_t *)"Paris", 5,
                                      1, 1, YAML_PLAIN_SCALAR_STYLE));

    EMIT(yaml_scalar_event_initialize(&event, NULL, NULL,
                                      (yaml_char_t *)"pop", 3,
                                      1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMIT(yaml_scalar_event_initialize(&event, NULL, NULL,
                                      (yaml_char_t *)"2161000", 7,
                                      1, 1, YAML_PLAIN_SCALAR_STYLE));

    EMIT(yaml_mapping_end_event_initialize(&event));
    EMIT(yaml_document_end_event_initialize(&event, 1));
    EMIT(yaml_stream_end_event_initialize(&event));

done:
    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);

#undef EMIT

    printf("%s", (char *)buf);
    if (written > 0 && buf[written - 1] != '\n')
        printf("\n");
}

static void test_emit_sequence(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[16384];
    size_t         written = 0;
    const char    *items[] = {"alpha", "beta", "gamma", NULL};

    printf("=== test_emit_sequence ===\n");

    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR: emitter init failed\n");
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_unicode(&emitter, 1);

#define EMIT2(init_call) \
    do { \
        if (!(init_call)) { printf("  INIT_ERROR\n"); goto done2; } \
        if (!yaml_emitter_emit(&emitter, &event)) { \
            printf("  EMIT_ERROR: %s\n", emitter.problem ? emitter.problem : "(unknown)"); \
            goto done2; \
        } \
    } while (0)

    EMIT2(yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING));
    EMIT2(yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1));
    EMIT2(yaml_sequence_start_event_initialize(&event, NULL, NULL, 1,
                                                YAML_BLOCK_SEQUENCE_STYLE));
    for (int i = 0; items[i]; i++) {
        EMIT2(yaml_scalar_event_initialize(&event, NULL, NULL,
                                            (yaml_char_t *)items[i],
                                            (int)strlen(items[i]),
                                            1, 1, YAML_PLAIN_SCALAR_STYLE));
    }
    EMIT2(yaml_sequence_end_event_initialize(&event));
    EMIT2(yaml_document_end_event_initialize(&event, 1));
    EMIT2(yaml_stream_end_event_initialize(&event));

done2:
    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);

#undef EMIT2

    printf("%s", (char *)buf);
    if (written > 0 && buf[written - 1] != '\n')
        printf("\n");
}

/* --- parse then emit round-trip --- */
static void test_roundtrip(void)
{
    const char *inputs[] = {
        "hello: world\n",
        "- 1\n- 2\n- 3\n",
        "a: 1\nb:\n  c: 2\n  d: 3\n",
        "\"quoted string\"\n",
        "key: 'single quoted'\n",
        NULL
    };
    const char *labels[] = {
        "simple_map", "int_seq", "nested_map", "quoted_scalar",
        "single_quoted_val", NULL
    };
    for (int i = 0; inputs[i]; i++)
        parse_and_emit(labels[i], inputs[i]);
}

/* --- bridge function tests --- */
static void test_bridge_check_utf8(void)
{
    printf("=== test_bridge_check_utf8 ===\n");
    /* valid ASCII */
    const unsigned char *ascii = (const unsigned char *)"hello";
    printf("check_utf8(ascii)=%d\n",
           bridge_yaml_check_utf8(ascii, 5));
    /* valid UTF-8 two-byte */
    const unsigned char utf2[] = {0xC3, 0xA9, 0x00}; /* é */
    printf("check_utf8(utf2)=%d\n",
           bridge_yaml_check_utf8(utf2, 2));
    /* invalid: lone continuation byte */
    const unsigned char bad[] = {0x80, 0x00};
    printf("check_utf8(bad)=%d\n",
           bridge_yaml_check_utf8(bad, 1));
}

static void test_bridge_read_handler(void)
{
    printf("=== test_bridge_read_handler ===\n");
    const char *data = "abcdef";
    /* Set up a string reader via parser to exercise bridge_yaml_string_read_handler */
    yaml_parser_t parser;
    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR: parser init\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)data,
                                 strlen(data));
    /* Just parse a stream — exercises the internal string read handler */
    yaml_event_t event;
    while (yaml_parser_parse(&parser, &event)) {
        if (event.type == YAML_STREAM_END_EVENT) {
            yaml_event_delete(&event);
            break;
        }
        yaml_event_delete(&event);
    }
    printf("  read_handler_ok=1\n");
    yaml_parser_delete(&parser);
}

static void test_bridge_emitter_anchor(void)
{
    printf("=== test_bridge_emitter_anchor ===\n");
    yaml_emitter_t emitter;
    unsigned char  buf[1024];
    size_t         written = 0;
    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR: emitter init\n");
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);

    /* Test generate_anchor for anchor IDs 1, 2, 3 */
    for (int id = 1; id <= 3; id++) {
        yaml_char_t *anchor = bridge_yaml_emitter_generate_anchor(&emitter, id);
        printf("  anchor(%d)=%s\n", id, anchor ? (char *)anchor : "(null)");
        free(anchor);
    }
    yaml_emitter_delete(&emitter);
}

static void test_bridge_emitter_check_empty(void)
{
    printf("=== test_bridge_emitter_check_empty ===\n");
    yaml_parser_t  parser;
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[4096];
    size_t         written = 0;
    const char    *input = "- a\n- b\n";

    if (!yaml_parser_initialize(&parser)) {
        printf("  ERROR\n");
        return;
    }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)input,
                                 strlen(input));
    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR\n");
        yaml_parser_delete(&parser);
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_unicode(&emitter, 1);

    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) break;
        int done = (event.type == YAML_STREAM_END_EVENT);
        if (!yaml_emitter_emit(&emitter, &event)) break;
        if (done) break;
    }
    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);
    yaml_parser_delete(&parser);

    printf("  emitted=%s", (char *)buf);
    if (written > 0 && buf[written - 1] != '\n') printf("\n");
}

/* --- misc edge cases --- */
static void test_empty_input(void)
{
    parse_and_print_events("empty_string", "");
    parse_and_print_events("only_newline", "\n");
    parse_and_print_events("only_doc_start", "---\n");
}

static void test_special_scalars(void)
{
    parse_and_print_events("tilde",          "~\n");
    parse_and_print_events("yes",            "yes\n");
    parse_and_print_events("no",             "no\n");
    parse_and_print_events("on",             "on\n");
    parse_and_print_events("off",            "off\n");
    parse_and_print_events("sci_notation",   "1.0e5\n");
    parse_and_print_events("hex_int",        "0xFF\n");
    parse_and_print_events("octal_int",      "0o77\n");
    parse_and_print_events("inf",            ".inf\n");
    parse_and_print_events("nan",            ".nan\n");
}

static void test_unicode_scalars(void)
{
    /* UTF-8 content */
    parse_and_print_events("unicode_simple", "caf\xC3\xA9\n");
    parse_and_emit("unicode_roundtrip",      "greeting: caf\xC3\xA9\n");
}

static void test_canonical_emitter(void)
{
    yaml_parser_t  parser;
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[16384];
    size_t         written = 0;
    const char    *input = "a: 1\nb: [2, 3]\n";

    printf("=== test_canonical_emitter ===\n");
    if (!yaml_parser_initialize(&parser)) { printf("  ERROR\n"); return; }
    yaml_parser_set_input_string(&parser,
                                 (const unsigned char *)input,
                                 strlen(input));
    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR\n");
        yaml_parser_delete(&parser);
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_canonical(&emitter, 1);
    yaml_emitter_set_unicode(&emitter, 1);

    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) break;
        int done = (event.type == YAML_STREAM_END_EVENT);
        if (!yaml_emitter_emit(&emitter, &event)) break;
        if (done) break;
    }
    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);
    yaml_parser_delete(&parser);

    printf("%s", (char *)buf);
    if (written > 0 && buf[written - 1] != '\n') printf("\n");
}

static void test_emitter_flow_style(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  buf[16384];
    size_t         written = 0;

    printf("=== test_emitter_flow_style ===\n");

    if (!yaml_emitter_initialize(&emitter)) {
        printf("  ERROR\n");
        return;
    }
    memset(buf, 0, sizeof(buf));
    yaml_emitter_set_output_string(&emitter, buf, sizeof(buf) - 1, &written);
    yaml_emitter_set_unicode(&emitter, 1);

#define EMITF(init_call) \
    do { \
        if (!(init_call)) { printf("  INIT_ERROR\n"); goto donef; } \
        if (!yaml_emitter_emit(&emitter, &event)) { \
            printf("  EMIT_ERROR: %s\n", emitter.problem ? emitter.problem : "(unknown)"); \
            goto donef; \
        } \
    } while (0)

    EMITF(yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING));
    EMITF(yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1));
    EMITF(yaml_mapping_start_event_initialize(&event, NULL, NULL, 1,
                                               YAML_FLOW_MAPPING_STYLE));
    EMITF(yaml_scalar_event_initialize(&event, NULL, NULL,
                                        (yaml_char_t *)"x", 1,
                                        1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMITF(yaml_scalar_event_initialize(&event, NULL, NULL,
                                        (yaml_char_t *)"10", 2,
                                        1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMITF(yaml_scalar_event_initialize(&event, NULL, NULL,
                                        (yaml_char_t *)"y", 1,
                                        1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMITF(yaml_sequence_start_event_initialize(&event, NULL, NULL, 1,
                                                YAML_FLOW_SEQUENCE_STYLE));
    EMITF(yaml_scalar_event_initialize(&event, NULL, NULL,
                                        (yaml_char_t *)"1", 1,
                                        1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMITF(yaml_scalar_event_initialize(&event, NULL, NULL,
                                        (yaml_char_t *)"2", 1,
                                        1, 1, YAML_PLAIN_SCALAR_STYLE));
    EMITF(yaml_sequence_end_event_initialize(&event));
    EMITF(yaml_mapping_end_event_initialize(&event));
    EMITF(yaml_document_end_event_initialize(&event, 1));
    EMITF(yaml_stream_end_event_initialize(&event));

donef:
    yaml_emitter_flush(&emitter);
    yaml_emitter_delete(&emitter);

#undef EMITF

    printf("%s", (char *)buf);
    if (written > 0 && buf[written - 1] != '\n') printf("\n");
}

/* ------------------------------------------------------------------ main */


/* ── Independent test execution wrapper ── */
#include <unistd.h>
#include <sys/wait.h>
#include <signal.h>
#include <string.h>

static void run_test(const char *name, void (*fn)(void), int timeout_sec) {
    fflush(stdout);
    fflush(stderr);
    pid_t pid = fork();
    if (pid == 0) {
        /* Child: run the test, exit */
        fn();
        fflush(stdout);
        _exit(0);
    }
    /* Parent: wait with timeout using alarm */
    int status;
    /* First try non-blocking wait — most tests finish instantly */
    usleep(1000); /* 1ms grace period */
    pid_t r = waitpid(pid, &status, WNOHANG);
    if (r == pid) goto done;
    if (r < 0) goto done;
    /* Still running — poll with 100ms intervals up to timeout */
    int polls = timeout_sec * 10; /* 100ms per poll */
    for (int i = 0; i < polls; i++) {
        usleep(100000); /* 100ms */
        r = waitpid(pid, &status, WNOHANG);
        if (r == pid) goto done;
        if (r < 0) goto done;
    }
    /* Timeout — kill child */
    kill(pid, SIGKILL);
    waitpid(pid, &status, 0);
    printf("FAULT %s TIMEOUT\n", name);
    fflush(stdout);
    return;
done:
    if (WIFSIGNALED(status)) {
        printf("FAULT %s SIGNAL %d\n", name, WTERMSIG(status));
        fflush(stdout);
    } else if (WIFEXITED(status) && WEXITSTATUS(status) != 0) {
        printf("FAULT %s EXIT %d\n", name, WEXITSTATUS(status));
        fflush(stdout);
    }
}
/* ── End wrapper ── */

int main(void)
{
    run_test("version", test_version, 2);
    run_test("parse_scalar", test_parse_scalar, 2);
    run_test("parse_sequence", test_parse_sequence, 2);
    run_test("parse_mapping", test_parse_mapping, 2);
    run_test("parse_complex", test_parse_complex, 2);
    run_test("parse_anchors", test_parse_anchors, 2);
    run_test("parse_tags", test_parse_tags, 2);
    run_test("parse_directive", test_parse_directive, 2);
    run_test("parse_multidoc", test_parse_multidoc, 2);
    run_test("parse_error_handling", test_parse_error_handling, 2);
    run_test("scan_tokens", test_scan_tokens, 2);
    run_test("load_document", test_load_document, 2);
    run_test("document_create_api", test_document_create_api, 2);
    run_test("emit_events", test_emit_events, 2);
    run_test("emit_sequence", test_emit_sequence, 2);
    run_test("roundtrip", test_roundtrip, 2);
    run_test("bridge_check_utf8", test_bridge_check_utf8, 2);
    run_test("bridge_read_handler", test_bridge_read_handler, 2);
    run_test("bridge_emitter_anchor", test_bridge_emitter_anchor, 2);
    run_test("bridge_emitter_check_empty", test_bridge_emitter_check_empty, 2);
    run_test("empty_input", test_empty_input, 2);
    run_test("special_scalars", test_special_scalars, 2);
    run_test("unicode_scalars", test_unicode_scalars, 2);
    run_test("canonical_emitter", test_canonical_emitter, 2);
    run_test("emitter_flow_style", test_emitter_flow_style, 2);
    return 0;
}
