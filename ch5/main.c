#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("test.txt", "r");
    if (fd == NULL) die("failed to read");
    char data[16];
    int max = -1;
    unsigned char found[1024];
    memset(found, 0, 1024);
    while (fgets(data, 16, fd) != NULL) {
        int sid = 0;
        for (int i = 0; i < 10; i++) {
            sid <<= 1;
            sid |= ((data[i] >> 2) & 1);
        }
        sid ^= 1023;
        if (sid > max) max = sid;
        found[sid] = 1;
    }
    printf("P1: %d\n", max);
    int i = 0;
    while (!found[i]) i++;
    while (found[i]) i++;
    printf("P2: %d\n", i);
    return 0;
}
