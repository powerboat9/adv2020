#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <complex.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

FILE *map_file(const char *filename) {
    int fd;
    if ((fd = open(filename, O_RDONLY)) == -1) die("failed to open file");
    struct stat fd_stat;
    if (fstat(fd, &fd_stat) == -1) die("failed to stat file");
    void *data;
    // never unmapped, probably fine
    if ((data = mmap(NULL,
                     fd_stat.st_size,
                     PROT_READ,
                     MAP_PRIVATE,
                     fd, 0)) == MAP_FAILED) die("failed to map file");
    close(fd);
    return fmemopen(data, fd_stat.st_size, "r");
}

#define MAX_LINE 32

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    _Complex int p1_s = 0;
    _Complex int p1_v = 1;
    _Complex int p2_s = 0;
    _Complex int p2_v = 10+1i;
    while (fgets(line, MAX_LINE, fd) != NULL) {
        char c;
        unsigned int n;
        if (sscanf(line, "%c%u", &c, &n) != 2) break;
// generates case for cardinal direction
#define PMSK(c, v) case c: p1_s += n * v; p2_v += n * v; break;
// generates case for ccw angle change
#define RMSK(a, v) case a: p1_v *= v; p2_v *= v; break;
// generates switch statements with trap on default
#define SW_SAFE_END default: __builtin_trap(); }
#define SW_SAFE(sw, body) switch (sw) { body SW_SAFE_END
        switch (c) {
            PMSK('N', 1i) PMSK('S', -1i) PMSK('E', 1) PMSK('W', -1)
            case 'R': n = 360 - n;
            case 'L':
                SW_SAFE(n, RMSK(90, 1i) RMSK(180, -1) RMSK(270, -1i))
                break;
            case 'F':
                p1_s += n * p1_v;
                p2_s += n * p2_v;
                break;
        SW_SAFE_END
    }
    // part one
    printf("P1: %d\n", abs(__real__ p1_s) + abs(__imag__ p1_s));
    // part two
    printf("P2: %d\n", abs(__real__ p2_s) + abs(__imag__ p2_s));
    return 0;
}
