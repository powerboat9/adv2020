#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <sys/mman.h>
#include <unistd.h>
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

void print_inum(__int128 n) {
    if (n < 0) {
        putchar('-');
        n = -n;
    }
    print_num((u128_t) n);
}

u128_t gcd(u128_t a, u128_t b, __int128 *s, __int128 *t) {
    if (b > a) return gcd(b, a, t, s);
    __int128 s0 = 1, s1 = 0, t0 = 0, t1 = 1;
    u128_t acc_a = a, acc_b = b;
    u128_t r;
    while (r = (acc_a % acc_b)) {
        u128_t q = acc_a / acc_b;
        // s
        u128_t tmp = s1;
        s1 = s0 - q * s1;
        s0 = tmp;
        // t
        tmp = t1;
        t1 = t0 - q * t1;
        t0 = tmp;
        // a, b
        acc_a = acc_b;
        acc_b = r;
    }
    *s = s1;
    *t = t1;
    return acc_b;
}

__int128_t emod(__int128 n, __int128 m) {
    n = n % m;
    if (n < 0) n += m;
    return n;
}

// Chinese Remainder Theorum Combiner
// "42... CRT... It's on"
void crt(u128_t aa, u128_t am, u128_t ba, u128_t bm, u128_t *oa, u128_t *om) {
    // calculate modulus
    u128_t rm = am * bm;
    // calculate x with extended ecludian
    __int128_t s, t;
    if (gcd(am, bm, &s, &t) - 1) die("gcd failure");
    if (am * s + bm * t - 1) die("st failure");
    __int128_t x = aa * bm * t + ba * am * s;
    *oa = (u128_t) emod(x, rm);
    *om = rm;
    if (emod(x, am) != aa) die("aa fail");
    else if (emod(x, bm) != ba) die("ba fail");
}

#define MAX_LINE 32

int main(int argc, char **argv) {
    // read input
    FILE *fd = map_file("test.txt");
    char line[MAX_LINE];
    unsigned int start_time;
    if (fscanf(fd, "%u", &start_time) != 1) die("failed to read start time");
    unsigned int best_wait_time = UINT_MAX;
    unsigned int best_bus_id = 0;
    unsigned int bus_id;
    unsigned int seq_cnt = (unsigned int) -1;
    u128_t p2_a = 0;
    u128_t p2_m = 1;
    char bf[8];
    while (fscanf(fd, "%*[\n,]%[^,]", (char *) &bf) == 1) {
        seq_cnt++;
        if (*bf == 'x') continue;
        bus_id = atoi(bf);
        // p1
        unsigned int wait_time = ((start_time + bus_id - 1) / bus_id) * bus_id - start_time;
        if (wait_time < best_wait_time) {
            best_bus_id = bus_id;
            best_wait_time = wait_time;
        }
        // p2
        // assumes bus ids are coprime
        unsigned int cur_seq = seq_cnt % bus_id;
        if (cur_seq) cur_seq = bus_id - cur_seq;
        crt(p2_a, p2_m, cur_seq, bus_id, &p2_a, &p2_m);
    }
    printf("P1: %u\n", best_wait_time * best_bus_id);
    printf("P2: ");
    print_num(p2_a);
    putchar('\n');
    return 0;
}
