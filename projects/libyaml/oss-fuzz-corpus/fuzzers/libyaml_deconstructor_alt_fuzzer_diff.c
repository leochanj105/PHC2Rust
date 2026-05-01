/*
 * libyaml_deconstructor_alt_fuzzer_diff.c — Parse events → build document tree → dump.
 * Prints the dumped output (YAML description of the parsed event structure).
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_deconstructor_alt_fuzzer.c
 *
 * The original builds a yaml_document_t describing each parsed event, then
 * dumps it via yaml_emitter_dump. We print the dumped buffer.
 * This exercises: parser + document API + emitter/dumper (broadest coverage).
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
    yaml_emitter_t emitter;
    yaml_event_t input_event;
    yaml_document_t output_document;
    int root;
    int count = 0;
    int error = 0;

    if (!yaml_parser_initialize(&parser))
        return 0;

    if (!yaml_emitter_initialize(&emitter)) {
        yaml_parser_delete(&parser);
        return 0;
    }

    yaml_parser_set_input_string(&parser, data, size);

    yaml_output_buffer_t out = {/*buf=*/NULL, /*size=*/0, /*capacity=*/1000};
    yaml_emitter_set_output(&emitter, yaml_write_handler, &out);
    yaml_emitter_set_canonical(&emitter, is_canonical);
    yaml_emitter_set_unicode(&emitter, is_unicode);

    if (!yaml_emitter_open(&emitter))
        goto error;

    if (!yaml_document_initialize(&output_document, NULL, NULL, NULL, 0, 0))
        goto error;

    root = yaml_document_add_sequence(&output_document, NULL,
                                      YAML_BLOCK_SEQUENCE_STYLE);
    if (!root)
        goto error;

    printf("canonical=%d unicode=%d\n", is_canonical, is_unicode);

    /* Parse events and print them; also build the document tree */
    while (!done) {
        int properties, key, value;

        if (!yaml_parser_parse(&parser, &input_event)) {
            error = 1;
            break;
        }

        done = (input_event.type == YAML_STREAM_END_EVENT);

        /* Print the event */
        printf("%s", event_type_name(input_event.type));
        switch (input_event.type) {
        case YAML_SCALAR_EVENT:
            printf(" v=");
            fwrite(input_event.data.scalar.value, 1,
                   input_event.data.scalar.length, stdout);
            if (input_event.data.scalar.anchor)
                printf(" a=%s", input_event.data.scalar.anchor);
            if (input_event.data.scalar.tag)
                printf(" t=%s", input_event.data.scalar.tag);
            break;
        case YAML_ALIAS_EVENT:
            if (input_event.data.alias.anchor)
                printf(" a=%s", input_event.data.alias.anchor);
            break;
        case YAML_SEQUENCE_START_EVENT:
            if (input_event.data.sequence_start.anchor)
                printf(" a=%s", input_event.data.sequence_start.anchor);
            if (input_event.data.sequence_start.tag)
                printf(" t=%s", input_event.data.sequence_start.tag);
            break;
        case YAML_MAPPING_START_EVENT:
            if (input_event.data.mapping_start.anchor)
                printf(" a=%s", input_event.data.mapping_start.anchor);
            if (input_event.data.mapping_start.tag)
                printf(" t=%s", input_event.data.mapping_start.tag);
            break;
        default:
            break;
        }
        printf("\n");
        count++;

        /* Build document node for this event */
        properties = yaml_document_add_mapping(&output_document, NULL,
                                               YAML_BLOCK_MAPPING_STYLE);
        if (!properties) goto error;
        if (!yaml_document_append_sequence_item(&output_document, root, properties))
            goto error;

        key = yaml_document_add_scalar(&output_document, NULL,
                                       (yaml_char_t *)"type", -1,
                                       YAML_PLAIN_SCALAR_STYLE);
        if (!key) goto error;
        value = yaml_document_add_scalar(&output_document, NULL,
                                         (yaml_char_t *)event_type_name(input_event.type), -1,
                                         YAML_PLAIN_SCALAR_STYLE);
        if (!value) goto error;
        if (!yaml_document_append_mapping_pair(&output_document, properties, key, value))
            goto error;

        yaml_event_delete(&input_event);
    }

    printf("EVENTS %d\n", count);

    /* Dump the document tree */
    if (!yaml_emitter_dump(&emitter, &output_document))
        error = 1;

    yaml_emitter_close(&emitter);

    /* Print the dumped output */
    if (out.buf && out.size > 0) {
        printf("DUMPED %zu\n", out.size);
        fwrite(out.buf, 1, out.size, stdout);
        if (out.size > 0 && out.buf[out.size - 1] != '\n')
            printf("\n");
    }

    printf("---\n%s\n", error ? "FAILURE" : "SUCCESS");

    free(out.buf);
    yaml_event_delete(&input_event);
    yaml_document_delete(&output_document);
    yaml_parser_delete(&parser);
    yaml_emitter_delete(&emitter);
    return 0;

error:
    printf("---\nFAILURE (error) %d events\n", count);
    free(out.buf);
    yaml_event_delete(&input_event);
    yaml_document_delete(&output_document);
    yaml_parser_delete(&parser);
    yaml_emitter_delete(&emitter);
    return 0;
}
