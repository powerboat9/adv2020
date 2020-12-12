#include <stdlib.h>
#include <stdio.h>

typedef __int128 num_t;

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

void swap(num_t *a, num_t *b) {
    num_t tmp = *a;
    *a = *b;
    *b = tmp;
}

// accending order
void my_qsort(num_t *a, size_t len) {
    switch (len) {
        case 2:
            if (a[0] > a[1]) swap(a, a+1);
        case 0:
        case 1:
            return;
        default:;
            num_t mid = a[0];
            size_t nxt = 1;
            for (size_t i = 1; i < len; i++) {
                if (a[i] <= mid) {
                    swap(a+nxt, a+i);
                    nxt++;
                }
            }
            swap(a, a+nxt-1);
            my_qsort(a, nxt-1);
            my_qsort(a+nxt, len-nxt);
    }
}

int check_for(num_t *a, size_t len, num_t val) {
    if (len == 0) return 0;
    else if (a[len/2] == val) return 1;
    else if (a[len/2] < val) return check_for(a + len/2 + 1, (len-1)/2, val);
    else return check_for(a, len/2, val);
}

#define TARGET 99920044L

#define MAX_DATA 1024L * 1024L

num_t data_store[MAX_DATA];
size_t data_pos = 0;

void part_one() {
    my_qsort(data_store, data_pos);
    for (size_t i = 0; i < data_pos; i++) {
        num_t r = TARGET - data_store[i];
        if (check_for(data_store+i, data_pos-i, r)) {
            printf("P1: %lld\n", (long long) (data_store[i] * r));
        }
    }
}

void print_num(num_t n) {
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

void part_two() {
    my_qsort(data_store, data_pos);
    for (size_t i = 0; (i < data_pos) && (data_store[i] <= (TARGET/3)); i++) {
        num_t r = TARGET - data_store[i];
        for (size_t j = i + 1; (j < data_pos) && (data_store[j] <= (r/2)); j++) {
            num_t rr = r - data_store[j];
            if (check_for(data_store+j+1, data_pos-j-1, rr)) {
                printf("P2: ");
                print_num(data_store[i] * data_store[j] * rr);
                printf("\n");
            }
        }
    }
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("bb.txt", "r");
    if (fd == NULL) die("failed to read");
    while (1) {
        if (data_pos == MAX_DATA) die("too many numbers");
        long long n;
        switch (fscanf(fd, "%lld", &n)) {
            case EOF:
                if (ferror(fd)) die("failed to read");
                goto done;
            case 0:
                die("failed to match");
        }
        data_store[data_pos] = (num_t) n;
        data_pos++;
    }
    done:
    // part 2
    part_two();
    return 0;
}
