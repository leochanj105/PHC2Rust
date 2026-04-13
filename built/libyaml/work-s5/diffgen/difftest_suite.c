/*
 * test_suite.c — Comprehensive libyaml test suite
 * Covers all public and static (bridge_*) functions.
 * Output is deterministic plain text for C/Rust diffing.
 */

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
            printf("  scan_error: %s\n", parser.problem ? parser.problem : "?");
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
            printf("  parse_error: %s\n", parser.problem ? parser.problem : "?");
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
        printf("  load_error: %s\n", parser.problem ? parser.problem : "?");
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
    printf("  emitter_error: %s\n", emitter.problem ? emitter.problem : "(null)");

    yaml_emitter_initialize(&emitter); /* reset */
    r = bridge_yaml_emitter_set_writer_error(&emitter, "test writer error");
    printf("  set_writer_error_ret: %d\n", r);
    printf("  writer_error: %s\n", emitter.problem ? emitter.problem : "(null)");

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
    printf("  parser_problem: %s\n", parser.problem ? parser.problem : "(null)");

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
            printf("  parse_error: %s\n",
                parser.problem ? parser.problem : "?");
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
    /* use the internal type via the private header approach in bridge */
    /* Instead: just call bridge directly with a constructed string */
    /* yaml_string_t: start, end, pointer — all yaml_char_t* */
    typedef struct { yaml_char_t *start; yaml_char_t *end; yaml_char_t *pointer; } ys_t;
    ys_t ys;
    ys.start   = (yaml_char_t *)val;
    ys.end     = (yaml_char_t *)(val + sizeof(val) - 1);
    ys.pointer = (yaml_char_t *)val;

    /* The bridge signature: int bridge_yaml_emitter_write_block_scalar_hints(
       yaml_emitter_t *emitter, yaml_string_t string)
       yaml_string_t is the same struct layout. */
    /* Cast through void pointer to avoid type mismatch */
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
            printf("  load_error: %s\n",
                parser.problem ? parser.problem : "?");
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
/* test_token_delete_all_types                                         */
/* Covers api.c: TAG_DIRECTIVE, ALIAS, ANCHOR, TAG token delete       */
/* ------------------------------------------------------------------ */
static void test_token_delete_all_types(void)
{
    printf("test_token_delete_all_types\n");
    yaml_token_t tok;

    /* TAG_DIRECTIVE token */
    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_TAG_DIRECTIVE_TOKEN;
    tok.data.tag_directive.handle = (yaml_char_t *)yaml_strdup(
        (const yaml_char_t *)"!");
    tok.data.tag_directive.prefix = (yaml_char_t *)yaml_strdup(
        (const yaml_char_t *)"!ns!");
    yaml_token_delete(&tok);
    printf("  tag_directive_delete_ok: 1\n");

    /* ALIAS token */
    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_ALIAS_TOKEN;
    tok.data.alias.value = (yaml_char_t *)yaml_strdup(
        (const yaml_char_t *)"myalias");
    yaml_token_delete(&tok);
    printf("  alias_delete_ok: 1\n");

    /* ANCHOR token */
    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_ANCHOR_TOKEN;
    tok.data.anchor.value = (yaml_char_t *)yaml_strdup(
        (const yaml_char_t *)"myanchor");
    yaml_token_delete(&tok);
    printf("  anchor_delete_ok: 1\n");

    /* TAG token */
    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_TAG_TOKEN;
    tok.data.tag.handle = (yaml_char_t *)yaml_strdup(
        (const yaml_char_t *)"!");
    tok.data.tag.suffix = (yaml_char_t *)yaml_strdup(
        (const yaml_char_t *)"str");
    yaml_token_delete(&tok);
    printf("  tag_delete_ok: 1\n");

    /* NO_TOKEN (default branch) */
    memset(&tok, 0, sizeof(tok));
    tok.type = YAML_NO_TOKEN;
    yaml_token_delete(&tok);
    printf("  no_token_delete_ok: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_emitter_cr_break                                               */
/* Covers emitter.c:28:11:true (CR_BREAK path in PUT_BREAK)           */
/* ------------------------------------------------------------------ */
static void test_emitter_cr_break(void)
{
    printf("test_emitter_cr_break\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_break(&emitter, YAML_CR_BREAK);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Literal block scalar triggers PUT_BREAK in write_literal_scalar */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"line1\nline2\nline3\n", 18,
        0, 1, YAML_LITERAL_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  cr_break_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_crln_break                                             */
/* Covers emitter.c:32:11:true (CRLN_BREAK path in PUT_BREAK)         */
/* ------------------------------------------------------------------ */
static void test_emitter_crln_break(void)
{
    printf("test_emitter_crln_break\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_break(&emitter, YAML_CRLN_BREAK);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"lineA\nlineB\nlineC\n", 18,
        0, 1, YAML_LITERAL_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  crln_break_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_utf16le_encoding                                       */
/* Covers emitter.c:533 true — non-UTF8 encoding triggers write_bom   */
/* ------------------------------------------------------------------ */
static void test_emitter_utf16le_encoding(void)
{
    printf("test_emitter_utf16le_encoding\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF16LE_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF16LE_ENCODING);
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  stream_start_ok: %d\n", r);
    if (r) {
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)"hello", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_end_event_initialize(&ev, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);
    }
    printf("  utf16le_written_gt0: %d\n", written > 0 ? 1 : 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_utf8_bom                                                */
/* Covers reader.c:76 — UTF-8 BOM detection path                      */
/* ------------------------------------------------------------------ */
static void test_parser_utf8_bom(void)
{
    printf("test_parser_utf8_bom\n");

    /* UTF-8 BOM: 0xEF 0xBB 0xBF followed by plain YAML */
    static const unsigned char bom_input[] =
        "\xEF\xBB\xBF" "key: value\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, bom_input, sizeof(bom_input) - 1);

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  bom_parse_error: %d\n", (int)parser.error);
            break;
        }
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  bom_events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_emitter_special_chars_dquoted                                  */
/* Covers emitter.c:2092-2152 switch branches in write_double_quoted  */
/* ------------------------------------------------------------------ */
static void test_emitter_special_chars_dquoted(void)
{
    printf("test_emitter_special_chars_dquoted\n");

    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* 0x00 → \0 */
    { static const yaml_char_t v[] = {0x00}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x07 → \a */
    { static const yaml_char_t v[] = {0x07}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x08 → \b */
    { static const yaml_char_t v[] = {0x08}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x09 → \t */
    { static const yaml_char_t v[] = {0x09}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x0A → \n */
    { static const yaml_char_t v[] = {0x0A}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x0B → \v */
    { static const yaml_char_t v[] = {0x0B}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x0C → \f */
    { static const yaml_char_t v[] = {0x0C}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x0D → \r */
    { static const yaml_char_t v[] = {0x0D}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x1B → \e */
    { static const yaml_char_t v[] = {0x1B}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x22 → \" */
    { static const yaml_char_t v[] = {0x22}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x5C → \\ */
    { static const yaml_char_t v[] = {0x5C}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* U+0085 (0xC2 0x85) → \N */
    { static const yaml_char_t v[] = {0xC2, 0x85}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 2, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* U+00A0 (0xC2 0xA0) → \_ */
    { static const yaml_char_t v[] = {0xC2, 0xA0}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 2, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* U+2028 (0xE2 0x80 0xA8) → \L */
    { static const yaml_char_t v[] = {0xE2, 0x80, 0xA8}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* U+2029 (0xE2 0x80 0xA9) → \P */
    { static const yaml_char_t v[] = {0xE2, 0x80, 0xA9}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* 0x01 → default case, value <= 0xFF → \xNN */
    { static const yaml_char_t v[] = {0x01}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 1, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* U+FFFE (0xEF 0xBF 0xBE) → default, value <= 0xFFFF → \uNNNN */
    { static const yaml_char_t v[] = {0xEF, 0xBF, 0xBE}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }
    /* BOM U+FEFF (0xEF 0xBB 0xBF) → IS_BOM path → \uFEFF */
    { static const yaml_char_t v[] = {0xEF, 0xBB, 0xBF}; yaml_scalar_event_initialize(&ev, NULL, NULL, (yaml_char_t *)v, 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE); yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  special_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_large_output                                           */
/* Covers emitter.c:9 false — internal buffer fills → flush triggered */
/* ------------------------------------------------------------------ */
static void test_emitter_large_output(void)
{
    printf("test_emitter_large_output\n");

    /* 64KB output buffer — large enough for many flushes */
    size_t bufsize = 65536;
    unsigned char *outbuf = (unsigned char *)yaml_malloc(bufsize);
    if (!outbuf) {
        printf("  skip_no_memory: 1\n");
        return;
    }
    memset(outbuf, 0, bufsize);
    size_t written = 0;

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, bufsize, &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* ~210 bytes per pair × 100 = ~21KB > 16KB internal buffer */
    char key_buf[32], val_buf[256];
    int i;
    for (i = 0; i < 100; i++) {
        int klen = snprintf(key_buf, sizeof(key_buf), "key%04d", i);
        /* 200-character value ensures we exceed the internal buffer */
        int vlen = snprintf(val_buf, sizeof(val_buf),
                            "%0200d", i);
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)key_buf, (size_t)klen,
            1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)val_buf, (size_t)vlen,
            1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
    }

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  large_written_gt0: %d\n", written > 0 ? 1 : 0);
    yaml_free(outbuf);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_narrow_width                                           */
/* Covers emitter.c:780 — column > best_width in flow sequences       */
/* ------------------------------------------------------------------ */
static void test_emitter_narrow_width(void)
{
    printf("test_emitter_narrow_width\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    /* Narrow width forces line wrapping inside flow sequences */
    yaml_emitter_set_width(&emitter, 10);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Long items force column > best_width */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"longscalar1", 11, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"longscalar2", 11, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"longscalar3", 11, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  narrow_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_dump_empty_document                                            */
/* Covers dumper.c:126 — yaml_emitter_dump with empty document        */
/* ------------------------------------------------------------------ */
static void test_dump_empty_document(void)
{
    printf("test_dump_empty_document\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    /* Empty document — no nodes added */
    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);

    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_empty_ok: %d\n", r);
    yaml_emitter_close(&emitter);
    printf("  written_gt0: %d\n", written > 0 ? 1 : 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_yaml12_directive                                       */
/* Covers emitter.c:612 — YAML 1.2 version directive path             */
/* ------------------------------------------------------------------ */
static void test_emitter_yaml12_directive(void)
{
    printf("test_emitter_yaml12_directive\n");

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

    /* YAML 1.2 version directive (minor != 1) */
    yaml_version_directive_t vd12 = {1, 2};
    yaml_document_start_event_initialize(&ev, &vd12, NULL, NULL, 0);
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  doc_start_12_ok: %d\n", r);

    if (r) {
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_end_event_initialize(&ev, 0);
        yaml_emitter_emit(&emitter, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);
    }
    printf("  yaml12_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_open_ended_sequence                                    */
/* Covers emitter.c:719 — open_ended path (implicit document end)     */
/* ------------------------------------------------------------------ */
static void test_emitter_open_ended_sequence(void)
{
    printf("test_emitter_open_ended_sequence\n");

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

    /* First document — implicit end */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"first", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);   /* implicit=1 */
    yaml_emitter_emit(&emitter, &ev);

    /* Second document — also implicit, exercises open_ended=1 */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"second", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  open_ended_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_single_quoted_special                                  */
/* Covers emitter.c:2016 — single-quoted with embedded newlines       */
/* and spaces at column boundary (emitter.c:2001-2005)                */
/* ------------------------------------------------------------------ */
static void test_emitter_single_quoted_special(void)
{
    printf("test_emitter_single_quoted_special\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 20);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Single-quoted scalar with embedded newline */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"hello\nworld", 11,
        0, 1, YAML_SINGLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Single-quoted scalar with embedded quote '' */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"it's here", 9,
        0, 1, YAML_SINGLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Single-quoted scalar with spaces forcing wrap */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"long word wrapping here ok", 26,
        0, 1, YAML_SINGLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  sq_special_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_plain_scalar_breaks                                    */
/* Covers emitter.c:1946-1968 — plain scalar with spaces/breaks       */
/* ------------------------------------------------------------------ */
static void test_emitter_plain_scalar_breaks(void)
{
    printf("test_emitter_plain_scalar_breaks\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 15);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Plain scalar with spaces — allow_breaks path in write_plain_scalar */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"word1 word2 word3 word4", 23,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  plain_breaks_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_sequence_canonical                               */
/* Covers emitter.c:762 — canonical mode with flow-sequence end       */
/* ------------------------------------------------------------------ */
static void test_emitter_flow_sequence_canonical(void)
{
    printf("test_emitter_flow_sequence_canonical\n");

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

    /* Flow sequence in canonical mode: SEQUENCE-END with !first triggers
       the canonical comma+indent path (emitter.c:762) */
    yaml_sequence_start_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:seq", 0,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"a", 1, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"b", 1, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  canonical_seq_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_flow_mapping_canonical                                 */
/* Covers flow mapping emit paths in canonical mode                   */
/* ------------------------------------------------------------------ */
static void test_emitter_flow_mapping_canonical(void)
{
    printf("test_emitter_flow_mapping_canonical\n");

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
        YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"k1", 2, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"v1", 2, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"k2", 2, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"v2", 2, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  canonical_map_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_explicit_doc                                           */
/* Covers emitter.c:642 true — !implicit → writes "---"              */
/* ------------------------------------------------------------------ */
static void test_emitter_explicit_doc(void)
{
    printf("test_emitter_explicit_doc\n");

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

    /* Explicit document start (implicit=0) */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"explicit", 8, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Explicit document end (implicit=0) */
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  explicit_doc_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_complex_flow                                            */
/* Covers various scanner/parser branches with flow scalars, escapes   */
/* ------------------------------------------------------------------ */
static void test_parser_complex_flow(void)
{
    printf("test_parser_complex_flow\n");

    /* Flow scalars with single-quote escaping ('') and double-quote
       escape sequences, plus various edge cases */
    const char *inputs[] = {
        /* Single-quoted with '' escaping */
        "- 'it''s a test'\n",
        /* Double-quoted with escapes */
        "- \"tab\\there\"\n",
        "- \"newline\\nhere\"\n",
        /* Flow sequence with trailing comma equivalent */
        "[a, b, c,]\n",
        /* Nested flow */
        "{a: {b: c}}\n",
        /* Block scalar with strip chomping */
        "- |-\n  stripped\n",
        /* Block scalar with keep chomping */
        "- |+\n  kept\n\n",
        /* Folded scalar */
        "- >\n  folded line\n  another\n",
        /* Double-quoted multiline */
        "- \"line1\\\n  line2\"\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_token_t tok;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        printf("  input%d_tokens: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_parser_special_scalars                                         */
/* Covers reader.c and scanner branches with Unicode / multibyte      */
/* ------------------------------------------------------------------ */
static void test_parser_special_scalars(void)
{
    printf("test_parser_special_scalars\n");

    /* Various YAML documents to exercise encoding/scanning branches */
    const char *inputs[] = {
        /* Multi-byte UTF-8 scalars */
        "- \xC3\xA9l\xC3\xA8ve\n",      /* "élève" */
        "- \xE4\xB8\xAD\xE6\x96\x87\n", /* Chinese chars */
        /* Tab handling in block scalar */
        "|\n  line with\ttab\n",
        /* Null bytes in plain context not valid, use double-quoted */
        "- \"\\0\"\n",
        /* Long plain scalar */
        "- this is a very long plain scalar that goes on and on for many characters\n",
        /* Numbers as scalars */
        "- 3.14\n",
        "- 0x1F\n",
        "- 0o17\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  input%d_error: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_mapping_complex                                  */
/* Covers block mapping emit paths with complex keys                  */
/* ------------------------------------------------------------------ */
static void test_emitter_block_mapping_complex(void)
{
    printf("test_emitter_block_mapping_complex\n");

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

    /* Block mapping with sequence as value (emit_block_mapping_value simple=0) */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"seq_key", 7, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Non-simple value: a sequence (exercises emit_block_mapping_value simple=0) */
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

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  block_map_complex_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_directive_errors                                        */
/* Covers scanner branches for invalid directives                     */
/* ------------------------------------------------------------------ */
static void test_parser_directive_errors(void)
{
    printf("test_parser_directive_errors\n");

    /* Duplicate YAML directive — scanner error path */
    const char *dup_yaml =
        "%YAML 1.1\n"
        "%YAML 1.1\n"
        "---\nfoo\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)dup_yaml, strlen(dup_yaml));

    int count = 0;
    yaml_token_t tok;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    /* May fail with scanner error — that's expected */
    printf("  dup_yaml_tokens_or_error: %d err: %d\n",
           count, (int)parser.error);
    yaml_parser_delete(&parser);

    /* TAB directive (unknown) */
    const char *tag_dir =
        "%TAG !! tag:example.com,2000:\n"
        "---\nfoo\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)tag_dir, strlen(tag_dir));
    count = 0;
    while (yaml_parser_scan(&parser, &tok)) {
        count++;
        int done = (tok.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&tok);
        if (done) break;
    }
    printf("  tag_dir_tokens: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_emitter_double_quoted_breaks                                   */
/* Covers emitter.c:2175-2179 — double-quoted with line breaks        */
/* ------------------------------------------------------------------ */
static void test_emitter_double_quoted_breaks(void)
{
    printf("test_emitter_double_quoted_breaks\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 20);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Double-quoted scalar with newline: exercises 2016-2021 path */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"line1\nline2", 11,
        0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Double-quoted with spaces at width boundary */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"word1 word2 word3 word4", 23,
        0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  dq_breaks_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_block_sequence_complex                                  */
/* Covers various block sequence/mapping scanner branches             */
/* ------------------------------------------------------------------ */
static void test_parser_block_sequence_complex(void)
{
    printf("test_parser_block_sequence_complex\n");

    const char *inputs[] = {
        /* Block sequence with complex keys */
        "? k\n: v\n? k2\n: v2\n",
        /* Nested block sequences */
        "- - a\n  - b\n- - c\n",
        /* Block mapping with sequence values */
        "a:\n  - 1\n  - 2\nb:\n  - 3\n",
        /* Multiple directives document */
        "%YAML 1.1\n---\nkey: val\n...\n",
        /* Anchor+alias within sequence */
        "- &x foo\n- *x\n- *x\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_write_indent_negative                                  */
/* Covers emitter.c:increase_indent with flow+indentless combos       */
/* ------------------------------------------------------------------ */
static void test_emitter_increase_indent_variants(void)
{
    printf("test_emitter_increase_indent_variants\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* flow=1, indentless=1 */
    int r = bridge_yaml_emitter_increase_indent(&emitter, 1, 1);
    printf("  flow_indentless_ok: %d indent: %d\n", r, emitter.indent);

    /* flow=0, indentless=1 */
    r = bridge_yaml_emitter_increase_indent(&emitter, 0, 1);
    printf("  noflow_indentless_ok: %d indent: %d\n", r, emitter.indent);

    /* flow=1, indentless=0 */
    r = bridge_yaml_emitter_increase_indent(&emitter, 1, 0);
    printf("  flow_noindentless_ok: %d indent: %d\n", r, emitter.indent);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_analyze_anchor_alias                                   */
/* Covers emitter.c:analyze_anchor with alias=1                       */
/* ------------------------------------------------------------------ */
static void test_emitter_analyze_anchor_alias(void)
{
    printf("test_emitter_analyze_anchor_alias\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* analyze_anchor with alias=1 (alias reference) */
    int r = bridge_yaml_emitter_analyze_anchor(&emitter,
                (yaml_char_t *)"myref", 1);
    printf("  analyze_alias_anchor_ok: %d\n", r);

    /* analyze_anchor with alias=0 (anchor definition) */
    r = bridge_yaml_emitter_analyze_anchor(&emitter,
                (yaml_char_t *)"myanchor2", 0);
    printf("  analyze_anchor_def_ok: %d\n", r);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_block_scalar_variants                                   */
/* Covers scanner branches in scan_block_scalar                       */
/* ------------------------------------------------------------------ */
static void test_parser_block_scalar_variants(void)
{
    printf("test_parser_block_scalar_variants\n");

    const char *inputs[] = {
        /* literal with explicit indent indicator */
        "- |2\n  indented\n",
        /* folded with strip */
        "- >-\n  folded strip\n",
        /* folded with keep */
        "- >+\n  folded keep\n\n",
        /* literal with leading empty lines */
        "- |\n\n  first\n  second\n",
        /* block scalar at indented level */
        "key:\n  value: |\n    multi\n    line\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_token_t tok;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        printf("  input%d_tokens: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_parser_flow_scalar_variants                                    */
/* Covers scanner branches in scan_flow_scalar                        */
/* ------------------------------------------------------------------ */
static void test_parser_flow_scalar_variants(void)
{
    printf("test_parser_flow_scalar_variants\n");

    const char *inputs[] = {
        /* Double-quoted with various escape sequences */
        "- \"\\0\\a\\b\\t\\n\\v\\f\\r\\e\\\\\\\"\"\n",
        "- \"\\N\\_\\L\\P\"\n",
        "- \"\\x41\\u0041\\U00000041\"\n",
        /* Single-quoted with '' escape */
        "- 'can''t stop won''t stop'\n",
        /* Double-quoted multiline fold */
        "- \"a \\\n  b\"\n",
        /* Double-quoted with spaces */
        "- \" leading and trailing \"\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  input%d_parse_error: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            if (ev.type == YAML_SCALAR_EVENT)
                printf("  input%d_scalar_len: %d\n",
                       i, (int)ev.data.scalar.length);
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_parser_plain_scalar_variants                                   */
/* Covers scanner branches in scan_plain_scalar                       */
/* ------------------------------------------------------------------ */
static void test_parser_plain_scalar_variants(void)
{
    printf("test_parser_plain_scalar_variants\n");

    const char *inputs[] = {
        /* Plain scalar with colon inside */
        "- a:b\n",
        /* Plain scalar that is empty after stripping */
        "{a: ,b: }\n",
        /* Plain scalar in flow context */
        "[plain in flow, another plain]\n",
        /* Multi-line plain scalar */
        "- first\n  second\n  third\n",
        /* Plain with hash in middle */
        "- value # comment\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_write_tag_content_variants                             */
/* Covers emitter.c:263-277 tag content character classification      */
/* ------------------------------------------------------------------ */
static void test_emitter_write_tag_content_variants(void)
{
    printf("test_emitter_write_tag_content_variants\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Emit scalars with tags requiring URI encoding */
    yaml_scalar_event_initialize(&ev,
        NULL,
        (yaml_char_t *)"!<tag:example.com,2000:str>",
        (yaml_char_t *)"hello", 5,
        0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Second doc with tag directive to exercise write_tag_content */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev,
        NULL,
        (yaml_char_t *)"!!str",
        (yaml_char_t *)"world", 5,
        0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  tag_content_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_nest_limit                                              */
/* Covers parser.c — yaml_set_max_nest_level with deep nesting        */
/* ------------------------------------------------------------------ */
static void test_parser_nest_limit(void)
{
    printf("test_parser_nest_limit\n");

    /* Set nest level so deep documents are rejected */
    yaml_set_max_nest_level(3);

    /* Document with depth 4 → should fail */
    const char *deep =
        "a:\n  b:\n    c:\n      d: leaf\n";

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)deep, strlen(deep));

    yaml_event_t ev;
    int ok = 1;
    int count = 0;
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
    printf("  deep_ok_or_err: %d error: %d\n", ok, (int)parser.error);
    yaml_parser_delete(&parser);

    /* Restore */
    yaml_set_max_nest_level(1000);
    printf("  nest_limit_restored: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_document_with_tags                                             */
/* Covers api.c:document_start_event_initialize with tag directives  */
/* ------------------------------------------------------------------ */
static void test_document_with_tags(void)
{
    printf("test_document_with_tags\n");

    yaml_event_t ev;

    /* document_start_event_initialize with version + tag directives */
    yaml_version_directive_t vd = {1, 1};
    yaml_tag_directive_t tds[2];
    tds[0].handle = (yaml_char_t *)"!";
    tds[0].prefix = (yaml_char_t *)"!myns!";
    tds[1].handle = (yaml_char_t *)"!!";
    tds[1].prefix = (yaml_char_t *)"tag:yaml.org,2002:";

    int r = yaml_document_start_event_initialize(&ev, &vd,
        tds, tds + 2, 0);
    printf("  doc_start_with_all_ok: %d\n", r);
    printf("  version_major: %d\n",
        ev.data.document_start.version_directive
        ? ev.data.document_start.version_directive->major : -1);
    yaml_event_delete(&ev);

    /* document_start_event_initialize with only version directive */
    r = yaml_document_start_event_initialize(&ev, &vd, NULL, NULL, 0);
    printf("  doc_start_vd_only_ok: %d\n", r);
    yaml_event_delete(&ev);
}

/* ------------------------------------------------------------------ */
/* test_emitter_folded_with_breaks                                     */
/* Covers emitter.c write_folded_scalar multi-break paths             */
/* ------------------------------------------------------------------ */
static void test_emitter_folded_with_breaks(void)
{
    printf("test_emitter_folded_with_breaks\n");

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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Folded scalar: multiple consecutive newlines, leading spaces */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"first paragraph\n\nsecond paragraph\n\n\nthird\n",
        43, 0, 1, YAML_FOLDED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Literal scalar with leading space (tests indent detection) */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)" leading space\nregular line\n", 28,
        0, 1, YAML_LITERAL_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  folded_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_api_malloc_zero_realloc                                        */
/* Covers api.c:33 (malloc size=0 false branch),                      */
/*        api.c:43 (realloc null/zero branches)                       */
/* ------------------------------------------------------------------ */
static void test_api_malloc_zero_realloc(void)
{
    printf("test_api_malloc_zero_realloc\n");

    /* yaml_malloc(0) → malloc(1): size ? size : 1 — false branch */
    void *p0 = yaml_malloc(0);
    printf("  malloc_zero_ok: %d\n", p0 != NULL);
    yaml_free(p0);

    /* yaml_realloc(NULL, 8) → malloc(8): ptr==NULL branch */
    void *p1 = yaml_realloc(NULL, 8);
    printf("  realloc_null_ok: %d\n", p1 != NULL);

    /* yaml_realloc(p1, 0) → realloc(p1,1): size==0 branch */
    void *p2 = yaml_realloc(p1, 0);
    printf("  realloc_zero_size_ok: %d\n", p2 != NULL);
    yaml_free(p2);

    /* yaml_realloc(NULL, 0) → malloc(1): ptr==NULL and size==0 */
    void *p3 = yaml_realloc(NULL, 0);
    printf("  realloc_null_zero_ok: %d\n", p3 != NULL);
    yaml_free(p3);
}

/* ------------------------------------------------------------------ */
/* test_api_event_utf8_errors                                          */
/* Covers api.c:805,836,841,885,940 — event_init with invalid UTF-8  */
/* ------------------------------------------------------------------ */
static void test_api_event_utf8_errors(void)
{
    printf("test_api_event_utf8_errors\n");

    yaml_event_t ev;

    /* alias_event_initialize: invalid UTF-8 anchor → returns 0 */
    int r = yaml_alias_event_initialize(&ev, (const yaml_char_t *)"\xff\xfe");
    printf("  alias_bad_utf8: %d\n", r);

    /* scalar: invalid UTF-8 anchor */
    r = yaml_scalar_event_initialize(&ev,
        (const yaml_char_t *)"\xff\xfe", NULL,
        (const yaml_char_t *)"v", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_bad_anchor: %d\n", r);

    /* scalar: invalid UTF-8 tag */
    r = yaml_scalar_event_initialize(&ev,
        NULL, (const yaml_char_t *)"\xff\xfe",
        (const yaml_char_t *)"v", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_bad_tag: %d\n", r);

    /* scalar: invalid UTF-8 value */
    r = yaml_scalar_event_initialize(&ev,
        NULL, NULL, (const yaml_char_t *)"\xff\xfe", 2,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_bad_value: %d\n", r);

    /* scalar: length=-1 (auto strlen) */
    r = yaml_scalar_event_initialize(&ev,
        NULL, NULL, (const yaml_char_t *)"hello", -1,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_len_minus1: %d\n", r);
    if (r) yaml_event_delete(&ev);

    /* sequence_start: invalid UTF-8 anchor */
    r = yaml_sequence_start_event_initialize(&ev,
        (const yaml_char_t *)"\xff\xfe", NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_bad_anchor: %d\n", r);

    /* sequence_start: invalid UTF-8 tag */
    r = yaml_sequence_start_event_initialize(&ev,
        NULL, (const yaml_char_t *)"\xff\xfe", 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_bad_tag: %d\n", r);

    /* mapping_start: invalid UTF-8 anchor */
    r = yaml_mapping_start_event_initialize(&ev,
        (const yaml_char_t *)"\xff\xfe", NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    printf("  map_bad_anchor: %d\n", r);

    /* mapping_start: invalid UTF-8 tag */
    r = yaml_mapping_start_event_initialize(&ev,
        NULL, (const yaml_char_t *)"\xff\xfe", 1,
        YAML_BLOCK_MAPPING_STYLE);
    printf("  map_bad_tag: %d\n", r);
}

/* ------------------------------------------------------------------ */
/* test_api_event_with_anchor_tag                                      */
/* Covers api.c:836-848, api.c:885-898, api.c:940-952 — event init   */
/*        with both anchor and tag set (both non-NULL paths)          */
/* ------------------------------------------------------------------ */
static void test_api_event_with_anchor_tag(void)
{
    printf("test_api_event_with_anchor_tag\n");

    yaml_event_t ev;

    /* scalar with both anchor and tag */
    int r = yaml_scalar_event_initialize(&ev,
        (const yaml_char_t *)"anc1",
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"hello", 5,
        0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    printf("  scalar_anchor_tag: %d\n", r);
    if (r) yaml_event_delete(&ev);

    /* sequence_start with both anchor and tag */
    r = yaml_sequence_start_event_initialize(&ev,
        (const yaml_char_t *)"seqanc",
        (const yaml_char_t *)"tag:yaml.org,2002:seq",
        0, YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_anchor_tag: %d\n", r);
    if (r) yaml_event_delete(&ev);

    /* mapping_start with both anchor and tag */
    r = yaml_mapping_start_event_initialize(&ev,
        (const yaml_char_t *)"mapanc",
        (const yaml_char_t *)"tag:yaml.org,2002:map",
        0, YAML_BLOCK_MAPPING_STYLE);
    printf("  map_anchor_tag: %d\n", r);
    if (r) yaml_event_delete(&ev);

    /* scalar with anchor only */
    r = yaml_scalar_event_initialize(&ev,
        (const yaml_char_t *)"anconly", NULL,
        (const yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_anchor_only: %d\n", r);
    if (r) yaml_event_delete(&ev);

    /* scalar with tag only */
    r = yaml_scalar_event_initialize(&ev,
        NULL, (const yaml_char_t *)"tag:yaml.org,2002:int",
        (const yaml_char_t *)"42", 2, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_tag_only: %d\n", r);
    if (r) yaml_event_delete(&ev);
}

/* ------------------------------------------------------------------ */
/* test_api_document_dirs                                              */
/* Covers api.c:1064-1094 — document_initialize with version+tags,   */
/*        api.c:1210-1229 — add_scalar/seq/map with explicit tags     */
/* ------------------------------------------------------------------ */
static void test_api_document_dirs(void)
{
    printf("test_api_document_dirs\n");

    yaml_version_directive_t vd = {1, 1};
    yaml_tag_directive_t tds[2];
    tds[0].handle = (yaml_char_t *)"!";
    tds[0].prefix = (yaml_char_t *)"!myns!";
    tds[1].handle = (yaml_char_t *)"!!";
    tds[1].prefix = (yaml_char_t *)"tag:yaml.org,2002:";

    /* document with version_directive and two tag_directives */
    yaml_document_t doc;
    int r = yaml_document_initialize(&doc, &vd, tds, tds+2, 0, 0);
    printf("  doc_init_dirs_ok: %d\n", r);
    if (r) {
        /* add_scalar with explicit tag and length=-1 */
        int s = yaml_document_add_scalar(&doc,
            (const yaml_char_t *)"tag:yaml.org,2002:str",
            (const yaml_char_t *)"hello", -1, YAML_PLAIN_SCALAR_STYLE);
        printf("  add_scalar_explicit_tag_len-1: %d\n", s);

        /* add_sequence with explicit tag */
        int seq = yaml_document_add_sequence(&doc,
            (const yaml_char_t *)"tag:yaml.org,2002:seq",
            YAML_BLOCK_SEQUENCE_STYLE);
        printf("  add_seq_explicit_tag: %d\n", seq);

        /* add_mapping with explicit tag */
        int map = yaml_document_add_mapping(&doc,
            (const yaml_char_t *)"tag:yaml.org,2002:map",
            YAML_BLOCK_MAPPING_STYLE);
        printf("  add_map_explicit_tag: %d\n", map);

        yaml_document_delete(&doc);
    }

    /* document with version only (no tag dirs) */
    r = yaml_document_initialize(&doc, &vd, NULL, NULL, 1, 1);
    printf("  doc_init_vd_only_ok: %d\n", r);
    if (r) {
        /* add_scalar with length=-1 */
        int s = yaml_document_add_scalar(&doc, NULL,
            (const yaml_char_t *)"auto", -1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
        printf("  add_scalar_auto_len: %d\n", s);
        yaml_document_delete(&doc);
    }
}

/* ------------------------------------------------------------------ */
/* test_api_string_write_overflow                                      */
/* Covers api.c:424 true — string_write_handler overflow path        */
/* ------------------------------------------------------------------ */
static void test_api_string_write_overflow(void)
{
    printf("test_api_string_write_overflow\n");

    /* 10-byte output buffer — too small for any YAML output */
    unsigned char tiny[10];
    size_t written = 0;
    memset(tiny, 0, sizeof(tiny));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, tiny, sizeof(tiny), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* 50-char scalar forces flush → triggers overflow in string_write_handler */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"this_scalar_definitely_overflows_the_tiny_buffer", 48,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  overflow_emit_result: %d\n", r);
    printf("  written_le_10: %d\n", (int)written <= 10 ? 1 : 0);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_api_emitter_indent_boundary                                    */
/* Covers api.c:540 — set_indent with boundary/invalid values        */
/* ------------------------------------------------------------------ */
static void test_api_emitter_indent_boundary(void)
{
    printf("test_api_emitter_indent_boundary\n");

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);

    /* indent=0: condition 1 < 0 is false → default 2 */
    yaml_emitter_set_indent(&emitter, 0);
    printf("  indent_0: %d\n", emitter.best_indent);

    /* indent=1: condition 1 < 1 is false → default 2 */
    yaml_emitter_set_indent(&emitter, 1);
    printf("  indent_1: %d\n", emitter.best_indent);

    /* indent=3: valid */
    yaml_emitter_set_indent(&emitter, 3);
    printf("  indent_3: %d\n", emitter.best_indent);

    /* indent=9: valid (< 10) */
    yaml_emitter_set_indent(&emitter, 9);
    printf("  indent_9: %d\n", emitter.best_indent);

    /* indent=10: condition 10 < 10 is false → default 2 */
    yaml_emitter_set_indent(&emitter, 10);
    printf("  indent_10: %d\n", emitter.best_indent);

    /* width=-2: negative → set to -1 */
    yaml_emitter_set_width(&emitter, -2);
    printf("  width_neg2: %d\n", emitter.best_width);

    /* width=0: zero → set to 0 */
    yaml_emitter_set_width(&emitter, 0);
    printf("  width_0: %d\n", emitter.best_width);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_api_queue_extend_move                                          */
/* Covers api.c:160-165 — queue_extend "move" path                   */
/*   (tail==end but head != start, no resize needed)                  */
/* ------------------------------------------------------------------ */
static void test_api_queue_extend_move(void)
{
    printf("test_api_queue_extend_move\n");

    size_t item = sizeof(int);
    size_t n    = 4;
    void *q_start = yaml_malloc(item * n);
    if (!q_start) { printf("  skip_oom: 1\n"); return; }

    /* Simulate: head has advanced 2 positions (items consumed),
       tail is at end, so tail==end but head!=start → "move" path */
    void *q_head  = (char *)q_start + item * 2;
    void *q_tail  = (char *)q_start + item * 4;  /* = end */
    void *q_end   = (char *)q_start + item * 4;

    int r = yaml_queue_extend(&q_start, &q_head, &q_tail, &q_end);
    printf("  queue_extend_move_ok: %d\n", r);
    /* After move: head should be at start */
    printf("  head_at_start: %d\n", (q_head == q_start) ? 1 : 0);
    yaml_free(q_start);
}

/* ------------------------------------------------------------------ */
/* test_emitter_flow_non_simple_key                                    */
/* Covers emitter.c:843 — flow mapping with non-simple key (?)        */
/* ------------------------------------------------------------------ */
static void test_emitter_flow_non_simple_key(void)
{
    printf("test_emitter_flow_non_simple_key\n");

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

    /* Flow mapping in canonical mode: sequence as key triggers non-simple path */
    yaml_mapping_start_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:map", 0, YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Sequence as key (non-simple) */
    yaml_sequence_start_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:seq", 0, YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"k", 1, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    /* Value */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"v", 1, 0, 0, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  flow_non_simple_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_non_simple_key                                   */
/* Covers emitter.c:947 — block mapping with non-simple key (?)       */
/* ------------------------------------------------------------------ */
static void test_emitter_block_non_simple_key(void)
{
    printf("test_emitter_block_non_simple_key\n");

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

    /* Block mapping with a sequence as key (non-simple) */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Flow sequence as key → non-simple block mapping key */
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"k1", 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    /* Simple scalar value */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"v1", 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  block_non_simple_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_dump_complex_nested                                            */
/* Covers dumper.c — mapping with seq+map values, anchor refs         */
/* ------------------------------------------------------------------ */
static void test_dump_complex_nested(void)
{
    printf("test_dump_complex_nested\n");

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

    /* Root: mapping */
    int root = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);

    /* k1 → sequence of scalars */
    int k1 = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"k1", 2, YAML_PLAIN_SCALAR_STYLE);
    int seq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    int a   = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"a", 1, YAML_PLAIN_SCALAR_STYLE);
    int b   = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"b", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, a);
    yaml_document_append_sequence_item(&doc, seq, b);
    yaml_document_append_mapping_pair(&doc, root, k1, seq);

    /* k2 → nested mapping */
    int k2  = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"k2", 2, YAML_PLAIN_SCALAR_STYLE);
    int nm  = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    int ik  = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"inner_k", 7, YAML_PLAIN_SCALAR_STYLE);
    int iv  = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"inner_v", 7, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, nm, ik, iv);
    yaml_document_append_mapping_pair(&doc, root, k2, nm);

    /* k3 → shared scalar (used twice → anchor) */
    int k3     = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"k3", 2, YAML_PLAIN_SCALAR_STYLE);
    int shared = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"shared", 6, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, root, k3, shared);
    /* Append shared again as value of k1's seq to trigger anchor */
    yaml_document_append_sequence_item(&doc, seq, shared);

    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  complex_nested_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_dump_with_directives                                           */
/* Covers dumper.c:122-149 — dump document with version+tag dirs      */
/* ------------------------------------------------------------------ */
static void test_dump_with_directives(void)
{
    printf("test_dump_with_directives\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_version_directive_t vd = {1, 1};
    yaml_tag_directive_t tds[2];
    tds[0].handle = (yaml_char_t *)"!";
    tds[0].prefix = (yaml_char_t *)"!ns!";
    tds[1].handle = (yaml_char_t *)"!!";
    tds[1].prefix = (yaml_char_t *)"tag:yaml.org,2002:";

    yaml_document_t doc;
    int r = yaml_document_initialize(&doc, &vd, tds, tds+2, 0, 0);
    printf("  doc_init_ok: %d\n", r);
    if (r) {
        int s = yaml_document_add_scalar(&doc, NULL,
            (yaml_char_t *)"directives_doc", 14, YAML_PLAIN_SCALAR_STYLE);
        printf("  scalar_ok: %d\n", s);
        r = yaml_emitter_dump(&emitter, &doc);
        printf("  dump_dirs_ok: %d written: %d\n", r, (int)written);
    }
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_multi_anchors                                           */
/* Covers loader.c:95-242 — load nodes with multiple anchors/aliases  */
/* ------------------------------------------------------------------ */
static void test_parser_multi_anchors(void)
{
    printf("test_parser_multi_anchors\n");

    const char *inputs[] = {
        /* two anchors + two aliases */
        "- &a x\n- &b y\n- *a\n- *b\n- *a\n",
        /* null value with anchor */
        "null_val: &n ~\nref: *n\n",
        /* seq of mappings with alias */
        "- &m {k: v}\n- *m\n",
        /* nested aliases */
        "base: &base\n  x: 1\nchild:\n  <<: *base\n  y: 2\n",
        /* empty sequence */
        "seq: []\n",
        /* empty mapping */
        "map: {}\n",
        /* deeply nested */
        "a:\n  b:\n    c: leaf\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_document_t doc;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int r = yaml_parser_load(&parser, &doc);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  input%d_ok: 1 root: %d\n", i, root ? root->type : -1);
            yaml_document_delete(&doc);
        } else {
            printf("  input%d_error: %d\n", i, (int)parser.error);
        }
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_unicode_chars                                          */
/* Covers emitter.c write_double_quoted unicode escape paths          */
/* ------------------------------------------------------------------ */
static void test_emitter_unicode_chars(void)
{
    printf("test_emitter_unicode_chars\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    /* unicode=0 forces non-ASCII to be escaped */
    yaml_emitter_set_unicode(&emitter, 0);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* 2-byte UTF-8: U+00E9 (é) — should be escaped as \uXXXX */
    { static const yaml_char_t v[] = {0xC3, 0xA9};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
          (yaml_char_t *)v, 2, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* 3-byte UTF-8: U+4E2D (中) */
    { static const yaml_char_t v[] = {0xE4, 0xB8, 0xAD};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
          (yaml_char_t *)v, 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* 4-byte UTF-8: U+1F600 (emoji) */
    { static const yaml_char_t v[] = {0xF0, 0x9F, 0x98, 0x80};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
          (yaml_char_t *)v, 4, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  unicode_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_utf16_bom                                               */
/* Covers reader.c — UTF-16 LE and BE BOM detection                  */
/* ------------------------------------------------------------------ */
static void test_parser_utf16_bom(void)
{
    printf("test_parser_utf16_bom\n");

    /* UTF-16 LE: BOM FF FE then "a: b\n" encoded as UTF-16 LE */
    static const unsigned char utf16le[] = {
        0xFF, 0xFE,
        'a', 0x00, ':', 0x00, ' ', 0x00, 'b', 0x00, '\n', 0x00
    };

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, utf16le, sizeof(utf16le));

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  utf16le_error: %d\n", (int)parser.error);
            break;
        }
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  utf16le_events: %d\n", count);
    yaml_parser_delete(&parser);

    /* UTF-16 BE: BOM FE FF then "a: b\n" as UTF-16 BE */
    static const unsigned char utf16be[] = {
        0xFE, 0xFF,
        0x00, 'a', 0x00, ':', 0x00, ' ', 0x00, 'b', 0x00, '\n'
    };

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, utf16be, sizeof(utf16be));

    count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  utf16be_error: %d\n", (int)parser.error);
            break;
        }
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  utf16be_events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_emitter_open_ended2                                            */
/* Covers emitter.c:666 — open_ended==2 path in emit_document_start  */
/* triggered by block scalar with trailing newlines at stream end     */
/* ------------------------------------------------------------------ */
static void test_emitter_open_ended2(void)
{
    printf("test_emitter_open_ended2\n");

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

    /* Document with literal block scalar with trailing empty lines */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"first\n\n", 7,
        0, 1, YAML_LITERAL_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Implicit document end → open_ended stays (not cleared to "...") */
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Next document: version directive → triggers "..." before it if open_ended */
    yaml_version_directive_t vd2 = {1, 1};
    yaml_document_start_event_initialize(&ev, &vd2, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"second", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  open_ended2_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_scalar_select_styles                                   */
/* Covers emitter.c:select_scalar_style various cases                 */
/* ------------------------------------------------------------------ */
static void test_emitter_scalar_select_styles(void)
{
    printf("test_emitter_scalar_select_styles\n");

    unsigned char outbuf[4096];
    size_t written;

    /* Each style with both implicit and non-implicit settings */
    struct {
        const char *value;
        int length;
        yaml_scalar_style_t style;
        int plain_implicit;
        int quoted_implicit;
        const char *name;
    } cases[] = {
        { "hello",     5, YAML_ANY_SCALAR_STYLE,           1, 1, "any_impl" },
        { "hello",     5, YAML_ANY_SCALAR_STYLE,           0, 0, "any_noimpl" },
        { "true",      4, YAML_ANY_SCALAR_STYLE,           1, 1, "any_bool" },
        { "",          0, YAML_ANY_SCALAR_STYLE,           1, 1, "any_empty" },
        { "key: val",  8, YAML_ANY_SCALAR_STYLE,           1, 1, "any_special" },
        { "hello",     5, YAML_SINGLE_QUOTED_SCALAR_STYLE, 1, 1, "single" },
        { "hello",     5, YAML_DOUBLE_QUOTED_SCALAR_STYLE, 0, 0, "double" },
    };

    for (int i = 0; i < (int)(sizeof(cases)/sizeof(cases[0])); i++) {
        written = 0;
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

        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)cases[i].value, cases[i].length,
            cases[i].plain_implicit, cases[i].quoted_implicit,
            cases[i].style);
        int r = yaml_emitter_emit(&emitter, &ev);

        if (r) {
            yaml_document_end_event_initialize(&ev, 1);
            yaml_emitter_emit(&emitter, &ev);
            yaml_stream_end_event_initialize(&ev);
            yaml_emitter_emit(&emitter, &ev);
        }

        printf("  style_%s_ok: %d written: %d\n",
               cases[i].name, r, (int)written);
        yaml_emitter_delete(&emitter);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_flow_mapping_value_complex                             */
/* Covers emitter.c:866 — flow mapping value in non-canonical/wide    */
/* ------------------------------------------------------------------ */
static void test_emitter_flow_mapping_value_complex(void)
{
    printf("test_emitter_flow_mapping_value_complex\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    /* narrow width forces column > best_width → write_indent in flow_mapping_value */
    yaml_emitter_set_width(&emitter, 5);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Flow mapping with narrow width — forces value to wrap */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"long_key_name", 13, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"long_val_name", 13, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  flow_value_complex_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_mapping_value_complex                            */
/* Covers emitter.c:965-973 — block mapping non-simple value          */
/* ------------------------------------------------------------------ */
static void test_emitter_block_mapping_value_complex(void)
{
    printf("test_emitter_block_mapping_value_complex\n");

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

    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Non-simple key (sequence) → BLOCK_MAPPING_VALUE_STATE simple=0 */
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    /* Value is a mapping (also exercises non-simple value branch) */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"inner", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"val", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  block_value_complex_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_alias_in_simple_key                                    */
/* Covers emitter.c:1026 — alias in simple_key_context (PUT ' ')      */
/* ------------------------------------------------------------------ */
static void test_emitter_alias_in_simple_key(void)
{
    printf("test_emitter_alias_in_simple_key\n");

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

    /* Flow mapping where an alias is the key (simple_key_context=1) */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Define anchor via scalar value first */
    yaml_scalar_event_initialize(&ev,
        (yaml_char_t *)"anc", NULL,
        (yaml_char_t *)"shared_key", 10, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"val1", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Use alias as the next key */
    yaml_alias_event_initialize(&ev, (yaml_char_t *)"anc");
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"val2", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  alias_simple_key_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_plain_scalar_with_breaks                               */
/* Covers emitter.c:1959 — plain scalar with embedded line breaks     */
/* ------------------------------------------------------------------ */
static void test_emitter_plain_scalar_with_breaks(void)
{
    printf("test_emitter_plain_scalar_with_breaks\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 10);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Multi-word plain scalar that causes line wrapping */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"word1 word2 word3 word4 word5", 29,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Plain scalar with break character inside */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"aa\nbb\ncc", 8,
        1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  plain_breaks_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_single_quoted_breaks                                   */
/* Covers emitter.c:2016-2039 — single-quoted with breaks at end      */
/* ------------------------------------------------------------------ */
static void test_emitter_single_quoted_breaks(void)
{
    printf("test_emitter_single_quoted_breaks\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 10);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Single-quoted with embedded newline (break path) */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"line1\nline2", 11,
        0, 1, YAML_SINGLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Single-quoted ending in newline (exercises breaks at end) */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"ends\nwith\n", 10,
        0, 1, YAML_SINGLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  sq_breaks_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_error_paths                                             */
/* Covers various error/invalid-input scanner/parser paths            */
/* ------------------------------------------------------------------ */
static void test_parser_error_paths(void)
{
    printf("test_parser_error_paths\n");

    /* Various invalid YAML inputs that should produce parse errors */
    const char *inputs[] = {
        /* Invalid directive */
        "%INVALID directive\n---\nfoo\n",
        /* Unclosed flow sequence */
        "[a, b, c",
        /* Tab in plain scalar context */
        "key:\tvalue\n",
        /* Double document indicator */
        "---\n---\nfoo\n",
        /* Bare > in block context (invalid anchor) */
        "&\nfoo: bar\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_token_t tok;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        printf("  input%d_tokens_or_err: %d err: %d\n",
               i, count, (int)parser.error);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_dumper_flow_sequence_mapping                                   */
/* Covers dumper.c:272-390 — dump_sequence, dump_mapping with styles  */
/* ------------------------------------------------------------------ */
static void test_dumper_flow_sequence_mapping(void)
{
    printf("test_dumper_flow_sequence_mapping\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    yaml_emitter_t emitter;
    yaml_document_t doc;

    /* Flow sequence */
    written = 0;
    memset(outbuf, 0, sizeof(outbuf));
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int seq = yaml_document_add_sequence(&doc, NULL, YAML_FLOW_SEQUENCE_STYLE);
    int s1  = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"x", 1, YAML_PLAIN_SCALAR_STYLE);
    int s2  = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"y", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, seq, s1);
    yaml_document_append_sequence_item(&doc, seq, s2);
    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  flow_seq_dump_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);

    /* Flow mapping */
    written = 0;
    memset(outbuf, 0, sizeof(outbuf));
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int map = yaml_document_add_mapping(&doc, NULL, YAML_FLOW_MAPPING_STYLE);
    int k   = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"k", 1, YAML_PLAIN_SCALAR_STYLE);
    int v   = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"v", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, map, k, v);
    r = yaml_emitter_dump(&emitter, &doc);
    printf("  flow_map_dump_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_scalar_with_tag_shorthand                              */
/* Covers emitter analyze_tag with !! shorthand and full URI          */
/* ------------------------------------------------------------------ */
static void test_emitter_scalar_with_tag_shorthand(void)
{
    printf("test_emitter_scalar_with_tag_shorthand\n");

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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* !! shorthand */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!!str",
        (yaml_char_t *)"strval", 6, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* ! local tag */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!local",
        (yaml_char_t *)"localval", 8, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Full URI */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:int",
        (yaml_char_t *)"42", 2, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* URI with percent-encoding needed */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!<tag:example.com,2000:type%20name>",
        (yaml_char_t *)"typed", 5, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  tag_shorthand_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_loader_prestarted_stream                                       */
/* Covers loader.c:97:9:false — stream_start_produced already true    */
/* ------------------------------------------------------------------ */
static void test_loader_prestarted_stream(void)
{
    printf("test_loader_prestarted_stream\n");

    const char *input = "key: value\n";
    yaml_parser_t parser;
    yaml_document_t doc;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* Manually parse STREAM_START so stream_start_produced becomes true */
    yaml_event_t ev;
    yaml_parser_parse(&parser, &ev);
    printf("  manual_start_event: %d\n", ev.type);
    yaml_event_delete(&ev);

    /* Now yaml_parser_load: stream_start_produced=1 → skip parse at line 97 */
    int r = yaml_parser_load(&parser, &doc);
    printf("  load_after_stream_start_ok: %d\n", r);
    if (r) {
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        printf("  root_type: %d\n", root ? root->type : -1);
        yaml_document_delete(&doc);
    }
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_loader_stream_end_reuse                                        */
/* Covers loader.c:103:9:true — stream_end_produced already true      */
/* ------------------------------------------------------------------ */
static void test_loader_stream_end_reuse(void)
{
    printf("test_loader_stream_end_reuse\n");

    const char *input = "a: b\n";
    yaml_parser_t parser;
    yaml_document_t doc;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    /* Load all documents until empty */
    int loaded = 0;
    while (1) {
        if (!yaml_parser_load(&parser, &doc)) {
            printf("  first_load_error: %d\n", (int)parser.error);
            break;
        }
        if (!yaml_document_get_root_node(&doc)) {
            yaml_document_delete(&doc);
            break;
        }
        loaded++;
        yaml_document_delete(&doc);
    }
    printf("  loaded: %d\n", loaded);

    /* Now stream_end_produced should be true.
       Calling load again should return 1 immediately (empty document). */
    int r = yaml_parser_load(&parser, &doc);
    printf("  second_load_ok: %d\n", r);
    yaml_node_t *root = yaml_document_get_root_node(&doc);
    printf("  second_root_null: %d\n", root == NULL ? 1 : 0);
    yaml_document_delete(&doc);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_loader_duplicate_anchor                                        */
/* Covers loader.c:278:13:true — duplicate anchor error path          */
/* ------------------------------------------------------------------ */
static void test_loader_duplicate_anchor(void)
{
    printf("test_loader_duplicate_anchor\n");

    /* Two nodes with the same anchor name — should trigger composer error */
    const char *input = "- &dup_anchor first\n- &dup_anchor second\n";
    yaml_parser_t parser;
    yaml_document_t doc;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int r = yaml_parser_load(&parser, &doc);
    printf("  dup_anchor_result: %d err: %d\n", r, (int)parser.error);
    if (r) yaml_document_delete(&doc);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_loader_undefined_alias                                         */
/* Covers loader.c:366 — undefined alias → composer error             */
/* ------------------------------------------------------------------ */
static void test_loader_undefined_alias(void)
{
    printf("test_loader_undefined_alias\n");

    const char *input = "- *no_such_anchor\n";
    yaml_parser_t parser;
    yaml_document_t doc;

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)input, strlen(input));

    int r = yaml_parser_load(&parser, &doc);
    printf("  undef_alias_result: %d err: %d\n", r, (int)parser.error);
    if (r) yaml_document_delete(&doc);

    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_loader_explicit_tags                                           */
/* Covers loader.c:384:9:false/true — tag exists/not-"!" paths        */
/* Also covers loader.c:429:9/17 and loader.c:497:9/17                */
/* ------------------------------------------------------------------ */
static void test_loader_explicit_tags(void)
{
    printf("test_loader_explicit_tags\n");

    /* Scalar with "!" tag (forces default replacement) */
    const char *bang =  "- ! bar\n";
    yaml_parser_t parser;
    yaml_document_t doc;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)bang, strlen(bang));
    int r = yaml_parser_load(&parser, &doc);
    printf("  bang_tag_ok: %d\n", r);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);

    /* Scalar with explicit !!str tag (non-"!" → keep) */
    const char *str_tag = "!!str explicit_tag\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)str_tag, strlen(str_tag));
    r = yaml_parser_load(&parser, &doc);
    printf("  str_tag_ok: %d\n", r);
    if (r) {
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        printf("  str_root_type: %d\n", root ? root->type : -1);
        yaml_document_delete(&doc);
    }
    yaml_parser_delete(&parser);

    /* Sequence with "!" tag (forces default replacement) */
    const char *seq_bang = "! [a, b]\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)seq_bang, strlen(seq_bang));
    r = yaml_parser_load(&parser, &doc);
    printf("  seq_bang_tag_ok: %d\n", r);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);

    /* Sequence with explicit !!seq tag (non-"!" → keep) */
    const char *seq_tag = "!!seq\n- x\n- y\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)seq_tag, strlen(seq_tag));
    r = yaml_parser_load(&parser, &doc);
    printf("  seq_explicit_tag_ok: %d\n", r);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);

    /* Mapping with explicit !!map tag (non-"!" → keep) */
    const char *map_tag = "!!map\nk1: v1\nk2: v2\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser,
        (const unsigned char *)map_tag, strlen(map_tag));
    r = yaml_parser_load(&parser, &doc);
    printf("  map_explicit_tag_ok: %d\n", r);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_dumper_close_twice                                             */
/* Covers dumper.c:93:4:true — yaml_emitter_close when already closed */
/* ------------------------------------------------------------------ */
static void test_dumper_close_twice(void)
{
    printf("test_dumper_close_twice\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_emitter_open(&emitter);

    int r1 = yaml_emitter_close(&emitter);
    printf("  first_close: %d\n", r1);

    /* Second close: emitter->closed is true → returns 1 immediately */
    int r2 = yaml_emitter_close(&emitter);
    printf("  second_close: %d\n", r2);

    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_dumper_dump_not_opened                                         */
/* Covers dumper.c:122:9:true — yaml_emitter_dump auto-opens emitter  */
/* ------------------------------------------------------------------ */
static void test_dumper_dump_not_opened(void)
{
    printf("test_dumper_dump_not_opened\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    /* NOTE: NOT calling yaml_emitter_open — dump should auto-open */

    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int s = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"auto_opened", 11, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_node: %d\n", s);

    /* yaml_emitter_dump with unopened emitter → auto-opens (dumper.c:122) */
    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  auto_open_dump_ok: %d written: %d\n", r, (int)written);

    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_api_document_node_edge_cases                                   */
/* Covers api.c:1169:22:false — get_node with index=0 or too large,  */
/*        api.c:1184:9:false — get_root on empty document             */
/* ------------------------------------------------------------------ */
static void test_api_document_node_edge_cases(void)
{
    printf("test_api_document_node_edge_cases\n");

    /* Empty document → get_root returns NULL */
    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    yaml_node_t *root = yaml_document_get_root_node(&doc);
    printf("  empty_root_null: %d\n", root == NULL ? 1 : 0);
    yaml_document_delete(&doc);

    /* Document with one node */
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    int s = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"hello", 5, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_idx: %d\n", s);

    /* Index 0 → invalid (1-based) → NULL */
    yaml_node_t *n0 = yaml_document_get_node(&doc, 0);
    printf("  node_at_0_null: %d\n", n0 == NULL ? 1 : 0);

    /* Index beyond top → NULL */
    yaml_node_t *n100 = yaml_document_get_node(&doc, 100);
    printf("  node_at_100_null: %d\n", n100 == NULL ? 1 : 0);

    /* Valid index */
    yaml_node_t *n1 = yaml_document_get_node(&doc, 1);
    printf("  node_at_1_ok: %d\n", n1 != NULL ? 1 : 0);

    yaml_document_delete(&doc);
}

/* ------------------------------------------------------------------ */
/* test_emitter_utf16be                                                */
/* Covers emitter.c UTF-16 BE encoding path                           */
/* ------------------------------------------------------------------ */
static void test_emitter_utf16be(void)
{
    printf("test_emitter_utf16be\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF16BE_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF16BE_ENCODING);
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  utf16be_stream_start: %d\n", r);

    if (r) {
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)"hi", 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_end_event_initialize(&ev, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);
    }
    printf("  utf16be_written_gt0: %d\n", (int)written > 0 ? 1 : 0);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_scalar_leading_trailing                                */
/* Covers emitter.c:1615:17 (trailing_space), emitter.c:1618:17      */
/*        (break_space), emitter.c:1626:17 (leading_break)            */
/*        Various paths in yaml_emitter_analyze_scalar                */
/* ------------------------------------------------------------------ */
static void test_emitter_scalar_leading_trailing(void)
{
    printf("test_emitter_scalar_leading_trailing\n");

    unsigned char outbuf[4096];
    size_t written;
    yaml_emitter_t emitter;
    yaml_event_t ev;

    struct {
        const char *name;
        const char *value;
        int length;
        yaml_scalar_style_t style;
        int plain_implicit;
        int quoted_implicit;
    } cases[] = {
        /* leading space */
        { "leading_space",   " hello",       6,  YAML_SINGLE_QUOTED_SCALAR_STYLE, 0, 1 },
        /* trailing space */
        { "trailing_space",  "hello ",       6,  YAML_SINGLE_QUOTED_SCALAR_STYLE, 0, 1 },
        /* leading break */
        { "leading_break",   "\nhello",      6,  YAML_DOUBLE_QUOTED_SCALAR_STYLE, 0, 1 },
        /* trailing break */
        { "trailing_break",  "hello\n",      6,  YAML_DOUBLE_QUOTED_SCALAR_STYLE, 0, 1 },
        /* break+space: newline followed by space → break_space */
        { "break_space",     "a\n b",        4,  YAML_DOUBLE_QUOTED_SCALAR_STYLE, 0, 1 },
        /* space+break: space followed by newline → space_break */
        { "space_break",     "a \nb",        4,  YAML_DOUBLE_QUOTED_SCALAR_STYLE, 0, 1 },
        /* hash preceded by whitespace → flow+block indicator */
        { "hash_after_space","a #b",         4,  YAML_PLAIN_SCALAR_STYLE,         1, 1 },
        /* colon followed by space → block indicator */
        { "colon_space",     "a: b",         4,  YAML_PLAIN_SCALAR_STYLE,         1, 1 },
        /* dash followed by space → flow+block indicator */
        { "dash_space",      "- val",        5,  YAML_PLAIN_SCALAR_STYLE,         1, 1 },
        /* question followed by space → flow+block indicator */
        { "question_space",  "? key",        5,  YAML_PLAIN_SCALAR_STYLE,         1, 1 },
    };

    for (int i = 0; i < (int)(sizeof(cases)/sizeof(cases[0])); i++) {
        written = 0;
        memset(outbuf, 0, sizeof(outbuf));
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

        yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&emitter, &ev);

        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)cases[i].value, cases[i].length,
            cases[i].plain_implicit, cases[i].quoted_implicit,
            cases[i].style);
        int r = yaml_emitter_emit(&emitter, &ev);

        if (r) {
            yaml_document_end_event_initialize(&ev, 1);
            yaml_emitter_emit(&emitter, &ev);
            yaml_stream_end_event_initialize(&ev);
            yaml_emitter_emit(&emitter, &ev);
        }
        printf("  %s_ok: %d written: %d\n", cases[i].name, r, (int)written);
        yaml_emitter_delete(&emitter);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_scalar_flow_indicators                                 */
/* Covers emitter.c:1582:13 — flow context indicators (not block)     */
/* ------------------------------------------------------------------ */
static void test_emitter_scalar_flow_indicators(void)
{
    printf("test_emitter_scalar_flow_indicators\n");

    unsigned char outbuf[4096];
    size_t written;
    yaml_emitter_t emitter;
    yaml_event_t ev;

    /* Scalars with characters that are flow indicators but not block indicators
       (comma, brackets, braces) — need to be in flow context */
    struct {
        const char *name;
        const char *value;
        int length;
    } cases[] = {
        { "comma",           "a,b",          3 },
        { "open_bracket",    "a[b",          3 },
        { "close_bracket",   "a]b",          3 },
        { "open_brace",      "a{b",          3 },
        { "close_brace",     "a}b",          3 },
        { "colon_nospace",   "a:b",          3 },
    };

    for (int i = 0; i < (int)(sizeof(cases)/sizeof(cases[0])); i++) {
        written = 0;
        memset(outbuf, 0, sizeof(outbuf));
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

        yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&emitter, &ev);

        /* Flow sequence context — exercises flow-specific indicator checks */
        yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
            YAML_FLOW_SEQUENCE_STYLE);
        yaml_emitter_emit(&emitter, &ev);

        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)cases[i].value, cases[i].length,
            1, 1, YAML_PLAIN_SCALAR_STYLE);
        int r = yaml_emitter_emit(&emitter, &ev);

        yaml_sequence_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_end_event_initialize(&ev, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);

        printf("  %s_ok: %d written: %d\n", cases[i].name, r, (int)written);
        yaml_emitter_delete(&emitter);
    }
}

/* ------------------------------------------------------------------ */
/* test_string_join_extend                                             */
/* Covers api.c:103:12 — yaml_string_join extend loop                 */
/* ------------------------------------------------------------------ */
static void test_string_join_extend(void)
{
    printf("test_string_join_extend\n");

    /* Make a small destination buffer and a large source that forces extend */
    size_t a_sz = 8;
    yaml_char_t *a_start = (yaml_char_t *)yaml_malloc(a_sz);
    yaml_char_t *a_ptr   = a_start;
    yaml_char_t *a_end   = a_start + a_sz;
    memset(a_start, 0, a_sz);

    /* Source larger than destination → forces the extend loop */
    const char *src = "hello world this is a long string for testing";
    yaml_char_t *b_start = (yaml_char_t *)src;
    yaml_char_t *b_ptr   = b_start + strlen(src);
    yaml_char_t *b_end   = b_ptr;

    int r = yaml_string_join(&a_start, &a_ptr, &a_end,
                             &b_start, &b_ptr, &b_end);
    printf("  string_join_large_ok: %d\n", r);
    printf("  joined_length: %d\n", (int)(a_ptr - a_start));
    yaml_free(a_start);

    /* Empty source — exercises early return at api.c:100 */
    a_sz    = 16;
    a_start = (yaml_char_t *)yaml_malloc(a_sz);
    a_ptr   = a_start;
    a_end   = a_start + a_sz;
    memset(a_start, 0, a_sz);

    const char *empty = "";
    b_start = (yaml_char_t *)empty;
    b_ptr   = b_start; /* empty */
    b_end   = b_start;

    r = yaml_string_join(&a_start, &a_ptr, &a_end,
                         &b_start, &b_ptr, &b_end);
    printf("  string_join_empty_ok: %d\n", r);
    yaml_free(a_start);
}

/* ------------------------------------------------------------------ */
/* test_loader_sequence_mapping_tags                                   */
/* Covers loader.c:429:9, loader.c:497:9 — seq/map with/without tag  */
/* ------------------------------------------------------------------ */
static void test_loader_sequence_mapping_tags(void)
{
    printf("test_loader_sequence_mapping_tags\n");

    const char *inputs[] = {
        /* Sequence with null tag → uses default */
        "- a\n- b\n",
        /* Mapping with null tag → uses default */
        "a: 1\nb: 2\n",
        /* Sequence with explicit !!seq tag */
        "!!seq\n- x\n- y\n",
        /* Mapping with explicit !!map tag */
        "!!map\nk1: v1\n",
        /* Nested: mapping contains sequence */
        "k:\n  - 1\n  - 2\n",
        /* Sequence of mappings */
        "- k1: v1\n- k2: v2\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_document_t doc;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int r = yaml_parser_load(&parser, &doc);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  input%d_ok: 1 root_type: %d\n", i, root ? root->type : -1);
            yaml_document_delete(&doc);
        } else {
            printf("  input%d_error: %d\n", i, (int)parser.error);
        }
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_parser_block_mapping_various                                   */
/* Covers parser.c branches: explicit block seq/map entries           */
/* ------------------------------------------------------------------ */
static void test_parser_block_mapping_various(void)
{
    printf("test_parser_block_mapping_various\n");

    const char *inputs[] = {
        /* Block seq inside block map as indentless sequence */
        "parent:\n- item1\n- item2\n",
        /* Block mapping with complex key (flow seq) */
        "[1, 2]: value\n",
        /* Tags on anchors */
        "- !!str &a tagged_value\n- *a\n",
        /* Implicit keys: plain scalar with colon */
        "simple: value\n",
        /* Multiple implicit docs */
        "first\n---\nsecond\n",
        /* Sequence with mixed content */
        "- plain\n- 'quoted'\n- |\n  literal\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  input%d_err: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_scalar_with_special_first_last                        */
/* Covers emitter.c:1509-1545 — first/last character checks in       */
/*        yaml_emitter_analyze_scalar                                  */
/* ------------------------------------------------------------------ */
static void test_emitter_scalar_with_special_first_last(void)
{
    printf("test_emitter_scalar_with_special_first_last\n");

    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalars starting/ending with special characters */
    /* Starting with flow indicator: { */
    { yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"{start}", 7, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Starting with block indicator: | */
    { yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"|block", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Starting with > */
    { yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)">fold", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Starting with @ */
    { yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"@social", 7, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Starting with ` */
    { yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"`backtick", 9, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Starting with % */
    { yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"%directive", 10, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  special_first_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_seq_in_block_map_indent                         */
/* Covers emitter.c:1384-1409 emit_block_sequence_item with first=0  */
/* and emitter.c:increase_indent indentless paths                     */
/* ------------------------------------------------------------------ */
static void test_emitter_block_seq_in_block_map_indent(void)
{
    printf("test_emitter_block_seq_in_block_map_indent\n");

    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_indent(&emitter, 4);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Block mapping with nested block sequence (exercises indentless seq) */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"list", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"item_a", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"item_b", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"item_c", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"scalar", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"val", 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  indented_seq_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_folded_scalar_complex                                  */
/* Covers emitter.c:1716-1760 write_folded_scalar complex paths       */
/* (leading_break, trailing break patterns, multiple consecutive NL) */
/* ------------------------------------------------------------------ */
static void test_emitter_folded_scalar_complex(void)
{
    printf("test_emitter_folded_scalar_complex\n");

    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* folded: multiple consecutive newlines between paragraphs */
    { const char v[] = "para1\n\n\npara2\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_FOLDED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* folded: trailing newlines (keep chomping) */
    { const char v[] = "line1\nline2\n\n\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_FOLDED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* folded: starting with blank line */
    { const char v[] = "\nfirst content\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_FOLDED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* literal: leading spaces */
    { const char v[] = "  indented content\n  more content\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_LITERAL_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  folded_complex_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_scanner_flow_scalar_whitespace                                 */
/* Covers scanner.c flow scalar leading/trailing whitespace paths     */
/* ------------------------------------------------------------------ */
static void test_scanner_flow_scalar_whitespace(void)
{
    printf("test_scanner_flow_scalar_whitespace\n");

    const char *inputs[] = {
        /* Single-quoted with leading/trailing whitespace trimming */
        "' leading and trailing '",
        /* Double-quoted with line join (backslash newline) */
        "\"line1\\\n  line2\"",
        /* Double-quoted with multiple escaped chars */
        "\"\\0\\a\\b\\t\\n\\v\\f\\r\\e\\\\\\\"\"",
        /* Double-quoted with \N \_ \L \P escapes */
        "\"\\N\\_\\L\\P\"",
        /* Double-quoted with \\xNN \\uNNNN \\UNNNNNNNN */
        "\"\\x41\\u0041\\U00000041\"",
        /* Single-quoted with '' escape */
        "'can''t stop won''t stop'",
        /* Empty double-quoted */
        "\"\"",
        /* Empty single-quoted */
        "''",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_token_t tok;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        printf("  input%d_tokens: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_analyze_tag_variations                                 */
/* Covers emitter.c analyze_tag with various tag formats              */
/* ------------------------------------------------------------------ */
static void test_emitter_analyze_tag_variations(void)
{
    printf("test_emitter_analyze_tag_variations\n");

    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Various tag formats */
    /* Full YAML type URI (starts with "tag:yaml.org") */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:str",
        (yaml_char_t *)"typed", 5, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* !! shorthand (resolved via default tag directives) */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!!int",
        (yaml_char_t *)"42", 2, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* ! local tag */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!mytype",
        (yaml_char_t *)"local", 5, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* URI tag with verbatim delimiters */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!<http://example.com/tag>",
        (yaml_char_t *)"verbatim", 8, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* tag:yaml.org,2002:bool */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"tag:yaml.org,2002:bool",
        (yaml_char_t *)"true", 4, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  tag_variations_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_reader_utf32                                                   */
/* Covers reader.c UTF-32 encoding detection paths                    */
/* ------------------------------------------------------------------ */
static void test_reader_utf32(void)
{
    printf("test_reader_utf32\n");

    /* UTF-32 LE: BOM 0xFF 0xFE 0x00 0x00 + "a:" as UTF-32 LE */
    static const unsigned char utf32le[] = {
        0xFF, 0xFE, 0x00, 0x00,
        'a',  0x00, 0x00, 0x00,
        ':',  0x00, 0x00, 0x00,
        ' ',  0x00, 0x00, 0x00,
        'b',  0x00, 0x00, 0x00,
        '\n', 0x00, 0x00, 0x00
    };

    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, utf32le, sizeof(utf32le));

    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  utf32le_err: %d\n", (int)parser.error);
            break;
        }
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  utf32le_events: %d\n", count);
    yaml_parser_delete(&parser);

    /* UTF-32 BE: BOM 0x00 0x00 0xFE 0xFF + "a:" as UTF-32 BE */
    static const unsigned char utf32be[] = {
        0x00, 0x00, 0xFE, 0xFF,
        0x00, 0x00, 0x00, 'a',
        0x00, 0x00, 0x00, ':',
        0x00, 0x00, 0x00, ' ',
        0x00, 0x00, 0x00, 'b',
        0x00, 0x00, 0x00, '\n'
    };

    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, utf32be, sizeof(utf32be));

    count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  utf32be_err: %d\n", (int)parser.error);
            break;
        }
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  utf32be_events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_emitter_multiple_documents_mixed                               */
/* Covers emitter open_ended paths across multiple documents          */
/* ------------------------------------------------------------------ */
static void test_emitter_multiple_documents_mixed(void)
{
    printf("test_emitter_multiple_documents_mixed\n");

    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* Doc 1: implicit start, explicit end */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"doc1", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 0); /* explicit */
    yaml_emitter_emit(&emitter, &ev);

    /* Doc 2: explicit start+end */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"doc2", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);

    /* Doc 3: implicit start, implicit end */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
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

    printf("  multi_doc_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_dumper_with_explicit_tags_nodes                                */
/* Covers dumper.c:288-390 dump_scalar with explicit tags, anchors   */
/* ------------------------------------------------------------------ */
static void test_dumper_with_explicit_tags_nodes(void)
{
    printf("test_dumper_with_explicit_tags_nodes\n");

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

    /* Root: sequence */
    int root = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);

    /* Scalar with explicit !!str tag */
    int s1 = yaml_document_add_scalar(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"typed_str", 9,
        YAML_PLAIN_SCALAR_STYLE);

    /* Scalar with explicit !!int tag */
    int s2 = yaml_document_add_scalar(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:int",
        (const yaml_char_t *)"42", 2,
        YAML_PLAIN_SCALAR_STYLE);

    /* Scalar with double-quoted style */
    int s3 = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"double quoted", 13,
        YAML_DOUBLE_QUOTED_SCALAR_STYLE);

    /* Scalar with single-quoted style */
    int s4 = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"single quoted", 13,
        YAML_SINGLE_QUOTED_SCALAR_STYLE);

    /* Scalar with literal style */
    int s5 = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"literal\ncontent\n", 16,
        YAML_LITERAL_SCALAR_STYLE);

    /* Scalar with folded style */
    int s6 = yaml_document_add_scalar(&doc, NULL,
        (const yaml_char_t *)"folded content\n", 15,
        YAML_FOLDED_SCALAR_STYLE);

    yaml_document_append_sequence_item(&doc, root, s1);
    yaml_document_append_sequence_item(&doc, root, s2);
    yaml_document_append_sequence_item(&doc, root, s3);
    yaml_document_append_sequence_item(&doc, root, s4);
    yaml_document_append_sequence_item(&doc, root, s5);
    yaml_document_append_sequence_item(&doc, root, s6);

    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  explicit_tags_dump_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_scan_complex_yaml                                       */
/* Covers more scanner.c branches with complex YAML structures        */
/* ------------------------------------------------------------------ */
static void test_parser_scan_complex_yaml(void)
{
    printf("test_parser_scan_complex_yaml\n");

    const char *inputs[] = {
        /* Tab characters in various positions */
        "key:\tvalue\n",
        /* Nested flow with explicit keys */
        "{? [1,2]: val, ? {a: b}: v2}\n",
        /* Multiple anchors + alias chains */
        "- &x 1\n- &y 2\n- *x\n- *y\n- *x\n",
        /* Null values in mapping */
        "a: ~\nb:\nc: null\n",
        /* Long anchor name */
        "- &very_long_anchor_name_here value\n- *very_long_anchor_name_here\n",
        /* Block scalar with indent indicator + chomping */
        "- |2-\n  indented\n  two spaces\n",
        /* Folded with indent + keep */
        "- >3+\n   keep\n   trailing\n\n",
        /* Double-quoted with hex/unicode escapes */
        "- \"\\x61\\u0062\\U00000063\"\n",
        /* Single-quoted multiline */
        "- 'line1\n  line2'\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_token_t tok;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        printf("  input%d_tokens: %d err: %d\n",
               i, count, (int)parser.error);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_write_double_quoted_multibyte                         */
/* Covers emitter.c:2232-2273 write_double_quoted multibyte paths     */
/* ------------------------------------------------------------------ */
static void test_emitter_write_double_quoted_multibyte(void)
{
    printf("test_emitter_write_double_quoted_multibyte\n");

    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    /* unicode=1 so multibyte chars are output directly */
    yaml_emitter_set_unicode(&emitter, 1);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* 2-byte UTF-8 printable (U+00E9 é) with unicode=1: written directly */
    { static const yaml_char_t v[] = {0xC3, 0xA9, 0x00};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, 2, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* 3-byte UTF-8 (U+4E2D 中) */
    { static const yaml_char_t v[] = {0xE4, 0xB8, 0xAD};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* 4-byte UTF-8 emoji (U+1F600) with unicode=1: written directly */
    { static const yaml_char_t v[] = {0xF0, 0x9F, 0x98, 0x80};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, 4, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Mix ASCII + multibyte */
    { static const yaml_char_t v[] =
        {'A', 0xC3, 0xA9, 'B', 0xE4, 0xB8, 0xAD, 'C'};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, 8, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  multibyte_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_implicit_docs                                           */
/* Covers scanner/parser branches for implicit documents              */
/* ------------------------------------------------------------------ */
static void test_parser_implicit_docs(void)
{
    printf("test_parser_implicit_docs\n");

    /* Scalar not preceded by --- (implicit document) */
    const char *inputs[] = {
        /* Multiple scalars as implicit documents */
        "first\n...\nsecond\n...\nthird\n",
        /* Implicit doc with no "---" at all */
        "plain scalar\n",
        /* Implicit sequence */
        "- a\n- b\n",
        /* Implicit mapping */
        "k1: v1\nk2: v2\n",
        /* Empty stream */
        "",
        /* Only comments */
        "# just a comment\n",
        NULL
    };

    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));

        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  input%d_err: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  input%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_scalar_chomping                                  */
/* Covers emitter.c block scalar hints for various chomping styles    */
/* ------------------------------------------------------------------ */
static void test_emitter_block_scalar_chomping(void)
{
    printf("test_emitter_block_scalar_chomping\n");

    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Literal with no trailing newline (strip chomping hint "-") */
    { const char v[] = "no trailing newline";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_LITERAL_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Literal with multiple trailing newlines (keep chomping hint "+") */
    { const char v[] = "keep trailing\n\n\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_LITERAL_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Literal starting with spaces (requires indent indicator) */
    { const char v[] = "   three spaces\n   indent\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_LITERAL_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Folded with no trailing newline */
    { const char v[] = "folded no trail";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_FOLDED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Folded with multiple trailing newlines */
    { const char v[] = "folded keep\n\n\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_FOLDED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  chomping_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_canonical_sequences_mappings                          */
/* Covers dumper.c:351-390 canonical dump_sequence / dump_mapping    */
/* ------------------------------------------------------------------ */
static void test_emitter_canonical_sequences_mappings(void)
{
    printf("test_emitter_canonical_sequences_mappings\n");

    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_canonical(&emitter, 1);
    yaml_emitter_open(&emitter);

    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 0, 0);

    /* Root: sequence (block style; canonical forces flow) */
    int root = yaml_document_add_sequence(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:seq",
        YAML_BLOCK_SEQUENCE_STYLE);

    /* Nested mapping as item */
    int nm = yaml_document_add_mapping(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:map",
        YAML_BLOCK_MAPPING_STYLE);
    int k1 = yaml_document_add_scalar(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"key1", 4, YAML_PLAIN_SCALAR_STYLE);
    int v1 = yaml_document_add_scalar(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"val1", 4, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_mapping_pair(&doc, nm, k1, v1);
    yaml_document_append_sequence_item(&doc, root, nm);

    /* Nested sequence as item */
    int ns = yaml_document_add_sequence(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:seq",
        YAML_BLOCK_SEQUENCE_STYLE);
    int s1 = yaml_document_add_scalar(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"a", 1, YAML_PLAIN_SCALAR_STYLE);
    int s2 = yaml_document_add_scalar(&doc,
        (const yaml_char_t *)"tag:yaml.org,2002:str",
        (const yaml_char_t *)"b", 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_document_append_sequence_item(&doc, ns, s1);
    yaml_document_append_sequence_item(&doc, ns, s2);
    yaml_document_append_sequence_item(&doc, root, ns);

    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  canonical_seq_map_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_api_invalid_utf8_events                                        */
/* Covers api.c:808:9:true, 838:13:true, 844:13:true, 853:9:true,    */
/*         888:13:true, 894:13:true, 943:13:true, 949:13:true         */
/* ------------------------------------------------------------------ */
static void test_api_invalid_utf8_events(void)
{
    printf("test_api_invalid_utf8_events\n");
    yaml_event_t ev;
    int r;
    const yaml_char_t *bad = (const yaml_char_t *)"\xff\xfe";

    /* alias: invalid UTF-8 anchor */
    r = yaml_alias_event_initialize(&ev, bad);
    printf("  alias_bad_anchor: %d\n", r);

    /* scalar: invalid UTF-8 anchor */
    r = yaml_scalar_event_initialize(&ev,
        bad, NULL,
        (const yaml_char_t *)"v", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_bad_anchor: %d\n", r);

    /* scalar: invalid UTF-8 tag */
    r = yaml_scalar_event_initialize(&ev,
        NULL, bad,
        (const yaml_char_t *)"v", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_bad_tag: %d\n", r);

    /* scalar: invalid UTF-8 value */
    r = yaml_scalar_event_initialize(&ev,
        NULL, NULL, bad, 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_bad_value: %d\n", r);

    /* sequence_start: invalid UTF-8 anchor */
    r = yaml_sequence_start_event_initialize(&ev,
        bad, NULL, 1, YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_bad_anchor: %d\n", r);

    /* sequence_start: invalid UTF-8 tag */
    r = yaml_sequence_start_event_initialize(&ev,
        NULL, bad, 1, YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_bad_tag: %d\n", r);

    /* mapping_start: invalid UTF-8 anchor */
    r = yaml_mapping_start_event_initialize(&ev,
        bad, NULL, 1, YAML_BLOCK_MAPPING_STYLE);
    printf("  map_bad_anchor: %d\n", r);

    /* mapping_start: invalid UTF-8 tag */
    r = yaml_mapping_start_event_initialize(&ev,
        NULL, bad, 1, YAML_BLOCK_MAPPING_STYLE);
    printf("  map_bad_tag: %d\n", r);
}

/* ------------------------------------------------------------------ */
/* test_api_check_utf8_overlong                                        */
/* Covers api.c:647:13:true, 650:17:true, 654-656 false paths        */
/* ------------------------------------------------------------------ */
static void test_api_check_utf8_overlong(void)
{
    printf("test_api_check_utf8_overlong\n");
    yaml_event_t ev;
    int r;

    /* Truncated 2-byte sequence (only 1 byte) — 647:13:true */
    { const yaml_char_t v[] = {0xC2};
      r = yaml_scalar_event_initialize(&ev, NULL, NULL,
          v, 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      printf("  trunc2: %d\n", r); }

    /* Invalid continuation byte (0x00 is not 10xxxxxx) — 650:17:true */
    { const yaml_char_t v[] = {0xC2, 0x00};
      r = yaml_scalar_event_initialize(&ev, NULL, NULL,
          v, 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      printf("  bad_cont: %d\n", r); }

    /* Overlong 2-byte of 0x00: \xC0\x80 — 655:28:false (value=0 < 0x80) */
    { const yaml_char_t v[] = {0xC0, 0x80};
      r = yaml_scalar_event_initialize(&ev, NULL, NULL,
          v, 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      printf("  overlong2: %d\n", r); }

    /* Overlong 3-byte of 'A': \xE0\x81\x81 — 656:14:false (value<0x800) */
    { const yaml_char_t v[] = {0xE0, 0x81, 0x81};
      r = yaml_scalar_event_initialize(&ev, NULL, NULL,
          v, 3, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      printf("  overlong3: %d\n", r); }

    /* Overlong 4-byte: \xF0\x80\x84\x80 — 656:28:false (value<0x10000) */
    { const yaml_char_t v[] = {0xF0, 0x80, 0x84, 0x80};
      r = yaml_scalar_event_initialize(&ev, NULL, NULL,
          v, 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      printf("  overlong4: %d\n", r); }

    /* Truncated 3-byte: \xE0\xA0 (only 2 of 3 bytes) */
    { const yaml_char_t v[] = {0xE0, 0xA0};
      r = yaml_scalar_event_initialize(&ev, NULL, NULL,
          v, 2, 1, 1, YAML_PLAIN_SCALAR_STYLE);
      printf("  trunc3: %d\n", r); }
}

/* ------------------------------------------------------------------ */
/* test_large_input_string_reader                                      */
/* Covers api.c:259:9:false — string_read_handler size <= remaining   */
/* ------------------------------------------------------------------ */
static void test_large_input_string_reader(void)
{
    printf("test_large_input_string_reader\n");
    /* Input > INPUT_RAW_BUFFER_SIZE (~16384 bytes) so the string read
       handler is called with size <= remaining bytes at first call. */
    const int N = 260;
    size_t buf_size = (size_t)N * 80 + 64;
    char *buf = (char *)yaml_malloc(buf_size);
    if (!buf) { printf("  skip_oom: 1\n"); return; }
    size_t pos = 0;
    for (int i = 0; i < N; i++) {
        pos += (size_t)snprintf(buf + pos, buf_size - pos,
            "- item_%05d_pad_0123456789abcdefghijk_%04d\n", i, i);
    }
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)buf, pos);
    int count = 0;
    yaml_event_t ev;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  large_err: %d\n", (int)parser.error);
            break;
        }
        count++;
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  large_events: %d\n", count);
    yaml_parser_delete(&parser);
    yaml_free(buf);
}

/* ------------------------------------------------------------------ */
/* test_parser_utf16be_input                                          */
/* Covers reader.c UTF-16BE decoding paths                           */
/* ------------------------------------------------------------------ */
static void test_parser_utf16be_input(void)
{
    printf("test_parser_utf16be_input\n");
    /* UTF-16BE BOM + "a: b\n" */
    static const unsigned char utf16be[] = {
        0xFE, 0xFF,             /* BOM: UTF-16BE */
        0x00, 0x61,             /* 'a' */
        0x00, 0x3A,             /* ':' */
        0x00, 0x20,             /* ' ' */
        0x00, 0x62,             /* 'b' */
        0x00, 0x0A              /* '\n' */
    };
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, utf16be, sizeof(utf16be));
    yaml_event_t ev;
    int count = 0;
    while (1) {
        if (!yaml_parser_parse(&parser, &ev)) {
            printf("  utf16be_err: %d\n", (int)parser.error);
            break;
        }
        count++;
        if (ev.type == YAML_SCALAR_EVENT)
            printf("  utf16be_scalar: %s\n", ev.data.scalar.value
                   ? (const char *)ev.data.scalar.value : "");
        int done = (ev.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&ev);
        if (done) break;
    }
    printf("  utf16be_events: %d\n", count);
    yaml_parser_delete(&parser);
}

/* ------------------------------------------------------------------ */
/* test_scanner_flow_scalar_unicode_escapes                            */
/* Covers scanner.c \u, \U, \N, \_, \L, \P escape branches           */
/* ------------------------------------------------------------------ */
static void test_scanner_flow_scalar_unicode_escapes(void)
{
    printf("test_scanner_flow_scalar_unicode_escapes\n");
    const char *inputs[] = {
        "- \"\\u0041\"\n",           /* \u0041 = 'A' */
        "- \"\\U00000042\"\n",       /* \U00000042 = 'B' */
        "- \"\\N\"\n",               /* NEL U+0085 */
        "- \"\\_\"\n",               /* NBSP U+00A0 */
        "- \"\\L\"\n",               /* LS U+2028 */
        "- \"\\P\"\n",               /* PS U+2029 */
        "- \"\\u0048\\u0065\\u006C\\u006C\\u006F\"\n", /* Hello */
        "- \"\\t\\n\\r\"\n",         /* \t \n \r */
        "- \"\\a\\b\\f\\v\"\n",      /* \a \b \f \v */
        "- \"\\e\"\n",               /* ESC */
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        int ok = 1;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        if (parser.error) ok = 0;
        printf("  esc%d_ok: %d tokens: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_scanner_block_scalar_edge_cases                                */
/* Covers scanner.c block scalar with tabs, empty lines, variants     */
/* ------------------------------------------------------------------ */
static void test_scanner_block_scalar_edge_cases(void)
{
    printf("test_scanner_block_scalar_edge_cases\n");
    const char *inputs[] = {
        "- |\n\n  content\n",             /* literal with leading empty line */
        "- |\n  line\twith\ttab\n",       /* literal with tab */
        "- >+\n  content\n\n\n",          /* folded with trailing (keep) */
        "- |1\n content_indent1\n",       /* literal explicit indent 1 */
        "|\n  standalone literal\n",      /* literal at doc root */
        "- >\n\n\n  content\n",           /* folded blank lines before */
        "- |-\n  strip\n\n",              /* literal strip */
        "- |2\n  two_spaces\n  more\n",   /* explicit indent 2 */
        "- >1\n one_space\n more\n",      /* folded explicit indent 1 */
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        int ok = 1;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        if (parser.error) ok = 0;
        printf("  blk%d_ok: %d tokens: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_scanner_tag_in_flow                                            */
/* Covers scanner.c:2478:36 tag+comma in flow, verbatim tag paths    */
/* ------------------------------------------------------------------ */
static void test_scanner_tag_in_flow(void)
{
    printf("test_scanner_tag_in_flow\n");
    const char *inputs[] = {
        "{!!str foo, !!int 42}\n",                          /* tag+comma */
        "- !<tag:example.com,2000:type> val\n",             /* verbatim */
        "- !<tag:example.com,2000:my%20type> val\n",        /* verbatim %enc */
        "[!foo bar, !baz qux]\n",                           /* local tags */
        "%TAG ! tag:example.com,2000:\n---\n- !foo bar\n",  /* TAG directive */
        "{!ns!type val, !ns!other more}\n",                 /* handle tags */
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        int ok = 1;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        if (parser.error) ok = 0;
        printf("  tag_flow%d_ok: %d tokens: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_scanner_plain_scalar_multiline                                 */
/* Covers scanner.c plain scalar multi-line, indent, break paths      */
/* ------------------------------------------------------------------ */
static void test_scanner_plain_scalar_multiline(void)
{
    printf("test_scanner_plain_scalar_multiline\n");
    const char *inputs[] = {
        "- first line\n  continuation\n",
        "key: first line\n  second line\n",
        "key:\n  value line one\n  value line two\n",
        "{key: plain value in flow}\n",
        "- plain  # comment after\n",
        "a: long plain scalar that wraps\n  across multiple lines here\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  plain%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_scanner_complex_anchors_aliases                                */
/* Covers scanner.c anchor/alias in various positions                 */
/* ------------------------------------------------------------------ */
static void test_scanner_complex_anchors_aliases(void)
{
    printf("test_scanner_complex_anchors_aliases\n");
    const char *inputs[] = {
        "{&anc key: &val value}\n",
        "a: &x foo\nb: *x\nc: *x\n",
        "seq: &myseq\n  - item1\n  - item2\n",
        "- &anc [1, 2, 3]\n- *anc\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        int ok = 1;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        if (parser.error) ok = 0;
        printf("  anchor%d_ok: %d tokens: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_double_quoted_multiline_wrap                           */
/* Covers emitter.c:2175-2179 double-quoted line-break paths          */
/* ------------------------------------------------------------------ */
static void test_emitter_double_quoted_multiline_wrap(void)
{
    printf("test_emitter_double_quoted_multiline_wrap\n");
    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 20);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Double-quoted with embedded newline */
    { const char v[] = "word1 word2\nword3 word4";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1,
        YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Double-quoted with NEL + LS */
    { const yaml_char_t v[] = {0xC2,0x85,0x20,0xE2,0x80,0xA8,0};
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, 6, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Double-quoted long string that hits width boundary */
    { const char v[] = "a b c d e f g h i j k l m n o p q r s t";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1,
        YAML_DOUBLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Single-quoted with newline */
    { const char v[] = "line1\nline2\nline3";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1,
        YAML_SINGLE_QUOTED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  dq_wrap_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_seq_in_flow                                      */
/* Covers emitter.c flow mapping + sequence nested states             */
/* ------------------------------------------------------------------ */
static void test_emitter_block_seq_in_flow(void)
{
    printf("test_emitter_block_seq_in_flow\n");
    unsigned char outbuf[8192];
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

    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"list", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"a", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"b", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    printf("  flow_map_seq_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_block_mapping_various2                                  */
/* Covers parser.c block mapping state transitions                    */
/* ------------------------------------------------------------------ */
static void test_parser_block_mapping_various2(void)
{
    printf("test_parser_block_mapping_various2\n");
    const char *inputs[] = {
        "? [complex, key]\n: value\n",
        "? seq_key\n: - v1\n  - v2\n",
        "outer:\n  inner:\n    deep: value\n",
        "k1:\nk2:\nk3: val\n",
        "---\nfoo: bar\nbaz: qux\n...\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  bm2_%d_err: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  bm2_%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_dumper_doc_with_version_tag_dirs                               */
/* Covers dumper.c:72-149 dump with version_directive and tag dirs    */
/* ------------------------------------------------------------------ */
static void test_dumper_doc_with_version_tag_dirs(void)
{
    printf("test_dumper_doc_with_version_tag_dirs\n");
    unsigned char outbuf[8192];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_open(&emitter);

    yaml_version_directive_t vd = {1, 1};
    yaml_tag_directive_t tds[1];
    tds[0].handle = (yaml_char_t *)"!";
    tds[0].prefix = (yaml_char_t *)"!myns!";

    yaml_document_t doc;
    yaml_document_initialize(&doc, &vd, tds, tds+1, 0, 0);
    int s = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"versioned_doc_value", 19, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_idx: %d\n", s);

    int r = yaml_emitter_dump(&emitter, &doc);
    printf("  dump_version_tag_ok: %d written: %d\n", r, (int)written);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_write_tag_content_uri                                  */
/* Covers emitter.c write_tag_content with URI-encoded content        */
/* ------------------------------------------------------------------ */
static void test_emitter_write_tag_content_uri(void)
{
    printf("test_emitter_write_tag_content_uri\n");
    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Verbatim tag */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!<tag:example.com,2000:foo>",
        (yaml_char_t *)"verbatim_tag_val", 16, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Tag with non-ASCII (percent-encode path) */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!<tag:example.com,2000:\xC3\xA9>",
        (yaml_char_t *)"encoded_uri_tag", 15, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Local tag with shorthand */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!mytype",
        (yaml_char_t *)"local_tag_val", 13, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  tag_uri_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_emitter_block_scalar_with_indent_indicator                     */
/* Covers emitter.c:1285-1301 block_scalar_hints indent indicator     */
/* ------------------------------------------------------------------ */
static void test_emitter_block_scalar_with_indent_indicator(void)
{
    printf("test_emitter_block_scalar_with_indent_indicator\n");
    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Block scalar starting with 2 spaces (indent indicator needed) */
    { const char v[] = "  two spaces at start\n  more\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_LITERAL_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Block scalar starting with 1 space */
    { const char v[] = " one space\n more\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_LITERAL_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    /* Folded starting with space */
    { const char v[] = "  space-leading folded\n  continuation\n\n";
      yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)v, (int)strlen(v), 0, 1, YAML_FOLDED_SCALAR_STYLE);
      yaml_emitter_emit(&emitter, &ev); }

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  indent_hint_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_flow_pairs                                              */
/* Covers parser.c parse_flow_pair and related branches               */
/* ------------------------------------------------------------------ */
static void test_parser_flow_pairs(void)
{
    printf("test_parser_flow_pairs\n");
    const char *inputs[] = {
        "[? k1, ? k2 : v2]\n",
        "? key: value\n",
        "{a: {b: {c: d}}}\n",
        "[{a: [1, 2]}, {b: [3, 4]}]\n",
        "{k1:, k2: v2}\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  fp%d_err: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  fp%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_loader_anchor_tag_combinations                                  */
/* Covers loader.c:315-340 load_scalar/seq/map with tags              */
/* ------------------------------------------------------------------ */
static void test_loader_anchor_tag_combinations(void)
{
    printf("test_loader_anchor_tag_combinations\n");
    const char *inputs[] = {
        "!!seq &myseq [a, b, c]\n",
        "!!map {k: v}\n",
        "!!int 42\n",
        "- &as !!seq [x, y]\n- *as\n",
        "root:\n  seq: !!seq [1, 2]\n  map: !!map {a: b}\n  str: !!str hi\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_document_t doc;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int r = yaml_parser_load(&parser, &doc);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  atc%d_ok: %d root: %d\n", i, r,
                   root ? root->type : -1);
            yaml_document_delete(&doc);
        } else {
            printf("  atc%d_err: %d\n", i, (int)parser.error);
        }
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_scanner_directive_variants                                      */
/* Covers scanner.c:827, 875, 880 directive version/name parsing      */
/* ------------------------------------------------------------------ */
static void test_scanner_directive_variants(void)
{
    printf("test_scanner_directive_variants\n");
    const char *inputs[] = {
        "%YAML 1.0\n---\nfoo\n",
        "%YAML 2.0\n---\nfoo\n",
        "%UNKNOWN foo bar\n---\nfoo\n",
        "%TAG !! tag:yaml.org,2002:\n---\n!!str hello\n",
        "%TAG ! tag:example.com,2000:\n---\n!foo bar\n",
        "%TAG !e! tag:example.com,2000:\n%TAG !y! tag:yaml.org,2002:\n---\nfoo\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        printf("  dir%d_tokens: %d err: %d\n", i, count, (int)parser.error);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_select_style_variants                                   */
/* Covers emitter.c select_scalar_style ANY/LITERAL/FOLDED paths      */
/* ------------------------------------------------------------------ */
static void test_emitter_select_style_variants(void)
{
    printf("test_emitter_select_style_variants\n");
    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* ANY style on plain-compatible */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"plain_ok", 8, 1, 0, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* ANY style on content needing quotes */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"key: value", 10, 0, 0, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* ANY on content needing block */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"line1\nline2\nline3\n", 18, 0, 0, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* LITERAL with no newline → fallback */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"no newline", 10, 0, 1, YAML_LITERAL_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* FOLDED scalar */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"folded\nscalar\n", 14, 0, 1, YAML_FOLDED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  select_style_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_node_with_anchor_tag                                    */
/* Covers parser.c:583-651 parse_node with anchor/tag combos          */
/* ------------------------------------------------------------------ */
static void test_parser_node_with_anchor_tag(void)
{
    printf("test_parser_node_with_anchor_tag\n");
    const char *inputs[] = {
        "- &anc !!str tagged_and_anchored\n",
        "- &anc !!int 42\n- *anc\n",
        "- &seq_anc !!seq [a, b]\n",
        "- !!float 3.14\n",
        "base: &bm {k: v}\nother: *bm\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  nat%d_err: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  nat%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_scanner_flow_scalar_line_break                                  */
/* Covers scanner.c:1938-1970 flow scalar with line-break handling    */
/* ------------------------------------------------------------------ */
static void test_scanner_flow_scalar_line_break(void)
{
    printf("test_scanner_flow_scalar_line_break\n");
    const char *inputs[] = {
        "- 'first\nsecond'\n",
        "- \"first\\\n  second\"\n",
        "- \"first\\nsecond\"\n",
        "- 'first\n\nsecond'\n",
        "- \"line1\n  line2\n  line3\"\n",
        "- 'a''b''c'\n",             /* single-quote escape */
        "- 'trailing spaces  '\n",   /* trailing spaces in single-quoted */
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        int ok = 1;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        if (parser.error) ok = 0;
        printf("  fsb%d_ok: %d tokens: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_state_machine_sequence_mapping                         */
/* Covers emitter.c:337-406 emit state machine for seq/map paths      */
/* ------------------------------------------------------------------ */
static void test_emitter_state_machine_sequence_mapping(void)
{
    printf("test_emitter_state_machine_sequence_mapping\n");
    unsigned char outbuf[8192];
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

    /* block mapping with anchored flow sequence and nested block seq */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"flows", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev,
        (yaml_char_t *)"seq_anchor", NULL, 1,
        YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"x", 1, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"blocks", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* flow mapping as block seq item */
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

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  sm_seq_map_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_parser_stream_end_edge                                         */
/* Covers parser.c:193-194 process_directives with edge cases         */
/* ------------------------------------------------------------------ */
static void test_parser_stream_end_edge(void)
{
    printf("test_parser_stream_end_edge\n");
    const char *inputs[] = {
        "",
        "---\n...\n",
        "---\na: 1\n...\n---\nb: 2\n...\n",
        "---\n",
        "...\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_event_t ev;
        while (1) {
            if (!yaml_parser_parse(&parser, &ev)) {
                printf("  se%d_err: %d\n", i, (int)parser.error);
                break;
            }
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  se%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* test_emitter_analyze_scalar_variants                                 */
/* Covers emitter.c analyze_scalar for various character classes       */
/* ------------------------------------------------------------------ */
static void test_emitter_analyze_scalar_variants(void)
{
    printf("test_emitter_analyze_scalar_variants\n");
    unsigned char outbuf[8192];
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

    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1,
        YAML_BLOCK_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar with flow indicators → not plain in flow */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"val,with,commas", 15, 1, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar starting with block indicator */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"|block_indicator", 16, 0, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar with : followed by space */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"key: value", 10, 0, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar with # (comment indicator) */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"val #comment", 12, 0, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar with leading space */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)" leading space", 14, 0, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar with trailing space */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"trailing space ", 15, 0, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* Scalar: "null" / "true" / "false" keywords */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"null", 4, 0, 1, YAML_ANY_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    printf("  analyze_scalar_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* ------------------------------------------------------------------ */
/* test_api_document_delete_node_types                                 */
/* Covers api.c:1133 — document_delete with all node types           */
/* ------------------------------------------------------------------ */
static void test_api_document_delete_node_types(void)
{
    printf("test_api_document_delete_node_types\n");

    yaml_document_t doc;
    yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);

    /* Add scalar, sequence, mapping — so delete exercises all 3 branches */
    int s  = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"scalar_val", 10, YAML_PLAIN_SCALAR_STYLE);
    int sq = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    int mp = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);

    yaml_document_append_sequence_item(&doc, sq, s);
    yaml_document_append_mapping_pair(&doc, mp, s, sq);

    printf("  scalar: %d seq: %d map: %d\n", s, sq, mp);

    yaml_document_delete(&doc);
    printf("  delete_all_types_ok: 1\n");
}

/* ------------------------------------------------------------------ */
/* test_scanner_complex_yaml_inputs                                     */
/* Covers various remaining scanner branches                           */
/* ------------------------------------------------------------------ */
static void test_scanner_complex_yaml_inputs(void)
{
    printf("test_scanner_complex_yaml_inputs\n");
    const char *inputs[] = {
        /* Multiline plain scalar with various continuations */
        "- long\n  continuation\n  more\n",
        /* Flow sequence with nested flow mapping */
        "[{k: v}, a, {m: n}]\n",
        /* Block seq with mixed content */
        "- a\n- b: c\n  d: e\n- [x, y]\n",
        /* Anchor on null value */
        "key: &anchor ~\nother: *anchor\n",
        /* Nested anchors */
        "a: &x\n  b: &y 1\nc: *x\nd: *y\n",
        /* Tabs before value */
        "key:\t value_after_tab\n",
        /* Flow mapping with complex values */
        "{a: [1,2,3], b: {c: d}}\n",
        /* Document with multiple keys having complex values */
        "a:\n  - 1\n  - 2\nb:\n  c: 3\n  d: 4\n",
        NULL
    };
    for (int i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int count = 0;
        yaml_token_t tok;
        int ok = 1;
        while (yaml_parser_scan(&parser, &tok)) {
            count++;
            int done = (tok.type == YAML_STREAM_END_TOKEN);
            yaml_token_delete(&tok);
            if (done) break;
        }
        if (parser.error) ok = 0;
        printf("  complex%d_ok: %d tokens: %d\n", i, ok, count);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* Round-5 coverage-guided tests                                       */
/* ------------------------------------------------------------------ */

/* Helper: emit a minimal document with one scalar */
static void emit_simple_scalar_doc(yaml_emitter_t *emitter, const char *val,
                                   yaml_scalar_style_t style)
{
    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)val, (int)strlen(val), 1, 1, style);
    yaml_emitter_emit(emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(emitter, &ev);
}

/* Test: invalid UTF-8 passed to various event init functions */
static void test_api_event_init_invalid_utf8(void)
{
    printf("test_api_event_init_invalid_utf8\n");
    yaml_event_t ev;

    /* bad value: invalid UTF-8 bytes */
    const yaml_char_t *bad = (const yaml_char_t *)"\x80\x81";
    int r = yaml_scalar_event_initialize(&ev, NULL, NULL, bad, 2,
                                         1, 0, YAML_PLAIN_SCALAR_STYLE);
    printf("  bad_value_scalar: %d\n", r);

    /* bad anchor: invalid UTF-8 */
    r = yaml_scalar_event_initialize(&ev, bad, NULL,
        (const yaml_char_t *)"ok", 2, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    printf("  bad_anchor_scalar: %d\n", r);

    /* bad tag: invalid UTF-8 */
    r = yaml_scalar_event_initialize(&ev, NULL, bad,
        (const yaml_char_t *)"ok", 2, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    printf("  bad_tag_scalar: %d\n", r);

    /* bad anchor in alias event */
    r = yaml_alias_event_initialize(&ev, bad);
    printf("  bad_anchor_alias: %d\n", r);

    /* bad anchor in sequence_start */
    r = yaml_sequence_start_event_initialize(&ev, bad, NULL, 1,
                                              YAML_BLOCK_SEQUENCE_STYLE);
    printf("  bad_anchor_seq: %d\n", r);

    /* bad tag in sequence_start */
    r = yaml_sequence_start_event_initialize(&ev, NULL, bad, 1,
                                              YAML_BLOCK_SEQUENCE_STYLE);
    printf("  bad_tag_seq: %d\n", r);

    /* bad anchor in mapping_start */
    r = yaml_mapping_start_event_initialize(&ev, bad, NULL, 1,
                                             YAML_BLOCK_MAPPING_STYLE);
    printf("  bad_anchor_map: %d\n", r);

    /* bad tag in mapping_start */
    r = yaml_mapping_start_event_initialize(&ev, NULL, bad, 1,
                                             YAML_BLOCK_MAPPING_STYLE);
    printf("  bad_tag_map: %d\n", r);

    /* length=-1 with valid UTF-8 */
    r = yaml_scalar_event_initialize(&ev, NULL, NULL,
        (const yaml_char_t *)"hello", -1, 1, 0, YAML_PLAIN_SCALAR_STYLE);
    printf("  neg_len_scalar: %d\n", r);
    if (r) yaml_event_delete(&ev);
}

/* Test: emit with YAML 1.1 version directive */
static void test_emitter_yaml11_version_directive(void)
{
    printf("test_emitter_yaml11_version_directive\n");
    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_event_t ev;
    yaml_version_directive_t ver = { 1, 1 };

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);

    /* document with YAML 1.1 version directive */
    yaml_document_start_event_initialize(&ev, &ver, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"yaml11", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);

    /* second document with YAML 1.2 version directive */
    yaml_version_directive_t ver12 = { 1, 2 };
    yaml_document_start_event_initialize(&ev, &ver12, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"yaml12", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);

    printf("  written: %d\n", (int)written);
    outbuf[written] = '\0';
    printf("  has_yaml: %d\n", (strstr((char *)outbuf, "YAML") != NULL));
    printf("  has_11: %d\n", (strstr((char *)outbuf, "1.1") != NULL));
    printf("  has_12: %d\n", (strstr((char *)outbuf, "1.2") != NULL));

    yaml_emitter_delete(&emitter);
}

/* Test: block scalar chomp modes (strip, clip, keep) */
static void test_emitter_block_scalar_chomp_variants(void)
{
    printf("test_emitter_block_scalar_chomp_variants\n");
    unsigned char outbuf[4096];
    size_t written;

    /* strip: no trailing newline */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter, "line1\nline2", YAML_LITERAL_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  strip_has_minus: %d\n", (strchr((char *)outbuf, '-') != NULL));
    }

    /* keep: multiple trailing newlines */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter, "line1\nline2\n\n", YAML_LITERAL_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  keep_has_plus: %d\n", (strchr((char *)outbuf, '+') != NULL));
    }

    /* block scalar starting with space -> indent hint */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter, " leading space\n", YAML_LITERAL_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  indent_hint_written: %d\n", (int)written > 0);
    }

    /* folded scalar with multiple lines */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter,
            "line1 line2\nline3 line4\n\nline5\n",
            YAML_FOLDED_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  folded_written: %d\n", (int)written > 0);
    }
}

/* Test: double-quoted scalar with all special escape sequences */
static void test_emitter_double_quoted_special_escapes(void)
{
    printf("test_emitter_double_quoted_special_escapes\n");
    unsigned char outbuf[4096];
    size_t written;

    /* control chars that map to named escapes */
    const char *controls[] = {
        "\x00",       /* \0 */
        "\x07",       /* \a */
        "\x08",       /* \b */
        "\x0B",       /* \v */
        "\x0C",       /* \f */
        "\x0D",       /* \r */
        "\x1B",       /* \e */
        "\xC2\x85",   /* U+0085 NEL -> \N */
        "\xC2\xA0",   /* U+00A0 NBSP -> \_ */
        NULL
    };
    int i;
    for (i = 0; controls[i]; i++) {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

        yaml_event_t ev;
        yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&emitter, &ev);
        int vlen = (controls[i][0] == '\xC2') ? 2 : 1;
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)controls[i], vlen, 0, 1,
            YAML_DOUBLE_QUOTED_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_end_event_initialize(&ev, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);
        yaml_emitter_flush(&emitter);
        outbuf[written] = '\0';
        printf("  escape%d_written: %d\n", i, (int)written > 0);
        yaml_emitter_delete(&emitter);
    }

    /* Unicode line/paragraph separators U+2028, U+2029 */
    const char *uni_seps[] = { "\xE2\x80\xA8", "\xE2\x80\xA9", NULL };
    for (i = 0; uni_seps[i]; i++) {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
        yaml_event_t ev;
        yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_scalar_event_initialize(&ev, NULL, NULL,
            (yaml_char_t *)uni_seps[i], 3, 0, 1, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
        yaml_document_end_event_initialize(&ev, 1);
        yaml_emitter_emit(&emitter, &ev);
        yaml_stream_end_event_initialize(&ev);
        yaml_emitter_emit(&emitter, &ev);
        yaml_emitter_flush(&emitter);
        outbuf[written] = '\0';
        printf("  unisep%d_written: %d\n", i, (int)written > 0);
        yaml_emitter_delete(&emitter);
    }
}

/* Test: load flow sequences and flow mappings via yaml_parser_load */
static void test_loader_flow_seq_and_map(void)
{
    printf("test_loader_flow_seq_and_map\n");
    yaml_parser_t parser;
    yaml_document_t doc;

    /* flow sequence */
    {
        const char *input = "[a, b, c]\n";
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                     strlen(input));
        int r = yaml_parser_load(&parser, &doc);
        printf("  flow_seq_load: %d\n", r);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  flow_seq_type: %d\n", root ? root->type : -1);
            if (root && root->type == YAML_SEQUENCE_NODE) {
                int n = (int)(root->data.sequence.items.top -
                              root->data.sequence.items.start);
                printf("  flow_seq_items: %d\n", n);
            }
            yaml_document_delete(&doc);
        }
        yaml_parser_delete(&parser);
    }

    /* flow mapping */
    {
        const char *input = "{key: value, x: 1}\n";
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                     strlen(input));
        int r = yaml_parser_load(&parser, &doc);
        printf("  flow_map_load: %d\n", r);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  flow_map_type: %d\n", root ? root->type : -1);
            if (root && root->type == YAML_MAPPING_NODE) {
                int n = (int)(root->data.mapping.pairs.top -
                              root->data.mapping.pairs.start);
                printf("  flow_map_pairs: %d\n", n);
            }
            yaml_document_delete(&doc);
        }
        yaml_parser_delete(&parser);
    }

    /* flow sequence with explicit tag on seq */
    {
        const char *input = "!!seq [a, b]\n";
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                     strlen(input));
        int r = yaml_parser_load(&parser, &doc);
        printf("  tagged_seq_load: %d\n", r);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  tagged_seq_type: %d\n", root ? root->type : -1);
            yaml_document_delete(&doc);
        }
        yaml_parser_delete(&parser);
    }

    /* flow mapping with explicit tag */
    {
        const char *input = "!!map {k: v}\n";
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                     strlen(input));
        int r = yaml_parser_load(&parser, &doc);
        printf("  tagged_map_load: %d\n", r);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            printf("  tagged_map_type: %d\n", root ? root->type : -1);
            yaml_document_delete(&doc);
        }
        yaml_parser_delete(&parser);
    }
}

/* Test: parser with indentless sequence entry */
static void test_parser_indentless_sequence_entry(void)
{
    printf("test_parser_indentless_sequence_entry\n");
    /* Indentless sequence: mapping value is a block sequence at same indent */
    const char *inputs[] = {
        /* key with indentless sequence */
        "map:\n- a\n- b\n",
        /* nested mapping with indentless seq */
        "outer:\n  inner:\n  - x\n  - y\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  indentless%d_events: %d\n", i, count);
        yaml_parser_delete(&parser);
    }
}

/* Test: parse with explicit document-end token */
static void test_parser_explicit_doc_end(void)
{
    printf("test_parser_explicit_doc_end\n");
    const char *inputs[] = {
        /* explicit doc end followed by another doc */
        "---\nfoo\n...\n---\nbar\n...\n",
        /* doc with explicit end and another implicit doc */
        "key: val\n...\nnext: doc\n",
        /* explicit doc start with version directive */
        "%YAML 1.1\n---\nvalue\n...\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  expl_doc%d_events: %d err: %d\n", i, count, parser.error);
        yaml_parser_delete(&parser);
    }
}

/* Test: emitter open_ended then second document with version directive */
static void test_emitter_open_ended_with_directive(void)
{
    printf("test_emitter_open_ended_with_directive\n");
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

    /* First doc: implicit start, creates open_ended=1 */
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"first", 5, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Second doc with version directive - triggers open_ended check */
    yaml_version_directive_t ver = { 1, 1 };
    yaml_document_start_event_initialize(&ev, &ver, NULL, NULL, 0);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"second", 6, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 0);
    yaml_emitter_emit(&emitter, &ev);

    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);

    outbuf[written] = '\0';
    printf("  written: %d\n", (int)written);
    /* Should contain "..." before the YAML directive */
    printf("  has_dotdotdot: %d\n", (strstr((char *)outbuf, "...") != NULL));
    printf("  has_yaml: %d\n", (strstr((char *)outbuf, "YAML") != NULL));

    yaml_emitter_delete(&emitter);
}

/* Test: document_initialize with version directive and tag directives */
static void test_api_document_init_with_dirs(void)
{
    printf("test_api_document_init_with_dirs\n");
    yaml_document_t doc;
    yaml_version_directive_t ver = { 1, 1 };
    yaml_tag_directive_t tags[2];
    tags[0].handle = (yaml_char_t *)"!custom!";
    tags[0].prefix = (yaml_char_t *)"tag:custom.org,2002:";
    tags[1].handle = (yaml_char_t *)"!other!";
    tags[1].prefix = (yaml_char_t *)"tag:other.org,2002:";

    int r = yaml_document_initialize(&doc, &ver, tags, tags + 2, 0, 0);
    printf("  init_with_dirs: %d\n", r);
    if (r) {
        printf("  has_ver_dir: %d\n", doc.version_directive != NULL);
        if (doc.version_directive)
            printf("  ver: %d.%d\n",
                doc.version_directive->major, doc.version_directive->minor);
        int ntags = (int)(doc.tag_directives.end - doc.tag_directives.start);
        printf("  ntags: %d\n", ntags);

        /* Add a scalar node */
        int idx = yaml_document_add_scalar(&doc, NULL,
            (yaml_char_t *)"hello", 5, YAML_PLAIN_SCALAR_STYLE);
        printf("  scalar_idx: %d\n", idx);

        yaml_document_delete(&doc);
    }

    /* Document with just a version directive */
    r = yaml_document_initialize(&doc, &ver, NULL, NULL, 1, 1);
    printf("  init_ver_only: %d\n", r);
    if (r) {
        printf("  ver_only_major: %d\n",
            doc.version_directive ? doc.version_directive->major : -1);
        yaml_document_delete(&doc);
    }
}

/* Test: emitter scalars with leading spaces (triggers block scalar indent hint) */
static void test_emitter_scalars_with_leading_spaces(void)
{
    printf("test_emitter_scalars_with_leading_spaces\n");
    unsigned char outbuf[4096];
    size_t written;

    /* Literal scalar starting with space */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter, " starts with space\n",
                               YAML_LITERAL_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  lead_space_lit: %d\n", (int)written > 0);
    }

    /* Folded scalar starting with newline */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter, "\nstarts with newline\n",
                               YAML_FOLDED_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  lead_nl_fold: %d\n", (int)written > 0);
    }

    /* Empty block scalar */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        emit_simple_scalar_doc(&emitter, "", YAML_LITERAL_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  empty_lit: %d\n", (int)written > 0);
    }
}

/* Test: loader with mapping that has anchor and alias */
static void test_loader_mapping_anchor_alias(void)
{
    printf("test_loader_mapping_anchor_alias\n");
    yaml_parser_t parser;
    yaml_document_t doc;

    /* mapping with anchor and alias */
    const char *input = "anchor: &a hello\nalias: *a\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                 strlen(input));
    int r = yaml_parser_load(&parser, &doc);
    printf("  map_alias_load: %d\n", r);
    if (r) {
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        printf("  map_alias_type: %d\n", root ? root->type : -1);
        if (root && root->type == YAML_MAPPING_NODE) {
            int n = (int)(root->data.mapping.pairs.top -
                          root->data.mapping.pairs.start);
            printf("  map_alias_pairs: %d\n", n);
        }
        yaml_document_delete(&doc);
    }
    yaml_parser_delete(&parser);

    /* mapping with anchor on mapping */
    const char *input2 = "&map_anchor\nkey: value\nalias: *map_anchor\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)input2,
                                 strlen(input2));
    r = yaml_parser_load(&parser, &doc);
    printf("  map_self_alias_load: %d err: %d\n", r, parser.error);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);
}

/* Test: scanner behavior with tabs in block context */
static void test_scanner_tab_in_block_context(void)
{
    printf("test_scanner_tab_in_block_context\n");
    const char *inputs[] = {
        /* Tab allowed in flow context */
        "{\tkey:\tvalue}\n",
        /* Tab after colon in value (allowed) */
        "key:\t value\n",
        /* Multiple tabs in flow */
        "[\ta,\tb,\tc]\n",
        /* Tab in comment */
        "key: val\t# comment\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  tab%d_events: %d err: %d\n", i, count, parser.error);
        yaml_parser_delete(&parser);
    }
}

/* Test: emitter tag content with characters requiring percent-encoding */
static void test_emitter_tag_content_percent_encode(void)
{
    printf("test_emitter_tag_content_percent_encode\n");
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

    /* Use a verbose tag with special chars that need percent-encoding */
    yaml_tag_directive_t tags[1];
    tags[0].handle = (yaml_char_t *)"!";
    tags[0].prefix = (yaml_char_t *)"!";

    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Emit scalar with a verbose tag (no handle) containing special chars */
    /* Actually emit using tag that has special chars in suffix */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!<tag:ex.org/a b>",
        (yaml_char_t *)"val", 3, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    int r = yaml_emitter_emit(&emitter, &ev);
    printf("  tag_special_emit: %d\n", r);
    if (!r) {
        /* try with simpler tag */
        yaml_event_delete(&ev);
        yaml_scalar_event_initialize(&ev, NULL,
            (yaml_char_t *)"!!str",
            (yaml_char_t *)"val", 3, 0, 0, YAML_PLAIN_SCALAR_STYLE);
        yaml_emitter_emit(&emitter, &ev);
    }

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);

    outbuf[written] = '\0';
    printf("  written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);

    /* Test with a tag that has URI chars that DON'T need encoding */
    memset(outbuf, 0, sizeof(outbuf)); written = 0;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    /* tag with chars that need % encoding: space -> %20 */
    yaml_scalar_event_initialize(&ev, NULL,
        (yaml_char_t *)"!<tag:ex.org/a%20b>",
        (yaml_char_t *)"val2", 4, 0, 0, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);
    outbuf[written] = '\0';
    printf("  pct_written: %d\n", (int)written);
    yaml_emitter_delete(&emitter);
}

/* Test: flow sequence with mapping pairs (? key : value syntax) */
static void test_parser_flow_seq_mapping_pairs(void)
{
    printf("test_parser_flow_seq_mapping_pairs\n");
    const char *inputs[] = {
        /* flow sequence containing a pair */
        "[? a : b, c]\n",
        /* flow sequence with explicit key mapping entry */
        "[{a: 1}, {b: 2}]\n",
        /* flow mapping with empty value (key only) */
        "{a:, b:, c:}\n",
        /* flow mapping with complex key */
        "{? [a,b] : val}\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  flow_pair%d_events: %d err: %d\n", i, count, parser.error);
        yaml_parser_delete(&parser);
    }
}

/* Test: single-quoted scalar with line breaks */
static void test_emitter_single_quoted_with_breaks(void)
{
    printf("test_emitter_single_quoted_with_breaks\n");
    unsigned char outbuf[4096];
    size_t written;

    const char *values[] = {
        "line1\nline2",          /* break in middle */
        "line1\nline2\nline3",   /* multiple breaks */
        "it's a test",           /* apostrophe */
        "ends with break\n",     /* trailing break */
        NULL
    };
    int i;
    for (i = 0; values[i]; i++) {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
        yaml_emitter_set_width(&emitter, 40);
        emit_simple_scalar_doc(&emitter, values[i], YAML_SINGLE_QUOTED_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        outbuf[written] = '\0';
        printf("  squote%d_written: %d\n", i, (int)written > 0);
    }
}

/* Test: loader with alias in sequences */
static void test_loader_sequence_anchor_alias(void)
{
    printf("test_loader_sequence_anchor_alias\n");
    yaml_parser_t parser;
    yaml_document_t doc;

    /* sequence with anchor and alias */
    const char *input = "- &a hello\n- *a\n- world\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                 strlen(input));
    int r = yaml_parser_load(&parser, &doc);
    printf("  seq_alias_load: %d\n", r);
    if (r) {
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        printf("  seq_alias_type: %d\n", root ? root->type : -1);
        if (root && root->type == YAML_SEQUENCE_NODE) {
            int n = (int)(root->data.sequence.items.top -
                          root->data.sequence.items.start);
            printf("  seq_alias_items: %d\n", n);
        }
        yaml_document_delete(&doc);
    }
    yaml_parser_delete(&parser);

    /* nested sequences with anchors */
    const char *input2 = "&seq\n- a\n- b\nalias_doc:\n  *seq\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)input2,
                                 strlen(input2));
    r = yaml_parser_load(&parser, &doc);
    printf("  nested_seq_load: %d err: %d\n", r, parser.error);
    if (r) yaml_document_delete(&doc);
    yaml_parser_delete(&parser);
}

/* Test: parse multiple documents in one stream */
static void test_parser_multi_docs_scan(void)
{
    printf("test_parser_multi_docs_scan\n");
    /* multiple explicit documents */
    const char *input = "---\ndoc1: a\n---\ndoc2: b\n---\ndoc3: c\n";
    yaml_parser_t parser;
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)input,
                                 strlen(input));
    int doc_count = 0;
    yaml_document_t doc;
    while (1) {
        int r = yaml_parser_load(&parser, &doc);
        if (!r) break;
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        if (!root) {
            yaml_document_delete(&doc);
            break;
        }
        doc_count++;
        yaml_document_delete(&doc);
    }
    printf("  multi_doc_count: %d\n", doc_count);
    yaml_parser_delete(&parser);

    /* stream with implicit and explicit docs */
    const char *input2 = "implicit: doc\n---\nexplicit: doc\n...\n";
    yaml_parser_initialize(&parser);
    yaml_parser_set_input_string(&parser, (const unsigned char *)input2,
                                 strlen(input2));
    doc_count = 0;
    while (1) {
        int r = yaml_parser_load(&parser, &doc);
        if (!r) break;
        yaml_node_t *root = yaml_document_get_root_node(&doc);
        if (!root) { yaml_document_delete(&doc); break; }
        doc_count++;
        yaml_document_delete(&doc);
    }
    printf("  mixed_doc_count: %d\n", doc_count);
    yaml_parser_delete(&parser);
}

/* Test: block scalar with indent indicator */
static void test_emitter_block_scalar_indent_hint(void)
{
    printf("test_emitter_block_scalar_indent_hint\n");
    unsigned char outbuf[4096];
    size_t written;

    /* Literal scalar that starts with multiple spaces - needs indent hint */
    const char *vals[] = {
        "  double indented\n",
        "   triple indented line\n",
        "\tsingle tab\n",
        NULL
    };
    int i;
    for (i = 0; vals[i]; i++) {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
        emit_simple_scalar_doc(&emitter, vals[i], YAML_LITERAL_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        printf("  indent_hint%d: %d\n", i, (int)written > 0);
    }

    /* Folded scalar with lines exceeding best_width -> triggers line fold */
    {
        memset(outbuf, 0, sizeof(outbuf)); written = 0;
        yaml_emitter_t emitter;
        yaml_emitter_initialize(&emitter);
        yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
        yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
        yaml_emitter_set_width(&emitter, 20);
        emit_simple_scalar_doc(&emitter,
            "This is a very long line that should be folded at best_width\n",
            YAML_FOLDED_SCALAR_STYLE);
        yaml_emitter_flush(&emitter);
        yaml_emitter_delete(&emitter);
        printf("  fold_at_width: %d\n", (int)written > 0);
    }
}

/* Test: API document node add operations */
static void test_api_document_node_add_ops(void)
{
    printf("test_api_document_node_add_ops\n");
    yaml_document_t doc;
    int r = yaml_document_initialize(&doc, NULL, NULL, NULL, 1, 1);
    printf("  doc_init: %d\n", r);
    if (!r) return;

    /* Add scalar nodes with different styles */
    int si1 = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"plain", 5, YAML_PLAIN_SCALAR_STYLE);
    printf("  scalar_plain: %d\n", si1);

    int si2 = yaml_document_add_scalar(&doc, NULL,
        (yaml_char_t *)"single", 6, YAML_SINGLE_QUOTED_SCALAR_STYLE);
    printf("  scalar_single: %d\n", si2);

    int si3 = yaml_document_add_scalar(&doc,
        (yaml_char_t *)"!!str",
        (yaml_char_t *)"tagged", 6, YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    printf("  scalar_tagged: %d\n", si3);

    /* Add sequence with items */
    int seqi = yaml_document_add_sequence(&doc, NULL, YAML_BLOCK_SEQUENCE_STYLE);
    printf("  seq_idx: %d\n", seqi);
    if (seqi > 0 && si1 > 0) {
        int ra = yaml_document_append_sequence_item(&doc, seqi, si1);
        printf("  seq_append1: %d\n", ra);
        ra = yaml_document_append_sequence_item(&doc, seqi, si2);
        printf("  seq_append2: %d\n", ra);
    }

    /* Add mapping with pairs */
    int mapi = yaml_document_add_mapping(&doc, NULL, YAML_BLOCK_MAPPING_STYLE);
    printf("  map_idx: %d\n", mapi);
    if (mapi > 0 && si1 > 0 && si2 > 0) {
        int rp = yaml_document_append_mapping_pair(&doc, mapi, si1, si2);
        printf("  map_pair: %d\n", rp);
        rp = yaml_document_append_mapping_pair(&doc, mapi, si3, seqi);
        printf("  map_pair2: %d\n", rp);
    }

    /* Flow variants */
    int fseq = yaml_document_add_sequence(&doc, NULL, YAML_FLOW_SEQUENCE_STYLE);
    printf("  flow_seq: %d\n", fseq);
    int fmap = yaml_document_add_mapping(&doc, NULL, YAML_FLOW_MAPPING_STYLE);
    printf("  flow_map: %d\n", fmap);

    yaml_document_delete(&doc);
}

/* Test: emitter check empty sequence and mapping */
static void test_emitter_check_empty_seq_map(void)
{
    printf("test_emitter_check_empty_seq_map\n");
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

    /* Mapping with empty sequence and empty mapping as values */
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1, YAML_BLOCK_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    /* key1: [] (empty flow sequence) */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"key1", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_start_event_initialize(&ev, NULL, NULL, 1, YAML_FLOW_SEQUENCE_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_sequence_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    /* key2: {} (empty flow mapping) */
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)"key2", 4, 1, 1, YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_start_event_initialize(&ev, NULL, NULL, 1, YAML_FLOW_MAPPING_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_mapping_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);

    outbuf[written] = '\0';
    printf("  written: %d\n", (int)written);
    printf("  has_empty_seq: %d\n", (strstr((char *)outbuf, "[]") != NULL));
    printf("  has_empty_map: %d\n", (strstr((char *)outbuf, "{}") != NULL));

    yaml_emitter_delete(&emitter);
}

/* Test: scanner with block scalar chomp strip mode */
static void test_scanner_block_scalar_chomp_strip(void)
{
    printf("test_scanner_block_scalar_chomp_strip\n");
    const char *inputs[] = {
        /* strip indicator */
        "key: |-\n  value\n",
        /* keep indicator */
        "key: |+\n  value\n\n",
        /* literal with indent indicator */
        "key: |2\n  value\n",
        /* folded strip */
        "key: >-\n  value\n",
        /* folded keep */
        "key: >+\n  value\n\n",
        /* literal with indent and strip */
        "key: |2-\n  value\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            if (ev.type == YAML_SCALAR_EVENT)
                printf("  chomp%d_val: %s\n", i,
                       ev.data.scalar.value ? (char *)ev.data.scalar.value : "(null)");
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  chomp%d_ok: %d\n", i, !parser.error);
        yaml_parser_delete(&parser);
    }
}

/* Test: parser flow mapping empty value */
static void test_parser_flow_mapping_empty_value(void)
{
    printf("test_parser_flow_mapping_empty_value\n");
    const char *inputs[] = {
        /* flow mapping with empty value (YAML_PARSE_FLOW_MAPPING_EMPTY_VALUE_STATE) */
        "{key:}\n",
        "{a:, b: val}\n",
        /* mixed */
        "{x: , y: z}\n",
        /* flow sequence with mapping entry having empty value */
        "[a: , b: 2]\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  empty_val%d_events: %d err: %d\n", i, count, parser.error);
        yaml_parser_delete(&parser);
    }
}

/* Test: plain scalar at width limit (triggers line break in plain scalar) */
static void test_emitter_plain_scalar_at_width(void)
{
    printf("test_emitter_plain_scalar_at_width\n");
    unsigned char outbuf[4096];
    size_t written = 0;
    memset(outbuf, 0, sizeof(outbuf));

    yaml_emitter_t emitter;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 20);

    yaml_event_t ev;
    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);

    /* Long plain scalar that exceeds best_width */
    const char *longval = "word1 word2 word3 word4 word5 word6 word7 word8";
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)longval, (int)strlen(longval), 1, 1,
        YAML_PLAIN_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);

    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);

    outbuf[written] = '\0';
    printf("  plain_at_width: %d\n", (int)written > 0);

    yaml_emitter_delete(&emitter);

    /* Double-quoted scalar at width */
    memset(outbuf, 0, sizeof(outbuf)); written = 0;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 20);

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)longval, (int)strlen(longval), 0, 1,
        YAML_DOUBLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);
    outbuf[written] = '\0';
    printf("  dquote_at_width: %d\n", (int)written > 0);
    yaml_emitter_delete(&emitter);

    /* Single-quoted scalar at width */
    memset(outbuf, 0, sizeof(outbuf)); written = 0;
    yaml_emitter_initialize(&emitter);
    yaml_emitter_set_output_string(&emitter, outbuf, sizeof(outbuf), &written);
    yaml_emitter_set_encoding(&emitter, YAML_UTF8_ENCODING);
    yaml_emitter_set_width(&emitter, 20);

    yaml_stream_start_event_initialize(&ev, YAML_UTF8_ENCODING);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_start_event_initialize(&ev, NULL, NULL, NULL, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_scalar_event_initialize(&ev, NULL, NULL,
        (yaml_char_t *)longval, (int)strlen(longval), 0, 1,
        YAML_SINGLE_QUOTED_SCALAR_STYLE);
    yaml_emitter_emit(&emitter, &ev);
    yaml_document_end_event_initialize(&ev, 1);
    yaml_emitter_emit(&emitter, &ev);
    yaml_stream_end_event_initialize(&ev);
    yaml_emitter_emit(&emitter, &ev);
    yaml_emitter_flush(&emitter);
    outbuf[written] = '\0';
    printf("  squote_at_width: %d\n", (int)written > 0);
    yaml_emitter_delete(&emitter);
}

/* Test: loader with explicit tag on scalar */
static void test_loader_explicit_tag_on_scalar(void)
{
    printf("test_loader_explicit_tag_on_scalar\n");
    yaml_parser_t parser;
    yaml_document_t doc;

    const char *inputs[] = {
        /* explicit !! tag on scalar */
        "!!str hello\n",
        /* local tag on scalar */
        "!mytag value\n",
        /* explicit tag on sequence item */
        "- !!int 42\n- !!str hello\n",
        /* explicit tag on mapping value */
        "key: !!bool true\n",
        /* tag:yaml.org,2002:null */
        "!!null ~\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        int r = yaml_parser_load(&parser, &doc);
        printf("  tag_scalar%d_load: %d err: %d\n", i, r, parser.error);
        if (r) {
            yaml_node_t *root = yaml_document_get_root_node(&doc);
            if (root && root->tag)
                printf("  tag_scalar%d_tag_ok: 1\n", i);
            yaml_document_delete(&doc);
        }
        yaml_parser_delete(&parser);
    }
}

/* Test: parser with anchor then tag (reversed order) */
static void test_parser_anchor_then_tag(void)
{
    printf("test_parser_anchor_then_tag\n");
    const char *inputs[] = {
        /* anchor then tag */
        "&anchor !!str value\n",
        /* tag then anchor */
        "!!str &anchor value\n",
        /* anchor on sequence */
        "&seq\n- a\n- b\n",
        /* anchor on mapping */
        "&map\nkey: value\n",
        /* anchor + tag on sequence */
        "&seq !!seq\n- a\n",
        /* anchor + tag on mapping */
        "&map !!map\nkey: val\n",
        NULL
    };
    int i;
    for (i = 0; inputs[i]; i++) {
        yaml_parser_t parser;
        yaml_parser_initialize(&parser);
        yaml_parser_set_input_string(&parser,
            (const unsigned char *)inputs[i], strlen(inputs[i]));
        yaml_event_t ev;
        int count = 0;
        int ok = 1;
        while (ok) {
            ok = yaml_parser_parse(&parser, &ev);
            if (!ok) break;
            count++;
            int done = (ev.type == YAML_STREAM_END_EVENT);
            yaml_event_delete(&ev);
            if (done) break;
        }
        printf("  anchor_tag%d_events: %d err: %d\n", i, count, parser.error);
        yaml_parser_delete(&parser);
    }
}

/* ------------------------------------------------------------------ */
/* main                                                                */
/* ------------------------------------------------------------------ */

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
    printf("=== libyaml test suite ===\n");

    run_test("version", test_version, 2);
    run_test("memory", test_memory, 2);
    run_test("string_extend_join", test_string_extend_join, 2);
    run_test("stack_queue_extend", test_stack_queue_extend, 2);
    run_test("token_delete", test_token_delete, 2);
    run_test("event_initialize", test_event_initialize, 2);
    run_test("parser_scan", test_parser_scan, 2);
    run_test("parser_parse", test_parser_parse, 2);
    run_test("parser_load", test_parser_load, 2);
    run_test("parser_encoding", test_parser_encoding, 2);
    run_test("parser_set_input", test_parser_set_input, 2);
    run_test("set_max_nest_level", test_set_max_nest_level, 2);
    run_test("emitter_basic", test_emitter_basic, 2);
    run_test("emitter_set_output", test_emitter_set_output, 2);
    run_test("emitter_open_close", test_emitter_open_close, 2);
    run_test("emitter_dump", test_emitter_dump, 2);
    run_test("document_api", test_document_api, 2);
    run_test("parser_fetch_update", test_parser_fetch_update, 2);
    run_test("bridge_check_utf8", test_bridge_check_utf8, 2);
    run_test("bridge_reader_handlers", test_bridge_reader_handlers, 2);
    run_test("bridge_writer_handlers", test_bridge_writer_handlers, 2);
    run_test("bridge_emitter_set_errors", test_bridge_emitter_set_errors, 2);
    run_test("bridge_emitter_analyze", test_bridge_emitter_analyze, 2);
    run_test("bridge_emitter_write", test_bridge_emitter_write, 2);
    run_test("bridge_emitter_write_quoted", test_bridge_emitter_write_quoted, 2);
    run_test("bridge_emitter_write_block", test_bridge_emitter_write_block, 2);
    run_test("bridge_emitter_checks", test_bridge_emitter_checks, 2);
    run_test("bridge_emitter_append_tag_directive", test_bridge_emitter_append_tag_directive, 2);
    run_test("bridge_emitter_increase_indent", test_bridge_emitter_increase_indent, 2);
    run_test("bridge_emitter_generate_anchor", test_bridge_emitter_generate_anchor, 2);
    run_test("bridge_emitter_dump_document", test_bridge_emitter_dump_document, 2);
    run_test("bridge_parser_set_errors", test_bridge_parser_set_errors, 2);
    run_test("bridge_parser_flow_levels", test_bridge_parser_flow_levels, 2);
    run_test("bridge_parser_scan_full", test_bridge_parser_scan_full, 2);
    run_test("bridge_parser_parse_full", test_bridge_parser_parse_full, 2);
    run_test("bridge_parser_load_aliases", test_bridge_parser_load_aliases, 2);
    run_test("bridge_parser_determine_encoding", test_bridge_parser_determine_encoding, 2);
    run_test("bridge_parser_scan_tokens", test_bridge_parser_scan_tokens, 2);
    run_test("bridge_parser_roll_indent", test_bridge_parser_roll_indent, 2);
    run_test("bridge_parser_fetch_stream", test_bridge_parser_fetch_stream, 2);
    run_test("bridge_parser_fetch_document_indicators", test_bridge_parser_fetch_document_indicators, 2);
    run_test("bridge_parser_fetch_collections", test_bridge_parser_fetch_collections, 2);
    run_test("bridge_parser_fetch_block_entry_key_value", test_bridge_parser_fetch_block_entry_key_value, 2);
    run_test("bridge_parser_fetch_anchors_tags", test_bridge_parser_fetch_anchors_tags, 2);
    run_test("bridge_parser_fetch_scalars", test_bridge_parser_fetch_scalars, 2);
    run_test("bridge_parser_scan_directives", test_bridge_parser_scan_directives, 2);
    run_test("bridge_parser_scan_uri_escapes", test_bridge_parser_scan_uri_escapes, 2);
    run_test("bridge_parser_process_directives", test_bridge_parser_process_directives, 2);
    run_test("bridge_parser_append_tag_directive", test_bridge_parser_append_tag_directive, 2);
    run_test("bridge_parser_stale_save_remove_keys", test_bridge_parser_stale_save_remove_keys, 2);
    run_test("bridge_emitter_emit_stream", test_bridge_emitter_emit_stream, 2);
    run_test("bridge_emitter_process_anchor_tag_scalar", test_bridge_emitter_process_anchor_tag_scalar, 2);
    run_test("bridge_emitter_select_scalar_style", test_bridge_emitter_select_scalar_style, 2);
    run_test("bridge_emitter_write_bom", test_bridge_emitter_write_bom, 2);
    run_test("bridge_emitter_block_scalar_hints", test_bridge_emitter_block_scalar_hints, 2);
    run_test("bridge_parser_parse_states", test_bridge_parser_parse_states, 2);
    run_test("bridge_parser_process_empty_scalar", test_bridge_parser_process_empty_scalar, 2);
    run_test("bridge_parser_register_anchor", test_bridge_parser_register_anchor, 2);
    run_test("bridge_emitter_emit_nodes", test_bridge_emitter_emit_nodes, 2);
    run_test("bridge_emitter_emit_alias", test_bridge_emitter_emit_alias, 2);
    run_test("bridge_parser_flow_sequence_mapping_entries", test_bridge_parser_flow_sequence_mapping_entries, 2);
    run_test("bridge_parser_indentless_sequence", test_bridge_parser_indentless_sequence, 2);
    run_test("multiple_documents", test_multiple_documents, 2);
    run_test("emitter_canonical", test_emitter_canonical, 2);
    run_test("emitter_version_tag_directives", test_emitter_version_tag_directives, 2);
    run_test("dump_sequence_mapping_alias", test_dump_sequence_mapping_alias, 2);
    run_test("emit_flow_sequence", test_emit_flow_sequence, 2);
    run_test("flow_sequence_compact_mapping", test_flow_sequence_compact_mapping, 2);

    /* New coverage-guided tests */
    run_test("token_delete_all_types", test_token_delete_all_types, 2);
    run_test("emitter_cr_break", test_emitter_cr_break, 2);
    run_test("emitter_crln_break", test_emitter_crln_break, 2);
    run_test("emitter_utf16le_encoding", test_emitter_utf16le_encoding, 2);
    run_test("parser_utf8_bom", test_parser_utf8_bom, 2);
    run_test("emitter_special_chars_dquoted", test_emitter_special_chars_dquoted, 2);
    run_test("emitter_large_output", test_emitter_large_output, 2);
    run_test("emitter_narrow_width", test_emitter_narrow_width, 2);
    run_test("dump_empty_document", test_dump_empty_document, 2);
    run_test("emitter_yaml12_directive", test_emitter_yaml12_directive, 2);
    run_test("emitter_open_ended_sequence", test_emitter_open_ended_sequence, 2);
    run_test("emitter_single_quoted_special", test_emitter_single_quoted_special, 2);
    run_test("emitter_plain_scalar_breaks", test_emitter_plain_scalar_breaks, 2);
    run_test("emitter_flow_sequence_canonical", test_emitter_flow_sequence_canonical, 2);
    run_test("emitter_flow_mapping_canonical", test_emitter_flow_mapping_canonical, 2);
    run_test("emitter_explicit_doc", test_emitter_explicit_doc, 2);
    run_test("parser_complex_flow", test_parser_complex_flow, 2);
    run_test("parser_special_scalars", test_parser_special_scalars, 2);
    run_test("emitter_block_mapping_complex", test_emitter_block_mapping_complex, 2);
    run_test("parser_directive_errors", test_parser_directive_errors, 2);
    run_test("emitter_double_quoted_breaks", test_emitter_double_quoted_breaks, 2);
    run_test("parser_block_sequence_complex", test_parser_block_sequence_complex, 2);
    run_test("emitter_increase_indent_variants", test_emitter_increase_indent_variants, 2);
    run_test("emitter_analyze_anchor_alias", test_emitter_analyze_anchor_alias, 2);
    run_test("parser_block_scalar_variants", test_parser_block_scalar_variants, 2);
    run_test("parser_flow_scalar_variants", test_parser_flow_scalar_variants, 2);
    run_test("parser_plain_scalar_variants", test_parser_plain_scalar_variants, 2);
    run_test("emitter_write_tag_content_variants", test_emitter_write_tag_content_variants, 2);
    run_test("parser_nest_limit", test_parser_nest_limit, 2);
    run_test("document_with_tags", test_document_with_tags, 2);
    run_test("emitter_folded_with_breaks", test_emitter_folded_with_breaks, 2);

    /* New coverage-guided tests (round 2) */
    run_test("api_malloc_zero_realloc", test_api_malloc_zero_realloc, 2);
    run_test("api_event_utf8_errors", test_api_event_utf8_errors, 2);
    run_test("api_event_with_anchor_tag", test_api_event_with_anchor_tag, 2);
    run_test("api_document_dirs", test_api_document_dirs, 2);
    run_test("api_string_write_overflow", test_api_string_write_overflow, 2);
    run_test("api_emitter_indent_boundary", test_api_emitter_indent_boundary, 2);
    run_test("api_queue_extend_move", test_api_queue_extend_move, 2);
    run_test("emitter_flow_non_simple_key", test_emitter_flow_non_simple_key, 2);
    run_test("emitter_block_non_simple_key", test_emitter_block_non_simple_key, 2);
    run_test("dump_complex_nested", test_dump_complex_nested, 2);
    run_test("dump_with_directives", test_dump_with_directives, 2);
    run_test("parser_multi_anchors", test_parser_multi_anchors, 2);
    run_test("emitter_unicode_chars", test_emitter_unicode_chars, 2);
    run_test("parser_utf16_bom", test_parser_utf16_bom, 2);
    run_test("emitter_open_ended2", test_emitter_open_ended2, 2);
    run_test("emitter_scalar_select_styles", test_emitter_scalar_select_styles, 2);
    run_test("emitter_flow_mapping_value_complex", test_emitter_flow_mapping_value_complex, 2);
    run_test("emitter_block_mapping_value_complex", test_emitter_block_mapping_value_complex, 2);
    run_test("emitter_alias_in_simple_key", test_emitter_alias_in_simple_key, 2);
    run_test("emitter_plain_scalar_with_breaks", test_emitter_plain_scalar_with_breaks, 2);
    run_test("emitter_single_quoted_breaks", test_emitter_single_quoted_breaks, 2);
    run_test("parser_error_paths", test_parser_error_paths, 2);
    run_test("dumper_flow_sequence_mapping", test_dumper_flow_sequence_mapping, 2);
    run_test("emitter_scalar_with_tag_shorthand", test_emitter_scalar_with_tag_shorthand, 2);
    run_test("loader_prestarted_stream", test_loader_prestarted_stream, 2);
    run_test("loader_stream_end_reuse", test_loader_stream_end_reuse, 2);
    run_test("loader_duplicate_anchor", test_loader_duplicate_anchor, 2);
    run_test("loader_undefined_alias", test_loader_undefined_alias, 2);
    run_test("loader_explicit_tags", test_loader_explicit_tags, 2);
    run_test("dumper_close_twice", test_dumper_close_twice, 2);
    run_test("dumper_dump_not_opened", test_dumper_dump_not_opened, 2);
    run_test("api_document_node_edge_cases", test_api_document_node_edge_cases, 2);
    run_test("emitter_utf16be", test_emitter_utf16be, 2);
    run_test("emitter_scalar_leading_trailing", test_emitter_scalar_leading_trailing, 2);
    run_test("emitter_scalar_flow_indicators", test_emitter_scalar_flow_indicators, 2);
    run_test("string_join_extend", test_string_join_extend, 2);
    run_test("loader_sequence_mapping_tags", test_loader_sequence_mapping_tags, 2);
    run_test("parser_block_mapping_various", test_parser_block_mapping_various, 2);
    run_test("emitter_scalar_with_special_first_last", test_emitter_scalar_with_special_first_last, 2);
    run_test("emitter_block_seq_in_block_map_indent", test_emitter_block_seq_in_block_map_indent, 2);
    run_test("emitter_folded_scalar_complex", test_emitter_folded_scalar_complex, 2);
    run_test("scanner_flow_scalar_whitespace", test_scanner_flow_scalar_whitespace, 2);
    run_test("emitter_analyze_tag_variations", test_emitter_analyze_tag_variations, 2);
    run_test("reader_utf32", test_reader_utf32, 2);
    run_test("emitter_multiple_documents_mixed", test_emitter_multiple_documents_mixed, 2);
    run_test("dumper_with_explicit_tags_nodes", test_dumper_with_explicit_tags_nodes, 2);
    run_test("parser_scan_complex_yaml", test_parser_scan_complex_yaml, 2);
    run_test("emitter_write_double_quoted_multibyte", test_emitter_write_double_quoted_multibyte, 2);
    run_test("parser_implicit_docs", test_parser_implicit_docs, 2);
    run_test("emitter_block_scalar_chomping", test_emitter_block_scalar_chomping, 2);
    run_test("emitter_canonical_sequences_mappings", test_emitter_canonical_sequences_mappings, 2);

    run_test("api_invalid_utf8_events", test_api_invalid_utf8_events, 2);
    run_test("api_check_utf8_overlong", test_api_check_utf8_overlong, 2);
    run_test("large_input_string_reader", test_large_input_string_reader, 2);
    run_test("parser_utf16be_input", test_parser_utf16be_input, 2);
    run_test("scanner_flow_scalar_unicode_escapes", test_scanner_flow_scalar_unicode_escapes, 2);
    run_test("scanner_block_scalar_edge_cases", test_scanner_block_scalar_edge_cases, 2);
    run_test("scanner_tag_in_flow", test_scanner_tag_in_flow, 2);
    run_test("scanner_plain_scalar_multiline", test_scanner_plain_scalar_multiline, 2);
    run_test("scanner_complex_anchors_aliases", test_scanner_complex_anchors_aliases, 2);
    run_test("emitter_double_quoted_multiline_wrap", test_emitter_double_quoted_multiline_wrap, 2);
    run_test("emitter_block_seq_in_flow", test_emitter_block_seq_in_flow, 2);
    run_test("parser_block_mapping_various2", test_parser_block_mapping_various2, 2);
    run_test("dumper_doc_with_version_tag_dirs", test_dumper_doc_with_version_tag_dirs, 2);
    run_test("emitter_write_tag_content_uri", test_emitter_write_tag_content_uri, 2);
    run_test("emitter_block_scalar_with_indent_indicator", test_emitter_block_scalar_with_indent_indicator, 2);
    run_test("parser_flow_pairs", test_parser_flow_pairs, 2);
    run_test("loader_anchor_tag_combinations", test_loader_anchor_tag_combinations, 2);
    run_test("scanner_directive_variants", test_scanner_directive_variants, 2);
    run_test("emitter_select_style_variants", test_emitter_select_style_variants, 2);
    run_test("parser_node_with_anchor_tag", test_parser_node_with_anchor_tag, 2);
    run_test("scanner_flow_scalar_line_break", test_scanner_flow_scalar_line_break, 2);
    run_test("emitter_state_machine_sequence_mapping", test_emitter_state_machine_sequence_mapping, 2);
    run_test("parser_stream_end_edge", test_parser_stream_end_edge, 2);
    run_test("emitter_analyze_scalar_variants", test_emitter_analyze_scalar_variants, 2);
    run_test("api_document_delete_node_types", test_api_document_delete_node_types, 2);
    run_test("scanner_complex_yaml_inputs", test_scanner_complex_yaml_inputs, 2);

    /* New coverage-guided tests (round 5) */
    run_test("api_event_init_invalid_utf8", test_api_event_init_invalid_utf8, 2);
    run_test("emitter_yaml11_version_directive", test_emitter_yaml11_version_directive, 2);
    run_test("emitter_block_scalar_chomp_variants", test_emitter_block_scalar_chomp_variants, 2);
    run_test("emitter_double_quoted_special_escapes", test_emitter_double_quoted_special_escapes, 2);
    run_test("loader_flow_seq_and_map", test_loader_flow_seq_and_map, 2);
    run_test("parser_indentless_sequence_entry", test_parser_indentless_sequence_entry, 2);
    run_test("parser_explicit_doc_end", test_parser_explicit_doc_end, 2);
    run_test("emitter_open_ended_with_directive", test_emitter_open_ended_with_directive, 2);
    run_test("api_document_init_with_dirs", test_api_document_init_with_dirs, 2);
    run_test("emitter_scalars_with_leading_spaces", test_emitter_scalars_with_leading_spaces, 2);
    run_test("loader_mapping_anchor_alias", test_loader_mapping_anchor_alias, 2);
    run_test("scanner_tab_in_block_context", test_scanner_tab_in_block_context, 2);
    run_test("emitter_tag_content_percent_encode", test_emitter_tag_content_percent_encode, 2);
    run_test("parser_flow_seq_mapping_pairs", test_parser_flow_seq_mapping_pairs, 2);
    run_test("emitter_single_quoted_with_breaks", test_emitter_single_quoted_with_breaks, 2);
    run_test("loader_sequence_anchor_alias", test_loader_sequence_anchor_alias, 2);
    run_test("parser_multi_docs_scan", test_parser_multi_docs_scan, 2);
    run_test("emitter_block_scalar_indent_hint", test_emitter_block_scalar_indent_hint, 2);
    run_test("api_document_node_add_ops", test_api_document_node_add_ops, 2);
    run_test("emitter_check_empty_seq_map", test_emitter_check_empty_seq_map, 2);
    run_test("scanner_block_scalar_chomp_strip", test_scanner_block_scalar_chomp_strip, 2);
    run_test("parser_flow_mapping_empty_value", test_parser_flow_mapping_empty_value, 2);
    run_test("emitter_plain_scalar_at_width", test_emitter_plain_scalar_at_width, 2);
    run_test("loader_explicit_tag_on_scalar", test_loader_explicit_tag_on_scalar, 2);
    run_test("parser_anchor_then_tag", test_parser_anchor_then_tag, 2);

    printf("=== done ===\n");
    return 0;
}

