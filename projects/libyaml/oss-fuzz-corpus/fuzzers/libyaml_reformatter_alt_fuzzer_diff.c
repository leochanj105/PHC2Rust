/*
 * libyaml_reformatter_alt_fuzzer_diff.c — Document-level load→dump roundtrip.
 * Prints the dumped output buffer for comparison.
 * Based on: google/oss-fuzz/projects/libyaml/libyaml_reformatter_alt_fuzzer.c
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
    yaml_document_t document;
    int error = 0;

    if (!yaml_parser_initialize(&parser))
        return 0;

    if (!yaml_emitter_initialize(&emitter))
        goto cleanup_parser;

    yaml_parser_set_input_string(&parser, data, size);

    yaml_output_buffer_t out = {/*buf=*/NULL, /*size=*/0, /*capacity=*/1000};
    yaml_emitter_set_output(&emitter, yaml_write_handler, &out);

    yaml_emitter_set_canonical(&emitter, is_canonical);
    yaml_emitter_set_unicode(&emitter, is_unicode);

    while (!done) {
        if (!yaml_parser_load(&parser, &document)) {
            error = 1;
            break;
        }

        done = (!yaml_document_get_root_node(&document));

        if (!yaml_emitter_dump(&emitter, &document)) {
            error = 1;
            break;
        }
    }

    printf("canonical=%d unicode=%d\n", is_canonical, is_unicode);
    if (out.buf && out.size > 0) {
        printf("OUTPUT %zu\n", out.size);
        fwrite(out.buf, 1, out.size, stdout);
        if (out.size > 0 && out.buf[out.size - 1] != '\n')
            printf("\n");
    }
    printf("---\n%s\n", error ? "FAILURE" : "SUCCESS");

    free(out.buf);
    yaml_emitter_delete(&emitter);

cleanup_parser:
    yaml_parser_delete(&parser);
    return 0;
}
