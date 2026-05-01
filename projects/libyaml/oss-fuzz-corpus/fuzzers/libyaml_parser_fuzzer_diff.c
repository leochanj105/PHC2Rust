/*
 * libyaml_parser_fuzzer_diff.c — Parser fuzzer adapted for differential testing.
 * Prints each event type so C vs Rust outputs are comparable.
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_parser_fuzzer.c
 */
#include "yaml.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

static const char *event_type_name(yaml_event_type_t type) {
    switch (type) {
        case YAML_NO_EVENT: return "NO-EVENT";
        case YAML_STREAM_START_EVENT: return "STREAM-START";
        case YAML_STREAM_END_EVENT: return "STREAM-END";
        case YAML_DOCUMENT_START_EVENT: return "DOCUMENT-START";
        case YAML_DOCUMENT_END_EVENT: return "DOCUMENT-END";
        case YAML_ALIAS_EVENT: return "ALIAS";
        case YAML_SCALAR_EVENT: return "SCALAR";
        case YAML_SEQUENCE_START_EVENT: return "SEQUENCE-START";
        case YAML_SEQUENCE_END_EVENT: return "SEQUENCE-END";
        case YAML_MAPPING_START_EVENT: return "MAPPING-START";
        case YAML_MAPPING_END_EVENT: return "MAPPING-END";
        default: return "UNKNOWN";
    }
}

int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
    yaml_parser_t parser;
    yaml_event_t event;
    bool done = false;
    int count = 0;
    int error = 0;

    if (!yaml_parser_initialize(&parser))
        return 0;

    yaml_parser_set_input_string(&parser, data, size);

    while (!done) {
        if (!yaml_parser_parse(&parser, &event)) {
            error = 1;
            break;
        }

        printf("%s", event_type_name(event.type));

        /* Print event-specific data */
        switch (event.type) {
        case YAML_SCALAR_EVENT:
            printf(" v=");
            fwrite(event.data.scalar.value, 1, event.data.scalar.length, stdout);
            if (event.data.scalar.anchor)
                printf(" a=%s", event.data.scalar.anchor);
            if (event.data.scalar.tag)
                printf(" t=%s", event.data.scalar.tag);
            break;
        case YAML_ALIAS_EVENT:
            if (event.data.alias.anchor)
                printf(" a=%s", event.data.alias.anchor);
            break;
        case YAML_SEQUENCE_START_EVENT:
            if (event.data.sequence_start.anchor)
                printf(" a=%s", event.data.sequence_start.anchor);
            if (event.data.sequence_start.tag)
                printf(" t=%s", event.data.sequence_start.tag);
            break;
        case YAML_MAPPING_START_EVENT:
            if (event.data.mapping_start.anchor)
                printf(" a=%s", event.data.mapping_start.anchor);
            if (event.data.mapping_start.tag)
                printf(" t=%s", event.data.mapping_start.tag);
            break;
        case YAML_DOCUMENT_START_EVENT:
            if (event.data.document_start.version_directive)
                printf(" ver=%d.%d",
                       event.data.document_start.version_directive->major,
                       event.data.document_start.version_directive->minor);
            break;
        default:
            break;
        }
        printf("\n");

        count++;
        done = (event.type == YAML_STREAM_END_EVENT);
        yaml_event_delete(&event);
    }

    printf("---\n%s %d events\n", error ? "FAILURE" : "SUCCESS", count);
    yaml_parser_delete(&parser);
    return 0;
}
