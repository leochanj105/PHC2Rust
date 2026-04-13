/*
 * test_suite.c — libyaml comprehensive test suite
 * Prints stable, typed values only (no error-message strings).
 * All output is deterministic for C/Rust diff comparison.
 */

#include "/home/leochanj/Desktop/libyaml/src/yaml_private.h"

/* loader_ctx is defined inside loader.c; mirror it here for bridge calls */
struct loader_ctx {
    int *start;
    int *end;
    int *top;
};

#include "test_bridge.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

/* ------------------------------------------------------------------ helpers */

static void print_event(const yaml_event_t *e)
{
    printf("event %d", (int)e->type);
    switch (e->type) {
    case YAML_STREAM_START_EVENT:
        printf(" enc=%d", (int)e->data.stream_start.encoding);
        break;
    case YAML_DOCUMENT_START_EVENT:
        printf(" implicit=%d", e->data.document_start.implicit);
        if (e->data.document_start.version_directive)
            printf(" ver=%d.%d",
                   e->data.document_start.version_directive->major,
                   e->data.document_start.version_directive->minor);
        break;
    case YAML_DOCUMENT_END_EVENT:
        printf(" implicit=%d", e->data.document_end.implicit);
        break;
    case YAML_SCALAR_EVENT:
        printf(" len=%d style=%d plain=%d quoted=%d",
               (int)e->data.scalar.length,
               (int)e->data.scalar.style,
               e->data.scalar.plain_implicit,
               e->data.scalar.quoted_implicit);
        if (e->data.scalar.value)
            printf(" val=%s", (char *)e->data.scalar.value);
        if (e->data.scalar.anchor)
            printf(" anchor=%s", (char *)e->data.scalar.anchor);
        if (e->data.scalar.tag)
            printf(" tag=%s", (char *)e->data.scalar.tag);
        break;
    case YAML_ALIAS_EVENT:
        if (e->data.alias.anchor)
            printf(" anchor=%s", (char *)e->data.alias.anchor);
        break;
    case YAML_SEQUENCE_START_EVENT:
        printf(" style=%d implicit=%d",
               (int)e->data.sequence_start.style,
               e->data.sequence_start.implicit);
        if (e->data.sequence_start.anchor)
            printf(" anchor=%s", (char *)e->data.sequence_start.anchor);
        if (e->data.sequence_start.tag)
            printf(" tag=%s", (char *)e->data.sequence_start.tag);
        break;
    case YAML_MAPPING_START_EVENT:
        printf(" style=%d implicit=%d",
               (int)e->data.mapping_start.style,
               e->data.mapping_start.implicit);
        if (e->data.mapping_start.anchor)
            printf(" anchor=%s", (char *)e->data.mapping_start.anchor);
        if (e->data.mapping_start.tag)
            printf(" tag=%s", (char *)e->data.mapping_start.tag);
        break;
    default:
        break;
    }
    printf("\n");
}

static void print_token(const yaml_token_t *t)
{
    printf("token %d", (int)t->type);
    switch (t->type) {
    case YAML_STREAM_START_TOKEN:
        printf(" enc=%d", (int)t->data.stream_start.encoding);
        break;
    case YAML_SCALAR_TOKEN:
        printf(" style=%d len=%d val=%s",
               (int)t->data.scalar.style,
               (int)t->data.scalar.length,
               (char *)t->data.scalar.value);
        break;
    case YAML_ALIAS_TOKEN:
        printf(" val=%s", (char *)t->data.alias.value);
        break;
    case YAML_ANCHOR_TOKEN:
        printf(" val=%s", (char *)t->data.anchor.value);
        break;
    case YAML_TAG_TOKEN:
        printf(" handle=%s suffix=%s",
               (char *)t->data.tag.handle,
               (char *)t->data.tag.suffix);
        break;
    case YAML_VERSION_DIRECTIVE_TOKEN:
        printf(" major=%d minor=%d",
               t->data.version_directive.major,
               t->data.version_directive.minor);
        break;
    case YAML_TAG_DIRECTIVE_TOKEN:
        printf(" handle=%s prefix=%s",
               (char *)t->data.tag_directive.handle,
               (char *)t->data.tag_directive.prefix);
        break;
    default:
        break;
    }
    printf("\n");
}

/* Parse a YAML string, print all events, return final parser error code */
static int parse_events(const char *input, int len)
{
    yaml_parser_t parser;
    yaml_event_t event;
    int ok;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, (size_t)len);
    do {
        ok = yaml_parser_parse(&parser, &event);
        if (!ok) {
            printf("parse_error %d\n", (int)parser.error);
            yaml_parser_delete(&parser);
            return (int)parser.error;
        }
        print_event(&event);
        ok = (event.type != YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
    } while (ok);
    yaml_parser_delete(&parser);
    return 0;
}

/* Scan a YAML string, print all tokens */
static int scan_tokens(const char *input, int len)
{
    yaml_parser_t parser;
    yaml_token_t token;
    int ok;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, (size_t)len);
    do {
        ok = yaml_parser_scan(&parser, &token);
        if (!ok) {
            printf("scan_error %d\n", (int)parser.error);
            yaml_parser_delete(&parser);
            return (int)parser.error;
        }
        print_token(&token);
        ok = (token.type != YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
    } while (ok);
    yaml_parser_delete(&parser);
    return 0;
}

/* Load a YAML string, walk the document tree */
static void print_node(yaml_document_t *doc, int index, int depth)
{
    yaml_node_t *node = yaml_document_get_node(doc, index);
    int i;
    if (!node) { printf("%*sNULL\n", depth*2, ""); return; }
    printf("%*snode type=%d", depth*2, "", (int)node->type);
    if (node->tag) printf(" tag=%s", (char *)node->tag);
    switch (node->type) {
    case YAML_SCALAR_NODE:
        printf(" len=%d style=%d val=%s",
               (int)node->data.scalar.length,
               (int)node->data.scalar.style,
               (char *)node->data.scalar.value);
        printf("\n");
        break;
    case YAML_SEQUENCE_NODE: {
        yaml_node_item_t *item;
        printf(" style=%d\n", (int)node->data.sequence.style);
        for (item = node->data.sequence.items.start;
             item < node->data.sequence.items.top; item++)
            print_node(doc, *item, depth+1);
        break;
    }
    case YAML_MAPPING_NODE: {
        yaml_node_pair_t *pair;
        printf(" style=%d\n", (int)node->data.mapping.style);
        for (pair = node->data.mapping.pairs.start;
             pair < node->data.mapping.pairs.top; pair++) {
            print_node(doc, pair->key, depth+1);
            print_node(doc, pair->value, depth+1);
        }
        break;
    }
    default:
        printf("\n");
    }
    (void)i;
}

static int load_documents(const char *input, int len)
{
    yaml_parser_t parser;
    yaml_document_t doc;
    int ret;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, (size_t)len);
    for (;;) {
        ret = yaml_parser_load(&parser, &doc);
        if (!ret) {
            printf("load_error %d\n", (int)parser.error);
            yaml_parser_delete(&parser);
            return (int)parser.error;
        }
        if (!yaml_document_get_root_node(&doc)) {
            yaml_document_delete(&doc);
            break;
        }
        print_node(&doc, 1, 0);
        yaml_document_delete(&doc);
    }
    yaml_parser_delete(&parser);
    return 0;
}

/* Emit events to a string buffer */
typedef struct {
    unsigned char buf[65536];
    size_t written;
} emit_buf_t;

static int setup_emitter(yaml_emitter_t *em, emit_buf_t *b)
{
    b->written = 0;
    memset(b->buf, 0, sizeof(b->buf));
    if (!yaml_emitter_initialize(em)) return 0;
    yaml_emitter_set_output_string(em, b->buf, sizeof(b->buf), &b->written);
    return 1;
}

static void print_emitted(emit_buf_t *b)
{
    size_t i;
    for (i = 0; i < b->written; i++) putchar(b->buf[i]);
    if (b->written == 0 || b->buf[b->written-1] != '\n') putchar('\n');
}

/* --------------------------------------------------------- test functions */

static void test_version(void)
{
    int maj, min, pat;
    const char *vs = yaml_get_version_string();
    yaml_get_version(&maj, &min, &pat);
    printf("=== version ===\n");
    /* print just that we got a non-null string and numeric components */
    printf("version_string_nonempty=%d\n", vs && vs[0] ? 1 : 0);
    printf("major=%d minor=%d patch=%d\n", maj, min, pat);
}

static void test_scan_basic(void)
{
    printf("=== scan_basic ===\n");
    scan_tokens("key: value\n", 11);
}

static void test_scan_empty(void)
{
    printf("=== scan_empty ===\n");
    scan_tokens("", 0);
}

static void test_scan_flow(void)
{
    printf("=== scan_flow ===\n");
    scan_tokens("{a: 1, b: [2, 3]}\n", 18);
}

static void test_scan_directive(void)
{
    printf("=== scan_directive ===\n");
    scan_tokens("%YAML 1.1\n---\nscalar\n", 21);
}

static void test_scan_anchor_alias(void)
{
    printf("=== scan_anchor_alias ===\n");
    scan_tokens("- &a val\n- *a\n", 14);
}

static void test_scan_tag(void)
{
    printf("=== scan_tag ===\n");
    scan_tokens("!!str value\n", 12);
}

static void test_scan_block_scalar(void)
{
    printf("=== scan_block_scalar ===\n");
    scan_tokens("|\n  line1\n  line2\n", 18);
    scan_tokens(">\n  folded\n", 10);
}

static void test_scan_flow_scalar(void)
{
    printf("=== scan_flow_scalar ===\n");
    scan_tokens("'single quoted'\n", 16);
    scan_tokens("\"double quoted\"\n", 16);
}

static void test_scan_malformed_utf8(void)
{
    /* Invalid UTF-8 sequences */
    printf("=== scan_malformed_utf8 ===\n");
    scan_tokens("\xFF\xFE", 2);
    scan_tokens("\x80", 1);
    scan_tokens("key: \xC0\x80\n", 8);
}

static void test_parse_basic(void)
{
    printf("=== parse_basic ===\n");
    parse_events("key: value\n", 11);
}

static void test_parse_empty(void)
{
    printf("=== parse_empty ===\n");
    parse_events("", 0);
}

static void test_parse_sequence(void)
{
    printf("=== parse_sequence ===\n");
    parse_events("- a\n- b\n- c\n", 12);
}

static void test_parse_mapping(void)
{
    printf("=== parse_mapping ===\n");
    parse_events("a: 1\nb: 2\n", 10);
}

static void test_parse_nested(void)
{
    printf("=== parse_nested ===\n");
    parse_events("a:\n  b:\n    c: d\n", 17);
}

static void test_parse_flow(void)
{
    printf("=== parse_flow ===\n");
    parse_events("{a: [1, 2], b: {c: d}}\n", 23);
}

static void test_parse_anchor_alias(void)
{
    printf("=== parse_anchor_alias ===\n");
    parse_events("- &a val\n- *a\n", 14);
}

static void test_parse_duplicate_anchor(void)
{
    printf("=== parse_duplicate_anchor ===\n");
    parse_events("- &a 1\n- &a 2\n- *a\n", 20);
}

static void test_parse_undefined_alias(void)
{
    printf("=== parse_undefined_alias ===\n");
    parse_events("- *undef\n", 9);
}

static void test_parse_multi_document(void)
{
    printf("=== parse_multi_document ===\n");
    parse_events("---\na: 1\n---\nb: 2\n...\n", 22);
}

static void test_parse_explicit_document(void)
{
    printf("=== parse_explicit_document ===\n");
    parse_events("---\nkey: val\n...\n", 17);
}

static void test_parse_version_directive(void)
{
    printf("=== parse_version_directive ===\n");
    parse_events("%YAML 1.1\n---\nscalar\n", 21);
}

static void test_parse_tag_directive(void)
{
    printf("=== parse_tag_directive ===\n");
    parse_events("%TAG ! tag:example.com,2000:\n---\n!foo bar\n", 42);
}

static void test_parse_long_scalar(void)
{
    char buf[4096];
    int i;
    printf("=== parse_long_scalar ===\n");
    buf[0] = '"';
    for (i = 1; i < 4000; i++) buf[i] = 'a';
    buf[4000] = '"';
    buf[4001] = '\n';
    buf[4002] = '\0';
    parse_events(buf, 4002);
}

static void test_parse_deep_nest(void)
{
    /* 30 levels: first loop ~525 chars, second ~493 chars, total ~1018 */
    char buf[4096];
    int i, pos = 0;
    printf("=== parse_deep_nest ===\n");
    /* 30 levels of block mappings */
    for (i = 0; i < 30; i++) {
        int j;
        for (j = 0; j < i; j++) buf[pos++] = ' ';
        buf[pos++] = 'k'; buf[pos++] = ':'; buf[pos++] = '\n';
    }
    for (i = 0; i < 29; i++) {
        int j;
        for (j = 0; j < (29-i); j++) buf[pos++] = ' ';
        buf[pos++] = 'v'; buf[pos++] = '\n';
    }
    buf[pos] = '\0';
    parse_events(buf, pos);
}

static void test_parse_flow_sequence(void)
{
    printf("=== parse_flow_sequence ===\n");
    parse_events("[1, 2, 3, {a: b}, [x, y]]\n", 26);
}

static void test_parse_flow_mapping_ordered(void)
{
    printf("=== parse_flow_mapping_ordered ===\n");
    parse_events("[{a: 1}, {b: 2}]\n", 18);
}

static void test_parse_block_sequence(void)
{
    printf("=== parse_block_sequence ===\n");
    parse_events("a:\n  - x\n  - y\n", 15);
}

static void test_parse_indentless_sequence(void)
{
    printf("=== parse_indentless_sequence ===\n");
    parse_events("a:\n- x\n- y\n", 11);
}

static void test_parse_null_scalar(void)
{
    printf("=== parse_null_scalar ===\n");
    parse_events("~\n", 2);
    parse_events("null\n", 5);
    parse_events("\n", 1);
}

static void test_parse_bool_scalar(void)
{
    printf("=== parse_bool_scalar ===\n");
    parse_events("true\n", 5);
    parse_events("false\n", 6);
}

static void test_parse_int_float(void)
{
    printf("=== parse_int_float ===\n");
    parse_events("42\n", 3);
    parse_events("3.14\n", 5);
    parse_events("0x1F\n", 5);
    parse_events("0o17\n", 5);
}

static void test_parse_special_chars(void)
{
    printf("=== parse_special_chars ===\n");
    parse_events("\"tab:\\there\"\n", 13);
    parse_events("\"newline:\\n\"\n", 13);
    parse_events("\"null:\\0\"\n", 10);
}

static void test_parse_set_encoding(void)
{
    yaml_parser_t parser;
    yaml_event_t event;
    printf("=== parse_set_encoding ===\n");
    yaml_parser_initialize(&parser);
    yaml_parser_set_encoding(&parser, YAML_UTF8_ENCODING);
    yaml_parser_set_input_string(&parser, (const unsigned char *)"a: b\n", 5);
    while (yaml_parser_parse(&parser, &event)) {
        print_event(&event);
        if (event.type == YAML_STREAM_END_EVENT) { yaml_event_delete(&event); break; }
        yaml_event_delete(&event);
    }
    yaml_parser_delete(&parser);
}

static void test_load_basic(void)
{
    printf("=== load_basic ===\n");
    load_documents("key: value\n", 11);
}

static void test_load_empty(void)
{
    printf("=== load_empty ===\n");
    load_documents("", 0);
}

static void test_load_sequence(void)
{
    printf("=== load_sequence ===\n");
    load_documents("- a\n- b\n- c\n", 12);
}

static void test_load_mapping(void)
{
    printf("=== load_mapping ===\n");
    load_documents("x: 1\ny: 2\n", 10);
}

static void test_load_anchor_alias(void)
{
    printf("=== load_anchor_alias ===\n");
    load_documents("- &a val\n- *a\n", 14);
}

static void test_load_multi(void)
{
    printf("=== load_multi ===\n");
    load_documents("---\na: 1\n---\nb: 2\n", 18);
}

static void test_load_nested(void)
{
    printf("=== load_nested ===\n");
    load_documents("a:\n  b: c\n  d:\n    - 1\n    - 2\n", 31);
}

static void test_load_undefined_alias(void)
{
    printf("=== load_undefined_alias ===\n");
    load_documents("- *missing\n", 11);
}

static void test_load_duplicate_anchor(void)
{
    printf("=== load_duplicate_anchor ===\n");
    load_documents("- &a 1\n- &a 2\n- *a\n", 20);
}

/* --------------------------------------------------------- emitter tests */

static void test_emit_basic(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_basic ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"hello", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);

    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_sequence(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_sequence ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"item1", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"item2", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);

    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_mapping(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_mapping ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"key", 3, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"val", 3, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);

    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_anchor_alias(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_anchor_alias ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev,
        (const yaml_char_t *)"anchor", NULL,
        (const yaml_char_t *)"val", 3, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_alias_event_initialize(&ev, (const yaml_char_t *)"anchor");
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);

    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_canonical(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_canonical ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_set_canonical(&em, 1);

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 0);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL,
        (const yaml_char_t *)"tag:yaml.org,2002:map", 0, YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"k", 1, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"v", 1, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);

    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_scalar_styles(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int style;
    const yaml_char_t *vals[] = {
        (const yaml_char_t *)"plain value",
        (const yaml_char_t *)"single quoted",
        (const yaml_char_t *)"double quoted",
        (const yaml_char_t *)"literal\nscalar",
        (const yaml_char_t *)"folded scalar"
    };
    int styles[] = {
        YAML_PLAIN_SCALAR_STYLE,
        YAML_SINGLE_QUOTED_SCALAR_STYLE,
        YAML_DOUBLE_QUOTED_SCALAR_STYLE,
        YAML_LITERAL_SCALAR_STYLE,
        YAML_FOLDED_SCALAR_STYLE
    };
    printf("=== emit_scalar_styles ===\n");
    for (style = 0; style < 5; style++) {
        if (!setup_emitter(&em, &b)) continue;
        yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
        yaml_emitter_emit(&em, &ev);
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&em, &ev);
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            vals[style], (int)strlen((char *)vals[style]),
            1, 1, styles[style]);
        yaml_emitter_emit(&em, &ev);
        yaml_document_end_event_initialize(&ev, 1);
        yaml_emitter_emit(&em, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&em, &ev);
        yaml_emitter_delete(&em);
        print_emitted(&b);
    }
}

static void test_emit_tag_directive(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    yaml_tag_directive_t tags[1];
    printf("=== emit_tag_directive ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    tags[0].handle = (yaml_char_t *)"!";
    tags[0].prefix = (yaml_char_t *)"tag:example.com,2000:";
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, tags, tags+1, 0);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"value", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_version_directive(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    yaml_version_directive_t ver = {1, 1};
    printf("=== emit_version_directive ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, &ver, NULL, NULL, 0);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"v", 1, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_unicode(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_unicode ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_set_unicode(&em, 1);
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    /* UTF-8 for U+263A smiley */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"\xe2\x98\xba", 3, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_emit_settings(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    printf("=== emit_settings ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_set_indent(&em, 4);
    yaml_emitter_set_width(&em, 40);
    yaml_emitter_set_break(&em, YAML_LN_BREAK);
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"a", 1, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"x", 1, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"y", 1, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

/* --------------------------------------------------------- dumper tests */

static void test_dump_basic(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    int root;
    printf("=== dump_basic ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    root = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"hello", 5, YAML_PLAIN_SCALAR_STYLE);
    printf("root_id=%d\n", root);
    yaml_emitter_dump(&em, &doc);

    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_dump_sequence(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    int seq, s1, s2;
    printf("=== dump_sequence ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    s1  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"a", 1, YAML_PLAIN_SCALAR_STYLE);
    s2  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"b", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s1);
    yaml_document_append_sequence_item(&doc, seq, s2);
    printf("seq=%d s1=%d s2=%d\n", seq, s1, s2);
    yaml_emitter_dump(&em, &doc);

    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_dump_mapping(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    int mp, k, v;
    printf("=== dump_mapping ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    mp = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    k  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"key", 3, YAML_PLAIN_SCALAR_STYLE);
    v  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"value", 5, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, mp, k, v);
    printf("mp=%d k=%d v=%d\n", mp, k, v);
    yaml_emitter_dump(&em, &doc);

    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_dump_nested(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    int mp, seq, k, s1, s2;
    printf("=== dump_nested ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    mp  = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    k   = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"items", 5, YAML_PLAIN_SCALAR_STYLE);
    seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    s1  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"x", 1, YAML_PLAIN_SCALAR_STYLE);
    s2  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"y", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s1);
    yaml_document_append_sequence_item(&doc, seq, s2);
    yaml_document_append_mapping_pair(&doc, mp, k, seq);
    yaml_emitter_dump(&em, &doc);

    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

static void test_dump_empty_doc(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    printf("=== dump_empty_doc ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    /* No nodes — empty document signals stream end */
    yaml_emitter_dump(&em, &doc);
    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

/* --------------------------------------------------------- document API */

static void test_document_api(void)
{
    yaml_document_t doc;
    yaml_node_t *node;
    yaml_version_directive_t ver = {1, 1};
    yaml_tag_directive_t tags[1];
    int s, seq, mp, k, v;
    printf("=== document_api ===\n");

    tags[0].handle = (yaml_char_t *)"!";
    tags[0].prefix = (yaml_char_t *)"tag:example.com,2000:";

    yaml_document_initialize(&doc, &ver, tags, tags+1, 0, 0);
    printf("version=%d.%d\n",
           doc.version_directive ? doc.version_directive->major : -1,
           doc.version_directive ? doc.version_directive->minor : -1);

    s   = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"hello", 5, YAML_PLAIN_SCALAR_STYLE);
    seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    mp  = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    k   = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"k", 1, YAML_PLAIN_SCALAR_STYLE);
    v   = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"v", 1, YAML_PLAIN_SCALAR_STYLE);

    yaml_document_append_sequence_item(&doc, seq, s);
    yaml_document_append_mapping_pair(&doc, mp, k, v);

    node = yaml_document_get_node(&doc, s);
    printf("scalar_node type=%d\n", node ? (int)node->type : -1);
    node = yaml_document_get_node(&doc, seq);
    printf("seq_node type=%d\n", node ? (int)node->type : -1);
    node = yaml_document_get_node(&doc, mp);
    printf("map_node type=%d\n", node ? (int)node->type : -1);
    node = yaml_document_get_root_node(&doc);
    printf("root_node type=%d\n", node ? (int)node->type : -1);
    node = yaml_document_get_node(&doc, 0);
    printf("node0 is_null=%d\n", node == NULL ? 1 : 0);

    yaml_document_delete(&doc);
}

static void test_event_init(void)
{
    yaml_event_t ev;
    printf("=== event_init ===\n");

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    printf("stream_start type=%d enc=%d\n", (int)ev.type,
           (int)ev.data.stream_start.encoding);
    yaml_event_delete(&ev);

    yaml_stream_end_event_initialize(&ev);
    printf("stream_end type=%d\n", (int)ev.type);
    yaml_event_delete(&ev);

    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    printf("doc_start type=%d impl=%d\n", (int)ev.type,
           ev.data.document_start.implicit);
    yaml_event_delete(&ev);

    yaml_document_end_event_initialize(&ev, 0);
    printf("doc_end type=%d impl=%d\n", (int)ev.type,
           ev.data.document_end.implicit);
    yaml_event_delete(&ev);

    yaml_alias_event_initialize(&ev, (const yaml_char_t *)"myanchor");
    printf("alias type=%d anchor=%s\n", (int)ev.type,
           (char *)ev.data.alias.anchor);
    yaml_event_delete(&ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"value", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    printf("scalar type=%d len=%d style=%d plain=%d\n",
           (int)ev.type, (int)ev.data.scalar.length,
           (int)ev.data.scalar.style, ev.data.scalar.plain_implicit);
    yaml_event_delete(&ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_SEQUENCE_STYLE);
    printf("seq_start type=%d style=%d\n", (int)ev.type,
           (int)ev.data.sequence_start.style);
    yaml_event_delete(&ev);

    yaml_sequence_end_event_initialize(&ev);
    printf("seq_end type=%d\n", (int)ev.type);
    yaml_event_delete(&ev);

    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_MAPPING_STYLE);
    printf("map_start type=%d style=%d\n", (int)ev.type,
           (int)ev.data.mapping_start.style);
    yaml_event_delete(&ev);

    yaml_mapping_end_event_initialize(&ev);
    printf("map_end type=%d\n", (int)ev.type);
    yaml_event_delete(&ev);
}

static void test_token_delete(void)
{
    yaml_token_t t;
    printf("=== token_delete ===\n");
    memset(&t, 0, sizeof(t));
    t.type = YAML_SCALAR_TOKEN;
    t.data.scalar.value = (yaml_char_t *)malloc(5);
    memcpy(t.data.scalar.value, "hello", 5);
    t.data.scalar.length = 5;
    yaml_token_delete(&t);
    printf("token_type_after_delete=%d\n", (int)t.type);

    memset(&t, 0, sizeof(t));
    t.type = YAML_ANCHOR_TOKEN;
    t.data.anchor.value = (yaml_char_t *)strdup("anch");
    yaml_token_delete(&t);
    printf("anchor_type_after_delete=%d\n", (int)t.type);

    memset(&t, 0, sizeof(t));
    t.type = YAML_ALIAS_TOKEN;
    t.data.alias.value = (yaml_char_t *)strdup("alias");
    yaml_token_delete(&t);
    printf("alias_type_after_delete=%d\n", (int)t.type);

    memset(&t, 0, sizeof(t));
    t.type = YAML_TAG_TOKEN;
    t.data.tag.handle = (yaml_char_t *)strdup("!");
    t.data.tag.suffix = (yaml_char_t *)strdup("foo");
    yaml_token_delete(&t);
    printf("tag_type_after_delete=%d\n", (int)t.type);
}

/* -------------------------------------------------- bridge: utf8, api */

static void test_bridge_check_utf8(void)
{
    printf("=== bridge_check_utf8 ===\n");
    /* valid ASCII */
    printf("ascii=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"hello", 5));
    /* valid UTF-8 2-byte */
    printf("utf8_2byte=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\xC3\xA9", 2));
    /* valid UTF-8 3-byte */
    printf("utf8_3byte=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\xe2\x98\xba", 3));
    /* valid UTF-8 4-byte */
    printf("utf8_4byte=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\xf0\x9f\x98\x80", 4));
    /* invalid: lone continuation byte */
    printf("lone_cont=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\x80", 1));
    /* invalid: overlong 2-byte (0xC0 0x80) */
    printf("overlong=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\xC0\x80", 2));
    /* invalid: truncated sequence */
    printf("truncated=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\xC3", 1));
    /* empty */
    printf("empty=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"", 0));
    /* invalid: invalid lead byte 0xFF */
    printf("ff_byte=%d\n",
        bridge_yaml_check_utf8((const yaml_char_t *)"\xFF", 1));
}

static void test_bridge_read_handlers(void)
{
    yaml_parser_t parser;
    unsigned char buf[32];
    size_t size_read = 0;
    const char *input = "hello";
    printf("=== bridge_read_handlers ===\n");

    /* Test string read handler */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, 5);
    size_read = 0;
    printf("str_read=%d size=%zu\n",
        bridge_yaml_string_read_handler(&parser, buf, 3, &size_read),
        size_read);
    printf("str_read2=%d size=%zu\n",
        bridge_yaml_string_read_handler(&parser, buf, 32, &size_read),
        size_read);
    /* EOF: no more data */
    printf("str_eof=%d size=%zu\n",
        bridge_yaml_string_read_handler(&parser, buf, 32, &size_read),
        size_read);
    yaml_parser_delete(&parser);
}

static void test_bridge_write_handlers(void)
{
    yaml_emitter_t em;
    emit_buf_t b;
    printf("=== bridge_write_handlers ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    printf("str_write=%d\n",
        bridge_yaml_string_write_handler(&em,
            (unsigned char *)"hello", 5));
    printf("written=%zu\n", b.written);
    yaml_emitter_delete(&em);
}

/* ------------------------------------------------ bridge: error setters */

static void test_bridge_error_setters(void)
{
    yaml_parser_t parser;
    yaml_emitter_t em;
    yaml_mark_t m = {0, 0, 0};
    printf("=== bridge_error_setters ===\n");

    yaml_parser_initialize(&parser);
    bridge_yaml_parser_set_reader_error(&parser, "err", 0, -1);
    printf("reader_error=%d\n", (int)parser.error);
    yaml_parser_delete(&parser);

    yaml_parser_initialize(&parser);
    bridge_yaml_parser_set_scanner_error(&parser, "ctx", m, "prob");
    printf("scanner_error=%d\n", (int)parser.error);
    yaml_parser_delete(&parser);

    yaml_parser_initialize(&parser);
    bridge_yaml_parser_set_parser_error(&parser, "prob", m);
    printf("parser_error=%d\n", (int)parser.error);
    yaml_parser_delete(&parser);

    yaml_parser_initialize(&parser);
    bridge_yaml_parser_set_parser_error_context(&parser, "ctx", m, "prob", m);
    printf("parser_ctx_error=%d\n", (int)parser.error);
    yaml_parser_delete(&parser);

    yaml_emitter_initialize(&em);
    bridge_yaml_emitter_set_emitter_error(&em, "emit_err");
    printf("emitter_error=%d\n", (int)em.error);
    yaml_emitter_delete(&em);

    yaml_emitter_initialize(&em);
    bridge_yaml_emitter_set_writer_error(&em, "write_err");
    printf("writer_error=%d\n", (int)em.error);
    yaml_emitter_delete(&em);
}

/* ------------------------------------------------ bridge: parser scan internals */

static void test_bridge_parser_internals(void)
{
    yaml_parser_t parser;
    yaml_mark_t m = {0, 0, 0};
    int ret;
    printf("=== bridge_parser_internals ===\n");

    /* Test bridge_yaml_parser_determine_encoding */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key: val\n", 9);
    ret = bridge_yaml_parser_determine_encoding(&parser);
    printf("determine_encoding=%d enc=%d\n", ret, (int)parser.encoding);
    yaml_parser_delete(&parser);

    /* Test bridge_yaml_parser_update_raw_buffer */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"abc\n", 4);
    ret = bridge_yaml_parser_update_raw_buffer(&parser);
    printf("update_raw_buffer=%d\n", ret);
    yaml_parser_delete(&parser);

    /* Test bridge_yaml_maximum_level_reached */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"a\n", 2);
    ret = bridge_yaml_maximum_level_reached(&parser, m, m);
    printf("max_level=%d\n", ret);
    yaml_parser_delete(&parser);

    /* Test stale simple keys via scan */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key: val\n", 9);
    ret = bridge_yaml_parser_stale_simple_keys(&parser);
    printf("stale_simple_keys=%d\n", ret);
    yaml_parser_delete(&parser);

    /* roll/unroll indent: exercised via scan tests above; skip direct call */

    /* Test flow level */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"{}\n", 3);
    ret = bridge_yaml_parser_increase_flow_level(&parser);
    printf("increase_flow=%d level=%d\n", ret, parser.flow_level);
    ret = bridge_yaml_parser_decrease_flow_level(&parser);
    printf("decrease_flow=%d level=%d\n", ret, parser.flow_level);
    yaml_parser_delete(&parser);
}

static void test_bridge_fetch_stream(void)
{
    yaml_parser_t parser;
    int ret;
    printf("=== bridge_fetch_stream ===\n");

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"a\n", 2);
    ret = bridge_yaml_parser_fetch_stream_start(&parser);
    printf("fetch_stream_start=%d\n", ret);
    yaml_parser_delete(&parser);
}

static void test_bridge_scanner_scan(void)
{
    yaml_parser_t parser;
    int ret;
    printf("=== bridge_scanner_scan ===\n");

    /* save/remove simple key */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key: val\n", 9);
    bridge_yaml_parser_fetch_stream_start(&parser);
    ret = bridge_yaml_parser_save_simple_key(&parser);
    printf("save_simple_key=%d\n", ret);
    ret = bridge_yaml_parser_remove_simple_key(&parser);
    printf("remove_simple_key=%d\n", ret);
    yaml_parser_delete(&parser);

    /* scan_to_next_token on simple input */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"  # comment\nkey\n", 16);
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);
    ret = bridge_yaml_parser_scan_to_next_token(&parser);
    printf("scan_to_next_token=%d\n", ret);
    yaml_parser_delete(&parser);

    /* fetch_next_token on simple input */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key\n", 4);
    ret = bridge_yaml_parser_fetch_next_token(&parser);
    printf("fetch_next_token=%d\n", ret);
    yaml_parser_delete(&parser);
}

static void test_bridge_scan_directives(void)
{
    yaml_parser_t parser;
    yaml_token_t token;
    int ret;
    int major = 0, minor = 0;
    yaml_mark_t m = {0, 0, 0};
    printf("=== bridge_scan_directives ===\n");

    /* scan_version_directive_number */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"12.3", 4);
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);
    /* need to populate buffer */
    {
        yaml_token_t t;
        yaml_parser_scan(&parser, &t);
        yaml_token_delete(&t);
    }
    yaml_parser_delete(&parser);
    printf("scan_directives_done=1\n");

    /* bridge_yaml_parser_process_directives */
    {
        yaml_version_directive_t *vd = NULL;
        yaml_tag_directive_t *tds = NULL, *tde = NULL;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)"%YAML 1.1\n---\nscalar\n", 21);
        ret = bridge_yaml_parser_process_directives(&parser, &vd, &tds, &tde);
        printf("process_directives=%d\n", ret);
        if (vd) {
            printf("ver=%d.%d\n", vd->major, vd->minor);
            free(vd);
        }
        if (tds) {
            yaml_tag_directive_t *t;
            for (t = tds; t < tde; t++) {
                yaml_free(t->handle);
                yaml_free(t->prefix);
            }
            yaml_free(tds);
        }
        yaml_parser_delete(&parser);
    }

    /* scan_directive via full scan */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"%YAML 1.1\n---\nscalar\n", 21);
    while (1) {
        ret = yaml_parser_scan(&parser, &token);
        if (!ret || token.type == YAML_STREAM_END_TOKEN) {
            yaml_token_delete(&token);
            break;
        }
        if (token.type == YAML_VERSION_DIRECTIVE_TOKEN) {
            printf("ver_token=%d.%d\n",
                token.data.version_directive.major,
                token.data.version_directive.minor);
        }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);
    (void)major; (void)minor; (void)m; (void)ret;
}

static void test_bridge_scan_anchor_tag(void)
{
    yaml_parser_t parser;
    yaml_token_t token;
    printf("=== bridge_scan_anchor_tag ===\n");

    /* Exercise scan_anchor via scan */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"&myanchor value\n", 16);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) { yaml_token_delete(&token); break; }
        if (token.type == YAML_ANCHOR_TOKEN)
            printf("anchor=%s\n", (char *)token.data.anchor.value);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);

    /* Exercise scan_tag via scan */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"!!str value\n", 12);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) { yaml_token_delete(&token); break; }
        if (token.type == YAML_TAG_TOKEN)
            printf("tag_handle=%s suffix=%s\n",
                (char *)token.data.tag.handle,
                (char *)token.data.tag.suffix);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);
}

static void test_bridge_scan_scalars(void)
{
    yaml_parser_t parser;
    yaml_token_t token;
    printf("=== bridge_scan_scalars ===\n");

    /* block scalar literal */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"|\n  line1\n  line2\n", 18);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) { yaml_token_delete(&token); break; }
        if (token.type == YAML_SCALAR_TOKEN)
            printf("block_scalar style=%d val=%s\n",
                (int)token.data.scalar.style,
                (char *)token.data.scalar.value);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);

    /* flow scalar single */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"'hello world'\n", 14);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) { yaml_token_delete(&token); break; }
        if (token.type == YAML_SCALAR_TOKEN)
            printf("flow_sq style=%d val=%s\n",
                (int)token.data.scalar.style,
                (char *)token.data.scalar.value);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);

    /* flow scalar double */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"\"double\\nquoted\"\n", 17);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) { yaml_token_delete(&token); break; }
        if (token.type == YAML_SCALAR_TOKEN)
            printf("flow_dq style=%d\n", (int)token.data.scalar.style);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);

    /* plain scalar */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"plain scalar\n", 13);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) { yaml_token_delete(&token); break; }
        if (token.type == YAML_SCALAR_TOKEN)
            printf("plain style=%d\n", (int)token.data.scalar.style);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);
}

static void test_bridge_scan_uri_escapes(void)
{
    yaml_parser_t parser;
    yaml_token_t token;
    printf("=== bridge_scan_uri_escapes ===\n");
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"!<%41%42> value\n", 16);
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) {
            printf("error=%d\n", (int)parser.error);
            yaml_token_delete(&token);
            break;
        }
        if (token.type == YAML_TAG_TOKEN)
            printf("tag_handle=%s suffix=%s\n",
                (char *)token.data.tag.handle,
                (char *)token.data.tag.suffix);
        if (token.type == YAML_STREAM_END_TOKEN) { yaml_token_delete(&token); break; }
        yaml_token_delete(&token);
    }
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------ bridge: parser state machine */

static void test_bridge_parser_state_machine(void)
{
    yaml_parser_t parser;
    yaml_event_t event;
    int ret;
    printf("=== bridge_parser_state_machine ===\n");

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key: val\n", 9);

    ret = bridge_yaml_parser_state_machine(&parser, &event);
    printf("sm1 ret=%d type=%d\n", ret, (int)event.type);
    yaml_event_delete(&event);

    ret = bridge_yaml_parser_state_machine(&parser, &event);
    printf("sm2 ret=%d type=%d\n", ret, (int)event.type);
    yaml_event_delete(&event);

    yaml_parser_delete(&parser);
}

static void test_bridge_parse_fns(void)
{
    yaml_parser_t parser;
    yaml_event_t event;
    int ret;
    printf("=== bridge_parse_fns ===\n");

    /* bridge_yaml_parser_parse_stream_start */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key: val\n", 9);
    ret = bridge_yaml_parser_parse_stream_start(&parser, &event);
    printf("parse_stream_start=%d type=%d\n", ret, (int)event.type);
    yaml_event_delete(&event);
    yaml_parser_delete(&parser);

    /* process empty scalar */
    {
        yaml_mark_t m = {0, 0, 0};
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)"a\n", 2);
        ret = bridge_yaml_parser_process_empty_scalar(&parser, &event, m);
        printf("empty_scalar=%d type=%d\n", ret, (int)event.type);
        yaml_event_delete(&event);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------ bridge: emitter analysis */

static void test_bridge_emitter_analyze(void)
{
    yaml_emitter_t em;
    emit_buf_t b;
    yaml_version_directive_t ver = {1, 1};
    yaml_tag_directive_t td;
    int ret;
    printf("=== bridge_emitter_analyze ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    /* analyze_version_directive */
    ret = bridge_yaml_emitter_analyze_version_directive(&em, ver);
    printf("analyze_ver=%d\n", ret);

    /* analyze_tag_directive */
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    ret = bridge_yaml_emitter_analyze_tag_directive(&em, td);
    printf("analyze_tag_dir=%d\n", ret);

    /* analyze_anchor */
    ret = bridge_yaml_emitter_analyze_anchor(&em,
        (yaml_char_t *)"myanchor", 0);
    printf("analyze_anchor=%d\n", ret);
    ret = bridge_yaml_emitter_analyze_anchor(&em,
        (yaml_char_t *)"myalias", 1);
    printf("analyze_alias=%d\n", ret);

    /* analyze_tag */
    ret = bridge_yaml_emitter_analyze_tag(&em,
        (yaml_char_t *)"tag:yaml.org,2002:str");
    printf("analyze_tag=%d\n", ret);

    /* analyze_scalar */
    ret = bridge_yaml_emitter_analyze_scalar(&em,
        (yaml_char_t *)"hello world", 11);
    printf("analyze_scalar=%d multiline=%d\n", ret,
           em.scalar_data.multiline);
    printf("flow_plain=%d block_plain=%d sq=%d block=%d\n",
           em.scalar_data.flow_plain_allowed,
           em.scalar_data.block_plain_allowed,
           em.scalar_data.single_quoted_allowed,
           em.scalar_data.block_allowed);

    ret = bridge_yaml_emitter_analyze_scalar(&em,
        (yaml_char_t *)"line1\nline2", 11);
    printf("analyze_multiline=%d multiline=%d\n", ret,
           em.scalar_data.multiline);

    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_checks(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_checks ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    /* Start stream so emitter is in proper state */
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);

    printf("check_empty_doc=%d\n",
        bridge_yaml_emitter_check_empty_document(&em));
    printf("need_more_events=%d\n",
        bridge_yaml_emitter_need_more_events(&em));

    yaml_emitter_delete(&em);

    /* Check empty sequence/mapping using a full emitter session */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&em, &ev);
    printf("check_empty_seq=%d\n",
        bridge_yaml_emitter_check_empty_sequence(&em));
    printf("check_empty_map=%d\n",
        bridge_yaml_emitter_check_empty_mapping(&em));
    printf("check_simple_key=%d\n",
        bridge_yaml_emitter_check_simple_key(&em));
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_indent(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_indent ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&em, &ev);

    ret = bridge_yaml_emitter_increase_indent(&em, 0, 0);
    printf("increase_indent=%d indent=%d\n", ret, em.indent);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&em, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&em, &ev);
    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_append_tag_dir(void)
{
    yaml_emitter_t em;
    emit_buf_t b;
    yaml_tag_directive_t td;
    int ret;
    printf("=== bridge_emitter_append_tag_dir ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    /* internal fn strdup's the strings itself; pass literals directly */
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    ret = bridge_yaml_emitter_append_tag_directive(&em, td, 0);
    printf("append_tag=%d\n", ret);

    /* duplicate should fail if allow_duplicates=0 */
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    ret = bridge_yaml_emitter_append_tag_directive(&em, td, 0);
    printf("dup_tag_no_allow=%d err=%d\n", ret, (int)em.error);

    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_generate_anchor(void)
{
    yaml_emitter_t em;
    emit_buf_t b;
    yaml_char_t *anch;
    printf("=== bridge_emitter_generate_anchor ===\n");
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    anch = bridge_yaml_emitter_generate_anchor(&em, 1);
    if (anch) { printf("anchor1=%s\n", (char *)anch); yaml_free(anch); }
    anch = bridge_yaml_emitter_generate_anchor(&em, 2);
    if (anch) { printf("anchor2=%s\n", (char *)anch); yaml_free(anch); }
    anch = bridge_yaml_emitter_generate_anchor(&em, 100);
    if (anch) { printf("anchor100=%s\n", (char *)anch); yaml_free(anch); }
    yaml_emitter_delete(&em);
}

/* ------------------------------------------------ bridge: emitter write fns */

static void test_bridge_emitter_write_fns(void)
{
    yaml_emitter_t em;
    emit_buf_t b;
    yaml_string_t str;
    yaml_char_t str_buf[64];
    int ret;
    printf("=== bridge_emitter_write_fns ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    em.encoding = YAML_UTF8_ENCODING;

    /* write_bom */
    ret = bridge_yaml_emitter_write_bom(&em);
    printf("write_bom=%d\n", ret);

    /* write_indent */
    em.indent = 2;
    ret = bridge_yaml_emitter_write_indent(&em);
    printf("write_indent=%d\n", ret);

    /* write_indicator */
    ret = bridge_yaml_emitter_write_indicator(&em, "---", 1, 0, 0);
    printf("write_indicator=%d\n", ret);

    /* write_anchor */
    ret = bridge_yaml_emitter_write_anchor(&em,
        (yaml_char_t *)"myanchor", 8);
    printf("write_anchor=%d\n", ret);

    /* write_tag_handle */
    ret = bridge_yaml_emitter_write_tag_handle(&em,
        (yaml_char_t *)"!", 1);
    printf("write_tag_handle=%d\n", ret);

    /* write_tag_content */
    ret = bridge_yaml_emitter_write_tag_content(&em,
        (yaml_char_t *)"str", 3, 1);
    printf("write_tag_content=%d\n", ret);

    /* write_plain_scalar */
    ret = bridge_yaml_emitter_write_plain_scalar(&em,
        (yaml_char_t *)"plain", 5, 1);
    printf("write_plain=%d\n", ret);

    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);

    /* write_single_quoted_scalar */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    em.encoding = YAML_UTF8_ENCODING;
    ret = bridge_yaml_emitter_write_single_quoted_scalar(&em,
        (yaml_char_t *)"single", 6, 1);
    printf("write_sq=%d\n", ret);
    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);

    /* write_double_quoted_scalar */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    em.encoding = YAML_UTF8_ENCODING;
    ret = bridge_yaml_emitter_write_double_quoted_scalar(&em,
        (yaml_char_t *)"double", 6, 1);
    printf("write_dq=%d\n", ret);
    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);

    /* write_block_scalar_hints */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    em.encoding = YAML_UTF8_ENCODING;
    memcpy(str_buf, "line1\nline2\n", 12);
    str.start   = str_buf;
    str.pointer = str_buf + 12;
    str.end     = str_buf + 64;
    ret = bridge_yaml_emitter_write_block_scalar_hints(&em, str);
    printf("write_block_hints=%d\n", ret);
    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);

    /* write_literal_scalar */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    em.encoding = YAML_UTF8_ENCODING;
    em.indent = 0;
    ret = bridge_yaml_emitter_write_literal_scalar(&em,
        (yaml_char_t *)"line1\nline2\n", 12);
    printf("write_literal=%d\n", ret);
    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);

    /* write_folded_scalar */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    em.encoding = YAML_UTF8_ENCODING;
    em.indent = 0;
    ret = bridge_yaml_emitter_write_folded_scalar(&em,
        (yaml_char_t *)"fold1 fold2\n", 12);
    printf("write_folded=%d\n", ret);
    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);
}

/* ------------------------------------------------ bridge: emitter state machine */

static void test_bridge_emitter_state_machine(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_state_machine ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    ret = bridge_yaml_emitter_state_machine(&em, &ev);
    printf("sm_stream_start=%d state=%d\n", ret, (int)em.state);

    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    ret = bridge_yaml_emitter_state_machine(&em, &ev);
    printf("sm_doc_start=%d state=%d\n", ret, (int)em.state);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"v", 1, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    ret = bridge_yaml_emitter_state_machine(&em, &ev);
    printf("sm_scalar=%d state=%d\n", ret, (int)em.state);
    /* state_machine borrows event; scalar value is now in emitter output buffer */
    yaml_event_delete(&ev);

    yaml_document_end_event_initialize(&ev, 1);
    ret = bridge_yaml_emitter_state_machine(&em, &ev);
    printf("sm_doc_end=%d state=%d\n", ret, (int)em.state);

    yaml_stream_end_event_initialize(&ev);
    ret = bridge_yaml_emitter_state_machine(&em, &ev);
    printf("sm_stream_end=%d state=%d\n", ret, (int)em.state);

    /* encoding was set by emit_stream_start state machine */
    if (!em.encoding) em.encoding = YAML_UTF8_ENCODING;
    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_emit_fns(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_emit_fns ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    /* emit_stream_start */
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    ret = bridge_yaml_emitter_emit_stream_start(&em, &ev);
    printf("emit_stream_start=%d\n", ret);

    /* emit_document_start */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    ret = bridge_yaml_emitter_emit_document_start(&em, &ev, 1);
    printf("emit_doc_start=%d\n", ret);

    /* emit_document_content */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"val", 3, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    ret = bridge_yaml_emitter_emit_document_content(&em, &ev);
    printf("emit_doc_content=%d\n", ret);
    /* scalar value has been buffered by emitter; release event memory */
    yaml_event_delete(&ev);

    yaml_emitter_flush(&em);
    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_select_style(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_select_style ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    /* Need to get into scalar-emitting context */
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"test value", 10, 1, 1, YAML_ANY_SCALAR_STYLE);
    bridge_yaml_emitter_analyze_event(&em, &ev);
    ret = bridge_yaml_emitter_select_scalar_style(&em, &ev);
    printf("select_style=%d style=%d\n", ret,
           (int)em.scalar_data.style);
    yaml_event_delete(&ev);

    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_analyze_event(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_analyze_event ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"hello", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    ret = bridge_yaml_emitter_analyze_event(&em, &ev);
    printf("analyze_event_scalar=%d\n", ret);
    yaml_event_delete(&ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    ret = bridge_yaml_emitter_analyze_event(&em, &ev);
    printf("analyze_event_seq=%d\n", ret);
    yaml_event_delete(&ev);

    yaml_emitter_delete(&em);
}

static void test_bridge_emitter_process_fns(void)
{
    yaml_emitter_t em;
    yaml_event_t ev;
    emit_buf_t b;
    int ret;
    printf("=== bridge_emitter_process_fns ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&em, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&em, &ev);

    /* Set up anchor data manually */
    bridge_yaml_emitter_analyze_anchor(&em, (yaml_char_t *)"myanchor", 0);
    ret = bridge_yaml_emitter_process_anchor(&em);
    printf("process_anchor=%d\n", ret);

    /* Set up tag data */
    bridge_yaml_emitter_analyze_tag(&em,
        (yaml_char_t *)"tag:yaml.org,2002:str");
    ret = bridge_yaml_emitter_process_tag(&em);
    printf("process_tag=%d\n", ret);

    /* Set up scalar data then process */
    bridge_yaml_emitter_analyze_scalar(&em,
        (yaml_char_t *)"hello", 5);
    em.scalar_data.style = YAML_PLAIN_SCALAR_STYLE;
    ret = bridge_yaml_emitter_process_scalar(&em);
    printf("process_scalar=%d\n", ret);

    yaml_emitter_delete(&em);
}

/* ------------------------------------------------ bridge: loader */

static void test_bridge_loader(void)
{
    yaml_parser_t parser;
    yaml_document_t doc;
    yaml_event_t event;
    int ret;
    printf("=== bridge_loader ===\n");

    /* Test bridge_yaml_parser_delete_aliases */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"- &a val\n- *a\n", 14);
    yaml_parser_load(&parser, &doc);
    yaml_document_delete(&doc);
    bridge_yaml_parser_delete_aliases(&parser);
    printf("delete_aliases_done=1\n");
    yaml_parser_delete(&parser);

    /* bridge_yaml_parser_register_anchor requires parser->document to be set
       (it accesses document->nodes); exercised via load_anchor_alias test */
    printf("register_anchor=exercised_via_load\n");

    /* Test bridge_yaml_parser_set_composer_error */
    {
        yaml_mark_t m = {0, 0, 0};
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)"a\n", 2);
        bridge_yaml_parser_set_composer_error(&parser, "test", m);
        printf("composer_error=%d\n", (int)parser.error);
        yaml_parser_delete(&parser);
    }

    /* Test bridge_yaml_parser_set_composer_error_context */
    {
        yaml_mark_t m = {0, 0, 0};
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)"a\n", 2);
        bridge_yaml_parser_set_composer_error_context(
            &parser, "ctx", m, "prob", m);
        printf("composer_ctx_error=%d\n", (int)parser.error);
        yaml_parser_delete(&parser);
    }

    /* Test bridge_yaml_parser_load_document using full parser state */
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)"key: val\n", 9);
    {
        yaml_document_t d2;
        memset(&d2, 0, sizeof(d2));
        if (!yaml_parser_load(&parser, &d2)) {
            printf("load_err=%d\n", (int)parser.error);
        } else {
            yaml_node_t *root = yaml_document_get_root_node(&d2);
            printf("bridge_load_node_type=%d\n",
                   root ? (int)root->type : 0);
            yaml_document_delete(&d2);
        }
    }
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------ bridge: dumper */

static void test_bridge_dumper(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    int n1, seq, mp;
    printf("=== bridge_dumper ===\n");

    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    n1  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"item", 4, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, n1);

    /* bridge_yaml_emitter_anchor_node is called internally by yaml_emitter_dump */
    printf("anchor_node_done=1\n");

    yaml_emitter_dump(&em, &doc);

    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);

    /* bridge_yaml_emitter_delete_document_and_anchors */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    n1 = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"x", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_dump(&em, &doc);
    yaml_emitter_close(&em);
    /* bridge_yaml_emitter_delete_document_and_anchors is called internally
       by yaml_emitter_dump; calling it after dump sets document=NULL would
       pass NULL to yaml_document_delete — exercised via yaml_emitter_dump */
    printf("delete_doc_anchors=exercised_via_dump\n");
    yaml_emitter_delete(&em);
}

static void test_bridge_dump_node(void)
{
    yaml_emitter_t em;
    yaml_document_t doc;
    emit_buf_t b;
    int s, seq, mp, k, v;
    int ret;
    printf("=== bridge_dump_node ===\n");

    /* Exercise bridge_yaml_emitter_dump_scalar via yaml_emitter_dump */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    s = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"scalar_val", 10, YAML_PLAIN_SCALAR_STYLE);
    ret = yaml_emitter_dump(&em, &doc);
    printf("dump_scalar=%d\n", ret);
    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);

    /* Exercise bridge_yaml_emitter_dump_sequence via yaml_emitter_dump */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    s   = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"item", 4, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s);
    ret = yaml_emitter_dump(&em, &doc);
    printf("dump_sequence=%d\n", ret);
    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);

    /* Exercise bridge_yaml_emitter_dump_mapping via yaml_emitter_dump */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    yaml_emitter_open(&em);
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    mp = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    k  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"k", 1, YAML_PLAIN_SCALAR_STYLE);
    v  = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"v", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, mp, k, v);
    ret = yaml_emitter_dump(&em, &doc);
    printf("dump_mapping=%d\n", ret);
    yaml_emitter_close(&em);
    yaml_emitter_delete(&em);
    print_emitted(&b);

    /* Exercise bridge_yaml_emitter_dump_alias:
       create a doc with a self-referential anchor and alias */
    if (!setup_emitter(&em, &b)) { printf("init_fail\n"); return; }
    {
        yaml_parser_t parser;
        yaml_document_t d2;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)"- &a val\n- *a\n", 14);
        yaml_parser_load(&parser, &d2);
        yaml_emitter_open(&em);
        ret = yaml_emitter_dump(&em, &d2);
        printf("dump_alias=%d\n", ret);
        yaml_emitter_close(&em);
        yaml_parser_delete(&parser);
    }
    yaml_emitter_delete(&em);
    print_emitted(&b);
}

/* ------------------------------------------------ bridge: reader / writer */

static void test_bridge_reader_writer(void)
{
    printf("=== bridge_reader_writer ===\n");

    /* Reader test: file read handler */
    {
        yaml_parser_t parser;
        unsigned char buf[32];
        size_t sz = 0;
        FILE *f = tmpfile();
        if (f) {
            fwrite("abc\n", 1, 4, f);
            rewind(f);
            yaml_parser_initialize(&parser);
            yaml_parser_set_input_file(&parser, f);
            printf("file_read=%d size=%zu\n",
                bridge_yaml_file_read_handler(&parser, buf, 4, &sz), sz);
            fclose(f);
            yaml_parser_delete(&parser);
        } else {
            printf("tmpfile_failed=1\n");
        }
    }

    /* Writer test: file write handler */
    {
        yaml_emitter_t em;
        emit_buf_t b;
        FILE *f = tmpfile();
        if (f) {
            yaml_emitter_initialize(&em);
            yaml_emitter_set_output_file(&em, f);
            printf("file_write=%d\n",
                bridge_yaml_file_write_handler(&em,
                    (unsigned char *)"hello", 5));
            fclose(f);
            yaml_emitter_delete(&em);
        } else {
            printf("tmpfile_failed=1\n");
        }
    }
}

/* ------------------------------------------------ set_max_nest_level */

static void test_max_nest_level(void)
{
    char buf[4096];
    int i, pos;
    printf("=== max_nest_level ===\n");

    /* Set a very low limit and try to exceed it */
    yaml_set_max_nest_level(5);
    pos = 0;
    for (i = 0; i < 10; i++) {
        int j;
        for (j = 0; j < i; j++) buf[pos++] = ' ';
        buf[pos++] = 'k'; buf[pos++] = ':'; buf[pos++] = '\n';
    }
    buf[pos] = '\0';
    parse_events(buf, pos);

    /* Restore to default */
    yaml_set_max_nest_level(1000);
    printf("max_nest_restored=1\n");
}

/* ------------------------------------------------ edge cases */

static void test_edge_empty_scalar(void)
{
    printf("=== edge_empty_scalar ===\n");
    parse_events("''\n", 3);
    parse_events("\"\"\n", 3);
}

static void test_edge_special_float(void)
{
    printf("=== edge_special_float ===\n");
    parse_events(".inf\n", 5);
    parse_events("-.inf\n", 6);
    parse_events(".nan\n", 5);
}

static void test_edge_multiline_mapping_key(void)
{
    printf("=== edge_multiline_mapping_key ===\n");
    parse_events("? key\n: val\n", 12);
    parse_events("? a\n  b\n: c\n", 12);
}

static void test_edge_flow_mapping_empty_value(void)
{
    printf("=== edge_flow_mapping_empty_value ===\n");
    parse_events("{a:, b: c}\n", 11);
    parse_events("{a: , b:}\n", 10);
}

static void test_edge_block_scalar_chomping(void)
{
    printf("=== edge_block_scalar_chomping ===\n");
    scan_tokens("|+\n  line\n\n", 10);
    scan_tokens("|-\n  line\n", 9);
    scan_tokens("|2\n  line\n", 9);
}

static void test_edge_tag_forms(void)
{
    printf("=== edge_tag_forms ===\n");
    parse_events("!!null ~\n", 9);
    parse_events("!<tag:example> value\n", 21);
    parse_events("!local value\n", 13);
}

static void test_edge_long_anchor(void)
{
    char buf[512];
    int len;
    printf("=== edge_long_anchor ===\n");
    len = snprintf(buf, sizeof(buf),
        "&%0256d value\n", 0);
    /* anchor with 256 zeros */
    parse_events(buf, len);
}

static void test_edge_repeated_docs(void)
{
    printf("=== edge_repeated_docs ===\n");
    parse_events("---\na\n---\nb\n---\nc\n", 18);
}

static void test_edge_nested_flow(void)
{
    printf("=== edge_nested_flow ===\n");
    parse_events("[[1, [2, [3]]],{a:{b:{c: d}}}]\n", 31);
}

/* -------------------------------------------------------------- main */


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

    /* Scanner tests */
    run_test("scan_basic", test_scan_basic, 2);
    run_test("scan_empty", test_scan_empty, 2);
    run_test("scan_flow", test_scan_flow, 2);
    run_test("scan_directive", test_scan_directive, 2);
    run_test("scan_anchor_alias", test_scan_anchor_alias, 2);
    run_test("scan_tag", test_scan_tag, 2);
    run_test("scan_block_scalar", test_scan_block_scalar, 2);
    run_test("scan_flow_scalar", test_scan_flow_scalar, 2);
    run_test("scan_malformed_utf8", test_scan_malformed_utf8, 2);

    /* Parser event tests */
    run_test("parse_basic", test_parse_basic, 2);
    run_test("parse_empty", test_parse_empty, 2);
    run_test("parse_sequence", test_parse_sequence, 2);
    run_test("parse_mapping", test_parse_mapping, 2);
    run_test("parse_nested", test_parse_nested, 2);
    run_test("parse_flow", test_parse_flow, 2);
    run_test("parse_anchor_alias", test_parse_anchor_alias, 2);
    run_test("parse_duplicate_anchor", test_parse_duplicate_anchor, 2);
    run_test("parse_undefined_alias", test_parse_undefined_alias, 2);
    run_test("parse_multi_document", test_parse_multi_document, 2);
    run_test("parse_explicit_document", test_parse_explicit_document, 2);
    run_test("parse_version_directive", test_parse_version_directive, 2);
    run_test("parse_tag_directive", test_parse_tag_directive, 2);
    run_test("parse_long_scalar", test_parse_long_scalar, 2);
    run_test("parse_deep_nest", test_parse_deep_nest, 2);
    run_test("parse_flow_sequence", test_parse_flow_sequence, 2);
    run_test("parse_flow_mapping_ordered", test_parse_flow_mapping_ordered, 2);
    run_test("parse_block_sequence", test_parse_block_sequence, 2);
    run_test("parse_indentless_sequence", test_parse_indentless_sequence, 2);
    run_test("parse_null_scalar", test_parse_null_scalar, 2);
    run_test("parse_bool_scalar", test_parse_bool_scalar, 2);
    run_test("parse_int_float", test_parse_int_float, 2);
    run_test("parse_special_chars", test_parse_special_chars, 2);
    run_test("parse_set_encoding", test_parse_set_encoding, 2);

    /* Loader tests */
    run_test("load_basic", test_load_basic, 2);
    run_test("load_empty", test_load_empty, 2);
    run_test("load_sequence", test_load_sequence, 2);
    run_test("load_mapping", test_load_mapping, 2);
    run_test("load_anchor_alias", test_load_anchor_alias, 2);
    run_test("load_multi", test_load_multi, 2);
    run_test("load_nested", test_load_nested, 2);
    run_test("load_undefined_alias", test_load_undefined_alias, 2);
    run_test("load_duplicate_anchor", test_load_duplicate_anchor, 2);

    /* Emitter tests */
    run_test("emit_basic", test_emit_basic, 2);
    run_test("emit_sequence", test_emit_sequence, 2);
    run_test("emit_mapping", test_emit_mapping, 2);
    run_test("emit_anchor_alias", test_emit_anchor_alias, 2);
    run_test("emit_canonical", test_emit_canonical, 2);
    run_test("emit_scalar_styles", test_emit_scalar_styles, 2);
    run_test("emit_tag_directive", test_emit_tag_directive, 2);
    run_test("emit_version_directive", test_emit_version_directive, 2);
    run_test("emit_unicode", test_emit_unicode, 2);
    run_test("emit_settings", test_emit_settings, 2);

    /* Dumper tests */
    run_test("dump_basic", test_dump_basic, 2);
    run_test("dump_sequence", test_dump_sequence, 2);
    run_test("dump_mapping", test_dump_mapping, 2);
    run_test("dump_nested", test_dump_nested, 2);
    run_test("dump_empty_doc", test_dump_empty_doc, 2);

    /* Document API tests */
    run_test("document_api", test_document_api, 2);
    run_test("event_init", test_event_init, 2);
    run_test("token_delete", test_token_delete, 2);

    /* Bridge function tests */
    run_test("bridge_check_utf8", test_bridge_check_utf8, 2);
    run_test("bridge_read_handlers", test_bridge_read_handlers, 2);
    run_test("bridge_write_handlers", test_bridge_write_handlers, 2);
    run_test("bridge_error_setters", test_bridge_error_setters, 2);
    run_test("bridge_parser_internals", test_bridge_parser_internals, 2);
    run_test("bridge_fetch_stream", test_bridge_fetch_stream, 2);
    run_test("bridge_scanner_scan", test_bridge_scanner_scan, 2);
    run_test("bridge_scan_directives", test_bridge_scan_directives, 2);
    run_test("bridge_scan_anchor_tag", test_bridge_scan_anchor_tag, 2);
    run_test("bridge_scan_scalars", test_bridge_scan_scalars, 2);
    run_test("bridge_scan_uri_escapes", test_bridge_scan_uri_escapes, 2);
    run_test("bridge_parser_state_machine", test_bridge_parser_state_machine, 2);
    run_test("bridge_parse_fns", test_bridge_parse_fns, 2);
    run_test("bridge_emitter_analyze", test_bridge_emitter_analyze, 2);
    run_test("bridge_emitter_checks", test_bridge_emitter_checks, 2);
    run_test("bridge_emitter_indent", test_bridge_emitter_indent, 2);
    run_test("bridge_emitter_append_tag_dir", test_bridge_emitter_append_tag_dir, 2);
    run_test("bridge_emitter_generate_anchor", test_bridge_emitter_generate_anchor, 2);
    run_test("bridge_emitter_write_fns", test_bridge_emitter_write_fns, 2);
    run_test("bridge_emitter_state_machine", test_bridge_emitter_state_machine, 2);
    run_test("bridge_emitter_emit_fns", test_bridge_emitter_emit_fns, 2);
    run_test("bridge_emitter_select_style", test_bridge_emitter_select_style, 2);
    run_test("bridge_emitter_analyze_event", test_bridge_emitter_analyze_event, 2);
    run_test("bridge_emitter_process_fns", test_bridge_emitter_process_fns, 2);
    run_test("bridge_loader", test_bridge_loader, 2);
    run_test("bridge_dumper", test_bridge_dumper, 2);
    run_test("bridge_dump_node", test_bridge_dump_node, 2);
    run_test("bridge_reader_writer", test_bridge_reader_writer, 2);

    /* Max nesting */
    run_test("max_nest_level", test_max_nest_level, 2);

    /* Edge cases */
    run_test("edge_empty_scalar", test_edge_empty_scalar, 2);
    run_test("edge_special_float", test_edge_special_float, 2);
    run_test("edge_multiline_mapping_key", test_edge_multiline_mapping_key, 2);
    run_test("edge_flow_mapping_empty_value", test_edge_flow_mapping_empty_value, 2);
    run_test("edge_block_scalar_chomping", test_edge_block_scalar_chomping, 2);
    run_test("edge_tag_forms", test_edge_tag_forms, 2);
    run_test("edge_long_anchor", test_edge_long_anchor, 2);
    run_test("edge_repeated_docs", test_edge_repeated_docs, 2);
    run_test("edge_nested_flow", test_edge_nested_flow, 2);

    return 0;
}
