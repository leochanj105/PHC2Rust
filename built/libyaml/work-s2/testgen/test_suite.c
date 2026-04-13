/*
 * test_suite.c — libyaml comprehensive test suite.
 * Exercises all public API functions and internal (bridge_*) static functions.
 * All output is deterministic: numeric/enum values only.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <assert.h>
#include <yaml.h>

/* Internal API functions declared in yaml_private.h but exported */
YAML_DECLARE(void *) yaml_malloc(size_t size);
YAML_DECLARE(void *) yaml_realloc(void *ptr, size_t size);
YAML_DECLARE(void)   yaml_free(void *ptr);
YAML_DECLARE(yaml_char_t *) yaml_strdup(const yaml_char_t *str);
YAML_DECLARE(int) yaml_string_extend(yaml_char_t **start,
        yaml_char_t **pointer, yaml_char_t **end);
YAML_DECLARE(int) yaml_string_join(
        yaml_char_t **a_start, yaml_char_t **a_pointer, yaml_char_t **a_end,
        yaml_char_t **b_start, yaml_char_t **b_pointer, yaml_char_t **b_end);
YAML_DECLARE(int) yaml_stack_extend(void **start, void **top, void **end);
YAML_DECLARE(int) yaml_queue_extend(void **start, void **head,
        void **tail, void **end);

/* Internal types needed for bridge declarations (must precede test_bridge.h) */
typedef struct {
    yaml_char_t *start;
    yaml_char_t *end;
    yaml_char_t *pointer;
} yaml_string_t;

struct loader_ctx {
    int *start;
    int *end;
    int *top;
};

#include "test_bridge.h"

/* ------------------------------------------------------------------ */
/*  Helpers                                                            */
/* ------------------------------------------------------------------ */

/* Accumulating write handler for emitter output */
typedef struct {
    unsigned char buf[65536];
    size_t        len;
} membuf_t;

static int membuf_write_handler(void *data, unsigned char *buffer, size_t size)
{
    membuf_t *mb = (membuf_t *)data;
    if (mb->len + size > sizeof(mb->buf)) return 0;
    memcpy(mb->buf + mb->len, buffer, size);
    mb->len += size;
    return 1;
}

/* Helper: emit a complete minimal YAML stream via event API and print output */
static void emit_stream_to_membuf(membuf_t *mb, int canonical, int use_unicode)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, mb);
    yaml_emitter_set_canonical(&emitter, canonical);
    yaml_emitter_set_unicode(&emitter, use_unicode);
    yaml_emitter_set_indent(&emitter, 2);
    yaml_emitter_set_width(&emitter, 80);
    yaml_emitter_set_break(&emitter, YAML_LN_BREAK);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"hello", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/*  1. Version                                                         */
/* ------------------------------------------------------------------ */

static void test_version(void)
{
    int major, minor, patch;
    yaml_get_version(&major, &minor, &patch);
    printf("version_major: %d\n", major);
    printf("version_minor: %d\n", minor);
    printf("version_patch: %d\n", patch);
}

/* ------------------------------------------------------------------ */
/*  2. Memory helpers                                                  */
/* ------------------------------------------------------------------ */

static void test_memory(void)
{
    void *p = yaml_malloc(16);
    printf("malloc_nonnull: %d\n", p != NULL);
    p = yaml_realloc(p, 32);
    printf("realloc_nonnull: %d\n", p != NULL);
    yaml_free(p);
    yaml_free(NULL); /* must not crash */
    printf("free_null_ok: 1\n");

    yaml_char_t *s = yaml_strdup((yaml_char_t *)"test");
    printf("strdup_nonnull: %d\n", s != NULL);
    printf("strdup_content: %d\n", strcmp((char *)s, "test") == 0);
    yaml_free(s);

    yaml_char_t *ns = yaml_strdup(NULL);
    printf("strdup_null: %d\n", ns == NULL);
}

/* ------------------------------------------------------------------ */
/*  3. String / Stack / Queue extend                                   */
/* ------------------------------------------------------------------ */

static void test_string_extend(void)
{
    yaml_char_t *start = (yaml_char_t *)yaml_malloc(16);
    yaml_char_t *end   = start + 16;
    yaml_char_t *ptr   = start;
    int ret = yaml_string_extend(&start, &ptr, &end);
    printf("string_extend_ret: %d\n", ret);
    printf("string_extend_grew: %d\n", (int)(end - start) == 32);
    yaml_free(start);
}

static void test_string_join(void)
{
    /* a = "hello", b = " world" */
    size_t alen = 8, blen = 6;
    yaml_char_t *as = (yaml_char_t *)yaml_malloc(alen);
    yaml_char_t *ae = as + alen;
    yaml_char_t *ap = as;
    memcpy(as, "hello\0\0\0", alen);
    ap = as + 5;

    yaml_char_t *bs = (yaml_char_t *)yaml_malloc(blen);
    yaml_char_t *be = bs + blen;
    yaml_char_t *bp = bs + blen;
    memcpy(bs, " world", blen);

    int ret = yaml_string_join(&as, &ap, &ae, &bs, &bp, &be);
    printf("string_join_ret: %d\n", ret);
    printf("string_join_len: %d\n", (int)(ap - as));
    yaml_free(as);
    yaml_free(bs);
}

static void test_stack_extend(void)
{
    int *start = (int *)yaml_malloc(4 * sizeof(int));
    int *top   = start;
    int *end   = start + 4;
    int ret = yaml_stack_extend((void **)&start, (void **)&top, (void **)&end);
    printf("stack_extend_ret: %d\n", ret);
    printf("stack_extend_grew: %d\n", (int)(end - start) == 8);
    yaml_free(start);
}

static void test_queue_extend(void)
{
    int *start = (int *)yaml_malloc(4 * sizeof(int));
    int *head  = start;
    int *tail  = start + 4;
    int *end   = start + 4;
    int ret = yaml_queue_extend((void **)&start, (void **)&head,
                                (void **)&tail, (void **)&end);
    printf("queue_extend_ret: %d\n", ret);
    printf("queue_extend_grew: %d\n", (int)(end - start) == 8);
    yaml_free(start);
}

/* ------------------------------------------------------------------ */
/*  4. Token                                                           */
/* ------------------------------------------------------------------ */

static void test_token_delete(void)
{
    yaml_token_t tok;
    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_SCALAR_TOKEN;
    tok.data.scalar.value = (yaml_char_t *)yaml_strdup((yaml_char_t *)"val");
    tok.data.scalar.length = 3;
    tok.data.scalar.style = YAML_PLAIN_SCALAR_STYLE;
    yaml_token_delete(&tok);
    printf("token_delete_scalar_ok: 1\n");

    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_TAG_TOKEN;
    tok.data.tag.handle = (yaml_char_t *)yaml_strdup((yaml_char_t *)"!");
    tok.data.tag.suffix = (yaml_char_t *)yaml_strdup((yaml_char_t *)"str");
    yaml_token_delete(&tok);
    printf("token_delete_tag_ok: 1\n");

    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_ALIAS_TOKEN;
    tok.data.alias.value = (yaml_char_t *)yaml_strdup((yaml_char_t *)"anchor1");
    yaml_token_delete(&tok);
    printf("token_delete_alias_ok: 1\n");

    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_ANCHOR_TOKEN;
    tok.data.anchor.value = (yaml_char_t *)yaml_strdup((yaml_char_t *)"anchor1");
    yaml_token_delete(&tok);
    printf("token_delete_anchor_ok: 1\n");

    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_TAG_DIRECTIVE_TOKEN;
    tok.data.tag_directive.handle = (yaml_char_t *)yaml_strdup((yaml_char_t *)"!");
    tok.data.tag_directive.prefix = (yaml_char_t *)yaml_strdup((yaml_char_t *)"tag:yaml.org,2002:");
    yaml_token_delete(&tok);
    printf("token_delete_tag_dir_ok: 1\n");
}

/* ------------------------------------------------------------------ */
/*  5. Event initialize / delete                                       */
/* ------------------------------------------------------------------ */

static void test_events_initialize(void)
{
    yaml_event_t ev;

    /* STREAM-START */
    assert(yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING));
    printf("event_type_stream_start: %d\n", ev.type);
    printf("event_stream_start_encoding: %d\n", ev.data.stream_start.encoding);
    yaml_event_delete(&ev);

    /* STREAM-END */
    assert(yaml_stream_end_event_initialize(&ev));
    printf("event_type_stream_end: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* DOCUMENT-START implicit */
    assert(yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1));
    printf("event_type_doc_start: %d\n", ev.type);
    printf("event_doc_start_implicit: %d\n", ev.data.document_start.implicit);
    yaml_event_delete(&ev);

    /* DOCUMENT-START with version directive */
    yaml_version_directive_t vd = {1, 1};
    assert(yaml_document_start_event_initialize(&ev, &vd, NULL, NULL, 0));
    printf("event_doc_start_ver_major: %d\n",
        ev.data.document_start.version_directive->major);
    printf("event_doc_start_ver_minor: %d\n",
        ev.data.document_start.version_directive->minor);
    yaml_event_delete(&ev);

    /* DOCUMENT-END */
    assert(yaml_document_end_event_initialize(&ev, 1));
    printf("event_type_doc_end: %d\n", ev.type);
    printf("event_doc_end_implicit: %d\n", ev.data.document_end.implicit);
    yaml_event_delete(&ev);

    /* ALIAS */
    assert(yaml_alias_event_initialize(&ev, (yaml_char_t *)"anc1"));
    printf("event_type_alias: %d\n", ev.type);
    printf("event_alias_anchor_len: %d\n", (int)strlen((char*)ev.data.alias.anchor));
    yaml_event_delete(&ev);

    /* SCALAR */
    assert(yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"value", 5, 1, 0, YAML_PLAIN_SCALAR_STYLE));
    printf("event_type_scalar: %d\n", ev.type);
    printf("event_scalar_length: %zu\n", ev.data.scalar.length);
    printf("event_scalar_style: %d\n", ev.data.scalar.style);
    printf("event_scalar_plain_implicit: %d\n", ev.data.scalar.plain_implicit);
    printf("event_scalar_quoted_implicit: %d\n", ev.data.scalar.quoted_implicit);
    yaml_event_delete(&ev);

    /* SCALAR with anchor and tag */
    assert(yaml_scalar_event_initialize(&ev,
        (yaml_char_t *)"anc", (yaml_char_t *)"!!str",
        (yaml_char_t *)"hello", -1, 0, 1, YAML_SINGLE_QUOTED_SCALAR_STYLE));
    printf("event_scalar_with_tag_len: %zu\n", ev.data.scalar.length);
    printf("event_scalar_with_tag_style: %d\n", ev.data.scalar.style);
    yaml_event_delete(&ev);

    /* SEQUENCE-START */
    assert(yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE));
    printf("event_type_seq_start: %d\n", ev.type);
    printf("event_seq_start_style: %d\n", ev.data.sequence_start.style);
    printf("event_seq_start_implicit: %d\n", ev.data.sequence_start.implicit);
    yaml_event_delete(&ev);

    /* SEQUENCE-END */
    assert(yaml_sequence_end_event_initialize(&ev));
    printf("event_type_seq_end: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* MAPPING-START */
    assert(yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE));
    printf("event_type_map_start: %d\n", ev.type);
    printf("event_map_start_style: %d\n", ev.data.mapping_start.style);
    yaml_event_delete(&ev);

    /* MAPPING-END */
    assert(yaml_mapping_end_event_initialize(&ev));
    printf("event_type_map_end: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* DOCUMENT-START with tag directives */
    yaml_tag_directive_t td[1];
    td[0].handle = (yaml_char_t *)"!";
    td[0].prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    assert(yaml_document_start_event_initialize(&ev, NULL, td, td+1, 0));
    printf("event_doc_start_tag_dirs: %d\n",
        (int)(ev.data.document_start.tag_directives.end -
              ev.data.document_start.tag_directives.start));
    yaml_event_delete(&ev);
}

/* ------------------------------------------------------------------ */
/*  6. Parser: scan (token-by-token)                                   */
/* ------------------------------------------------------------------ */

static void test_parser_scan(void)
{
    static const unsigned char yaml[] = "key: value\n";
    yaml_parser_t parser;
    yaml_token_t  token;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    int count = 0;
    for (;;) {
        if (!yaml_parser_scan(&parser, &token)) {
            printf("scan_error: %d\n", parser.error);
            break;
        }
        printf("token_%d_type: %d\n", count, token.type);
        if (token.type == YAML_SCALAR_TOKEN) {
            printf("token_%d_scalar_len: %zu\n", count, token.data.scalar.length);
            printf("token_%d_scalar_style: %d\n", count, token.data.scalar.style);
        }
        int done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
        count++;
        if (done) break;
    }
    printf("scan_token_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/*  7. Parser: parse (event-by-event)                                  */
/* ------------------------------------------------------------------ */

static void print_parse_events(const unsigned char *input, size_t len,
                                const char *label)
{
    yaml_parser_t parser;
    yaml_event_t  event;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, input, len);

    int count = 0;
    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("%s_parse_error: %d\n", label, parser.error);
            break;
        }
        printf("%s_event_%d: %d\n", label, count, event.type);
        if (event.type == YAML_SCALAR_EVENT) {
            printf("%s_scalar_%d_len: %zu\n", label, count,
                   event.data.scalar.length);
            printf("%s_scalar_%d_style: %d\n", label, count,
                   event.data.scalar.style);
            printf("%s_scalar_%d_plain_implicit: %d\n", label, count,
                   event.data.scalar.plain_implicit);
        }
        if (event.type == YAML_SEQUENCE_START_EVENT) {
            printf("%s_seq_start_%d_style: %d\n", label, count,
                   event.data.sequence_start.style);
        }
        if (event.type == YAML_MAPPING_START_EVENT) {
            printf("%s_map_start_%d_style: %d\n", label, count,
                   event.data.mapping_start.style);
        }
        if (event.type == YAML_DOCUMENT_START_EVENT) {
            printf("%s_doc_start_%d_implicit: %d\n", label, count,
                   event.data.document_start.implicit);
        }
        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        count++;
        if (done) break;
    }
    printf("%s_event_count: %d\n", label, count);
    yaml_parser_delete(&parser);
}

static void test_parser_parse(void)
{
    /* Simple scalar */
    static const unsigned char y1[] = "hello\n";
    print_parse_events(y1, sizeof(y1)-1, "scalar");

    /* Mapping */
    static const unsigned char y2[] = "key: value\nfoo: bar\n";
    print_parse_events(y2, sizeof(y2)-1, "mapping");

    /* Sequence */
    static const unsigned char y3[] = "- a\n- b\n- c\n";
    print_parse_events(y3, sizeof(y3)-1, "sequence");

    /* Flow sequence */
    static const unsigned char y4[] = "[1, 2, 3]\n";
    print_parse_events(y4, sizeof(y4)-1, "flow_seq");

    /* Flow mapping */
    static const unsigned char y5[] = "{a: 1, b: 2}\n";
    print_parse_events(y5, sizeof(y5)-1, "flow_map");

    /* Anchors and aliases */
    static const unsigned char y6[] = "a: &anc val\nb: *anc\n";
    print_parse_events(y6, sizeof(y6)-1, "anchor_alias");

    /* Double-quoted scalar */
    static const unsigned char y7[] = "\"hello world\"\n";
    print_parse_events(y7, sizeof(y7)-1, "double_quoted");

    /* Single-quoted scalar */
    static const unsigned char y8[] = "'hello world'\n";
    print_parse_events(y8, sizeof(y8)-1, "single_quoted");

    /* Literal block scalar */
    static const unsigned char y9[] = "|\n  hello\n  world\n";
    print_parse_events(y9, sizeof(y9)-1, "literal_block");

    /* Folded block scalar */
    static const unsigned char ya[] = ">\n  hello\n  world\n";
    print_parse_events(ya, sizeof(ya)-1, "folded_block");

    /* Explicit document markers */
    static const unsigned char yb[] = "---\nhello\n...\n";
    print_parse_events(yb, sizeof(yb)-1, "explicit_doc");

    /* Version directive */
    static const unsigned char yc[] = "%YAML 1.1\n---\nhello\n";
    print_parse_events(yc, sizeof(yc)-1, "version_dir");

    /* Tag directive */
    static const unsigned char yd[] = "%TAG ! tag:example.com,2000:\n---\nhello\n";
    print_parse_events(yd, sizeof(yd)-1, "tag_dir");

    /* Nested structures */
    static const unsigned char ye[] = "a:\n  b:\n    c: d\n";
    print_parse_events(ye, sizeof(ye)-1, "nested");

    /* Multiple documents */
    static const unsigned char yf[] = "---\nhello\n---\nworld\n";
    print_parse_events(yf, sizeof(yf)-1, "multi_doc");

    /* Empty document */
    static const unsigned char yg[] = "---\n...\n";
    print_parse_events(yg, sizeof(yg)-1, "empty_doc");

    /* Null value */
    static const unsigned char yh[] = "key:\n";
    print_parse_events(yh, sizeof(yh)-1, "null_value");

    /* Integer-like scalar */
    static const unsigned char yi[] = "42\n";
    print_parse_events(yi, sizeof(yi)-1, "int_scalar");

    /* Tagged scalar */
    static const unsigned char yj[] = "!!int 42\n";
    print_parse_events(yj, sizeof(yj)-1, "tagged_scalar");

    /* Parser error: invalid YAML */
    static const unsigned char bad[] = ": :\n  bad: [unclosed\n";
    yaml_parser_t parser;
    yaml_event_t  ev;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, bad, sizeof(bad)-1);
    int ok = 1;
    while (ok) {
        ok = yaml_parser_parse(&parser, &ev);
        if (!ok) {
            printf("parse_error_type: %d\n", parser.error);
        } else {
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
    }
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/*  8. Parser: load (document API)                                     */
/* ------------------------------------------------------------------ */

static void test_parser_load(void)
{
    static const unsigned char yaml[] = "key: value\nlist:\n  - a\n  - b\n";
    yaml_parser_t    parser;
    yaml_document_t  doc;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    int doc_count = 0;
    while (1) {
        if (!yaml_parser_load(&parser, &doc)) {
            printf("load_error: %d\n", parser.error);
            break;
        }
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        if (!root) {
            yaml_document_delete(&doc);
            break;
        }
        printf("load_doc_%d_root_type: %d\n", doc_count, root->type);

        /* Walk the document */
        int n = (int)(doc.nodes.top - doc.nodes.start);
        printf("load_doc_%d_node_count: %d\n", doc_count, n);
        for (int i = 1; i <= n; i++) {
            yaml_node_t *node = yaml_document_get_node(&doc, i);
            if (!node) continue;
            printf("load_doc_%d_node_%d_type: %d\n", doc_count, i, node->type);
            if (node->type == YAML_SCALAR_NODE) {
                printf("load_doc_%d_node_%d_scalar_len: %zu\n",
                    doc_count, i, node->data.scalar.length);
                printf("load_doc_%d_node_%d_scalar_style: %d\n",
                    doc_count, i, node->data.scalar.style);
            }
        }
        yaml_document_delete(&doc);
        doc_count++;
    }
    printf("load_doc_count: %d\n", doc_count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/*  9. Document API                                                    */
/* ------------------------------------------------------------------ */

static void test_document_api(void)
{
    yaml_document_t doc;

    /* Initialize empty document */
    assert(yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1));
    printf("doc_init_start_implicit: %d\n", doc.start_implicit);
    printf("doc_init_end_implicit: %d\n", doc.end_implicit);

    int s1 = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"hello", 5,
        YAML_PLAIN_SCALAR_STYLE);
    printf("doc_add_scalar_id: %d\n", s1);

    int s2 = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_INT_TAG, (yaml_char_t *)"42", 2,
        YAML_PLAIN_SCALAR_STYLE);
    printf("doc_add_scalar2_id: %d\n", s2);

    int seq = yaml_document_add_sequence(&doc,
        (yaml_char_t *)YAML_SEQ_TAG, YAML_BLOCK_SEQUENCE_STYLE);
    printf("doc_add_seq_id: %d\n", seq);

    assert(yaml_document_append_sequence_item(&doc, seq, s1));
    assert(yaml_document_append_sequence_item(&doc, seq, s2));

    yaml_node_t *seq_node = yaml_document_get_node(&doc, seq);
    printf("doc_seq_node_type: %d\n", seq_node->type);
    int item_count = (int)(seq_node->data.sequence.items.top -
                           seq_node->data.sequence.items.start);
    printf("doc_seq_item_count: %d\n", item_count);

    int map = yaml_document_add_mapping(&doc,
        (yaml_char_t *)YAML_MAP_TAG, YAML_BLOCK_MAPPING_STYLE);
    printf("doc_add_map_id: %d\n", map);

    int key = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"key", 3,
        YAML_PLAIN_SCALAR_STYLE);
    int val = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"val", 3,
        YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_document_append_mapping_pair(&doc, map, key, val));

    yaml_node_t *map_node = yaml_document_get_node(&doc, map);
    printf("doc_map_node_type: %d\n", map_node->type);
    int pair_count = (int)(map_node->data.mapping.pairs.top -
                           map_node->data.mapping.pairs.start);
    printf("doc_map_pair_count: %d\n", pair_count);

    yaml_node_t *root = yaml_document_get_root_node(&doc);
    printf("doc_root_type: %d\n", root ? root->type : 0);

    /* get_node on out-of-range returns NULL */
    yaml_node_t *oob = yaml_document_get_node(&doc, 999);
    printf("doc_get_node_oob: %d\n", oob == NULL);

    yaml_document_delete(&doc);
    printf("doc_delete_ok: 1\n");

    /* Document with version directive */
    yaml_version_directive_t vd = {1, 1};
    assert(yaml_document_initialize(&doc, &vd, NULL, NULL, 0, 0));
    printf("doc_with_ver_major: %d\n", doc.version_directive->major);
    printf("doc_with_ver_minor: %d\n", doc.version_directive->minor);
    yaml_document_delete(&doc);
}

/* ------------------------------------------------------------------ */
/* 10. Emitter: emit events                                            */
/* ------------------------------------------------------------------ */

static void test_emitter_emit(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    emit_stream_to_membuf(&mb, 0, 0);
    printf("emitter_output_len_gt0: %d\n", mb.len > 0);
    /* Print length as stable value */
    printf("emitter_output_len: %zu\n", mb.len);

    /* Canonical mode */
    membuf_t mb2;
    memset(&mb2, 0, sizeof(mb2));
    emit_stream_to_membuf(&mb2, 1, 0);
    printf("emitter_canonical_len_gt0: %d\n", mb2.len > 0);

    /* Emit a mapping */
    membuf_t mb3;
    memset(&mb3, 0, sizeof(mb3));
    yaml_emitter_t emitter;
    yaml_event_t   event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb3);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_mapping_start_event_initialize(&event, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"key", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"value", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_mapping_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_emitter_delete(&emitter);
    printf("emitter_map_len_gt0: %d\n", mb3.len > 0);

    /* Emit a sequence */
    membuf_t mb4;
    memset(&mb4, 0, sizeof(mb4));
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb4);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_sequence_start_event_initialize(&event, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"item1", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"item2", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_sequence_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_emitter_delete(&emitter);
    printf("emitter_seq_len_gt0: %d\n", mb4.len > 0);

    /* Flush */
    membuf_t mb5;
    memset(&mb5, 0, sizeof(mb5));
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb5);
    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    int flush_ret = yaml_emitter_flush(&emitter);
    printf("emitter_flush_ret: %d\n", flush_ret);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 11. Emitter: dump (document API)                                    */
/* ------------------------------------------------------------------ */

static void test_emitter_dump(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));

    yaml_emitter_t  emitter;
    yaml_document_t doc;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    assert(yaml_emitter_open(&emitter));
    printf("emitter_open_ok: 1\n");

    assert(yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1));
    int s = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"dumped", 6,
        YAML_PLAIN_SCALAR_STYLE);
    printf("emitter_dump_scalar_id: %d\n", s);
    assert(yaml_emitter_dump(&emitter, &doc));
    printf("emitter_dump_ok: 1\n");

    /* dump null document to close */
    yaml_document_t empty;
    memset(&empty, 0, sizeof(empty));
    yaml_emitter_dump(&emitter, &empty);

    assert(yaml_emitter_close(&emitter));
    printf("emitter_close_ok: 1\n");
    printf("emitter_dump_len_gt0: %d\n", mb.len > 0);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 12. Parser: set_max_nest_level                                      */
/* ------------------------------------------------------------------ */

static void test_max_nest_level(void)
{
    yaml_set_max_nest_level(5);
    /* Build deeply nested YAML that exceeds level 5 */
    static const unsigned char deep[] =
        "a:\n b:\n  c:\n   d:\n    e:\n     f: too_deep\n";
    yaml_parser_t parser;
    yaml_event_t  event;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, deep, sizeof(deep)-1);
    int got_error = 0;
    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("max_nest_error: %d\n", parser.error);
            got_error = 1;
            break;
        }
        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        if (done) break;
    }
    if (!got_error) {
        printf("max_nest_no_error: 1\n");
    }
    yaml_parser_delete(&parser);

    /* Restore default */
    yaml_set_max_nest_level(1000);
}

/* ------------------------------------------------------------------ */
/* 13. Parser encoding                                                 */
/* ------------------------------------------------------------------ */

static void test_parser_encoding(void)
{
    /* UTF-8 input */
    static const unsigned char utf8[] = "hello\n";
    yaml_parser_t parser;
    yaml_event_t  event;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_encoding(&parser, YAML_UTF8_ENCODING);
    yaml_parser_set_input_string(&parser, utf8, sizeof(utf8)-1);
    assert(yaml_parser_parse(&parser, &event));
    printf("encoding_event0_type: %d\n", event.type);
    printf("encoding_stream_start_encoding: %d\n",
        event.data.stream_start.encoding);
    yaml_event_delete(&event);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 14. Bridge: check_utf8                                              */
/* ------------------------------------------------------------------ */

static void test_bridge_check_utf8(void)
{
    /* Valid ASCII */
    int r = bridge_yaml_check_utf8((yaml_char_t *)"hello", 5);
    printf("utf8_ascii: %d\n", r);

    /* Valid 2-byte UTF-8: U+00E9 = 0xC3 0xA9 */
    static const yaml_char_t u2[] = {0xC3, 0xA9};
    r = bridge_yaml_check_utf8(u2, 2);
    printf("utf8_2byte: %d\n", r);

    /* Valid 3-byte UTF-8: U+4E2D = 0xE4 0xB8 0xAD */
    static const yaml_char_t u3[] = {0xE4, 0xB8, 0xAD};
    r = bridge_yaml_check_utf8(u3, 3);
    printf("utf8_3byte: %d\n", r);

    /* Invalid: 0x80 continuation byte without leading byte */
    static const yaml_char_t bad[] = {0x80};
    r = bridge_yaml_check_utf8(bad, 1);
    printf("utf8_invalid: %d\n", r);

    /* Invalid: truncated 2-byte sequence */
    static const yaml_char_t trunc[] = {0xC3};
    r = bridge_yaml_check_utf8(trunc, 1);
    printf("utf8_truncated: %d\n", r);

    /* Empty */
    r = bridge_yaml_check_utf8((yaml_char_t *)"", 0);
    printf("utf8_empty: %d\n", r);
}

/* ------------------------------------------------------------------ */
/* 15. Bridge: read/write handlers                                     */
/* ------------------------------------------------------------------ */

static void test_bridge_read_handlers(void)
{
    yaml_parser_t parser;
    unsigned char buf[32];
    size_t size_read = 0;

    /* string read handler */
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"hello world", 11);
    int ret = bridge_yaml_string_read_handler(&parser, buf, 5, &size_read);
    printf("string_read_ret: %d\n", ret);
    printf("string_read_size: %zu\n", size_read);
    yaml_parser_delete(&parser);

    /* string read at EOF */
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, (unsigned char *)"", 0);
    size_read = 99;
    ret = bridge_yaml_string_read_handler(&parser, buf, 32, &size_read);
    printf("string_read_eof_ret: %d\n", ret);
    printf("string_read_eof_size: %zu\n", size_read);
    yaml_parser_delete(&parser);
}

static void test_bridge_write_handlers(void)
{
    yaml_emitter_t emitter;
    unsigned char outbuf[64];
    size_t written = 0;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);

    unsigned char data[] = "test";
    int ret = bridge_yaml_string_write_handler(&emitter, data, 4);
    printf("string_write_ret: %d\n", ret);
    printf("string_write_size: %zu\n", written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 16. Bridge: emitter error setters                                   */
/* ------------------------------------------------------------------ */

static void test_bridge_emitter_error(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));

    int ret = bridge_yaml_emitter_set_emitter_error(&emitter, "test error");
    printf("emitter_set_error_ret: %d\n", ret);
    printf("emitter_error_type: %d\n", emitter.error);

    yaml_emitter_delete(&emitter);
}

static void test_bridge_writer_error(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));

    int ret = bridge_yaml_emitter_set_writer_error(&emitter, "writer error");
    printf("writer_set_error_ret: %d\n", ret);
    printf("writer_error_type: %d\n", emitter.error);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 17. Bridge: emitter need_more_events                                */
/* ------------------------------------------------------------------ */

static void test_bridge_need_more_events(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    /* Fresh emitter with no events queued */
    int ret = bridge_yaml_emitter_need_more_events(&emitter);
    printf("need_more_events_empty: %d\n", ret);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 18. Bridge: emitter append_tag_directive                            */
/* ------------------------------------------------------------------ */

static void test_bridge_append_tag_directive(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));

    yaml_tag_directive_t td;
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    int ret = bridge_yaml_emitter_append_tag_directive(&emitter, td, 1);
    printf("append_tag_directive_ret: %d\n", ret);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 19. Bridge: emitter increase_indent                                 */
/* ------------------------------------------------------------------ */

static void test_bridge_increase_indent(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    emitter.best_indent = 2;
    emitter.indent = -1;

    int ret = bridge_yaml_emitter_increase_indent(&emitter, 0, 0);
    printf("increase_indent_ret: %d\n", ret);
    printf("increase_indent_val: %d\n", emitter.indent);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 20. Bridge: emitter check helpers                                   */
/* ------------------------------------------------------------------ */

static void test_bridge_emitter_checks(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));

    int r;

    r = bridge_yaml_emitter_check_empty_document(&emitter);
    printf("check_empty_doc: %d\n", r);

    r = bridge_yaml_emitter_check_empty_sequence(&emitter);
    printf("check_empty_seq: %d\n", r);

    r = bridge_yaml_emitter_check_empty_mapping(&emitter);
    printf("check_empty_map: %d\n", r);

    r = bridge_yaml_emitter_check_simple_key(&emitter);
    printf("check_simple_key: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 21. Bridge: emitter analyze functions                               */
/* ------------------------------------------------------------------ */

static void test_bridge_analyze(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;

    int ret;

    /* analyze_version_directive */
    yaml_version_directive_t vd = {1, 1};
    ret = bridge_yaml_emitter_analyze_version_directive(&emitter, vd);
    printf("analyze_ver_dir_ret: %d\n", ret);

    /* analyze_tag_directive */
    yaml_tag_directive_t td;
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    ret = bridge_yaml_emitter_analyze_tag_directive(&emitter, td);
    printf("analyze_tag_dir_ret: %d\n", ret);

    /* analyze_anchor (anchor) */
    ret = bridge_yaml_emitter_analyze_anchor(&emitter,
        (yaml_char_t *)"anchor1", 0);
    printf("analyze_anchor_ret: %d\n", ret);
    printf("analyze_anchor_len: %zu\n", emitter.anchor_data.anchor_length);
    printf("analyze_anchor_alias: %d\n", emitter.anchor_data.alias);

    /* analyze_anchor (alias) */
    ret = bridge_yaml_emitter_analyze_anchor(&emitter,
        (yaml_char_t *)"anchor1", 1);
    printf("analyze_alias_ret: %d\n", ret);
    printf("analyze_alias_flag: %d\n", emitter.anchor_data.alias);

    /* analyze_tag */
    ret = bridge_yaml_emitter_analyze_tag(&emitter,
        (yaml_char_t *)"tag:yaml.org,2002:str");
    printf("analyze_tag_ret: %d\n", ret);

    /* analyze_scalar */
    ret = bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"hello", 5);
    printf("analyze_scalar_ret: %d\n", ret);
    printf("analyze_scalar_flow_plain: %d\n",
        emitter.scalar_data.flow_plain_allowed);
    printf("analyze_scalar_block_plain: %d\n",
        emitter.scalar_data.block_plain_allowed);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 22. Bridge: emitter process functions                               */
/* ------------------------------------------------------------------ */

static void test_bridge_process(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.best_indent = 2;
    emitter.best_width = 80;
    emitter.line_break = YAML_LN_BREAK;
    emitter.whitespace = 1;
    emitter.indention = 1;
    emitter.column = 0;

    /* Set up anchor data for process_anchor */
    bridge_yaml_emitter_analyze_anchor(&emitter, (yaml_char_t *)"anc1", 0);
    int ret = bridge_yaml_emitter_process_anchor(&emitter);
    printf("process_anchor_ret: %d\n", ret);

    /* Set up scalar data for process_scalar */
    bridge_yaml_emitter_analyze_scalar(&emitter, (yaml_char_t *)"hello", 5);
    emitter.scalar_data.style = YAML_PLAIN_SCALAR_STYLE;
    ret = bridge_yaml_emitter_process_scalar(&emitter);
    printf("process_scalar_ret: %d\n", ret);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 23. Bridge: emitter write functions                                 */
/* ------------------------------------------------------------------ */

static void test_bridge_write(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.best_indent = 2;
    emitter.best_width = 80;
    emitter.line_break = YAML_LN_BREAK;
    emitter.column = 0;
    emitter.whitespace = 1;
    emitter.indention = 1;

    int ret;

    ret = bridge_yaml_emitter_write_indicator(&emitter, "---", 0, 0, 0);
    printf("write_indicator_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_indent(&emitter);
    printf("write_indent_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_anchor(&emitter,
        (yaml_char_t *)"anc1", 4);
    printf("write_anchor_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_tag_handle(&emitter,
        (yaml_char_t *)"!", 1);
    printf("write_tag_handle_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_tag_content(&emitter,
        (yaml_char_t *)"str", 3, 0);
    printf("write_tag_content_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_plain_scalar(&emitter,
        (yaml_char_t *)"hello", 5, 1);
    printf("write_plain_scalar_ret: %d\n", ret);

    yaml_emitter_delete(&emitter);
}

static void test_bridge_write_quoted(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.best_indent = 2;
    emitter.best_width = 80;
    emitter.line_break = YAML_LN_BREAK;
    emitter.column = 0;
    emitter.whitespace = 1;
    emitter.indention = 1;

    int ret;

    ret = bridge_yaml_emitter_write_single_quoted_scalar(&emitter,
        (yaml_char_t *)"hello", 5, 1);
    printf("write_single_quoted_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_double_quoted_scalar(&emitter,
        (yaml_char_t *)"world", 5, 1);
    printf("write_double_quoted_ret: %d\n", ret);

    yaml_emitter_delete(&emitter);
}

static void test_bridge_write_block(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.best_indent = 2;
    emitter.best_width = 80;
    emitter.line_break = YAML_LN_BREAK;
    emitter.column = 0;
    emitter.indent = 0;
    emitter.whitespace = 1;
    emitter.indention = 1;

    int ret;

    ret = bridge_yaml_emitter_write_literal_scalar(&emitter,
        (yaml_char_t *)"hello\nworld\n", 12);
    printf("write_literal_scalar_ret: %d\n", ret);

    ret = bridge_yaml_emitter_write_folded_scalar(&emitter,
        (yaml_char_t *)"hello world\n", 12);
    printf("write_folded_scalar_ret: %d\n", ret);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 24. Bridge: emitter generate anchor                                 */
/* ------------------------------------------------------------------ */

static void test_bridge_generate_anchor(void)
{
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));

    yaml_char_t *anc = bridge_yaml_emitter_generate_anchor(&emitter, 1);
    printf("generate_anchor_nonnull: %d\n", anc != NULL);
    if (anc) {
        printf("generate_anchor_len_gt0: %d\n", strlen((char *)anc) > 0);
        yaml_free(anc);
    }

    anc = bridge_yaml_emitter_generate_anchor(&emitter, 42);
    printf("generate_anchor_42_nonnull: %d\n", anc != NULL);
    if (anc) yaml_free(anc);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 25. Bridge: parser error setters                                    */
/* ------------------------------------------------------------------ */

static void test_bridge_parser_errors(void)
{
    yaml_parser_t parser;
    yaml_mark_t mark = {0, 0, 0};

    assert(yaml_parser_initialize(&parser));
    int ret = bridge_yaml_parser_set_parser_error(&parser, "test error", mark);
    printf("parser_set_error_ret: %d\n", ret);
    printf("parser_error_type: %d\n", parser.error);
    yaml_parser_delete(&parser);

    assert(yaml_parser_initialize(&parser));
    ret = bridge_yaml_parser_set_parser_error_context(&parser,
        "ctx", mark, "prob", mark);
    printf("parser_set_error_ctx_ret: %d\n", ret);
    printf("parser_error_ctx_type: %d\n", parser.error);
    yaml_parser_delete(&parser);

    assert(yaml_parser_initialize(&parser));
    ret = bridge_yaml_parser_set_scanner_error(&parser, "ctx", mark, "prob");
    printf("scanner_set_error_ret: %d\n", ret);
    printf("scanner_error_type: %d\n", parser.error);
    yaml_parser_delete(&parser);

    assert(yaml_parser_initialize(&parser));
    ret = bridge_yaml_parser_set_reader_error(&parser, "prob", 0, -1);
    printf("reader_set_error_ret: %d\n", ret);
    printf("reader_error_type: %d\n", parser.error);
    yaml_parser_delete(&parser);

    assert(yaml_parser_initialize(&parser));
    ret = bridge_yaml_parser_set_composer_error(&parser, "prob", mark);
    printf("composer_set_error_ret: %d\n", ret);
    printf("composer_error_type: %d\n", parser.error);
    yaml_parser_delete(&parser);

    assert(yaml_parser_initialize(&parser));
    ret = bridge_yaml_parser_set_composer_error_context(&parser,
        "ctx", mark, "prob", mark);
    printf("composer_error_ctx_ret: %d\n", ret);
    printf("composer_error_ctx_type: %d\n", parser.error);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 26. Bridge: parser scanner helpers via full parse                   */
/* ------------------------------------------------------------------ */

static void test_bridge_scanner_via_parse(void)
{
    /* Use bridge_yaml_parser_fetch_next_token via a real parse */
    static const unsigned char yaml[] = "key: value\n";
    yaml_parser_t parser;
    yaml_token_t  token;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    /* Use bridge_yaml_parser_determine_encoding */
    int ret = bridge_yaml_parser_determine_encoding(&parser);
    printf("determine_encoding_ret: %d\n", ret);
    printf("determine_encoding_val: %d\n", parser.encoding);

    /* Use bridge_yaml_parser_update_raw_buffer */
    /* Parser needs input already set above, raw_buffer partially filled */
    ret = bridge_yaml_parser_update_raw_buffer(&parser);
    printf("update_raw_buffer_ret: %d\n", ret);

    /* Now scan via bridge */
    ret = bridge_yaml_parser_fetch_next_token(&parser);
    printf("fetch_next_token_ret: %d\n", ret);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 27. Bridge: parser flow/indent level helpers                        */
/* ------------------------------------------------------------------ */

static void test_bridge_flow_indent(void)
{
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"[a, b]", 6);

    /* increase flow level */
    int ret = bridge_yaml_parser_increase_flow_level(&parser);
    printf("increase_flow_level_ret: %d\n", ret);
    printf("flow_level_after_inc: %d\n", parser.flow_level);

    /* decrease flow level */
    ret = bridge_yaml_parser_decrease_flow_level(&parser);
    printf("decrease_flow_level_ret: %d\n", ret);
    printf("flow_level_after_dec: %d\n", parser.flow_level);

    yaml_parser_delete(&parser);
}

static void test_bridge_roll_unroll_indent(void)
{
    yaml_parser_t parser;
    yaml_mark_t mark = {0, 0, 0};
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"key: value\n", 11);
    parser.indent = -1;

    int ret = bridge_yaml_parser_roll_indent(&parser, 0, -1,
        YAML_BLOCK_MAPPING_START_TOKEN, mark);
    printf("roll_indent_ret: %d\n", ret);
    printf("indent_after_roll: %d\n", parser.indent);

    ret = bridge_yaml_parser_unroll_indent(&parser, -1);
    printf("unroll_indent_ret: %d\n", ret);
    printf("indent_after_unroll: %d\n", parser.indent);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 28. Bridge: parser simple key helpers                               */
/* ------------------------------------------------------------------ */

static void test_bridge_simple_keys(void)
{
    static const unsigned char yaml[] = "key: value\n";
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    /* Need to prime token stream first */
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);

    int ret;
    ret = bridge_yaml_parser_stale_simple_keys(&parser);
    printf("stale_simple_keys_ret: %d\n", ret);

    ret = bridge_yaml_parser_save_simple_key(&parser);
    printf("save_simple_key_ret: %d\n", ret);

    ret = bridge_yaml_parser_remove_simple_key(&parser);
    printf("remove_simple_key_ret: %d\n", ret);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 29. Bridge: parser fetch functions                                  */
/* ------------------------------------------------------------------ */

static void test_bridge_fetch_stream(void)
{
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"hello\n", 6);
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);

    int ret = bridge_yaml_parser_fetch_stream_start(&parser);
    printf("fetch_stream_start_ret: %d\n", ret);
    printf("stream_start_produced: %d\n", parser.stream_start_produced);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 30. Bridge: parser scan_to_next_token                               */
/* ------------------------------------------------------------------ */

static void test_bridge_scan_to_next_token(void)
{
    static const unsigned char yaml[] = "   # comment\nkey: value\n";
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);

    /* Fill buffer first */
    int ret = bridge_yaml_parser_scan_to_next_token(&parser);
    printf("scan_to_next_token_ret: %d\n", ret);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 31. Bridge: parser state_machine via full parse                     */
/* ------------------------------------------------------------------ */

static void test_bridge_parser_state_machine(void)
{
    static const unsigned char yaml[] = "hello\n";
    yaml_parser_t parser;
    yaml_event_t  event;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    /* Initial state */
    printf("parser_initial_state: %d\n", parser.state);

    /* Use state machine directly for stream start */
    int ret = bridge_yaml_parser_state_machine(&parser, &event);
    printf("parser_state_machine_ret: %d\n", ret);
    printf("parser_state_machine_event: %d\n", event.type);
    yaml_event_delete(&event);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 32. Bridge: emitter state_machine                                   */
/* ------------------------------------------------------------------ */

static void test_bridge_emitter_state_machine(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;

    /* STREAM-START event */
    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    int ret = bridge_yaml_emitter_state_machine(&emitter, &event);
    printf("emitter_state_machine_ret: %d\n", ret);
    printf("emitter_state_after_stream_start: %d\n", emitter.state);
    yaml_event_delete(&event);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 33. Bridge: emitter emit_stream_start                               */
/* ------------------------------------------------------------------ */

static void test_bridge_emit_stream_start(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    int ret = bridge_yaml_emitter_emit_stream_start(&emitter, &event);
    printf("emit_stream_start_ret: %d\n", ret);
    yaml_event_delete(&event);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 34. Bridge: emitter BOM write                                       */
/* ------------------------------------------------------------------ */

static void test_bridge_write_bom(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;

    int ret = bridge_yaml_emitter_write_bom(&emitter);
    printf("write_bom_ret: %d\n", ret);
    printf("write_bom_len: %zu\n", mb.len);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 35. Bridge: emitter anchor/dump helpers                             */
/* ------------------------------------------------------------------ */

static void test_bridge_emitter_anchor_node(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t  emitter;
    yaml_document_t doc;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    assert(yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1));
    int s = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"hello", 5,
        YAML_PLAIN_SCALAR_STYLE);

    /* Set up emitter for anchor analysis */
    assert(yaml_emitter_open(&emitter));

    /* Set document on emitter (needed by anchor_node) */
    emitter.document = &doc;
    int n_nodes = (int)(doc.nodes.top - doc.nodes.start);
    emitter.anchors = (yaml_anchors_t *)yaml_malloc(
        (n_nodes + 1) * sizeof(yaml_anchors_t));
    assert(emitter.anchors);
    memset(emitter.anchors, 0, (n_nodes + 1) * sizeof(yaml_anchors_t));
    emitter.last_anchor_id = 0;

    bridge_yaml_emitter_anchor_node(&emitter, s);
    printf("anchor_node_refs: %d\n", emitter.anchors[s-1].references);

    yaml_free(emitter.anchors);
    emitter.anchors = NULL;
    emitter.document = NULL;

    yaml_document_delete(&doc);

    /* close the stream */
    yaml_event_t ev;
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 36. Bridge: emitter select_scalar_style / analyze_event            */
/* ------------------------------------------------------------------ */

static void test_bridge_select_scalar_style(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.canonical = 0;

    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"hello", 5, 1, 1, YAML_ANY_SCALAR_STYLE);

    int ret = bridge_yaml_emitter_analyze_event(&emitter, &event);
    printf("analyze_event_ret: %d\n", ret);

    ret = bridge_yaml_emitter_select_scalar_style(&emitter, &event);
    printf("select_scalar_style_ret: %d\n", ret);
    printf("selected_scalar_style: %d\n", emitter.scalar_data.style);

    yaml_event_delete(&event);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 37. Bridge: emitter delete_document_and_anchors                    */
/* ------------------------------------------------------------------ */

static void test_bridge_delete_doc_anchors(void)
{
    yaml_emitter_t  emitter;
    yaml_document_t doc;

    assert(yaml_emitter_initialize(&emitter));
    assert(yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1));
    yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"test", 4,
        YAML_PLAIN_SCALAR_STYLE);

    int n = (int)(doc.nodes.top - doc.nodes.start);
    emitter.anchors = (yaml_anchors_t *)yaml_malloc(
        (n + 1) * sizeof(yaml_anchors_t));
    assert(emitter.anchors);
    memset(emitter.anchors, 0, (n + 1) * sizeof(yaml_anchors_t));
    emitter.document = &doc;

    bridge_yaml_emitter_delete_document_and_anchors(&emitter);
    printf("delete_doc_anchors_ok: 1\n");
    printf("anchors_null_after: %d\n", emitter.anchors == NULL);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 38. Bridge: parser delete_aliases                                   */
/* ------------------------------------------------------------------ */

static void test_bridge_delete_aliases(void)
{
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));

    /* Load a doc with aliases so aliases struct is populated */
    static const unsigned char yaml[] = "a: &anc val\nb: *anc\n";
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);
    yaml_document_t doc;
    yaml_parser_load(&parser, &doc);
    yaml_document_delete(&doc);

    bridge_yaml_parser_delete_aliases(&parser);
    printf("delete_aliases_ok: 1\n");

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 39. Bridge: maximum_level_reached                                   */
/* ------------------------------------------------------------------ */

static void test_bridge_maximum_level(void)
{
    yaml_parser_t parser;
    yaml_mark_t mark = {0, 0, 0};
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"hello\n", 6);

    int ret = bridge_yaml_maximum_level_reached(&parser, mark, mark);
    printf("maximum_level_reached_ret: %d\n", ret);
    printf("maximum_level_error: %d\n", parser.error);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 40. Bridge: parser process_empty_scalar                             */
/* ------------------------------------------------------------------ */

static void test_bridge_process_empty_scalar(void)
{
    yaml_parser_t parser;
    yaml_event_t  event;
    yaml_mark_t   mark = {0, 0, 0};

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"hello\n", 6);

    int ret = bridge_yaml_parser_process_empty_scalar(&parser, &event, mark);
    printf("process_empty_scalar_ret: %d\n", ret);
    if (ret) {
        printf("process_empty_scalar_type: %d\n", event.type);
        printf("process_empty_scalar_len: %zu\n",
            event.data.scalar.length);
        yaml_event_delete(&event);
    }
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 41. Bridge: string_read_handler with partial read                   */
/* ------------------------------------------------------------------ */

static void test_bridge_string_read_partial(void)
{
    yaml_parser_t parser;
    unsigned char buf[4];
    size_t size_read;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"hello world", 11);

    /* Read 4 bytes */
    size_read = 0;
    int ret = bridge_yaml_string_read_handler(&parser, buf, 4, &size_read);
    printf("partial_read_ret: %d\n", ret);
    printf("partial_read_size: %zu\n", size_read);

    /* Read next 4 bytes */
    size_read = 0;
    ret = bridge_yaml_string_read_handler(&parser, buf, 4, &size_read);
    printf("partial_read2_ret: %d\n", ret);
    printf("partial_read2_size: %zu\n", size_read);

    /* Read remaining 3 bytes */
    size_read = 0;
    ret = bridge_yaml_string_read_handler(&parser, buf, 4, &size_read);
    printf("partial_read3_ret: %d\n", ret);
    printf("partial_read3_size: %zu\n", size_read);

    /* EOF */
    size_read = 99;
    ret = bridge_yaml_string_read_handler(&parser, buf, 4, &size_read);
    printf("partial_read_eof_ret: %d\n", ret);
    printf("partial_read_eof_size: %zu\n", size_read);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 42. Emitter: string output API                                      */
/* ------------------------------------------------------------------ */

static void test_emitter_string_output(void)
{
    yaml_emitter_t emitter;
    yaml_event_t   event;
    unsigned char  outbuf[1024];
    size_t         written = 0;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"test", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("string_output_written_gt0: %d\n", written > 0);
    printf("string_output_written: %zu\n", written);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 43. Parser: custom read handler                                     */
/* ------------------------------------------------------------------ */

typedef struct {
    const unsigned char *data;
    size_t len;
    size_t pos;
} custom_reader_t;

static int custom_read_handler(void *data, unsigned char *buffer,
                                size_t size, size_t *size_read)
{
    custom_reader_t *rd = (custom_reader_t *)data;
    size_t remaining = rd->len - rd->pos;
    if (remaining == 0) { *size_read = 0; return 1; }
    if (size > remaining) size = remaining;
    memcpy(buffer, rd->data + rd->pos, size);
    rd->pos += size;
    *size_read = size;
    return 1;
}

static void test_custom_read_handler(void)
{
    static const unsigned char yaml[] = "custom: handler\n";
    custom_reader_t rd = { yaml, sizeof(yaml)-1, 0 };

    yaml_parser_t parser;
    yaml_event_t  event;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input(&parser, custom_read_handler, &rd);

    int count = 0;
    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("custom_read_error: %d\n", parser.error);
            break;
        }
        printf("custom_event_%d: %d\n", count, event.type);
        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        count++;
        if (done) break;
    }
    printf("custom_event_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 44. Emitter: custom write handler                                   */
/* ------------------------------------------------------------------ */

static void test_custom_write_handler(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_mapping_start_event_initialize(&event, NULL, NULL, 1,
        YAML_FLOW_MAPPING_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"1", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_mapping_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("custom_write_len_gt0: %d\n", mb.len > 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 45. Emitter: unicode scalar                                         */
/* ------------------------------------------------------------------ */

static void test_emitter_unicode(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    yaml_emitter_set_unicode(&emitter, 1);

    /* U+00E9 = é = 0xC3 0xA9 */
    static const yaml_char_t uval[] = {0xC3, 0xA9, 0x00};

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        uval, 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("unicode_output_len_gt0: %d\n", mb.len > 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 46. Round-trip: parse then re-emit                                  */
/* ------------------------------------------------------------------ */

static void test_roundtrip(void)
{
    static const unsigned char yaml[] =
        "name: John\nage: 30\nhobbies:\n  - reading\n  - coding\n";

    /* Parse */
    yaml_parser_t parser;
    yaml_event_t  event;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    for (;;) {
        if (!yaml_parser_parse(&parser, &event)) break;
        int done = (event.type == YAML_STREAM_END_EVENT);
        printf("roundtrip_event: %d\n", event.type);
        yaml_emitter_emit(&emitter, &event);
        if (done) break;
    }

    printf("roundtrip_output_len_gt0: %d\n", mb.len > 0);
    yaml_parser_delete(&parser);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 47. Various scalar styles emitted                                   */
/* ------------------------------------------------------------------ */

static void test_scalar_styles(void)
{
    yaml_scalar_style_t styles[] = {
        YAML_PLAIN_SCALAR_STYLE,
        YAML_SINGLE_QUOTED_SCALAR_STYLE,
        YAML_DOUBLE_QUOTED_SCALAR_STYLE,
        YAML_LITERAL_SCALAR_STYLE,
        YAML_FOLDED_SCALAR_STYLE
    };

    for (int i = 0; i < 5; i++) {
        membuf_t mb;
        memset(&mb, 0, sizeof(mb));
        yaml_emitter_t emitter;
        yaml_event_t event;
        assert(yaml_emitter_initialize(&emitter));
        yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

        yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
        assert(yaml_emitter_emit(&emitter, &event));
        yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
        assert(yaml_emitter_emit(&emitter, &event));

        yaml_scalar_event_initialize(&event, NULL, NULL,
            (yaml_char_t *)"hello\nworld", 11,
            0, 1, styles[i]);
        assert(yaml_emitter_emit(&emitter, &event));

        yaml_document_end_event_initialize(&event, 1);
        assert(yaml_emitter_emit(&emitter, &event));
        yaml_stream_end_event_initialize(&event);
        assert(yaml_emitter_emit(&emitter, &event));

        printf("scalar_style_%d_len_gt0: %d\n", styles[i], mb.len > 0);
        yaml_emitter_delete(&emitter);
    }
}

/* ------------------------------------------------------------------ */
/* 48. Emitter: anchors and aliases                                    */
/* ------------------------------------------------------------------ */

static void test_emitter_anchor_alias(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_sequence_start_event_initialize(&event, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));

    /* scalar with anchor */
    yaml_scalar_event_initialize(&event, (yaml_char_t *)"anc1", NULL,
        (yaml_char_t *)"anchored", 8, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));

    /* alias */
    yaml_alias_event_initialize(&event, (yaml_char_t *)"anc1");
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_sequence_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("anchor_alias_len_gt0: %d\n", mb.len > 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 49. Emitter: tagged scalars                                         */
/* ------------------------------------------------------------------ */

static void test_emitter_tagged(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_scalar_event_initialize(&event, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:int",
        (yaml_char_t *)"42", 2, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));

    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("tagged_scalar_len_gt0: %d\n", mb.len > 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 50. Bridge: emitter process_tag                                     */
/* ------------------------------------------------------------------ */

static void test_bridge_process_tag(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.whitespace = 1;
    emitter.column = 0;
    emitter.line_break = YAML_LN_BREAK;

    /* Prime tag directive */
    yaml_tag_directive_t td;
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"tag:yaml.org,2002:";
    bridge_yaml_emitter_append_tag_directive(&emitter, td, 1);

    /* Analyze a tag */
    bridge_yaml_emitter_analyze_tag(&emitter,
        (yaml_char_t *)"tag:yaml.org,2002:str");

    int ret = bridge_yaml_emitter_process_tag(&emitter);
    printf("process_tag_ret: %d\n", ret);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 51. Bridge: parser parse_stream_start                               */
/* ------------------------------------------------------------------ */

static void test_bridge_parse_stream_start(void)
{
    yaml_parser_t parser;
    yaml_event_t  event;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser,
        (unsigned char *)"hello\n", 6);

    /* Must have at least one token in queue */
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);
    bridge_yaml_parser_fetch_next_token(&parser);

    int ret = bridge_yaml_parser_parse_stream_start(&parser, &event);
    printf("parse_stream_start_ret: %d\n", ret);
    if (ret) {
        printf("parse_stream_start_event: %d\n", event.type);
        yaml_event_delete(&event);
    }
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 52. Bridge: parser process_directives                               */
/* ------------------------------------------------------------------ */

static void test_bridge_process_directives(void)
{
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));
    static const unsigned char yaml[] = "%YAML 1.1\n---\nhello\n";
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    /* Prime the scanner */
    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);
    /* fetch stream start token */
    bridge_yaml_parser_fetch_next_token(&parser);
    /* skip past it by scanning manually */
    yaml_event_t ev;
    bridge_yaml_parser_parse_stream_start(&parser, &ev);
    yaml_event_delete(&ev);

    yaml_version_directive_t *vd = NULL;
    yaml_tag_directive_t *td_start = NULL, *td_end = NULL;
    int ret = bridge_yaml_parser_process_directives(&parser, &vd,
        &td_start, &td_end);
    printf("process_directives_ret: %d\n", ret);
    if (vd) {
        printf("process_directives_major: %d\n", vd->major);
        printf("process_directives_minor: %d\n", vd->minor);
        yaml_free(vd);
    }
    /* free tag directives */
    for (yaml_tag_directive_t *td = td_start; td != td_end; td++) {
        yaml_free(td->handle);
        yaml_free(td->prefix);
    }
    yaml_free(td_start);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 53. Emitter: flow sequence                                          */
/* ------------------------------------------------------------------ */

static void test_emitter_flow_sequence(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_sequence_start_event_initialize(&event, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"a", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"b", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_sequence_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("flow_seq_len_gt0: %d\n", mb.len > 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 54. Emitter: flow mapping                                           */
/* ------------------------------------------------------------------ */

static void test_emitter_flow_mapping(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t   event;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);

    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_mapping_start_event_initialize(&event, NULL, NULL, 1,
        YAML_FLOW_MAPPING_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"k", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"v", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_mapping_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_document_end_event_initialize(&event, 1);
    assert(yaml_emitter_emit(&emitter, &event));
    yaml_stream_end_event_initialize(&event);
    assert(yaml_emitter_emit(&emitter, &event));

    printf("flow_map_len_gt0: %d\n", mb.len > 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 55. Parser: scanner directive fetch                                  */
/* ------------------------------------------------------------------ */

static void test_bridge_fetch_directives(void)
{
    static const unsigned char yaml[] = "%YAML 1.1\n---\nhello\n";
    yaml_parser_t parser;
    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    bridge_yaml_parser_determine_encoding(&parser);
    bridge_yaml_parser_update_raw_buffer(&parser);

    /* fetch stream start */
    int ret = bridge_yaml_parser_fetch_stream_start(&parser);
    printf("fetch_stream_start2_ret: %d\n", ret);

    /* scan to next token (skips whitespace) */
    ret = bridge_yaml_parser_scan_to_next_token(&parser);
    printf("scan_to_next2_ret: %d\n", ret);

    /* fetch directive */
    ret = bridge_yaml_parser_fetch_directive(&parser);
    printf("fetch_directive_ret: %d\n", ret);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 56. Bridge: emitter dump_alias / dump_scalar via document dump      */
/* ------------------------------------------------------------------ */

static void test_bridge_emitter_dump(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t  emitter;
    yaml_document_t doc;

    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    assert(yaml_emitter_open(&emitter));

    assert(yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1));
    int s1 = yaml_document_add_scalar(&doc,
        (yaml_char_t *)YAML_STR_TAG, (yaml_char_t *)"hello", 5,
        YAML_PLAIN_SCALAR_STYLE);
    int seq = yaml_document_add_sequence(&doc,
        (yaml_char_t *)YAML_SEQ_TAG, YAML_BLOCK_SEQUENCE_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s1);
    yaml_document_append_sequence_item(&doc, seq, s1);  /* ref to same node */

    int ret = yaml_emitter_dump(&emitter, &doc);
    printf("dump_seq_ret: %d\n", ret);
    printf("dump_seq_len_gt0: %d\n", mb.len > 0);

    /* Close by dumping empty doc */
    yaml_document_t empty_doc;
    memset(&empty_doc, 0, sizeof(empty_doc));
    yaml_emitter_dump(&emitter, &empty_doc);
    yaml_emitter_close(&emitter);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 57. Parser: scan directive tokens                                   */
/* ------------------------------------------------------------------ */

static void test_parser_scan_directives(void)
{
    static const unsigned char yaml[] =
        "%YAML 1.1\n%TAG ! tag:example.com,2000:\n---\nhello\n";
    yaml_parser_t parser;
    yaml_token_t  token;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    int count = 0;
    for (;;) {
        if (!yaml_parser_scan(&parser, &token)) {
            printf("scan_dir_error: %d\n", parser.error);
            break;
        }
        printf("scan_dir_token_%d: %d\n", count, token.type);
        if (token.type == YAML_VERSION_DIRECTIVE_TOKEN) {
            printf("scan_dir_ver_major: %d\n",
                token.data.version_directive.major);
            printf("scan_dir_ver_minor: %d\n",
                token.data.version_directive.minor);
        }
        int done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
        count++;
        if (done) break;
    }
    printf("scan_dir_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 58. Bridge: emitter emit_document_start, emit_document_end         */
/* ------------------------------------------------------------------ */

static void test_bridge_emit_doc(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.line_break = YAML_LN_BREAK;

    /* Emit stream start first */
    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    bridge_yaml_emitter_emit_stream_start(&emitter, &event);
    yaml_event_delete(&event);

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    int ret = bridge_yaml_emitter_emit_document_start(&emitter, &event, 1);
    printf("emit_doc_start_ret: %d\n", ret);
    yaml_event_delete(&event);

    /* emit a scalar for document content */
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    bridge_yaml_emitter_emit_document_content(&emitter, &event);
    yaml_event_delete(&event);

    yaml_document_end_event_initialize(&event, 1);
    ret = bridge_yaml_emitter_emit_document_end(&emitter, &event);
    printf("emit_doc_end_ret: %d\n", ret);
    yaml_event_delete(&event);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* 59. Multiple documents via load API                                 */
/* ------------------------------------------------------------------ */

static void test_multi_doc_load(void)
{
    static const unsigned char yaml[] =
        "---\nhello\n---\nworld\n---\n42\n";
    yaml_parser_t   parser;
    yaml_document_t doc;

    assert(yaml_parser_initialize(&parser));
    yaml_parser_set_input_string(&parser, yaml, sizeof(yaml)-1);

    int count = 0;
    for (;;) {
        if (!yaml_parser_load(&parser, &doc)) {
            printf("multi_load_error: %d\n", parser.error);
            break;
        }
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        if (!root) { yaml_document_delete(&doc); break; }
        printf("multi_doc_%d_root_type: %d\n", count, root->type);
        yaml_document_delete(&doc);
        count++;
    }
    printf("multi_doc_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* 60. Bridge: emitter emit_node / emit_alias                         */
/* ------------------------------------------------------------------ */

static void test_bridge_emit_node(void)
{
    membuf_t mb;
    memset(&mb, 0, sizeof(mb));
    yaml_emitter_t emitter;
    yaml_event_t event;
    assert(yaml_emitter_initialize(&emitter));
    yaml_emitter_set_output(&emitter, membuf_write_handler, &mb);
    emitter.encoding = YAML_UTF8_ENCODING;
    emitter.line_break = YAML_LN_BREAK;
    emitter.best_indent = 2;
    emitter.best_width = 80;

    /* Prime state: need stream/doc start */
    yaml_stream_start_event_initialize(&event, YAML_UTF8_ENCODING);
    bridge_yaml_emitter_emit_stream_start(&emitter, &event);
    yaml_event_delete(&event);

    yaml_document_start_event_initialize(&event, NULL, NULL, NULL, 1);
    bridge_yaml_emitter_emit_document_start(&emitter, &event, 1);
    yaml_event_delete(&event);

    /* emit_node (root=1, sequence=0, mapping=0, simple_key=0) */
    yaml_scalar_event_initialize(&event, NULL, NULL,
        (yaml_char_t *)"hello", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    int ret = bridge_yaml_emitter_emit_node(&emitter, &event, 1, 0, 0, 0);
    printf("emit_node_ret: %d\n", ret);
    yaml_event_delete(&event);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* main                                                                */
/* ------------------------------------------------------------------ */

int main(void)
{
    test_version();
    test_memory();
    test_string_extend();
    test_string_join();
    test_stack_extend();
    test_queue_extend();
    test_token_delete();
    test_events_initialize();
    test_parser_scan();
    test_parser_parse();
    test_parser_load();
    test_document_api();
    test_emitter_emit();
    test_emitter_dump();
    test_max_nest_level();
    test_parser_encoding();
    test_bridge_check_utf8();
    test_bridge_read_handlers();
    test_bridge_write_handlers();
    test_bridge_emitter_error();
    test_bridge_writer_error();
    test_bridge_need_more_events();
    test_bridge_append_tag_directive();
    test_bridge_increase_indent();
    test_bridge_emitter_checks();
    test_bridge_analyze();
    test_bridge_process();
    test_bridge_write();
    test_bridge_write_quoted();
    test_bridge_write_block();
    test_bridge_generate_anchor();
    test_bridge_parser_errors();
    test_bridge_scanner_via_parse();
    test_bridge_flow_indent();
    test_bridge_roll_unroll_indent();
    test_bridge_simple_keys();
    test_bridge_fetch_stream();
    test_bridge_scan_to_next_token();
    test_bridge_parser_state_machine();
    test_bridge_emitter_state_machine();
    test_bridge_emit_stream_start();
    test_bridge_write_bom();
    test_bridge_emitter_anchor_node();
    test_bridge_select_scalar_style();
    test_bridge_delete_doc_anchors();
    test_bridge_delete_aliases();
    test_bridge_maximum_level();
    test_bridge_process_empty_scalar();
    test_bridge_string_read_partial();
    test_emitter_string_output();
    test_custom_read_handler();
    test_custom_write_handler();
    test_emitter_unicode();
    test_roundtrip();
    test_scalar_styles();
    test_emitter_anchor_alias();
    test_emitter_tagged();
    test_bridge_process_tag();
    test_bridge_parse_stream_start();
    test_bridge_process_directives();
    test_emitter_flow_sequence();
    test_emitter_flow_mapping();
    test_bridge_fetch_directives();
    test_bridge_emitter_dump();
    test_parser_scan_directives();
    test_bridge_emit_doc();
    test_multi_doc_load();
    test_bridge_emit_node();
    return 0;
}
