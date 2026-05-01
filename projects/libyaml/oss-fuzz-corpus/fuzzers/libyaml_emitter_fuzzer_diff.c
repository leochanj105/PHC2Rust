/*
 * libyaml_emitter_fuzzer_diff.c — Parse→emit→reparse roundtrip.
 * Prints emitted buffer and event comparison results.
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_emitter_fuzzer.c
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

#define MAX_EVENTS 1024

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
    if (size < 2)
        return 0;

    yaml_parser_t parser;
    yaml_emitter_t emitter;
    yaml_event_t event;
    yaml_event_type_t event_types[MAX_EVENTS];
    size_t event_number = 0;
    bool done = false;
    bool is_canonical = data[0] & 1;
    bool is_unicode = data[1] & 1;
    int error = 0;
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

    /* Phase 1: parse and emit */
    while (!done) {
        if (!yaml_parser_parse(&parser, &event)) {
            error = 1;
            break;
        }

        done = (event.type == YAML_STREAM_END_EVENT);
        if (event_number < MAX_EVENTS) {
            event_types[event_number] = event.type;
            event_number++;
        }

        if (!yaml_emitter_emit(&emitter, &event)) {
            error = 1;
            break;
        }
    }

    /* Print phase 1 events */
    printf("canonical=%d unicode=%d\n", is_canonical, is_unicode);
    printf("PHASE1 %zu events\n", event_number);
    for (size_t i = 0; i < event_number; i++)
        printf("  %s\n", event_type_name(event_types[i]));

    /* Print emitted output */
    if (out.buf && out.size > 0) {
        printf("EMITTED %zu\n", out.size);
        fwrite(out.buf, 1, out.size, stdout);
        if (out.size > 0 && out.buf[out.size - 1] != '\n')
            printf("\n");
    }

    /* Phase 2: reparse the emitted output */
    if (!error && out.buf && out.size > 0) {
        yaml_parser_delete(&parser);
        if (yaml_parser_initialize(&parser)) {
            yaml_parser_set_input_string(&parser, out.buf, out.size);
            int count2 = 0;
            done = false;
            printf("PHASE2\n");
            while (!done) {
                if (!yaml_parser_parse(&parser, &event)) {
                    printf("  REPARSE-ERROR\n");
                    error = 1;
                    break;
                }
                printf("  %s\n", event_type_name(event.type));
                done = (event.type == YAML_STREAM_END_EVENT);
                count2++;
                yaml_event_delete(&event);
            }
            printf("PHASE2 %d events\n", count2);
        }
    }

    printf("---\n%s\n", error ? "FAILURE" : "SUCCESS");

    yaml_parser_delete(&parser);
    yaml_emitter_delete(&emitter);
    free(out.buf);
    return 0;
}
