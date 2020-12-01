#include <stdlib.h>
#include <stdio.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

#define MAX_DATA 4096

int data_store[MAX_DATA];
size_t data_pos = 0;

void part_one() {
    for (size_t i = 0; i < data_pos; i++) {
        for (size_t j = 0; j < i; j++) {
            if ((data_store[i] + data_store[j]) == 2020) {
                printf("P1: %d\n", data_store[i] * data_store[j]);
                return;
            }
        }
    }
}

void part_two() {
    for (size_t i = 0; i < data_pos; i++) {
        for (size_t j = i + 1; j < data_pos; j++) {
            int im = data_store[i] + data_store[j];
            if (im > 2020) continue;
            for (size_t k = j + 1; k < data_pos; k++) {
                if ((im + data_store[k]) == 2020) {
                    printf("P2: %d\n", data_store[i] * data_store[j] * data_store[k]);
                    return;
                }
            }
        }
    }
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("data.txt", "r");
    if (fd == NULL) die("failed to read");
    while (1) {
        if (data_pos == MAX_DATA) die("too many numbers");
        switch (fscanf(fd, "%d", &data_store[data_pos])) {
            case EOF:
                if (ferror(fd)) die("failed to read");
                goto done;
            case 0:
                die("failed to match");
        }
        data_pos++;
    }
    done:
    // part 2
    part_two();
}
