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
    unsigned char bins_any[26];
    unsigned char bins_all[26];
    memset(bins_any, 0, 26);
    memset(bins_all, 1, 26);
    int any_acc = 0;
    int all_acc = 0;
    while (fgets(data, 64, fd) != NULL) {
        if (data[0] < 'a') {
            for (int i = 0; i < 26; i++) {
                any_acc += bins_any[i];
                all_acc += bins_all[i];
            }
            memset(bins_any, 0, 26);
            memset(bins_all, 1, 26);
        } else {
            unsigned char min_bins[26];
            memset(min_bins, 0, 26);
            for (int i = 0; data[i] >= 'a'; i++) {
                bins_any[data[i] - 'a'] = min_bins[data[i] - 'a'] = 1;
            }
            for (int i = 0; i < 26; i++) {
                bins_all[i] &= min_bins[i];
            }
        }
    }
    for (int i = 0; i < 26; i++) {
        any_acc += bins_any[i];
        all_acc += bins_all[i];
    }
    printf("P1: %d\n", any_acc);
    printf("P2: %d\n", all_acc);
    return 0;
}
