#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
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

int starts_with(char *s, char *prefix) {
    if (!*prefix) return 1;
    else return (*s == *prefix) ? starts_with(s + 1, prefix + 1) : 0;
}

#define MAX_TYPES 32
#define MAX_TYPE_NAME 32
#define TICKET_ARGS 20
#define OTH_TICK_MAX 4096

#define COL_FIELD_DEF ((1 << TICKET_ARGS) - 1)

struct ticket_type {
    char name[MAX_TYPE_NAME];
    unsigned int r1s;
    unsigned int r1l;
    unsigned int r2s;
    unsigned int r2l;
    unsigned int col_field;
};

struct ticket_type ttypes[MAX_TYPES];
int type_cnt = 0;

struct ticket {
    unsigned int vals[TICKET_ARGS];
};

struct ticket my_ticket;
struct ticket other_tickets[OTH_TICK_MAX];
int tick_cnt = 0;

#define MAX_LINE 128

char line[MAX_LINE];

int read_line(FILE *fd) {
    return fgets(line, MAX_LINE, fd) ? 1 : 0;
}

int read_ticket(struct ticket *tk, FILE *fd) {
    if (!read_line(fd)) return 0;
    int idx = 0;
    char *ptr = line;
    unsigned int v;
    int r;
    while (sscanf(ptr, "%u%*[,]%n", &v, &r) == 1) {
        ptr += r;
        tk->vals[idx++] = v;
    }
    return 1;
}

int read_until(char *start, FILE *fd) {
    if (!read_line(fd)) return 0;
    else if (*line <= '\n') return read_until(start, fd);
    else if (starts_with(line, start)) return 0;
    else return 1;
}

unsigned int p2_acc = 1;
int check_idx(int ok_map, int idx) {
    for (unsigned int tr = 0; tr < TICKET_ARGS; tr++) {
        for (int i = 0; i < (idx + 1); i++) putchar('>');
        printf(" %d\n", tr);
        if (!(ttypes[idx].col_field & ok_map & (1 << tr))) continue;
        if ((idx + 1) != type_cnt) {
            if (!check_idx(ok_map & ~(1 << tr), idx + 1)) continue;
        }
        // it works
        if (starts_with(ttypes[idx].name, "departure")) {
            printf("> %s\n", ttypes[idx].name);
            printf("(%u) => ", p2_acc);
            p2_acc *= my_ticket.vals[tr];
            printf("(%u)\n", p2_acc);
        }
        return 1;
    }
    return 0;
}

int does_fit(int n, struct ticket_type *tkt) {
    return ((tkt->r1s <= n) && (tkt->r1l >= n)) || ((tkt->r2s <= n) && (tkt->r2l >= n));
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    // handle data types
    while (read_until("your ticket:", fd)) {
        int i;
        for (i = 0; ; i++) {
            if (line[i] == ':') {
                ttypes[type_cnt].name[i] = '\0';
                break;
            } else {
                ttypes[type_cnt].name[i] = line[i];
            }
        }
        sscanf(line + i + 1, "%u-%u or %u-%u",
               &ttypes[type_cnt].r1s,
               &ttypes[type_cnt].r1l,
               &ttypes[type_cnt].r2s,
               &ttypes[type_cnt].r2l);
        ttypes[type_cnt].col_field = COL_FIELD_DEF;
        type_cnt++;
    }
    // handle "your ticket"
    read_ticket(&my_ticket, fd);
    read_line(fd);
    // handle "nearby tickets"
    // also p1
    read_line(fd);
    int p1_acc = 0;
    while (read_ticket(&other_tickets[tick_cnt], fd)) {
        int ok = 1;
        for (int i = 0; i < TICKET_ARGS; i++) {
            int v = other_tickets[tick_cnt].vals[i];
            for (int j = 0; j < type_cnt; j++) {
                if (does_fit(v, &ttypes[j])) goto p1_ok;
            }
            p1_acc += v;
            ok = 0;
            continue;
            p1_ok: continue;
        }
        if (ok) tick_cnt++;
    }
    printf("P1: %u\n", p1_acc);
    // p2
    for (int tk = 0; tk < tick_cnt; tk++) {
        for (int idx = 0; idx < TICKET_ARGS; idx++) {
            unsigned int v = other_tickets[tk].vals[idx];
            for (int ty = 0; ty < type_cnt; ty++) {
                if (!does_fit(v, &ttypes[ty])) {
                    ttypes[ty].col_field &= ~(1L << idx);
                }
            }
        }
    }
    // efficiency
    int chng;
    do {
        chng = 0;
        for (int ty = 0; ty < type_cnt; ty++) {
            unsigned int slf = ttypes[ty].col_field;
            unsigned int oth = 0;
            for (int ty2 = 0; ty2 < type_cnt; ty2++) {
                if (ty == ty2) continue;
                oth |= ttypes[ty2].col_field;
            }
            unsigned int only = slf & ~oth;
            if (only && (only != slf)) {
                ttypes[ty].col_field = only;
                chng = 1;
            }
        }
    } while (chng);
    // internal data display
    for (int ty = 0; ty < type_cnt; ty++) {
        printf("(%02d):", ty);
        unsigned int tmp = ttypes[ty].col_field;
        for (int i = 0; i < 32; i++) {
            putchar(' ');
            putchar((tmp & 1) ? '1' : '0');
            tmp >>= 1;
        }
        putchar('\n');
    }
    // attempt to solve simple way
    int p2aac = 1;
    for (int ty = 0; ty < type_cnt; ty++) {
        printf("$$ %s: %d\n", ttypes[ty].name, (int) __builtin_ctz(ttypes[ty].col_field));
        if (!starts_with(ttypes[ty].name, "departure")) continue;
        int si = ttypes[ty].col_field;
        printf("> si: %d\n", si);
        if (si & (si - 1)) die("FOOP");
        unsigned int *v_ptr = my_ticket.vals;
        while (si >>= 1) v_ptr++;
        p2aac *= *v_ptr;
    }
    printf("P@A: %d\n", p2aac);
/*
    for (int ty = 0; ty < type_cnt; ty++) {
        int v = ttypes[ty].col_field;
        if (!(v & (v - 1))) printf("FOLD: %d\n", ty);
    }
*/
    // p2 calculation
    if (!check_idx(COL_FIELD_DEF, 0)) die("p2 fail");
    printf("P2: %d\n", p2_acc);
    return 0;
}
