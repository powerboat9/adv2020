#include <stdlib.h>
#include <stdio.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

// accending order
void my_qsort(int *a, size_t len) {
    switch (len) {
        case 2:
            if (a[0] > a[1]) {
                int tmp = a[0];
                a[0] = a[1];
                a[1] = tmp;
            }
        case 0:
        case 1:
            return;
        default:;
            int mid = a[0];
            size_t nxt = 1;
            for (size_t i = 1; i < len; i++) {
                if (a[i] <= mid) {
                    int tmp = a[nxt];
                    a[nxt] = a[i];
                    a[i] = tmp;
                    nxt++;
                }
            }
            int tmp = a[0];
            a[0] = a[nxt-1];
            a[nxt-1] = tmp;
            my_qsort(a, nxt-1);
            my_qsort(a+nxt, len-nxt);
    }
}

int check_for(int *a, size_t len, int val) {
    if (len == 0) return 0;
    else if (a[len/2] == val) return 1;
    else if (a[len/2] < val) return check_for(a + len/2 + 1, (len-1)/2, val);
    else return check_for(a, len/2, val);
}

#define MAX_DATA 4096

int data_store[MAX_DATA];
size_t data_pos = 0;

void part_one() {
    my_qsort(data_store, data_pos);
    for (size_t i = 0; i < data_pos; i++) {
        int r = 2020 - data_store[i];
        if (check_for(data_store+i, data_pos-i, r)) {
            printf("P1: %d\n", data_store[i] * r);
        }
    }
}

void part_two() {
    my_qsort(data_store, data_pos);
    for (size_t i = 0; (i < data_pos) && (data_store[i] <= (2020/3)); i++) {
        int r = 2020 - data_store[i];
        for (size_t j = i + 1; (j < data_pos) && (data_store[j] <= (r/2)); j++) {
            int rr = r - data_store[j];
            if (check_for(data_store+j+1, data_pos-j-1, rr)) {
                printf("P2: %d\n", data_store[i] * data_store[j] * rr);
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
