#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
#include <complex.h>
#include <stdint.h>
#include <inttypes.h>

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

int starts_with(const char *s, const char *prefix) {
    return (!*prefix) || ((*s == *prefix) && starts_with(s+1, prefix+1));
}

struct node {
    union {
        struct node *ptr[2];
        uint64_t val;
    } inner;
};

typedef unsigned __int128 u128_t;

void prt_addr(uint64_t a) {
    for (int i = 0; i < 36; i++) {
        int addr_part = (a >> (35 - i)) & 1;
        putchar(("01")[addr_part]);
    }
}

void prt_mi(u128_t n) {
    for (int i = 0; i < 36; i++) {
        int msk_part = (n >> (70 - i * 2)) & 3;
        putchar(("01X#")[msk_part]);
    }
}

u128_t make_idx(u128_t mask, uint64_t addr) {
    u128_t ret = 0;
    for (int i = 0; i < 36; i++) {
        int msk_part = (mask >> (i * 2)) & 3;
        int addr_part = (addr >> i) & 1;
        ret |= ((u128_t) (msk_part ? msk_part : addr_part)) << (i * 2);
    }
    return ret;
}

struct node *create_node() {
    struct node *ret = (struct node *) malloc(sizeof(struct node));
    ret->inner.ptr[0] = NULL;
    ret->inner.ptr[1] = NULL;
    return ret;
}

struct node *clone_node(struct node *n, int depth);

void insert(u128_t idx, struct node *n, uint64_t val, int depth) {
    if (depth == 36) {
        printf("INS: ");
        prt_mi(idx);
        printf("\nVAL: %lu\n", val);
    }
    if (depth) {
        int idx_part = (idx >> (depth * 2 - 2)) & 3;
        switch (idx_part) {
            case 0:
            case 1:
                if (!n->inner.ptr[idx_part]) n->inner.ptr[idx_part] = create_node();
                else if (n->inner.ptr[0] == n->inner.ptr[1]) {
                    n->inner.ptr[0] = clone_node(n->inner.ptr[0], depth - 1);
                }
                insert(idx, n->inner.ptr[idx_part], val, depth - 1);
                return;
            case 2:
                if (n->inner.ptr[0] == n->inner.ptr[1]) {
                    if (!n->inner.ptr[0]) n->inner.ptr[0] = n->inner.ptr[1] = create_node();
                    insert(idx, n->inner.ptr[0], val, depth - 1);
                    return;
                } else {
                    if (!n->inner.ptr[0]) n->inner.ptr[0] = create_node();
                    else if (!n->inner.ptr[1]) n->inner.ptr[1] = create_node();
                    insert(idx, n->inner.ptr[0], val, depth - 1);
                    insert(idx, n->inner.ptr[1], val, depth - 1);
                    return;
                }
            default: __builtin_trap();
        }
    } else {
        n->inner.val = val;
    }
}

struct node *clone_node(struct node *n, int depth) {
    if (!n) return NULL;
    struct node *ret = create_node();
    if (depth) for (int i = 0; i < 2; i++) ret->inner.ptr[i] = clone_node(n->inner.ptr[i], depth - 1);
    else ret->inner.val = n->inner.val;
    return ret;
}

uint64_t cnt_node(struct node *n, int depth) {
    if (!n) return 0;
    if (!depth) return n->inner.val;
    if (n->inner.ptr[0] == n->inner.ptr[1]) return 2 * cnt_node(n->inner.ptr[0], depth - 1);
    return cnt_node(n->inner.ptr[0], depth - 1) + cnt_node(n->inner.ptr[1], depth - 1);
}

#define MAX_LINE 128

#define MAX_SPACE (1 << 16)

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    // p1
    uint64_t p1_space[MAX_SPACE];
    memset(p1_space, 0, sizeof(p1_space));
    uint64_t mask_and;
    uint64_t mask_or;
    // p2
    struct node p2_root = {.inner = {.ptr = {NULL, NULL}}};
    u128_t p2_mask;
    // main loop
    while (fgets(line, MAX_LINE, fd) != NULL) {
        char c;
        unsigned int n;
        if (starts_with(line, "mask = ")) {
            mask_and = 0;
            mask_or = 0;
            p2_mask = 0;
            for (int i = 7; line[i] > '\n'; i++) {
                mask_and <<= 1;
                mask_or <<= 1;
                p2_mask <<= 2;
                switch (line[i]) {
                    case '0':
                        break;
                    case 'X':
                        p2_mask |= 2;
                        mask_and |= 1;
                        break;
                    case '1':
                        p2_mask |= 1;
                        mask_and |= 1;
                        mask_or |= 1;
                }
            }
        } else if (starts_with(line, "mem[")) {
            uint64_t addr;
            uint64_t val;
            sscanf(line + 4, "%" SCNu64 "] = %" SCNu64, &addr, &val);
            if (addr >= MAX_SPACE) die("space fill");
            p1_space[addr] = (val & mask_and) | mask_or;
            // p2
            insert(make_idx(p2_mask, addr), &p2_root, val, 36);
        } // ignore empty lines
    }
    // part one
    uint64_t p1_acc = 0;
    for (int i = 0; i < MAX_SPACE; i++) p1_acc += p1_space[i];
    printf("P1: %" PRIu64 "\n", p1_acc);
    // part two
    printf("P2: %" PRIu64 "\n", cnt_node(&p2_root, 36));
    return 0;
}
