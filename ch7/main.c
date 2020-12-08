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

struct bag {
    uint32_t bag_id;
    int32_t p1_state;
    uint32_t contain_cnt;
    struct {
        uint32_t bag_id;
        uint32_t bag_cnt;
    } contain[16];
};

struct bag data[MAX_DATA];
int data_len = 0;

#define MAX_LINE 256

#define MOD_HASH(acc, c) acc = (((acc) << 5) - (acc)) ^ (c)

int compare_bag(const void *a, const void *b) {
    uint32_t an = ((struct bag *) a)->bag_id;
    uint32_t bn = ((struct bag *) b)->bag_id;
    // assume no collisions
    if (an < bn) return -1;
    if (an > bn) return 1;
    __builtin_trap();
}

uint32_t scan_color(char **s) {
    char *orig = *s;
    uint32_t acc = 0;
    while (**s != ' ') {
        acc = ((acc << 5) - acc) ^ *((*s)++);
    }
    acc = ((acc << 5) - acc) ^ ' ';
    (*s)++;
    while (**s > ' ') {
        acc = ((acc << 5) - acc) ^ *((*s)++);
    }
    return acc;
}

int find_bag_idx(uint32_t bag_id) {
/*
    int min = 0;
    int max = data_len - 1;
    printf("finding %u\n", bag_id);
    while (1) {
        printf("picking...\n");
        int pick = (min + max) / 2;
        if (data[pick].bag_id == bag_id) return pick;
        else if (min == max) __builtin_trap();
        else if (data[pick].bag_id < bag_id) min = pick + 1;
        else if (data[pick].bag_id > bag_id) max = pick - 1;
    }
*/
    for (int i = 0; i < data_len; i++) if (data[i].bag_id == bag_id) return i;
    __builtin_trap();
}

static uint32_t shiny_gold_hash;

int is_p1_mark_idx(int idx);

int is_p1_mark(int id) {
    if (id == shiny_gold_hash) return 1;
    return is_p1_mark_idx(find_bag_idx(id));
}

int is_p1_mark_idx(int idx) {
    if (data[idx].p1_state == -1) {
        int acc = 0;
        for (int i = 0; i < data[idx].contain_cnt; i++) {
            if (is_p1_mark(data[idx].contain[i].bag_id)) {
                return data[idx].p1_state = 1;
            }
        }
        return data[idx].p1_state = 0;
    } else return data[idx].p1_state;
}

int count_bag(uint32_t id) {
    int idx = find_bag_idx(id);
    int acc = 1;
    for (int i = 0; i < data[idx].contain_cnt; i++) {
        acc += count_bag(data[idx].contain[i].bag_id) * data[idx].contain[i].bag_cnt;
    }
    return acc;
}

void calc_sgh() {
    char *ptr = "shiny gold";
    shiny_gold_hash = scan_color(&ptr);
}

int main(int argc, char **argv) {
    calc_sgh();
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    while (fgets(line, MAX_LINE, fd) != NULL) {
        if (line[0] <= '\n') continue;
        char *ptr = line;
        uint32_t head_id = scan_color(&ptr);
        ptr += 14;
        if (!strcmp(ptr, "no other bags.\n")) {
            data[data_len++] = (struct bag) {.bag_id = head_id, .p1_state = 0, .contain_cnt = 0};
        } else {
            data[data_len] = (struct bag) {.bag_id = head_id, .p1_state = -1, .contain_cnt = 0};
            do {
                while (*ptr < '0') ptr++;
                data[data_len].contain[data[data_len].contain_cnt].bag_cnt = 0;
                while (*ptr != ' ') {
                    data[data_len].contain[data[data_len].contain_cnt].bag_cnt *= 10;
                    data[data_len].contain[data[data_len].contain_cnt].bag_cnt += *ptr - '0';
                    ptr++;
                }
                ptr++;
                data[data_len].contain[data[data_len].contain_cnt++].bag_id = scan_color(&ptr);
                ptr += 4;
                if (*ptr == 's') ptr++;
            } while (*ptr != '.');
            data_len++;
        }
    }
    int p1_acc = 0;
    qsort(data, data_len, sizeof(struct bag), &compare_bag);
    for (int i = 0; i < data_len; i++) {
        p1_acc += is_p1_mark_idx(i);
    }
    printf("P1: %d\n", (int) p1_acc);
    printf("P2: %d\n", count_bag(shiny_gold_hash) - 1);
    return 0;
}
