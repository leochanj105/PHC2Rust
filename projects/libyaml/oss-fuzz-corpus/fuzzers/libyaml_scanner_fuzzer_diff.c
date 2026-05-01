/*
 * libyaml_scanner_fuzzer_diff.c — Scanner fuzzer adapted for differential testing.
 * Prints each token type so C vs Rust outputs are comparable.
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_scanner_fuzzer.c
 */
#include "yaml.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

static const char *token_type_name(yaml_token_type_t type) {
    switch (type) {
        case YAML_NO_TOKEN: return "NO_TOKEN";
        case YAML_STREAM_START_TOKEN: return "STREAM-START";
        case YAML_STREAM_END_TOKEN: return "STREAM-END";
        case YAML_VERSION_DIRECTIVE_TOKEN: return "VERSION-DIRECTIVE";
        case YAML_TAG_DIRECTIVE_TOKEN: return "TAG-DIRECTIVE";
        case YAML_DOCUMENT_START_TOKEN: return "DOCUMENT-START";
        case YAML_DOCUMENT_END_TOKEN: return "DOCUMENT-END";
        case YAML_BLOCK_SEQUENCE_START_TOKEN: return "BLOCK-SEQUENCE-START";
        case YAML_BLOCK_MAPPING_START_TOKEN: return "BLOCK-MAPPING-START";
        case YAML_BLOCK_END_TOKEN: return "BLOCK-END";
        case YAML_FLOW_SEQUENCE_START_TOKEN: return "FLOW-SEQUENCE-START";
        case YAML_FLOW_SEQUENCE_END_TOKEN: return "FLOW-SEQUENCE-END";
        case YAML_FLOW_MAPPING_START_TOKEN: return "FLOW-MAPPING-START";
        case YAML_FLOW_MAPPING_END_TOKEN: return "FLOW-MAPPING-END";
        case YAML_BLOCK_ENTRY_TOKEN: return "BLOCK-ENTRY";
        case YAML_FLOW_ENTRY_TOKEN: return "FLOW-ENTRY";
        case YAML_KEY_TOKEN: return "KEY";
        case YAML_VALUE_TOKEN: return "VALUE";
        case YAML_ALIAS_TOKEN: return "ALIAS";
        case YAML_ANCHOR_TOKEN: return "ANCHOR";
        case YAML_TAG_TOKEN: return "TAG";
        case YAML_SCALAR_TOKEN: return "SCALAR";
        default: return "UNKNOWN";
    }
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    yaml_parser_t parser;
    yaml_token_t token;
    bool done = false;
    int count = 0;
    int error = 0;

    if (!yaml_parser_initialize(&parser))
        return 0;

    yaml_parser_set_input_string(&parser, data, size);

    while (!done) {
        if (!yaml_parser_scan(&parser, &token)) {
            error = 1;
            break;
        }

        printf("%s", token_type_name(token.type));

        /* Print token-specific data */
        switch (token.type) {
        case YAML_SCALAR_TOKEN:
            printf(" v=");
            fwrite(token.data.scalar.value, 1, token.data.scalar.length, stdout);
            break;
        case YAML_ALIAS_TOKEN:
            if (token.data.alias.value)
                printf(" a=%s", token.data.alias.value);
            break;
        case YAML_ANCHOR_TOKEN:
            if (token.data.anchor.value)
                printf(" a=%s", token.data.anchor.value);
            break;
        case YAML_TAG_TOKEN:
            if (token.data.tag.handle)
                printf(" h=%s", token.data.tag.handle);
            if (token.data.tag.suffix)
                printf(" s=%s", token.data.tag.suffix);
            break;
        case YAML_VERSION_DIRECTIVE_TOKEN:
            printf(" %d.%d", token.data.version_directive.major,
                   token.data.version_directive.minor);
            break;
        case YAML_TAG_DIRECTIVE_TOKEN:
            if (token.data.tag_directive.handle)
                printf(" h=%s", token.data.tag_directive.handle);
            if (token.data.tag_directive.prefix)
                printf(" p=%s", token.data.tag_directive.prefix);
            break;
        default:
            break;
        }
        printf("\n");

        count++;
        done = (token.type == YAML_STREAM_END_TOKEN);
        yaml_token_delete(&token);
    }

    printf("---\n%s %d tokens\n", error ? "FAILURE" : "SUCCESS", count);
    yaml_parser_delete(&parser);
    return 0;
}
