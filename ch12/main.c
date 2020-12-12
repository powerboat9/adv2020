#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <stdint.h>
#include <limits.h>
#include <math.h>

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
    int p1_x = 0, p1_y = 0;
    int p1_dx = 1, p1_dy = 0;
    int p2_x = 0, p2_y = 0;
    int way_x = 10, way_y = 1;
    while (fgets(line, MAX_LINE, fd) != NULL) {
        char c;
        unsigned int n;
        if (sscanf(line, "%c%u", &c, &n) != 2) break;
        switch (c) {
            case 'N': p1_y += n; way_y += n; break;
            case 'S': p1_y -= n; way_y -= n; break;
            case 'E': p1_x += n; way_x += n; break;
            case 'W': p1_x -= n; way_x -= n; break;
            case 'R': n = 360 - n;
            case 'L':
                switch (n) {
                    int tmp;
                    case 90:
                        tmp = p1_dx;
                        p1_dx = -p1_dy;
                        p1_dy = tmp;
                        // p2
                        tmp = way_x;
                        way_x = -way_y;
                        way_y = tmp;
                        break;
                    case 180:
                        p1_dx = -p1_dx;
                        p1_dy = -p1_dy;
                        // p2
                        way_x = -way_x;
                        way_y = -way_y;
                        break;
                    case 270:
                        tmp = p1_dx;
                        p1_dx = p1_dy;
                        p1_dy = -tmp;
                        // p2
                        tmp = way_x;
                        way_x = way_y;
                        way_y = -tmp;
                        break;
                    default: __builtin_trap();
                }
                break;
            case 'F':
                p1_x += n * p1_dx;
                p1_y += n * p1_dy;
                // p2
                p2_x += n * way_x;
                p2_y += n * way_y;
                break;
            default: __builtin_trap();
        }
    }
    // part one
    printf("P1: %d\n", abs(p1_x) + abs(p1_y));
    // part two
    printf("P2: %d\n", abs(p2_x) + abs(p2_y));
    return 0;
}
