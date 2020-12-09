#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <stdint.h>
#include <setjmp.h>

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

#define MAX_DATA (1024L * 1024L * 32L)

#define INS_ACC 0
#define INS_JMP 1
#define INS_NOP 2

struct ins {
    uint32_t op;
    uint32_t val;
};

struct pause_data {
    uint32_t mark;
    uint32_t has_flipped;
    uint32_t source;
    uint32_t insta_block;
};

struct ins asm_data[MAX_DATA];
struct pause_data asm_pause[MAX_DATA];
long asm_len = 0;
long asm_idx = 0;

#define MAX_LINE 256

int exec_p1(int pos, int acc) {
    if ((pos > asm_len) || (pos < 0)) die("ip overflow");
    if (pos == asm_len) die("unexpected success");
    if (asm_mark[pos]) return acc;
    asm_mark[pos] = 1;
    int pos_n, acc_n;
    switch (asm_data[pos].op) {
        case INS_ACC: acc_n = acc + asm_data[pos].val; pos_n = pos + 1; break;
        case INS_JMP: pos_n = pos + asm_data[pos].val; break;
        case INS_NOP: pos_n = pos + 1; break;
    }
    printf("NEXT\n");
    int r = exec_p1(pos_n, acc_n);
    asm_mark[pos] = 0;
    return r;
}

int exec_p2(int pos, int acc, jmp_buf fail, int may_flip) {
    if ((pos > asm_len) || (pos < 0)) longjmp(fail, 1);
    else if (pos == asm_len) return acc;
    else if (asm_mark[pos]) longjmp(fail, 1);
    asm_mark[pos] = 1;
    jmp_buf my_fail;
    int r;
    switch (asm_data[pos].op) {
        case INS_ACC:
            if (setjmp(my_fail)) {
                asm_mark[pos] = 0;
                longjmp(fail, 1);
            } else {
                return exec_p2(pos + 1, acc + asm_data[pos].val, my_fail, may_flip);
            }
        case INS_JMP:
            if (setjmp(my_fail)) {
                if (!may_flip || setjmp(my_fail)) {
                    asm_mark[pos] = 0;
                    longjmp(fail, 1);
                } else {
                    return exec_p2(pos + 1, acc, my_fail, 0);
                }
            } else {
                return exec_p2(pos + asm_data[pos].val, acc, my_fail, may_flip);
            }
        case INS_NOP:
            if (setjmp(my_fail)) {
                if (!may_flip || setjmp(my_fail)) {
                    asm_mark[pos] = 0;
                    longjmp(fail, 1);
                } else {
                    return exec_p2(pos + asm_data[pos].val, acc, my_fail, 0);
                }
            } else {
                return exec_p2(pos + 1, acc, my_fail, may_flip);
            }
    }
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("bb.txt");
    char line[MAX_LINE];
    while (fgets(line, MAX_LINE, fd) != NULL) {
        char optxt[8];
        int arg;
        if (sscanf(line, "%s %d", optxt, &arg) != 2) break;
        asm_data[asm_len].val = arg;
        switch (*optxt) {
            case '\n':
            case '\0': break;
            case 'a': asm_data[asm_len++].op = INS_ACC; break;
            case 'j': asm_data[asm_len++].op = INS_JMP; break;
            case 'n': asm_data[asm_len++].op = INS_NOP; break;
        }
    }
    printf("P1: %d\n", exec_p1(0, 0));
    printf("P2: %d\n", exec_p2(0, 0, NULL, 1));
    return 0;
}
