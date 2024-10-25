#include <stdbool.h>
#include <stdio.h>

bool
prove_pow(int x, int y, char **resp);

int
main(int argc, char *argv[]) {
    char *resp;
    bool r = prove_pow(2, 3, &resp);
    if (!r) {
        return 1;
    }
    printf("%s\n", resp);
    return 0;
}
