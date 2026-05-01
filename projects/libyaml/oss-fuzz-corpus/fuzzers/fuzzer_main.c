/* fuzzer_main.c — CLI wrapper for libFuzzer-style fuzz targets.
 * Reads a file and passes its contents to LLVMFuzzerTestOneInput().
 * Usage: ./fuzzer_binary input_file
 */
#include <stdio.h>
#include <stdlib.h>

extern int LLVMFuzzerTestOneInput(const unsigned char *data, size_t size);

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <input_file>\n", argv[0]);
        return 1;
    }
    FILE *f = fopen(argv[1], "rb");
    if (!f) {
        fprintf(stderr, "Cannot open %s\n", argv[1]);
        return 1;
    }
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    unsigned char *buf = malloc(len);
    if (len > 0 && !buf) {
        fclose(f);
        return 1;
    }
    fread(buf, 1, len, f);
    fclose(f);
    LLVMFuzzerTestOneInput(buf, len);
    free(buf);
    return 0;
}
