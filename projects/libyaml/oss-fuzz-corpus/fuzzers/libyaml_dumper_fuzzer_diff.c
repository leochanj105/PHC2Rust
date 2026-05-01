/*
 * libyaml_dumper_fuzzer_diff.c — Load→dump→reload roundtrip.
 * Prints dumped output buffer and document counts.
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_dumper_fuzzer.c
 */
#include "yaml.h"
#include "yaml_write_handler.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

#ifdef NDEBUG
#undef NDEBUG
#endif

static void print_node(yaml_document_t *doc, yaml_node_t *node, int depth) {
    yaml_node_pair_t *pair;
    yaml_node_item_t *item;

    if (!node || depth > 64) return;

    switch (node->type) {
    case YAML_SCALAR_NODE:
        printf("SCALAR v=");
        fwrite(node->data.scalar.value, 1, node->data.scalar.length, stdout);
        if (node->tag) printf(" t=%s", node->tag);
        printf("\n");
        break;
    case YAML_SEQUENCE_NODE:
        printf("SEQ-START");
        if (node->tag) printf(" t=%s", node->tag);
        printf("\n");
        for (item = node->data.sequence.items.start;
             item < node->data.sequence.items.top; item++) {
            yaml_node_t *child = yaml_document_get_node(doc, *item);
            print_node(doc, child, depth + 1);
        }
        printf("SEQ-END\n");
        break;
    case YAML_MAPPING_NODE:
        printf("MAP-START");
        if (node->tag) printf(" t=%s", node->tag);
        printf("\n");
        for (pair = node->data.mapping.pairs.start;
             pair < node->data.mapping.pairs.top; pair++) {
            yaml_node_t *key = yaml_document_get_node(doc, pair->key);
            yaml_node_t *val = yaml_document_get_node(doc, pair->value);
            print_node(doc, key, depth + 1);
            print_node(doc, val, depth + 1);
        }
        printf("MAP-END\n");
        break;
    default:
        break;
    }
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    if (size < 2)
        return 0;

    yaml_parser_t parser;
    yaml_emitter_t emitter;
    yaml_document_t document;
    bool done = false;
    int count = 0;
    int error = 0;
    bool is_canonical = data[0] & 1;
    bool is_unicode = data[1] & 1;
    data += 2;
    size -= 2;

    if (!yaml_parser_initialize(&parser))
        return 0;

    yaml_parser_set_input_string(&parser, data, size);
    if (!yaml_emitter_initialize(&emitter)) {
        yaml_parser_delete(&parser);
        return 0;
    }

    yaml_emitter_set_canonical(&emitter, is_canonical);
    yaml_emitter_set_unicode(&emitter, is_unicode);

    yaml_output_buffer_t out = {/*buf=*/NULL, /*size=*/0, /*capacity=*/1000};
    yaml_emitter_set_output(&emitter, yaml_write_handler, &out);
    yaml_emitter_open(&emitter);

    /* Phase 1: load and dump */
    printf("canonical=%d unicode=%d\n", is_canonical, is_unicode);
    printf("PHASE1\n");
    while (!done) {
        if (!yaml_parser_load(&parser, &document)) {
            error = 1;
            break;
        }

        done = (!yaml_document_get_root_node(&document));
        if (!done) {
            printf("DOC %d\n", count);
            print_node(&document, yaml_document_get_root_node(&document), 0);

            if (!yaml_emitter_dump(&emitter, &document)) {
                error = 1;
                break;
            }
            count++;
        } else {
            yaml_document_delete(&document);
        }
    }
    printf("PHASE1 %d documents\n", count);

    yaml_parser_delete(&parser);
    yaml_emitter_close(&emitter);
    yaml_emitter_delete(&emitter);

    /* Print dumped output */
    if (out.buf && out.size > 0) {
        printf("DUMPED %zu\n", out.size);
        fwrite(out.buf, 1, out.size, stdout);
        if (out.size > 0 && out.buf[out.size - 1] != '\n')
            printf("\n");
    }

    /* Phase 2: reload the dumped output */
    if (!error && out.buf && out.size > 0) {
        if (yaml_parser_initialize(&parser)) {
            yaml_parser_set_input_string(&parser, out.buf, out.size);
            int count2 = 0;
            done = false;
            printf("PHASE2\n");
            while (!done) {
                if (!yaml_parser_load(&parser, &document)) {
                    printf("  RELOAD-ERROR\n");
                    error = 1;
                    break;
                }
                done = (!yaml_document_get_root_node(&document));
                if (!done) {
                    printf("DOC %d\n", count2);
                    print_node(&document, yaml_document_get_root_node(&document), 0);
                    count2++;
                }
                yaml_document_delete(&document);
            }
            printf("PHASE2 %d documents\n", count2);
            yaml_parser_delete(&parser);
        }
    }

    printf("---\n%s\n", error ? "FAILURE" : "SUCCESS");
    free(out.buf);
    return 0;
}
