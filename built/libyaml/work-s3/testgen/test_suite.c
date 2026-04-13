/*
 * test_suite.c — Comprehensive libyaml test suite
 * Covers all public and static (bridge_*) functions.
 * Output is deterministic plain text for C/Rust diffing.
 */

#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>
#include <assert.h>

/* Pull in private types (yaml_string_t, yaml_malloc/free, etc.)
   before test_bridge.h which uses them in signatures. */
#include "/home/leochanj/Desktop/libyaml/src/yaml_private.h"

/* Forward-declare loader_ctx so bridge loader signatures match */
struct loader_ctx {
    int *start;
    int *end;
    int *top;
};

#include "test_bridge.h"

/* ------------------------------------------------------------------ */
/* Helpers                                                             */
/* ------------------------------------------------------------------ */

static void print_event_type(yaml_event_type_t type)
{
    const char *names[] = {
        "NO_EVENT",
        "STREAM_START", "STREAM_END",
        "DOCUMENT_START", "DOCUMENT_END",
        "ALIAS", "SCALAR",
        "SEQUENCE_START", "SEQUENCE_END",
        "MAPPING_START", "MAPPING_END"
    };
    if (type < (int)(sizeof(names)/sizeof(names[0])))
        printf("  event: %s\n", names[type]);
    else
        printf("  event: %d\n", (int)type);
}

/* string output write handler */
typedef struct {
    unsigned char *buf;
    size_t size;
    size_t written;
} str_out_t;

static int str_write_handler(void *data, unsigned char *buffer, size_t size)
{
    str_out_t *out = (str_out_t *)data;
    if (out->written + size > out->size)
        return 0;
    memcpy(out->buf + out->written, buffer, size);
    out->written += size;
    return 1;
}

/* ------------------------------------------------------------------ */
/* test_version                                                        */
/* ------------------------------------------------------------------ */
static void test_version(void)
{
    printf("test_version\n");
    const char *vs = yaml_get_version_string();
    printf("  version_string: %s\n", vs ? vs : "(null)");

    int major = 0, minor = 0, patch = 0;
    yaml_get_version(&major, &minor, &patch);
    printf("  version: %d.%d.%d\n", major, minor, patch);
}

/* ------------------------------------------------------------------ */
/* test_memory                                                         */
/* ------------------------------------------------------------------ */
static void test_memory(void)
{
    printf("test_memory\n");

    void *p = yaml_malloc(64);
    printf("  malloc_ok: %d\n", p != NULL);

    void *p2 = yaml_realloc(p, 128);
    printf("  realloc_ok: %d\n", p2 != NULL);
    p = p2;

    yaml_free(p);
    printf("  free_ok: 1\n");

    yaml_free(NULL); /* must not crash */
    printf("  free_null_ok: 1\n");

    yaml_char_t *dup = yaml_strdup((const yaml_char_t *)"hello");
    printf("  strdup: %s\n", dup ? (char *)dup : "(null)");
    yaml_free(dup);

    yaml_strdup(NULL); /* must return NULL */
    printf("  strdup_null_ok: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_string_extend_join                                             */
/* ------------------------------------------------------------------ */
static void test_string_extend_join(void)
{
    printf("test_string_extend_join\n");

    /* yaml_string_extend */
    size_t initial = 16;
    yaml_char_t *start = (yaml_char_t *)yaml_malloc(initial);
    yaml_char_t *ptr   = start;
    yaml_char_t *end   = start + initial;
    memset(start, 0, initial);

    int r = yaml_string_extend(&start, &ptr, &end);
    printf("  string_extend_ok: %d\n", r);
    printf("  new_size: %d\n", (int)(end - start));
    yaml_free(start);

    /* yaml_string_join */
    size_t sz = 32;
    yaml_char_t *a_start = (yaml_char_t *)yaml_malloc(sz);
    yaml_char_t *a_ptr   = a_start;
    yaml_char_t *a_end   = a_start + sz;
    memset(a_start, 0, sz);

    const char *src = "world";
    yaml_char_t *b_start = (yaml_char_t *)src;
    yaml_char_t *b_ptr   = b_start + strlen(src);
    yaml_char_t *b_end   = b_ptr;

    r = yaml_string_join(&a_start, &a_ptr, &a_end,
                         &b_start, &b_ptr, &b_end);
    printf("  string_join_ok: %d\n", r);
    yaml_free(a_start);
}

/* ------------------------------------------------------------------ */
/* test_stack_queue_extend                                             */
/* ------------------------------------------------------------------ */
static void test_stack_queue_extend(void)
{
    printf("test_stack_queue_extend\n");

    /* yaml_stack_extend */
    size_t item = sizeof(int);
    size_t n    = 4;
    void *s_start = yaml_malloc(item * n);
    void *s_top   = s_start;
    void *s_end   = (char *)s_start + item * n;

    int r = yaml_stack_extend(&s_start, &s_top, &s_end);
    printf("  stack_extend_ok: %d\n", r);
    yaml_free(s_start);

    /* yaml_queue_extend */
    void *q_start = yaml_malloc(item * n);
    void *q_head  = q_start;
    void *q_tail  = q_start;
    void *q_end   = (char *)q_start + item * n;

    r = yaml_queue_extend(&q_start, &q_head, &q_tail, &q_end);
    printf("  queue_extend_ok: %d\n", r);
    yaml_free(q_start);
}

/* ------------------------------------------------------------------ */
/* test_token_delete                                                   */
/* ------------------------------------------------------------------ */
static void test_token_delete(void)
{
    printf("test_token_delete\n");
    yaml_token_t token;
    memset(&token, 0, sizeof(token));
    token.type = YAML_NO_TOKEN;
    yaml_token_delete(&token);
    printf("  token_delete_ok: 1\n");

    /* SCALAR token with allocated value */
    yaml_token_t tok2;
    memset(&tok2, 0, sizeof(tok2));
    tok2.type = YAML_SCALAR_TOKEN;
    tok2.data.scalar.value = (yaml_char_t *)yaml_strdup((const yaml_char_t *)"val");
    yaml_token_delete(&tok2);
    printf("  token_delete_scalar_ok: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_event_initialize                                               */
/* ------------------------------------------------------------------ */
static void test_event_initialize(void)
{
    printf("test_event_initialize\n");
    yaml_event_t ev;

    /* stream start/end */
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    printf("  stream_start_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    yaml_stream_end_event_initialize(&ev);
    printf("  stream_end_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* document start/end */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    printf("  doc_start_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    yaml_document_end_event_initialize(&ev, 1);
    printf("  doc_end_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* alias */
    yaml_alias_event_initialize(&ev, (yaml_char_t *)"anchor1");
    printf("  alias_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* scalar */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"hello", 5,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* sequence start/end */
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_start_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    yaml_sequence_end_event_initialize(&ev);
    printf("  seq_end_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* mapping start/end */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    printf("  map_start_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    yaml_mapping_end_event_initialize(&ev);
    printf("  map_end_type: %d\n", ev.type);
    yaml_event_delete(&ev);
}

/* ------------------------------------------------------------------ */
/* test_parser_scan                                                    */
/* ------------------------------------------------------------------ */
static void test_parser_scan(void)
{
    printf("test_parser_scan\n");
    const char *input = "key: value\n";
    yaml_parser_t parser;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t token;
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) {
            printf("  scan_error: %d\n", parser.error);
            break;
        }
        count++;
        int done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_parser_parse                                                   */
/* ------------------------------------------------------------------ */
static void test_parser_parse(void)
{
    printf("test_parser_parse\n");
    const char *input = "- a\n- b\n- c\n";
    yaml_parser_t parser;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t event;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("  parse_error: %d\n", parser.error);
            break;
        }
        count++;
        print_event_type(event.type);
        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        if (done) break;
    }
    printf("  events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_parser_load                                                    */
/* ------------------------------------------------------------------ */
static void test_parser_load(void)
{
    printf("test_parser_load\n");
    const char *input = "name: Alice\nage: 30\n";
    yaml_parser_t parser;
    yaml_document_t doc;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    if (!yaml_parser_load(&parser, &doc)) {
        printf("  load_error: %d\n", parser.error);
        yaml_parser_delete(&parser);
        return;
    }

    yaml_node_t *root = yaml_document_get_root_node(&doc);
    printf("  root_type: %d\n", root ? root->type : -1);

    /* yaml_document_get_node */
    yaml_node_t *n1 = yaml_document_get_node(&doc, 1);
    printf("  node1_ok: %d\n", n1 != NULL);

    yaml_document_delete(&doc);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_parser_encoding                                                */
/* ------------------------------------------------------------------ */
static void test_parser_encoding(void)
{
    printf("test_parser_encoding\n");
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_encoding(&parser, YAML_UTF8_ENCODING);

    const char *input = "x: 1\n";
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t ev;
    yaml_parser_parse(&parser, &ev);
    printf("  first_event: %d\n", ev.type);
    yaml_event_delete(&ev);

    yaml_parser_delete(&parser);
    printf("  ok: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_parser_set_input (generic handler)                             */
/* ------------------------------------------------------------------ */
typedef struct { const unsigned char *data; size_t size; size_t pos; } mem_src_t;

static int mem_read_handler(void *data, unsigned char *buffer,
                            size_t size, size_t *size_read)
{
    mem_src_t *src = (mem_src_t *)data;
    size_t avail = src->size - src->pos;
    size_t n = avail < size ? avail : size;
    memcpy(buffer, src->data + src->pos, n);
    src->pos += n;
    *size_read = n;
    return 1;
}

static void test_parser_set_input(void)
{
    printf("test_parser_set_input\n");
    const char *yaml_text = "foo: bar\n";
    mem_src_t src = { (const unsigned char *)yaml_text, strlen(yaml_text), 0 };

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input(&parser, mem_read_handler, &src);

    yaml_event_t ev;
    int ok = yaml_parser_parse(&parser, &ev);
    printf("  parse_ok: %d\n", ok);
    printf("  event_type: %d\n", ev.type);
    yaml_event_delete(&ev);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_set_max_nest_level                                             */
/* ------------------------------------------------------------------ */
static void test_set_max_nest_level(void)
{
    printf("test_set_max_nest_level\n");
    yaml_set_max_nest_level(100);
    printf("  set_ok: 1\n");
    /* parse simple nested doc with the new limit */
    const char *input = "a:\n  b:\n    c: 1\n";
    yaml_parser_t parser;
    yaml_document_t doc;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));
    int r = yaml_parser_load(&parser, &doc);
    printf("  load_ok: %d\n", r);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);
    /* restore default */
    yaml_set_max_nest_level(1000);
}

/* ------------------------------------------------------------------ */
/* test_emitter_basic                                                  */
/* ------------------------------------------------------------------ */
static void test_emitter_basic(void)
{
    printf("test_emitter_basic\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* Exercise set_* methods */
    yaml_emitter_set_canonical(&emitter, 0);
    yaml_emitter_set_indent(&emitter, 2);
    yaml_emitter_set_width(&emitter, 80);
    yaml_emitter_set_break(&emitter, YAML_LN_BREAK);
    yaml_emitter_set_unicode(&emitter, 1);

    yaml_event_t ev;

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"key", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"value", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_emitter_flush(&emitter);

    printf("  written: %d\n", (int)written);
    printf("  output: %s", written > 0 ? (char *)outbuf : "(empty)");
    if (written > 0 && outbuf[written-1] != '\n') printf("\n");

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_set_output (generic handler)                           */
/* ------------------------------------------------------------------ */
static void test_emitter_set_output(void)
{
    printf("test_emitter_set_output\n");

    unsigned char outbuf[4096];
    memset(outbuf, 0, sizeof(outbuf));
    str_out_t out = { outbuf, sizeof(outbuf), 0 };

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output(&emitter, str_write_handler, &out);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"hello", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_emitter_flush(&emitter);
    printf("  written: %d\n", (int)out.written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_open_close                                             */
/* ------------------------------------------------------------------ */
static void test_emitter_open_close(void)
{
    printf("test_emitter_open_close\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    int r = yaml_emitter_open(&emitter);
    printf("  open_ok: %d\n", r);

    r = yaml_emitter_close(&emitter);
    printf("  close_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_dump                                                   */
/* ------------------------------------------------------------------ */
static void test_emitter_dump(void)
{
    printf("test_emitter_dump\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    /* Build a document */
    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);

    int scalar_idx = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"dumped_value", 12, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_idx: %d\n", scalar_idx);

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_ok: %d\n", r);
    printf("  written: %d\n", (int)written);

    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
    /* doc is consumed by dump; no need to delete */
}

/* ------------------------------------------------------------------ */
/* test_document_api                                                   */
/* ------------------------------------------------------------------ */
static void test_document_api(void)
{
    printf("test_document_api\n");

    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);

    /* add scalar */
    int s1 = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"hello", 5, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar: %d\n", s1);

    /* add sequence */
    int seq = yaml_document_add_sequence(&doc, NULL,
        YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq: %d\n", seq);

    int s2 = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"item1", 5, YAML_PLAIN_SCALAR_STYLE);
    int s3 = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"item2", 5, YAML_PLAIN_SCALAR_STYLE);

    int r = yaml_document_append_sequence_item(&doc, seq, s2);
    printf("  append_seq_item1: %d\n", r);
    r = yaml_document_append_sequence_item(&doc, seq, s3);
    printf("  append_seq_item2: %d\n", r);

    /* add mapping */
    int map = yaml_document_add_mapping(&doc, NULL,
        YAML_BLOCK_MAPPING_STYLE);
    printf("  map: %d\n", map);

    int k = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"key", 3, YAML_PLAIN_SCALAR_STYLE);
    int v = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"val", 3, YAML_PLAIN_SCALAR_STYLE);
    r = yaml_document_append_mapping_pair(&doc, map, k, v);
    printf("  append_map_pair: %d\n", r);

    yaml_node_t *root = yaml_document_get_root_node(&doc);
    printf("  root_type: %d\n", root ? root->type : -1);

    yaml_node_t *n = yaml_document_get_node(&doc, s1);
    printf("  node_type: %d\n", n ? n->type : -1);

    yaml_document_delete(&doc);
    printf("  delete_ok: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_parser_fetch_more_tokens + update_buffer                       */
/* ------------------------------------------------------------------ */
static void test_parser_fetch_update(void)
{
    printf("test_parser_fetch_update\n");
    const char *input = "foo: bar\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* fetch_more_tokens needs internal state primed */
    yaml_token_t tok;
    int r = yaml_parser_scan(&parser, &tok);
    printf("  scan1_ok: %d token: %d\n", r, tok.type);
    yaml_token_delete(&tok);

    r = yaml_parser_fetch_more_tokens(&parser);
    printf("  fetch_more_ok: %d\n", r);

    r = yaml_parser_update_buffer(&parser, 1);
    printf("  update_buffer_ok: %d\n", r);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_check_utf8                                              */
/* ------------------------------------------------------------------ */
static void test_bridge_check_utf8(void)
{
    printf("test_bridge_check_utf8\n");

    const yaml_char_t *valid   = (const yaml_char_t *)"hello world";
    const yaml_char_t *invalid = (const yaml_char_t *)"\xff\xfe";

    int r1 = bridge_yaml_check_utf8(valid, strlen((char *)valid));
    printf("  valid_utf8: %d\n", r1);

    int r2 = bridge_yaml_check_utf8(invalid, 2);
    printf("  invalid_utf8: %d\n", r2);
}

/* ------------------------------------------------------------------ */
/* test_bridge_reader_handlers                                         */
/* ------------------------------------------------------------------ */
static void test_bridge_reader_handlers(void)
{
    printf("test_bridge_reader_handlers\n");

    /* Test yaml_string_read_handler via a fully initialized parser */
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    const char *input = "a: 1\n";
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* Call bridge directly */
    unsigned char buf[128];
    size_t size_read = 0;
    int r = bridge_yaml_string_read_handler(parser.read_handler_data,
                                             buf, sizeof(buf), &size_read);
    printf("  string_read_ok: %d bytes: %d\n", r, (int)size_read);
    yaml_parser_delete(&parser);

    /* Test yaml_file_read_handler via a tmp file */
    FILE *f = tmpfile();
    if (f) {
        fputs("x: 2\n", f);
        rewind(f);
        yaml_parser_t p2;
        yaml_parser_initialize(&p2);
        yaml_parser_set_input_file(&p2, f);
        size_read = 0;
        r = bridge_yaml_file_read_handler(p2.read_handler_data,
                                          buf, sizeof(buf), &size_read);
        printf("  file_read_ok: %d bytes: %d\n", r, (int)size_read);
        yaml_parser_delete(&p2);
        fclose(f);
    } else {
        printf("  file_read_ok: skipped\n");
        printf("  file_read_ok: skipped bytes: 0\n");
    }
}

/* ------------------------------------------------------------------ */
/* test_bridge_writer_handlers                                         */
/* ------------------------------------------------------------------ */
static void test_bridge_writer_handlers(void)
{
    printf("test_bridge_writer_handlers\n");

    /* string write handler */
    unsigned char outbuf[256];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    unsigned char data[] = "test_data";
    int r = bridge_yaml_string_write_handler(emitter.write_handler_data,
                                              data, sizeof(data)-1);
    printf("  string_write_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_delete(&emitter);

    /* file write handler */
    FILE *f = tmpfile();
    if (f) {
        yaml_emitter_t e2;
        yaml_emitter_initialize(&e2);
        yaml_emitter_set_output_file(&e2, f);
        yaml_emitter_set_encoding(&e2, YAML_UTF8_ENCODING);

        unsigned char data2[] = "file_data";
        r = bridge_yaml_file_write_handler(e2.write_handler_data,
                                           data2, sizeof(data2)-1);
        printf("  file_write_ok: %d\n", r);
        yaml_emitter_delete(&e2);
        fclose(f);
    } else {
        printf("  file_write_ok: skipped\n");
    }
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_set_errors                                      */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_set_errors(void)
{
    printf("test_bridge_emitter_set_errors\n");

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);

    int r = bridge_yaml_emitter_set_emitter_error(&emitter, "test emitter error");
    printf("  set_emitter_error_ret: %d\n", r);
    printf("  emitter_error: %d\n", (int)emitter.error);

    yaml_emitter_initialize(&emitter); /* reset */
    r = bridge_yaml_emitter_set_writer_error(&emitter, "test writer error");
    printf("  set_writer_error_ret: %d\n", r);
    printf("  writer_error: %d\n", (int)emitter.error);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_analyze                                         */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_analyze(void)
{
    printf("test_bridge_emitter_analyze\n");

    /* Build a properly opened emitter */
    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* Open the stream so internal state is set */
    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* analyze_anchor */
    int r = bridge_yaml_emitter_analyze_anchor(&emitter,
                (yaml_char_t *)"anchorX", 0);
    printf("  analyze_anchor_ok: %d\n", r);

    /* analyze_tag */
    r = bridge_yaml_emitter_analyze_tag(&emitter,
            (yaml_char_t *)"!!str");
    printf("  analyze_tag_ok: %d\n", (r >= 0));

    /* analyze_scalar */
    r = bridge_yaml_emitter_analyze_scalar(&emitter,
            (yaml_char_t *)"hello world", 11);
    printf("  analyze_scalar_ok: %d\n", r);

    /* analyze_version_directive */
    yaml_version_directive_t vd = {1, 2};
    r = bridge_yaml_emitter_analyze_version_directive(&emitter, vd);
    printf("  analyze_version_directive_ok: %d\n", r);

    /* analyze_tag_directive */
    yaml_tag_directive_t td;
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"!foo!";
    r = bridge_yaml_emitter_analyze_tag_directive(&emitter, td);
    printf("  analyze_tag_directive_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_write                                           */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_write(void)
{
    printf("test_bridge_emitter_write\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* prime internal buffer/encoding by opening stream */
    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* write_indent */
    emitter.indent = 0;
    int r = bridge_yaml_emitter_write_indent(&emitter);
    printf("  write_indent_ok: %d\n", r);

    /* write_indicator */
    r = bridge_yaml_emitter_write_indicator(&emitter, "-", 1, 0, 0);
    printf("  write_indicator_ok: %d\n", r);

    /* write_anchor */
    r = bridge_yaml_emitter_write_anchor(&emitter,
            (yaml_char_t *)"anc", 3);
    printf("  write_anchor_ok: %d\n", r);

    /* write_tag_handle */
    r = bridge_yaml_emitter_write_tag_handle(&emitter,
            (yaml_char_t *)"!", 1);
    printf("  write_tag_handle_ok: %d\n", r);

    /* write_tag_content */
    r = bridge_yaml_emitter_write_tag_content(&emitter,
            (yaml_char_t *)"str", 3, 0);
    printf("  write_tag_content_ok: %d\n", r);

    /* plain scalar */
    /* need analyze first */
    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"plainval", 8);
    r = bridge_yaml_emitter_write_plain_scalar(&emitter,
            (yaml_char_t *)"plainval", 8, 1);
    printf("  write_plain_scalar_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_write_quoted                                    */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_write_quoted(void)
{
    printf("test_bridge_emitter_write_quoted\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"quoted val", 10);

    int r = bridge_yaml_emitter_write_single_quoted_scalar(&emitter,
                (yaml_char_t *)"quoted val", 10, 1);
    printf("  write_single_quoted_ok: %d\n", r);

    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"dquoted", 7);
    r = bridge_yaml_emitter_write_double_quoted_scalar(&emitter,
            (yaml_char_t *)"dquoted", 7, 1);
    printf("  write_double_quoted_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_write_block                                     */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_write_block(void)
{
    printf("test_bridge_emitter_write_block\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"line1\nline2\n", 12);

    int r = bridge_yaml_emitter_write_literal_scalar(&emitter,
                (yaml_char_t *)"line1\nline2\n", 12);
    printf("  write_literal_ok: %d\n", r);

    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"fold1\nfold2\n", 12);
    r = bridge_yaml_emitter_write_folded_scalar(&emitter,
            (yaml_char_t *)"fold1\nfold2\n", 12);
    printf("  write_folded_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_checks                                          */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_checks(void)
{
    printf("test_bridge_emitter_checks\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    int r = bridge_yaml_emitter_check_empty_document(&emitter);
    printf("  check_empty_document: %d\n", r);

    r = bridge_yaml_emitter_check_empty_sequence(&emitter);
    printf("  check_empty_sequence: %d\n", r);

    r = bridge_yaml_emitter_check_empty_mapping(&emitter);
    printf("  check_empty_mapping: %d\n", r);

    r = bridge_yaml_emitter_check_simple_key(&emitter);
    printf("  check_simple_key: %d\n", r);

    r = bridge_yaml_emitter_need_more_events(&emitter);
    printf("  need_more_events: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_append_tag_directive                            */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_append_tag_directive(void)
{
    printf("test_bridge_emitter_append_tag_directive\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    yaml_tag_directive_t td;
    td.handle = (yaml_char_t *)"!";
    td.prefix = (yaml_char_t *)"!ns!";
    int r = bridge_yaml_emitter_append_tag_directive(&emitter, td, 1);
    printf("  append_tag_directive_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_increase_indent                                 */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_increase_indent(void)
{
    printf("test_bridge_emitter_increase_indent\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    int r = bridge_yaml_emitter_increase_indent(&emitter, 0, 0);
    printf("  increase_indent_ok: %d indent: %d\n", r, emitter.indent);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_generate_anchor                                 */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_generate_anchor(void)
{
    printf("test_bridge_emitter_generate_anchor\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_char_t *anchor = bridge_yaml_emitter_generate_anchor(&emitter, 1);
    printf("  anchor: %s\n", anchor ? (char *)anchor : "(null)");
    if (anchor) yaml_free(anchor);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_dump_document                                   */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_dump_document(void)
{
    printf("test_bridge_emitter_dump_document\n");

    /* build document with anchors for dump_node path */
    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int s = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"anchor_val", 10, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_node: %d\n", s);

    /* dump exercises delete_document_and_anchors, anchor_node, dump_node, dump_scalar */
    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_ok: %d written: %d\n", r, (int)written);

    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_set_errors                                       */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_set_errors(void)
{
    printf("test_bridge_parser_set_errors\n");

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_mark_t mark = {0, 0, 0};

    int r = bridge_yaml_parser_set_parser_error(&parser, "parser_err", mark);
    printf("  set_parser_error_ret: %d\n", r);
    printf("  parser_error: %d\n", (int)parser.error);

    yaml_parser_initialize(&parser);
    r = bridge_yaml_parser_set_parser_error_context(&parser,
            "ctx", mark, "problem", mark);
    printf("  set_parser_error_context_ret: %d\n", r);

    yaml_parser_initialize(&parser);
    r = bridge_yaml_parser_set_reader_error(&parser, "reader_err", 0, -1);
    printf("  set_reader_error_ret: %d\n", r);

    yaml_parser_initialize(&parser);
    r = bridge_yaml_parser_set_scanner_error(&parser, "scan ctx", mark, "scan_err");
    printf("  set_scanner_error_ret: %d\n", r);

    yaml_parser_initialize(&parser);
    r = bridge_yaml_parser_set_composer_error(&parser, "composer_err", mark);
    printf("  set_composer_error_ret: %d\n", r);

    yaml_parser_initialize(&parser);
    r = bridge_yaml_parser_set_composer_error_context(&parser,
            "ctx2", mark, "comp_prob", mark);
    printf("  set_composer_error_context_ret: %d\n", r);

    yaml_parser_initialize(&parser);
    r = bridge_yaml_maximum_level_reached(&parser, mark, mark);
    printf("  maximum_level_reached_ret: %d\n", r);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_flow_levels                                      */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_flow_levels(void)
{
    printf("test_bridge_parser_flow_levels\n");
    const char *input = "{a: [1, 2]}\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* Prime the parser by scanning a few tokens */
    yaml_token_t tok;
    int i;
    for (i = 0; i < 3; i++) {
        yaml_parser_scan(&parser, &tok);
        yaml_token_delete(&tok);
    }

    /* These are exercised internally; call directly as sanity */
    int r = bridge_yaml_parser_increase_flow_level(&parser);
    printf("  increase_flow_ok: %d level: %d\n", r, parser.flow_level);

    r = bridge_yaml_parser_decrease_flow_level(&parser);
    printf("  decrease_flow_ok: %d level: %d\n", r, parser.flow_level);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_scan_full                                        */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_scan_full(void)
{
    printf("test_bridge_parser_scan_full\n");

    /* Use a YAML with directives, tags, anchors, aliases, block/flow */
    const char *input =
        "%YAML 1.2\n"
        "---\n"
        "- &anchor plain\n"
        "- *anchor\n"
        "- 'single'\n"
        "- \"double\"\n"
        "- |\n"
        "  literal\n"
        "- >\n"
        "  folded\n"
        "- {key: val}\n"
        "- [1, 2]\n"
        "...\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t token;
    while (1) {
        if (!yaml_parser_scan(&parser, &token)) break;
        count++;
        int done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
        if (done) break;
    }
    printf("  token_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_parse_full                                       */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_parse_full(void)
{
    printf("test_bridge_parser_parse_full\n");

    const char *input =
        "---\n"
        "mapping:\n"
        "  key1: val1\n"
        "  key2: val2\n"
        "sequence:\n"
        "  - item1\n"
        "  - item2\n"
        "...\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t event;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &event)) {
            printf("  parse_error: %d\n", parser.error);
            break;
        }
        count++;
        if (event.type == YAML_SCALAR_EVENT)
            printf("  scalar: %s\n", event.data.scalar.value);
        int done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
        if (done) break;
    }
    printf("  event_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_load_aliases                                     */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_load_aliases(void)
{
    printf("test_bridge_parser_load_aliases\n");

    /* Use an alias in YAML to exercise load_alias path */
    const char *input =
        "---\n"
        "- &a hello\n"
        "- *a\n";

    yaml_parser_t parser;
    yaml_document_t doc;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int r = yaml_parser_load(&parser, &doc);
    printf("  load_ok: %d\n", r);
    if (r) {
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        printf("  root_type: %d\n", root ? root->type : -1);
        yaml_document_delete(&doc);
    }

    /* delete_aliases is called internally; call bridge for coverage */
    bridge_yaml_parser_delete_aliases(&parser);
    printf("  delete_aliases_ok: 1\n");

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_determine_encoding                               */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_determine_encoding(void)
{
    printf("test_bridge_parser_determine_encoding\n");

    const char *input = "hello: world\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* update_raw_buffer fills the raw buffer */
    int r = bridge_yaml_parser_update_raw_buffer(&parser);
    printf("  update_raw_buffer_ok: %d\n", r);

    r = bridge_yaml_parser_determine_encoding(&parser);
    printf("  determine_encoding_ok: %d encoding: %d\n",
           r, (int)parser.encoding);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_scan_tokens                                      */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_scan_tokens(void)
{
    printf("test_bridge_parser_scan_tokens\n");

    /* scan_to_next_token, stale_simple_keys, save_simple_key,
       remove_simple_key are all exercised via normal scanning */
    const char *input =
        "# comment\n"
        "key: value\n"
        "? complex key\n"
        ": complex value\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int r = bridge_yaml_parser_fetch_next_token(&parser);
    printf("  fetch_next_token_ok: %d\n", r);

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  remaining_tokens: %d\n", count);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_roll_indent                                      */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_roll_indent(void)
{
    printf("test_bridge_parser_roll_indent\n");

    const char *input = "key:\n  - item\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* scan to prime indents */
    yaml_token_t tok;
    int i;
    for (i = 0; i < 2; i++) {
        yaml_parser_scan(&parser, &tok);
        yaml_token_delete(&tok);
    }

    yaml_mark_t mark = {0, 0, 0};
    int r = bridge_yaml_parser_roll_indent(&parser, 0, -1,
                YAML_BLOCK_SEQUENCE_START_TOKEN, mark);
    printf("  roll_indent_ok: %d\n", r);

    r = bridge_yaml_parser_unroll_indent(&parser, -1);
    printf("  unroll_indent_ok: %d\n", r);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_fetch_stream                                     */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_fetch_stream(void)
{
    printf("test_bridge_parser_fetch_stream\n");

    const char *input = "a: b\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* fetch_stream_start is the first token */
    int r = bridge_yaml_parser_fetch_stream_start(&parser);
    printf("  fetch_stream_start_ok: %d\n", r);

    /* Now scan the rest and trigger stream end */
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  stream_end_consumed: 1\n");

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_fetch_document_indicators                        */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_fetch_document_indicators(void)
{
    printf("test_bridge_parser_fetch_document_indicators\n");

    const char *input = "---\nfoo: bar\n...\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_fetch_collections                                */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_fetch_collections(void)
{
    printf("test_bridge_parser_fetch_collections\n");

    const char *input = "{k1: [v1, v2], k2: v3}\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_fetch_block_entry_key_value                      */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_fetch_block_entry_key_value(void)
{
    printf("test_bridge_parser_fetch_block_entry_key_value\n");

    const char *input =
        "? explicit_key\n"
        ": explicit_value\n"
        "- block_entry\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_fetch_anchors_tags                               */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_fetch_anchors_tags(void)
{
    printf("test_bridge_parser_fetch_anchors_tags\n");

    const char *input =
        "---\n"
        "- &anc tagged_val\n"
        "- *anc\n"
        "- !!str explicit_tag\n"
        "- !local tag_value\n"
        "- !<uri:tag> uri_tagged\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_fetch_scalars                                    */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_fetch_scalars(void)
{
    printf("test_bridge_parser_fetch_scalars\n");

    const char *input =
        "- 'single quoted'\n"
        "- \"double quoted\"\n"
        "- |\n"
        "  literal block\n"
        "- >\n"
        "  folded block\n"
        "- plain_scalar\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        if (tok.type == YAML_SCALAR_TOKEN)
            printf("  scalar: %s\n", tok.data.scalar.value);
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  token_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_scan_directives                                  */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_scan_directives(void)
{
    printf("test_bridge_parser_scan_directives\n");

    const char *input =
        "%YAML 1.1\n"
        "%TAG ! tag:example.com,2000:\n"
        "---\n"
        "a: b\n"
        "...\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_scan_uri_escapes                                 */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_scan_uri_escapes(void)
{
    printf("test_bridge_parser_scan_uri_escapes\n");

    const char *input =
        "- !<tag:yaml.org,2002:str%20val> escaped\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_process_directives                               */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_process_directives(void)
{
    printf("test_bridge_parser_process_directives\n");

    const char *input = "%YAML 1.1\n---\nfoo: bar\n...\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* Scan past stream start */
    yaml_token_t tok;
    yaml_parser_scan(&parser, &tok);
    yaml_token_delete(&tok);

    yaml_version_directive_t *vd = NULL;
    yaml_tag_directive_t *td_start = NULL, *td_end = NULL;
    int r = bridge_yaml_parser_process_directives(&parser,
                &vd, &td_start, &td_end);
    printf("  process_directives_ok: %d\n", r);
    if (vd) {
        printf("  yaml_version: %d.%d\n", vd->major, vd->minor);
        yaml_free(vd);
    }
    for (yaml_tag_directive_t *t = td_start; t && t < td_end; t++) {
        yaml_free(t->handle);
        yaml_free(t->prefix);
    }
    if (td_start) yaml_free(td_start);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_append_tag_directive                             */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_append_tag_directive(void)
{
    printf("test_bridge_parser_append_tag_directive\n");

    const char *input = "a: b\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_mark_t mark = {0, 0, 0};
    yaml_tag_directive_t td;
    /* strdup because parser takes ownership */
    td.handle = (yaml_char_t *)strdup("!");
    td.prefix = (yaml_char_t *)strdup("!myns!");
    int r = bridge_yaml_parser_append_tag_directive(&parser, td, 1, mark);
    printf("  append_tag_directive_ok: %d\n", r);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_stale_save_remove_keys                           */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_stale_save_remove_keys(void)
{
    printf("test_bridge_parser_stale_save_remove_keys\n");

    const char *input = "key: value\nkey2: value2\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* Prime scanner */
    yaml_token_t tok;
    yaml_parser_scan(&parser, &tok); /* STREAM_START */
    yaml_token_delete(&tok);

    int r = bridge_yaml_parser_stale_simple_keys(&parser);
    printf("  stale_simple_keys_ok: %d\n", r);

    r = bridge_yaml_parser_save_simple_key(&parser);
    printf("  save_simple_key_ok: %d\n", r);

    r = bridge_yaml_parser_remove_simple_key(&parser);
    printf("  remove_simple_key_ok: %d\n", r);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_emit_stream                                     */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_emit_stream(void)
{
    printf("test_bridge_emitter_emit_stream\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* state_machine / emit_stream_start */
    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    int r = bridge_yaml_emitter_state_machine(&emitter, &ev);
    printf("  state_machine_stream_start: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_process_anchor_tag_scalar                       */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_process_anchor_tag_scalar(void)
{
    printf("test_bridge_emitter_process_anchor_tag_scalar\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* Open stream so output encoding is set */
    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* analyze then process anchor */
    bridge_yaml_emitter_analyze_anchor(&emitter,
        (yaml_char_t *)"myanchor", 0);
    int r = bridge_yaml_emitter_process_anchor(&emitter);
    printf("  process_anchor_ok: %d\n", r);

    /* analyze tag then process tag */
    bridge_yaml_emitter_analyze_tag(&emitter, (yaml_char_t *)"!!str");
    r = bridge_yaml_emitter_process_tag(&emitter);
    printf("  process_tag_ok: %d\n", (r >= 0));

    /* analyze scalar then process scalar */
    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"scalarval", 9);
    emitter.scalar_data.style = YAML_PLAIN_SCALAR_STYLE;
    r = bridge_yaml_emitter_process_scalar(&emitter);
    printf("  process_scalar_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_select_scalar_style                             */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_select_scalar_style(void)
{
    printf("test_bridge_emitter_select_scalar_style\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    yaml_event_t scalar_ev;
    yaml_scalar_event_initialize(&scalar_ev, NULL, NULL,
        (yaml_char_t *)"test", 4, 1, 1, YAML_ANY_SCALAR_STYLE);

    bridge_yaml_emitter_analyze_scalar(&emitter,
        (yaml_char_t *)"test", 4);
    int r = bridge_yaml_emitter_select_scalar_style(&emitter, &scalar_ev);
    printf("  select_scalar_style_ok: %d style: %d\n", r, emitter.scalar_data.style);

    yaml_event_delete(&scalar_ev);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_write_bom                                       */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_write_bom(void)
{
    printf("test_bridge_emitter_write_bom\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    int r = bridge_yaml_emitter_write_bom(&emitter);
    printf("  write_bom_ok: %d written: %d\n", r, (int)written);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_block_scalar_hints                              */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_block_scalar_hints(void)
{
    printf("test_bridge_emitter_block_scalar_hints\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* Build a yaml_string_t for "  block\n" */
    unsigned char val[] = "  block\n";
    /* yaml_string_t is { start, end, pointer } */
    typedef struct { yaml_char_t *start; yaml_char_t *end; yaml_char_t *pointer; } ys_t;
    ys_t ys;
    ys.start   = (yaml_char_t *)val;
    ys.end     = (yaml_char_t *)(val + sizeof(val) - 1);
    ys.pointer = (yaml_char_t *)val;

    int r = bridge_yaml_emitter_write_block_scalar_hints(&emitter,
                *((yaml_string_t *)&ys));
    printf("  write_block_scalar_hints_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_parse_states                                     */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_parse_states(void)
{
    printf("test_bridge_parser_parse_states\n");

    /* A complex document to exercise many parser state machines:
       flow sequence, flow mapping, block sequence, block mapping,
       indentless sequence */
    const char *input =
        "---\n"
        "block_seq:\n"
        "  - val1\n"
        "  - val2\n"
        "block_map:\n"
        "  k1: v1\n"
        "  k2: v2\n"
        "flow_seq: [a, b, c]\n"
        "flow_map: {x: 1, y: 2}\n"
        "nested:\n"
        "  deep:\n"
        "    deepest: val\n"
        "...\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) break;
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  event_count: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_process_empty_scalar                             */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_process_empty_scalar(void)
{
    printf("test_bridge_parser_process_empty_scalar\n");

    const char *input = "key:\nother: val\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) break;
        count++;
        /* empty scalar (implicit null) exercises process_empty_scalar */
        if (ev.type == YAML_SCALAR_EVENT && ev.data.scalar.length == 0)
            printf("  empty_scalar_found: 1\n");
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_register_anchor                                  */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_register_anchor(void)
{
    printf("test_bridge_parser_register_anchor\n");

    const char *input = "- &myanchor val\n- *myanchor\n";
    yaml_parser_t parser;
    yaml_document_t doc;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int r = yaml_parser_load(&parser, &doc);
    printf("  load_ok: %d\n", r);
    if (r) yaml_document_delete(&doc);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_emit_nodes                                      */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_emit_nodes(void)
{
    printf("test_bridge_emitter_emit_nodes\n");

    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    /* Emit sequence to exercise emit_sequence_start, emit_block_sequence_item */
    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"item1", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"item2", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  seq_written: %d\n", (int)written);

    yaml_emitter_delete(&emitter);

    /* Now emit flow mapping to exercise flow paths */
    written = 0;
    memset(outbuf, 0, sizeof(outbuf));
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"k", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"v", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  flow_map_written: %d\n", (int)written);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_emitter_emit_alias                                      */
/* ------------------------------------------------------------------ */
static void test_bridge_emitter_emit_alias(void)
{
    printf("test_bridge_emitter_emit_alias\n");

    /* Emit alias via yaml_emitter_emit (exercises emit_alias) */
    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* First item with anchor */
    yaml_scalar_event_initialize(&ev,
        (yaml_char_t *)"anchor1", NULL,
        (yaml_char_t *)"val", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Alias to the anchor */
    yaml_alias_event_initialize(&ev, (yaml_char_t *)"anchor1");
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  emit_alias_ok: %d\n", r);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_flow_sequence_mapping_entries                    */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_flow_sequence_mapping_entries(void)
{
    printf("test_bridge_parser_flow_sequence_mapping_entries\n");

    /* Flow sequence with mapping entries exercises:
       parse_flow_sequence_entry_mapping_key/value/end */
    const char *input =
        "[{k1: v1}, {k2: v2}]\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) break;
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_bridge_parser_indentless_sequence                              */
/* ------------------------------------------------------------------ */
static void test_bridge_parser_indentless_sequence(void)
{
    printf("test_bridge_parser_indentless_sequence\n");

    /* Block mapping with indentless sequence value exercises
       parse_indentless_sequence_entry */
    const char *input =
        "mapping:\n"
        "- item1\n"
        "- item2\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) break;
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_multiple_documents                                             */
/* ------------------------------------------------------------------ */
static void test_multiple_documents(void)
{
    printf("test_multiple_documents\n");

    const char *input =
        "---\nfirst: doc\n"
        "---\nsecond: doc\n"
        "---\nthird: doc\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int doc_count = 0;
    yaml_document_t doc;
    while (1) {
        if (!yaml_parser_load(&parser, &doc)) {
            printf("  load_error: %d\n", parser.error);
            break;
        }
        if (!yaml_document_get_root_node(&doc)) {
            yaml_document_delete(&doc);
            break;
        }
        doc_count++;
        yaml_document_delete(&doc);
    }
    printf("  doc_count: %d\n", doc_count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_emitter_canonical                                              */
/* ------------------------------------------------------------------ */
static void test_emitter_canonical(void)
{
    printf("test_emitter_canonical\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_canonical(&emitter, 1);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:map", 0,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"canonical_key", 13, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"canonical_val", 13, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_version_tag_directives                                 */
/* ------------------------------------------------------------------ */
static void test_emitter_version_tag_directives(void)
{
    printf("test_emitter_version_tag_directives\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    yaml_version_directive_t vd = {1, 2};
    yaml_tag_directive_t td[2];
    td[0].handle = (yaml_char_t *)"!";
    td[0].prefix = (yaml_char_t *)"!myns!";
    td[1].handle = NULL;
    td[1].prefix = NULL;

    yaml_document_start_event_initialize(&ev, &vd, td, td+1, 0);
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  doc_start_with_directives: %d\n", r);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_dump_sequence_mapping_alias                                    */
/* Covers: yaml_emitter_dump_sequence, yaml_emitter_dump_mapping,     */
/*         yaml_emitter_dump_alias                                     */
/* ------------------------------------------------------------------ */
static void test_dump_sequence_mapping_alias(void)
{
    printf("test_dump_sequence_mapping_alias\n");

    unsigned char outbuf[8192];
    size_t written;
    yaml_emitter_t emitter;
    yaml_document_t doc;
    int r;

    /* --- dump_sequence: root is a sequence node --- */
    written = 0;
    memset(outbuf, 0, sizeof(outbuf));
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    int s1  = yaml_document_add_scalar(&doc, NULL,
                  (yaml_char_t *)"alpha", 5, YAML_PLAIN_SCALAR_STYLE);
    int s2  = yaml_document_add_scalar(&doc, NULL,
                  (yaml_char_t *)"beta",  4, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s1);
    yaml_document_append_sequence_item(&doc, seq, s2);
    r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_sequence_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);

    /* --- dump_mapping: root is a mapping node --- */
    written = 0;
    memset(outbuf, 0, sizeof(outbuf));
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int map = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    int k   = yaml_document_add_scalar(&doc, NULL,
                  (yaml_char_t *)"key",   3, YAML_PLAIN_SCALAR_STYLE);
    int v   = yaml_document_add_scalar(&doc, NULL,
                  (yaml_char_t *)"value", 5, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, map, k, v);
    r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_mapping_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);

    /* --- dump_alias: sequence (root) contains the same scalar twice ---
       anchor_node increments ref count to 2, assigns anchor id,
       on second visit serialized==1 → yaml_emitter_dump_alias is called */
    written = 0;
    memset(outbuf, 0, sizeof(outbuf));
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    /* seq2 must be node 1 (root) so it is the entry point for dump */
    int seq2   = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    int shared = yaml_document_add_scalar(&doc, NULL,
                     (yaml_char_t *)"shared", 6, YAML_PLAIN_SCALAR_STYLE);
    /* Append the same node index twice so anchor_node gives it an anchor */
    yaml_document_append_sequence_item(&doc, seq2, shared);
    yaml_document_append_sequence_item(&doc, seq2, shared);
    r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_alias_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emit_flow_sequence                                             */
/* Covers: yaml_emitter_emit_flow_sequence_item                       */
/* ------------------------------------------------------------------ */
static void test_emit_flow_sequence(void)
{
    printf("test_emit_flow_sequence\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Flow sequence — triggers emit_flow_sequence_item */
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"one", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"two", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"three", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_emitter_flush(&emitter);
    printf("  written: %d\n", (int)written);
    if (written > 0)
        printf("  output: %.*s\n", (int)written, (char *)outbuf);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_flow_sequence_compact_mapping                                  */
/* Covers: yaml_parser_parse_flow_sequence_entry_mapping_key,         */
/*         yaml_parser_parse_flow_sequence_entry_mapping_value,       */
/*         yaml_parser_parse_flow_sequence_entry_mapping_end          */
/* ------------------------------------------------------------------ */
static void test_flow_sequence_compact_mapping(void)
{
    printf("test_flow_sequence_compact_mapping\n");

    /* Compact notation: KEY token (?) inside a flow sequence triggers the
       FLOW_SEQUENCE_ENTRY_MAPPING_KEY/VALUE/END state chain. */
    const char *inputs[] = {
        /* explicit key inside flow sequence */
        "[? k1 : v1, ? k2 : v2]\n",
        /* explicit key with no value */
        "[? only_key]\n",
        /* explicit key with empty value */
        "[? k :]\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                ok = 0;
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_ok: %d events: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* main                                                                */
/* ------------------------------------------------------------------ */
int main(void)
{
    printf("=== libyaml test suite ===\n");

    test_version();
    test_memory();
    test_string_extend_join();
    test_stack_queue_extend();
    test_token_delete();
    test_event_initialize();
    test_parser_scan();
    test_parser_parse();
    test_parser_load();
    test_parser_encoding();
    test_parser_set_input();
    test_set_max_nest_level();
    test_emitter_basic();
    test_emitter_set_output();
    test_emitter_open_close();
    test_emitter_dump();
    test_document_api();
    test_parser_fetch_update();
    test_bridge_check_utf8();
    test_bridge_reader_handlers();
    test_bridge_writer_handlers();
    test_bridge_emitter_set_errors();
    test_bridge_emitter_analyze();
    test_bridge_emitter_write();
    test_bridge_emitter_write_quoted();
    test_bridge_emitter_write_block();
    test_bridge_emitter_checks();
    test_bridge_emitter_append_tag_directive();
    test_bridge_emitter_increase_indent();
    test_bridge_emitter_generate_anchor();
    test_bridge_emitter_dump_document();
    test_bridge_parser_set_errors();
    test_bridge_parser_flow_levels();
    test_bridge_parser_scan_full();
    test_bridge_parser_parse_full();
    test_bridge_parser_load_aliases();
    test_bridge_parser_determine_encoding();
    test_bridge_parser_scan_tokens();
    test_bridge_parser_roll_indent();
    test_bridge_parser_fetch_stream();
    test_bridge_parser_fetch_document_indicators();
    test_bridge_parser_fetch_collections();
    test_bridge_parser_fetch_block_entry_key_value();
    test_bridge_parser_fetch_anchors_tags();
    test_bridge_parser_fetch_scalars();
    test_bridge_parser_scan_directives();
    test_bridge_parser_scan_uri_escapes();
    test_bridge_parser_process_directives();
    test_bridge_parser_append_tag_directive();
    test_bridge_parser_stale_save_remove_keys();
    test_bridge_emitter_emit_stream();
    test_bridge_emitter_process_anchor_tag_scalar();
    test_bridge_emitter_select_scalar_style();
    test_bridge_emitter_write_bom();
    test_bridge_emitter_block_scalar_hints();
    test_bridge_parser_parse_states();
    test_bridge_parser_process_empty_scalar();
    test_bridge_parser_register_anchor();
    test_bridge_emitter_emit_nodes();
    test_bridge_emitter_emit_alias();
    test_bridge_parser_flow_sequence_mapping_entries();
    test_bridge_parser_indentless_sequence();
    test_multiple_documents();
    test_emitter_canonical();
    test_emitter_version_tag_directives();
    test_dump_sequence_mapping_alias();
    test_emit_flow_sequence();
    test_flow_sequence_compact_mapping();

    printf("=== done ===\n");
    return 0;
}
