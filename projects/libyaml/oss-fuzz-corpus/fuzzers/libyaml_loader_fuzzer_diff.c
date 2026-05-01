/*
 * libyaml_loader_fuzzer_diff.c — Loader fuzzer adapted for differential testing.
 * Prints document structure so C vs Rust outputs are comparable.
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_loader_fuzzer.c
 */
#include "yaml.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

static void print_node(yaml_document_t *doc, yaml_node_t *node, int depth) {
    yaml_node_pair_t *pair;
    yaml_node_item_t *item;

    if (!node || depth > 64) return;

    switch (node->type) {
    case YAML_SCALAR_NODE:
        printf("SCALAR v=");
        fwrite(node->data.scalar.value, 1, node->data.scalar.length, stdout);
        if (node->tag)
            printf(" t=%s", node->tag);
        printf("\n");
        break;
    case YAML_SEQUENCE_NODE:
        printf("SEQUENCE-START");
        if (node->tag) printf(" t=%s", node->tag);
        printf("\n");
        for (item = node->data.sequence.items.start;
             item < node->data.sequence.items.top; item++) {
            yaml_node_t *child = yaml_document_get_node(doc, *item);
            print_node(doc, child, depth + 1);
        }
        printf("SEQUENCE-END\n");
        break;
    case YAML_MAPPING_NODE:
        printf("MAPPING-START");
        if (node->tag) printf(" t=%s", node->tag);
        printf("\n");
        for (pair = node->data.mapping.pairs.start;
             pair < node->data.mapping.pairs.top; pair++) {
            yaml_node_t *key = yaml_document_get_node(doc, pair->key);
            yaml_node_t *val = yaml_document_get_node(doc, pair->value);
            printf("KEY\n");
            print_node(doc, key, depth + 1);
            printf("VALUE\n");
            print_node(doc, val, depth + 1);
        }
        printf("MAPPING-END\n");
        break;
    default:
        printf("UNKNOWN-NODE\n");
        break;
    }
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    yaml_parser_t parser;
    yaml_document_t document;
    bool done = false;
    int count = 0;
    int error = 0;

    if (!yaml_parser_initialize(&parser))
        return 0;

    yaml_parser_set_input_string(&parser, data, size);

    while (!done) {
        if (!yaml_parser_load(&parser, &document)) {
            error = 1;
            break;
        }

        yaml_node_t *root = yaml_document_get_root_node(&document);
        done = (!root);

        if (!done) {
            printf("DOC %d\n", count);
            print_node(&document, root, 0);
            count++;
        }

        yaml_document_delete(&document);
    }

    printf("---\n%s %d documents\n", error ? "FAILURE" : "SUCCESS", count);
    yaml_parser_delete(&parser);
    return 0;
}
