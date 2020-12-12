#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void die(char *str) {
    fprintf(stderr, "[ERROR] %s\n", str);
    exit(-1);
}

#define MAX_DATA 4096

struct data_entry {
    char byr;
    char iyr;
    char eyr;
    char hgt;
    char hcl;
    char ecl;
    char pid;
    char cid;
};

struct data_entry data[MAX_DATA];
size_t data_len = 0;

#define HAS_BYR 1
#define HAS_IYR 2
#define HAS_EYR 4
#define HAS_HGT 8
#define HAS_HCL 16
#define HAS_ECL 32
#define HAS_PID 64

char *xgetline(FILE *fd) {
    char *ptr = NULL;
    size_t n = 0;
    return (getline(&ptr, &n, fd) > 0) ? ptr : (free(ptr), NULL);
}

void part() {
    int acc = 0;
    for (int i = 0; i < data_len; i++) {
        if (data[i].byr &&
            data[i].iyr &&
            data[i].eyr &&
            data[i].hgt &&
            data[i].hcl &&
            data[i].ecl &&
            data[i].pid) acc++;
    }
    printf("P1: %d\n", acc);
}

#define IC(a, b, c) ( (((int) a) << 16) | (((int) b) << 8) | ((int) c) )

#define PART_TWO 1

#define SUF_NONE 0
#define SUF_IN 1
#define SUF_CM 2

int read_int(char *s, int *n, int suffix) {
    *n = 0;
    int i = 0;
    for (; (s[i] >= '0') && (s[i] <= '9'); i++) {
        *n *= 10;
        *n += s[i] - '0';
    }
    switch (suffix) {
        case SUF_NONE: if (s[i] != '\0') return 0; return 1;
        case SUF_IN: if ((s[i] != 'i') || (s[i+1] != 'n') || s[i+2]) return 0; return 1;
        case SUF_CM: if ((s[i] != 'c') || (s[i+1] != 'm') || s[i+2]) return 0; return 1;
    }
}

int is_bad_range(char *s, int min, int max, int suffix) {
    int n;
    if (!read_int(s, &n, suffix)) return 1;
    if ((n < min) || (n > max)) return 1;
    return 0;
}

int main(int argc, char **argv) {
    // read input
    FILE *fd = fopen("test.txt", "r");
    if (fd == NULL) die("failed to read");
    char *line;
    int sep = 0;
    while ((line = xgetline(fd))) {
        if (line[0] == '\n') {
            sep++;
            data_len++;
            continue;
        }
        char k[8];
        char v[32];
        int len;
        for (char *l_ptr = line; sscanf(l_ptr, " %[a-z]:%s%n", k, v, &len) == 2; l_ptr += len) {
            switch (IC(k[0], k[1], k[2])) {
                case IC('b', 'y', 'r'):
                    if (PART_TWO && is_bad_range(v, 1920, 2002, SUF_NONE)) break;
                    data[data_len].byr = 1;
                    break;
                case IC('i', 'y', 'r'):
                    if (PART_TWO && is_bad_range(v, 2010, 2020, SUF_NONE)) break;
                    data[data_len].iyr = 1;
                    break;
                case IC('e', 'y', 'r'):
                    if (PART_TWO && is_bad_range(v, 2020, 2030, SUF_NONE)) break;
                    data[data_len].eyr = 1;
                    break;
                case IC('h', 'g', 't'):
                    if (PART_TWO && is_bad_range(v, 59, 76, SUF_IN) && is_bad_range(v, 150, 193, SUF_CM)) break;
                    data[data_len].hgt = 1;
                    break;
                case IC('h', 'c', 'l'):
                    if (PART_TWO) {
                        if (v[0] != '#') break;
                        for (int i = 1; i < 7; i++) {
                            switch (v[i]) {
                                default: goto fail;
                                case '0' ... '9':
                                case 'a' ... 'f': break;
                            }
                        }
                        if (v[7]) break;
                    }
                    data[data_len].hcl = 1;
                    fail:
                    break;
                case IC('e', 'c', 'l'):
                    if (PART_TWO) {
#define PASS(s) if (!strcmp(v, s)) goto ok;
                        PASS("amb") PASS("blu") PASS("brn") PASS("gry") PASS("grn") PASS("hzl") PASS("oth") break;
                    }
                    ok:
                    data[data_len].ecl = 1;
                    break;
                case IC('p', 'i', 'd'):
                    if (PART_TWO) {
                        for (int i = 0; i < 9; i++) if ((v[i] < '0') || (v[i] > '9')) goto fail;
                        if (v[9]) break;
                    }
                    data[data_len].pid = 1;
                    break;
            }
        }
        free(line);
    }
    data_len++;
    done:
    part();
    return 0;
}
