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

uint16_t data[MAX_DATA];
size_t data_len = 0;

int cmp_u16(const void *a_ptr, const void *b_ptr) {
    const uint16_t *a = a_ptr;
    const uint16_t *b = b_ptr;
    if (*a > *b) return 1;
    else if (*a < *b) return -1;
    else return 0;
}

#define MAX_LINE 64

typedef unsigned __int128 u128_t;

void print_num(u128_t n) {
    if (n == 0) {
        putchar('0');
        return;
    }
    char bank[512];
    bank[511] = '\0';
    char *ptr = bank + 511;
    while (n != 0) {
        *(--ptr) = (n % 10) + '0';
        n /= 10;
    }
    printf("%s", ptr);
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    data[data_len++] = 0;
    while (fgets(line, MAX_LINE, fd) != NULL) {
        unsigned int arg;
        if (sscanf(line, "%u", &arg) != 1) break;
        data[data_len++] = (uint16_t) arg;
    }
    qsort(data + 1, data_len - 1, sizeof(uint16_t), &cmp_u16);
    data[data_len] = data[data_len - 1] + 3;
    data_len++;
    /* everything here will break if
       the adaptors can not in fact
       be chained together, as part
       one's description assumes */
    // part one
    int p1_acc1 = 0;
    int p1_acc3 = 0;
    for (int i = 1; i < data_len; i++) {
        if ((data[i] - data[i - 1]) == 1) p1_acc1++;
        else if ((data[i] - data[i - 1]) == 3) p1_acc3++;
    }
    printf("P1: %d\n", p1_acc1 * p1_acc3);
    // part two
    u128_t p2_acc[4] = {1, 0, 0, 0};
    for (size_t i = 1; i < (data_len - 1); i++) {
        int diff = data[i] - data[i - 1];
        memmove(p2_acc + diff, p2_acc, (4 - diff) * sizeof(u128_t));
        for (int j = 0; j < diff; j++) {
            p2_acc[j] = 0;
        }
        p2_acc[0] += p2_acc[0] + p2_acc[1] + p2_acc[2] + p2_acc[3];
    }
    u128_t p2_r = 0;
    for (int i = 0; i < 4 - (data[data_len - 1] - data[data_len - 2]); i++) p2_r += p2_acc[i];
    printf("P2: ");
    print_num(p2_r);
    putchar('\n');
    return 0;
}
