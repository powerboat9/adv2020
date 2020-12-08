#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <stdint.h>

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

#define MAX_DATA 4096

#define INS_ACC 0
#define INS_JMP 1
#define INS_NOP 2

struct ins {
    uint32_t op;
    uint32_t val;
};

struct ins asm_data[MAX_DATA];
int asm_mark[MAX_DATA];
int asm_len = 0;

#define MAX_LINE 256

int exec_asm() {
    int pos = 0, acc = 0;
    while (1) {
        if ((pos > asm_len) || (pos < 0)) return 0;
        if (pos == asm_len) {
            printf("P2: %d\n", acc);
            exit(0);
        } else if (asm_mark[pos]) {
            memset(asm_mark, 0, sizeof(asm_mark));
            return acc;
        }
        asm_mark[pos] = 1;
        switch (asm_data[pos].op) {
            case INS_ACC: acc += asm_data[pos].val; pos++; break;
            case INS_JMP: pos += asm_data[pos].val; break;
            case INS_NOP: pos++; break;
        }
    }
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
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
    printf("P1: %d\n", exec_asm());
    for (int i = 0; i < asm_len; i++) {
        if (asm_data[i].op == INS_ACC) continue;
        asm_data[i].op ^= 3;
        exec_asm();
        asm_data[i].op ^= 3;
    }
    return 0;
}
