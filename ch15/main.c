#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>

void die(char *s) {
    fprintf(stderr, "[ERROR] %s\n", s);
    exit(-1);
}

#define MAX_HASH (1024L * 1024L)

struct node {
    uint32_t idx;
    uint32_t vals[2];
    struct node *nxt;
};

struct node last_map[MAX_HASH];
int last_called;
uint32_t cur_turn = 1;

uint32_t hash(uint32_t n) {
    return (n * n) % MAX_HASH;
}

uint32_t *fetch_slot(uint32_t n) {
    struct node *cur = last_map + hash(n);
    // search for existing node
    if (!cur->idx) {
        cur->idx = n + 1;
        return cur->vals;
    }
    while (1) {
        if (cur->idx == (n + 1)) {
            return cur->vals;
        }
        if (!cur->nxt) break;
        cur = cur->nxt;
    }
    // allocate
    cur = (cur->nxt = (struct node *) malloc(sizeof(struct node)));
    memset(cur, 0, sizeof(*cur));
    cur->idx = n + 1;
    return cur->vals;
}

void mark_called(uint32_t n, int turn) {
    uint32_t *ptr = fetch_slot(n);
    if (ptr[0]) {
        if (ptr[1]) {
            ptr[0] = ptr[1];
            ptr[1] = turn;
        } else {
            ptr[1] = turn;
        }
    } else {
        ptr[0] = turn;
    }
}

uint32_t calc_diff(uint32_t n) {
    uint32_t *ptr = fetch_slot(n);
    if (ptr[1]) {
        return ptr[1] - ptr[0];
    } else {
        return 0;
    }
}

void call_imm(int n) {
    mark_called(n, cur_turn++);
    last_called = n;
}

void call_next() {
    int d = calc_diff(last_called);
    mark_called(d, cur_turn++);
    last_called = d;
}

int get_at(int turn) {
    int last_p = 0;
    while (cur_turn <= turn) {
        int new_p = cur_turn * 100 / turn;
        if (new_p != last_p) {
            last_p = new_p;
        }
        call_next();
    }
    return last_called;
}

int main() {
    int in[] = {0, 13, 1, 8, 6, 15};
    for (int i = 0; i < (sizeof(in) / sizeof(int)); i++) call_imm(in[i]);
    printf("P1: %d\n", get_at(2020));
    printf("P2: %d\n", get_at(30000000));
    return 0;
}
