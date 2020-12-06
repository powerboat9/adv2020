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

#define SEAT_CNT 1024

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char data[16];
    int min = SEAT_CNT;
    int max = -1;
    long acc = 0;
    while (fgets(data, 16, fd) != NULL) {
        int sid = 0;
        for (int i = 0; i < 10; i++) {
            sid <<= 1;
            sid |= ((data[i] >> 2) & 1);
        }
        sid ^= SEAT_CNT - 1;
        if (sid > max) max = sid;
        if (sid < min) min = sid;
        acc += sid;
    }
    printf("P1: %d\n", max);
    printf("P2: %d\n", (int) (((max - min + 1) * (min + max) / 2) - acc));
    return 0;
}
