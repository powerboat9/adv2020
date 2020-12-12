#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>

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

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char data[64];
    int any_acc = 0;
    int all_acc = 0;
    unsigned int cur_any = 0;
    unsigned int cur_all = ~0;
    while (fgets(data, 64, fd) != NULL) {
        if (data[0] < 'a') {
            any_acc += __builtin_popcount(cur_any);
            all_acc += __builtin_popcount(cur_all);
            cur_any = 0;
            cur_all = ~0;
        } else {
            unsigned int row = 0;
            for (int i = 0; data[i] >= 'a'; i++) {
                row |= 1 << (data[i] - 'a');
            }
            cur_any |= row;
            cur_all &= row;
        }
    }
    any_acc += __builtin_popcount(cur_any);
    all_acc += __builtin_popcount(cur_all);
    printf("P1: %d\n", any_acc);
    printf("P2: %d\n", all_acc);
    return 0;
}
