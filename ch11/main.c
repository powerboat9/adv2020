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

#define MAX_X 128
#define MAX_Y 4096

uint8_t data_orig[MAX_X*MAX_Y];
uint8_t data_1[MAX_X*MAX_Y];
uint8_t data_2[MAX_X*MAX_Y];
uint8_t *data_cur = data_orig;
uint8_t *data_nxt = data_1;
int row_cnt = 0;

#define MAX_LINE 512

#define FLOOR 0
#define EMPTY 1
#define FULL 2

void restore() {
    data_cur = data_orig;
    data_nxt = data_1;
}

void swap() {
    data_cur = (data_cur == data_1) ? data_2 : data_1;
    data_nxt = (data_nxt == data_1) ? data_2 : data_1;
}

uint8_t *next_seat(int x, int y, int dx, int dy, int is_p1) {
    while (1) {
        x += dx;
        y += dy;
        if ((x < 0) || (x >= MAX_X) || (y < 0) || (y >= row_cnt)) return NULL;
        uint8_t *r = data_cur + (y*MAX_X+x);
        if (*r != FLOOR) return r;
        if (is_p1) return NULL;
    }
}

int find_adj(int is_p1, int x, int y) {
    int acc = 0;
    for (int dx = -1; dx < 2; dx++) {
        for (int dy = -1; dy < 2; dy++) {
            if (dx | dy) {
                uint8_t *ptr = next_seat(x, y, dx, dy, is_p1);
                if (ptr && (*ptr == FULL)) acc++;
            }
        }
    }
    return acc;
}

int tick(int is_p1) {
    for (int y = 0; y < row_cnt; y++) {
        for (int x = 0; x < MAX_X; x++) {
            switch (data_cur[y*MAX_X+x]) {
                case FLOOR: data_nxt[y*MAX_X+x] = FLOOR; break;
                case EMPTY: data_nxt[y*MAX_X+x] = find_adj(is_p1, x, y) ? EMPTY : FULL; break;
                case FULL: data_nxt[y*MAX_X+x] = ((find_adj(is_p1, x, y) + is_p1) >= 5) ? EMPTY : FULL; break;
                default: __builtin_trap();
            }
        }
    }
    uint8_t *old_nxt = data_nxt;
    int r = !!memcmp(data_cur, data_nxt, row_cnt*MAX_X);
    swap();
    return r;
}

void prt() {
    for (int y = 0; y < row_cnt; y++) {
        for (int x = 0; x < MAX_X; x++)
        switch (data_cur[y*MAX_X+x]) {
            case FLOOR: putchar('.'); break;
            case EMPTY: putchar('L'); break;
            case FULL: putchar('@'); break;
        }
        putchar('\n');
    }
}

int cnt_occ() {
    int acc = 0;
    for (int i = 0; i < MAX_X*row_cnt; i++) {
        acc += data_cur[i] == FULL;
    }
    return acc;
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    while (fgets(line, MAX_LINE, fd) != NULL) {
        for (int i = 0; line[i] > '\n'; i++) {
            if (i == MAX_X) die("overflow");
            uint8_t v;
            switch (line[i]) {
                case 'L': v = EMPTY; break;
                case '#': v = FULL; break;
                default: v = FLOOR; break;
            }
            data_orig[MAX_X*row_cnt+i] = v;
        }
        row_cnt++;
    }
    // part one
    while (tick(1)) {}
    printf("P1: %d\n", cnt_occ());
    restore();
    // part two
    while (tick(0)) {}
    printf("P2: %d\n", cnt_occ());
    return 0;
}
