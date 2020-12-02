#include <stdlib.h>
#include <stdio.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

#define MAX_DATA 4096

struct data_entry {
    unsigned int low;
    unsigned int high;
    char str[64];
    char c;
};

struct data_entry data[MAX_DATA];

size_t data_len = 0;

void part_one() {
    int acc = 0;
    for (size_t i = 0; i < data_len; i++) {
        int n = 0;
        for (int j = 0; data[i].str[j]; j++) {
            if (data[i].str[j] == data[i].c) n++;
        }
        if ((n >= data[i].low) && (n <= data[i].high)) acc++;
    }
    printf("P1: %d\n", acc);
}

void part_two() {
    int acc = 0;
    for (size_t i = 0; i < data_len; i++) {
        if ((data[i].str[data[i].low - 1] == data[i].c) ^ (data[i].str[data[i].high - 1] == data[i].c)) acc++;
    }
    printf("P2: %d\n", acc);
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("test.txt", "r");
    if (fd == NULL) die("failed to read");
    while (1) {
        struct data_entry v;
        switch (fscanf(fd, "%u-%u %c: %s", &v.low, &v.high, &v.c, v.str)) {
            case EOF:
                if (ferror(fd)) die("failed to read");
                goto done;
            case 4:
                break;
            default:
                die("failed to match");
        }
        if (data_len == MAX_DATA) die("data full");
        data[data_len++] = v;
    }
    done:
    part_two();
    return 0;
}
