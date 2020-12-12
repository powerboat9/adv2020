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

#define MAX_DATA 4096

uint64_t data[MAX_DATA];
size_t data_len = 0;

uint64_t sort_data[MAX_DATA];

void insert_sorted(uint64_t n) {
    size_t min = 0;
    size_t max = data_len;
    size_t idx;
    while (min != max) {
        size_t pick = (min + max) / 2;
        if (sort_data[pick] > n) {
            if (!pick) {
                min = 0;
                break;
            } else max = pick;
        } else if (sort_data[pick] < n) {
            min = pick + 1;
        } else {
            min = pick;
            break;
        }
    }
    uint64_t store = n;
    for (size_t i = min; i <= data_len; i++) {
        uint64_t tmp = sort_data[i];
        sort_data[i] = store;
        store = tmp;
    }
    data[data_len++] = n;
}

int cmp_u64(const void *a_ptr, const void *b_ptr) {
    const uint64_t *a = a_ptr;
    const uint64_t *b = b_ptr;
    if (*a > *b) return 1;
    else if (*a < *b) return -1;
    else return 0;
}

int has_sum(uint64_t n) {
    for (size_t i = 0; i < data_len; i++) {
        if (sort_data[i] > (n / 2)) return 0;
        uint64_t r = n - sort_data[i];
        if (bsearch(&r, sort_data, data_len, sizeof(*sort_data), &cmp_u64) != NULL) return 1;
    }
    return 0;
}

#define MAX_LINE 64

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    uint64_t p1 = 0;
    while (fgets(line, MAX_LINE, fd) != NULL) {
        unsigned long arg;
        if (sscanf(line, "%lu", &arg) != 1) break;
        uint64_t n = (uint64_t) arg;
        if (p1) {
            data[data_len++] = n;
        } else {
            if ((data_len >= 25) && !has_sum(n)) {
                p1 = arg;
                data[data_len++] = n;
            } else insert_sorted(n);
        }
    }
    printf("P1: %lu\n", p1);
    int min = 0, max = 0;
    uint64_t sum = 0;
    while (max <= data_len) {
        if (sum < p1) {
            sum += data[max++];
        } else if (sum > p1) {
            sum -= data[min++];
        } else {
            uint64_t rmin = ULONG_MAX, rmax = 0;
            for (size_t i = min; i < max; i++) {
                if (data[i] > rmax) rmax = data[i];
                if (data[i] < rmin) rmin = data[i];
            }
            printf("P2: %lu\n", rmin + rmax);
            break;
        }
    }
    return 0;
}
