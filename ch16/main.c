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

struct ticket_type {
    char name[MAX_TYPE_NAME];
    unsigned int r1s;
    unsigned int r1l;
    unsigned int r2s;
    unsigned int r2l;
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
        type_cnt++;
    }
    // handle "your ticket"
    read_ticket(&my_ticket, fd);
    read_line(fd);
    // handle "nearby tickets"
    read_line(fd);
    while (read_ticket(&other_tickets[tick_cnt], fd)) tick_cnt++;
    // p1
    int p1_acc = 0;
    for (int i = 0; i < tick_cnt; i++) {
        for (int j = 0; j < TICKET_ARGS; j++) {
            int v = other_tickets[i].vals[j];
            for (int k = 0; k < type_cnt; k++) {
                if (((ttypes[k].r1s <= v) && (ttypes[k].r1l >= v)) || ((ttypes[k].r2s <= v) && (ttypes[k].r2l >= v))) goto p1_ok;
            }
            p1_acc += v;
            p1_ok: continue;;
        }
    }
    printf("P1: %u\n", p1_acc);
    return 0;
}
