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
    while (fscanf(fd, " %[FBLR] ", data) > 0) {
        int row = 0;
        for (int i = 0; i < 7; i++) {
            row <<= 1;
            row |= (data[i] == 'B');
        }
        int col = 0;
        for (int i = 7; i < 10; i++) {
            col <<= 1;
            col |= (data[i] == 'R');
        }
        int v = row * 8 + col;
        if (v > max) max = v;
        found[v] = 1;
    }
    printf("P1: %d\n", max);
    int i = 0;
    while (!found[i]) i++;
    while (found[i]) i++;
    printf("P2: %d\n", i);
    return 0;
}
