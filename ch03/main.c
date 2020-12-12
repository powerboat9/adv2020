#include <stdlib.h>
#include <stdio.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

#define X_SIZE 31
#define Y_MAX 4096

unsigned char data[X_SIZE * Y_MAX];
size_t row_cnt = 0;


int check_slope(int xd, int yd) {
    int acc = 0;
    int x = 0;
    for (int y = 0; y < row_cnt; y += yd) {
        if (data[y*X_SIZE+x]) acc++;
        x += xd;
        x %= X_SIZE;
    }
    return acc;
}

void part_one() {
    printf("P1: %d\n", check_slope(3, 1));
}

void part_two() {
    int tst[] = {1, 1, 3, 1, 5, 1, 7, 1, 1, 2};
    long mul = 1;
    for (int i = 0; i < 10; i += 2) {
        printf("(%d, %d)\n", tst[i], tst[i+1]);
        mul *= check_slope(tst[i], tst[i+1]);
    }
    printf("P2: %ld\n", mul);
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("test.txt", "r");
    if (fd == NULL) die("failed to read");
    int x = 0;
    while (1) {
        switch (getc(fd)) {
            case EOF:
                if (ferror(fd)) die("failed to read");
                goto done;
            case '.':
                data[row_cnt*X_SIZE+(x++)] = 0;
                break;
            case '#':
                data[row_cnt*X_SIZE+(x++)] = 1;
                break;
            case '\n':
                x = 0;
                row_cnt++;
                break;
            default:
                die("failed to match");
        }
    }
    done:
    part_two();
    return 0;
}
