#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

bool
receipt_verify(const char *);

int
main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "usage: %s <receipt.json>\n", argv[0]);
        return 1;
    }

    FILE *f = fopen(argv[1], "r");
    if (f == NULL) {
        perror("fopen");
        return 1;
    }

    fseek(f, 0, SEEK_END);
    long file_size = ftell(f);
    fseek(f, 0, SEEK_SET);

    char* buffer = (char*)malloc(file_size + 1);
    if (buffer == NULL) {
        perror("Failed to allocate memory");
        fclose(f);
        return 1;
    }

    size_t bytes_read = fread(buffer, 1, file_size, f);
    if (bytes_read != file_size) {
        perror("Failed to read the entire file");
        free(buffer);
        fclose(f);
        return 1;
    }

    // Null-terminate the string
    buffer[file_size] = '\0';

    fclose(f);

    bool valid = receipt_verify(buffer);
    printf("is receipt OK: %d\n", valid);

    return 0;
}
