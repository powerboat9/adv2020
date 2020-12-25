use std::ops::{Index, IndexMut, Range, RangeInclusive};
use std::collections::VecDeque;
use std::iter::{once, FromIterator};

const INPUT: &'static str = "942387615";
const DATA: [u32; 9] = [9, 4, 2, 3, 8, 7, 6, 1, 5];
const IDX_MAP: [u32; 9] = [7, 2, 3, 1, 8, 6, 5, 4, 0];

fn shl(a: &mut [u32]) {
    let tmp = a[0];
    for i in 1..a.len() {
        a[i - 1] = a[i];
    }
    a[a.len() - 1] = tmp;
}

/*
fn val_at(round: u32, idx: u32) -> u32 {
    if idx >= 1_000_000 {
        val_at(round, idx % 1_000_000)
    } else if round == 0 {
        if idx > 8 {
            idx + 1
        } else {
            DATA[idx as usize]
        }
    } else {
        if (idx == 0) ||
        let sel = val_at(round - 1, 0);
        if sel == n {
            return 0
        }
        let m1 = val_at(round - 1, 1);
        if m1 == n {
            return 1
        }
        let m2 = val_at(round - 1, 2);
        if m2 == n {
            return 2
        }
        let m3 = val_at(round - 1, 3);
        if m3 == n {
            return 3
        }
        let mut dest = sel - 1;
        loop {
            if dest == 0 {
                dest = 999_999;
            }
            if (m1 != dest) && (m2 != dest) && (m3 != dest) {
                break;
            }
        }
        let dest_idx = idx_of_n(round - 1, dest);
        let old_idx = idx_of_n(round - 1, n);
        if old_idx > dest_idx {
            old_idx - 1
        } else {
            old_idx - 4
        }
    }
}

fn idx_of_n(round: u32, n: u32) -> u32 {
    if round == 0 {
        if n > 9 {
            n - 0
        } else {
            IDX_MAP[(n - 1) as usize]
        }
    } else {
        let sel = val_at(round - 1, 0);
        if sel == n {
            return 0
        }
        let m1 = val_at(round - 1, 1);
        if m1 == n {
            return 1
        }
        let m2 = val_at(round - 1, 2);
        if m2 == n {
            return 2
        }
        let m3 = val_at(round - 1, 3);
        if m3 == n {
            return 3
        }
        let mut dest = sel - 1;
        loop {
            if dest == 0 {
                dest = 999_999;
            }
            if (m1 != dest) && (m2 != dest) && (m3 != dest) {
                break;
            }
        }
        let dest_idx = idx_of_n(round - 1, dest);
        let old_idx = idx_of_n(round - 1, n);
        if old_idx > dest_idx {
            old_idx - 1
        } else {
            old_idx - 4
        }
    }
}
*/

enum PartSlice {
    List(VecDeque<u32>),
    Seq(Range<u32>)
}

struct Buffer {
    slices: VecDeque<PartSlice>
}

impl Buffer {
    fn get(&self, mut idx: usize) -> u32 {
        for s in self.slices.iter() {
            match s {
                PartSlice::List(ls) => {
                    if idx >= ls.len() {
                        idx -= ls.len();
                    } else {
                        return ls[idx];
                    }
                },
                PartSlice::Seq(r) => {
                    if idx >= ((r.end - r.start) as usize) {
                        idx -= (r.end - r.start) as usize;
                    } else {
                        return r.start + (idx as u32);
                    }
                }
            }
        }
        panic!("out of bounds");
    }

    fn remove_front(&mut self) -> u32 {
        let mut should_pop = false;
        let mut ret = 0;
        match &mut self.slices[0] {
            PartSlice::List(ls) => {
                ret = ls.pop_front().unwrap();
                should_pop = ls.is_empty();
            },
            PartSlice::Seq(r) => {
                ret = r.start;
                r.start += 1;
                if r.start == r.end {
                    should_pop = true;
                }
            }
        }
        if should_pop {
            self.slices.pop_front().unwrap();
        }
        ret
    }

    fn add_back(&mut self, n: u32) {
        let last_idx = self.slices.len() - 1;
        match &mut self.slices[last_idx] {
            PartSlice::List(ls) => {
                ls.push_back(n);
            }
            PartSlice::Seq(r) => {
                if r.end == n {
                    r.end += 1;
                } else {
                    self.slices.push_back(PartSlice::List(VecDeque::from_iter(once(n))));
                }
            }
        }
    }

    fn search_for(&self, n: u32) -> usize {
        let mut idx = 0;
        for s in self.slices.iter() {
            match s {
                PartSlice::List(ls) => {
                    for v in ls.iter().enumerate() {
                        if *v.1 == n {
                            return idx + v.0
                        }
                    }
                    idx += ls.len();
                },
                PartSlice::Seq(r) => {
                    if (r.start <= n) && (r.end > n) {
                        return idx + ((n - r.start) as usize);
                    }
                    idx += (r.end - r.start) as usize;
                }
            }
        }
        panic!("could not find");
    }

    fn shl(&mut self) {
        let n = self.remove_front();
        self.add_back(n);
    }

    fn mv_trip(&mut self, mut src: usize, dst: usize) {
        let mut taken = Vec::new();
        let mut to_remove = Vec::new();
        'big: for (s_idx, s) in self.slices.iter_mut().enumerate() {
            match s {
                PartSlice::List(ls) => {
                    loop {
                        if src >= ls.len() {
                            break;
                        }
                        taken.push(ls.pop_back().unwrap());
                        if taken.len() == 3 {
                            break 'big;
                        }
                    }
                    src -= ls.len();
                },
                PartSlice::Seq(r) => {
                    loop {
                        if src >= ((r.end - r.start) as usize) {
                            break;
                        }
                        if src == ((r.end - r.start - 1) as usize) {
                            taken.push(r.end);
                            r.end -= 1;
                        }
                    }
                    src -= (r.end - r.start) as usize;
                }
            }
        }
        if taken.len() != 3 {
            panic!("src fail");
        }
    }
}

struct Ring {
    ls: Vec<u32>,
    r: usize
}

impl Ring {
    fn shl(&mut self) {
        self.r += 1;
        if self.r == self.ls.len() {
            self.r = 0;
        }
    }

    fn len(&self) -> usize {
        self.ls.len()
    }

    fn new(ls: Vec<u32>) -> Self {
        Ring {
            ls,
            r: 0
        }
    }

    fn iterate<'a>(&'a self) -> impl 'a + Iterator<Item = u32> {
        (0..self.ls.len()).map(move |i| self[i])
    }
}

impl Index<usize> for Ring {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.ls[(index + self.r) % self.ls.len()]
    }
}

impl IndexMut<usize> for Ring {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.ls.len();
        &mut self.ls[(index + self.r) % len]
    }
}

fn round(a: &mut Ring) {
    let mut dest = a[0] - 1;
    let mut dest_idx = 0;
    'big: loop {
        if dest == 0 {
            dest = 9;
        }
        if (a[1] != dest) && (a[2] != dest) && (a[3] != dest) {
            for i in 4..a.len() {
                if a[i] == dest {
                    dest_idx = i;
                    break 'big;
                }
            }
        }
        dest -= 1;
    }
    let a1 = a[1];
    let a2 = a[2];
    let a3 = a[3];
    for i in 4..=dest_idx {
        a[i - 3] = a[i];
    }
    a[dest_idx - 2] = a1;
    a[dest_idx - 1] = a2;
    a[dest_idx] = a3;
    a.shl();
}

fn main() {
    let cups = {
        let mut cups = [0u32; 9];
        for n in INPUT.chars().map(|c| {
            c.to_string().parse().unwrap()
        }).enumerate() {
            cups[n.0] = n.1;
        }
        cups
    };
    let mut p1_cups = Ring::new(Vec::from(&cups as &[u32]));
    for _ in 0..100 {
        round(&mut p1_cups);
    }
    while p1_cups[0] != 1 {
        p1_cups.shl();
    }
    let p1 = p1_cups.iterate().skip(1).map(|n| ((n as u8) + ('0' as u8)) as char).collect::<String>();
    println!("P1: {}", p1);
    let mut p2_cups = Vec::with_capacity(1_000_000);
    for i in 0..9 {
        p2_cups.push(cups[i]);
    }
    for i in 10..=1_000_000 {
        p2_cups.push(i);
    }
    let mut p2_cups = Ring::new(p2_cups);
    for i in 0..1_000_000 {
        println!("> {}", i);
        round(&mut p2_cups)
    }
}
