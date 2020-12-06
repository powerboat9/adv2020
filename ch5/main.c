#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

#define SEAT_CNT 1024

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("test.txt", "r");
    if (fd == NULL) die("failed to read");
    char data[16];
    int min = SEAT_CNT;
    int max = -1;
    long acc = 0;
    while (fgets(data, 16, fd) != NULL) {
        int sid = 0;
        for (int i = 0; i < 10; i++) {
            sid <<= 1;
            sid |= ((data[i] >> 2) & 1);
        }
        sid ^= SEAT_CNT - 1;
        if (sid > max) max = sid;
        if (sid < min) min = sid;
        acc += sid;
    }
    printf("P1: %d\n", max);
    printf("P2: %d\n", (int) (((max - min + 1) * (min + max) / 2) - acc));
    return 0;
}
