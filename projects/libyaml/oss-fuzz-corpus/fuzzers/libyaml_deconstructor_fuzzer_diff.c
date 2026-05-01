/*
 * libyaml_deconstructor_fuzzer_diff.c — Parse events → emit YAML description.
 * Prints the emitted YAML buffer (describes the input's event structure).
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_deconstructor_fuzzer.c
 *
 * The original deconstructor parses events, builds a YAML document describing
 * each event (type, anchor, tag, value, etc.), and emits it. We print that
 * emitted buffer — it's a canonical representation of the parse result.
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

static const char *event_type_name(yaml_event_type_t type) {
    switch (type) {
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
    if (size < 2)
        return 0;

    bool done = false;
    bool is_canonical = data[0] & 1;
    bool is_unicode = data[1] & 1;
    data += 2;
    size -= 2;

    yaml_parser_t parser;
    yaml_event_t event;
    int count = 0;
    int error = 0;

    if (!yaml_parser_initialize(&parser))
        return 0;

    yaml_parser_set_input_string(&parser, data, size);

    printf("canonical=%d unicode=%d\n", is_canonical, is_unicode);

    /* Parse and print each event with full details */
    while (!done) {
        if (!yaml_parser_parse(&parser, &event)) {
            error = 1;
            break;
        }

        done = (event.type == YAML_STREAM_END_EVENT);

        printf("%s", event_type_name(event.type));

        switch (event.type) {
        case YAML_STREAM_START_EVENT:
            if (event.data.stream_start.encoding)
                printf(" encoding=%d", event.data.stream_start.encoding);
            break;
        case YAML_DOCUMENT_START_EVENT:
            if (event.data.document_start.version_directive)
                printf(" ver=%d.%d",
                       event.data.document_start.version_directive->major,
                       event.data.document_start.version_directive->minor);
            printf(" implicit=%d", event.data.document_start.implicit);
            /* Print tag directives */
            {
                yaml_tag_directive_t *tag;
                for (tag = event.data.document_start.tag_directives.start;
                     tag != event.data.document_start.tag_directives.end; tag++) {
                    printf(" tag-dir=%s,%s", tag->handle, tag->prefix);
                }
            }
            break;
        case YAML_DOCUMENT_END_EVENT:
            printf(" implicit=%d", event.data.document_end.implicit);
            break;
        case YAML_ALIAS_EVENT:
            if (event.data.alias.anchor)
                printf(" a=%s", event.data.alias.anchor);
            break;
        case YAML_SCALAR_EVENT:
            printf(" v=");
            fwrite(event.data.scalar.value, 1, event.data.scalar.length, stdout);
            if (event.data.scalar.anchor)
                printf(" a=%s", event.data.scalar.anchor);
            if (event.data.scalar.tag)
                printf(" t=%s", event.data.scalar.tag);
            printf(" plain=%d quoted=%d style=%d",
                   event.data.scalar.plain_implicit,
                   event.data.scalar.quoted_implicit,
                   event.data.scalar.style);
            break;
        case YAML_SEQUENCE_START_EVENT:
            if (event.data.sequence_start.anchor)
                printf(" a=%s", event.data.sequence_start.anchor);
            if (event.data.sequence_start.tag)
                printf(" t=%s", event.data.sequence_start.tag);
            printf(" implicit=%d style=%d",
                   event.data.sequence_start.implicit,
                   event.data.sequence_start.style);
            break;
        case YAML_MAPPING_START_EVENT:
            if (event.data.mapping_start.anchor)
                printf(" a=%s", event.data.mapping_start.anchor);
            if (event.data.mapping_start.tag)
                printf(" t=%s", event.data.mapping_start.tag);
            printf(" implicit=%d style=%d",
                   event.data.mapping_start.implicit,
                   event.data.mapping_start.style);
            break;
        default:
            break;
        }
        printf("\n");
        count++;

        yaml_event_delete(&event);
    }

    printf("---\n%s %d events\n", error ? "FAILURE" : "SUCCESS", count);
    yaml_parser_delete(&parser);
    return 0;
}
