use std::ops::Range;
use std::cell::Cell;

fn die(str: &str) {
    panic!(format!("[ERROR] {}", str));
}

const INPUT: &str = include_str!("../test.txt");

const TICKET_ARGS: u32 = 20;

const COL_FIELD_DEF: u32 = ((1 << TICKET_ARGS) - 1);

struct TicketType {
    name: &'static str,
    ranges: [Range<u32>; 2],
    col_field: Cell<u32>
}

impl TicketType {
    fn new(name: &'static str, r1: Range<u32>, r2: Range<u32>) -> Self {
        TicketType {
            name,
            ranges: [r1, r2],
            col_field: Cell::new(COL_FIELD_DEF)
        }
    }

    fn does_fit(&self, n: u32) -> bool {
        self.ranges.iter().any(|v| v.contains(&n))
    }

    fn cancel_fit(&self, arg: u32) {
        self.col_field.set(self.col_field.get() & !(1 << arg));
    }
}

struct Ticket {
    vals: [u32; TICKET_ARGS as usize]
}

fn read_ticket(line: &str) -> Ticket {
    let mut vals_iter = line.split(',').map(|v| {
        let r: u32 = v.parse().expect("invalid ticket");
        r
    });
    let mut vals = [0; TICKET_ARGS as usize];
    for i in 0..TICKET_ARGS {
        vals[i as usize] = vals_iter.next().unwrap();
    }
    if vals_iter.next().is_some() {
        panic!("extra ticket args")
    }
    Ticket {vals}
}
/*
unsigned int p2_acc = 1;
int check_idx(int ok_map, int idx) {
for (unsigned int tr = 0; tr < TICKET_ARGS; tr++) {
for (int i = 0; i < (idx + 1); i++) putchar('>');
printf(" %d\n", tr);
if (!(ttypes[idx].col_field & ok_map & (1 << tr))) continue;
if ((idx + 1) != type_cnt) {
if (!check_idx(ok_map & ~(1 << tr), idx + 1)) continue;
}
// it works
if (starts_with(ttypes[idx].name, "departure")) {
printf("> %s\n", ttypes[idx].name);
printf("(%u) => ", p2_acc);
p2_acc *= my_ticket.vals[tr];
printf("(%u)\n", p2_acc);
}
return 1;
}
return 0;
}
*/
fn main() {
    let mut line_iter = INPUT.lines();
    let mut ticket_types = Vec::new();
    loop {
        let line = line_iter.next().unwrap();
        println!("orig: {}", line);
        if line == "" {
            break
        }
        let (name, rest) = {
            let idx = line.find(':').unwrap();
            (line.split_at(idx).0, line.split_at(idx + 1).1)
        };
        let (range_1, range_2) = {
            let mut ret_it = rest.split(" or ").map(|v| {
                let mut ret: [u32; 2] = [0; 2];
                let mut ret_it = v.split('-').map(|v| v.replace(' ', "").parse().unwrap());
                for i in 0..2 {
                    ret[i] = ret_it.next().unwrap();
                }
                if ret_it.next().is_some() {
                    panic!("failed to parse field type")
                }
                ret[0]..(ret[1] + 1)
            });
            let ret_1 = ret_it.next().unwrap();
            let ret_2 = ret_it.next().unwrap();
            if ret_it.next().is_some() {
                panic!("failed to parse field type")
            }
            (ret_1, ret_2)
        };
        println!("new: {}: {}-{} or {}-{}", name, range_1.start, range_1.end - 1, range_2.start, range_2.end - 1);
        ticket_types.push(TicketType::new(name, range_1, range_2))
    }
    line_iter.next().unwrap();
    let my_ticket = read_ticket(line_iter.next().unwrap());
    line_iter.next().unwrap();
    line_iter.next().unwrap();
    let mut p1_acc = 0;
    let mut tickets: Vec<Ticket> = line_iter
        .map(|line| read_ticket(line))
        .filter(|tk| {
            for n in tk.vals.iter().copied() {
                let mut ok = false;
                for tick_type in ticket_types.iter() {
                    if tick_type.does_fit(n) {
                        ok = true;
                        break
                    }
                }
                if !ok {
                    p1_acc += n;
                    return false
                }
            }
            true
        })
        .collect();
    println!("P1: {}", p1_acc);
    // p2
    for tk in tickets.iter() {
        for arg in 0..TICKET_ARGS {
            for ty in ticket_types.iter_mut() {
                if !ty.does_fit(tk.vals[arg as usize]) {
                    ty.cancel_fit(arg);
                }
            }
        }
    }
    // optimise
    {
        let mut has_change = true;
        while has_change {
            println!("OP");
            has_change = false;
            for ty in ticket_types.iter().enumerate() {
                let other = ticket_types.iter()
                    .enumerate()
                    .filter_map(|v| {
                        if v.0 != ty.0 {
                            Some(v.1.col_field.get())
                        } else {
                            None
                        }
                    })
                    .fold(0, |acc, v| acc | v);
                let only = ty.1.col_field.get() & !other;
                if only == 0 {
                    continue
                }
                if ty.1.col_field.get() != only {
                    ty.1.col_field.set(only);
                    has_change = true;
                }
            }
        }
    }
    for ty in ticket_types.iter() {
        let f = ty.col_field.get();
        println!("{}: {}", ty.name, f);
    }
    for ty in ticket_types.iter() {
        if (ty.col_field.get() & (ty.col_field.get() - 1)) != 0 {
            panic!("failed to find single solution")
        }
    }
    let p2_res = ticket_types.iter()
        .filter_map(|v| {
            if v.name.starts_with("departure") {
                Some(v.col_field.get())
            } else {
                None
            }
        })
        .map(|mut v| {
            v >>= 1;
            let mut acc = 0;
            while v != 0 {
                acc += 1;
                v >>= 1;
            }
            acc
        })
        .map(|arg| my_ticket.vals[arg] as u64)
        .fold(1, |acc, v| acc * v);
    println!("P2: {}", p2_res)
}
/*
// attempt to solve simple way
int p2aac = 1;
for (int ty = 0; ty < type_cnt; ty++) {
printf("$$ %s: %d\n", ttypes[ty].name, (int) __builtin_ctz(ttypes[ty].col_field));
if (!starts_with(ttypes[ty].name, "departure")) continue;
int si = ttypes[ty].col_field;
printf("> si: %d\n", si);
if (si & (si - 1)) die("FOOP");
unsigned int *v_ptr = my_ticket.vals;
while (si >>= 1) v_ptr++;
p2aac *= *v_ptr;
}
printf("P@A: %d\n", p2aac);
/*
    for (int ty = 0; ty < type_cnt; ty++) {
        int v = ttypes[ty].col_field;
        if (!(v & (v - 1))) printf("FOLD: %d\n", ty);
    }
*/
// p2 calculation
if (!check_idx(COL_FIELD_DEF, 0)) die("p2 fail");
printf("P2: %d\n", p2_acc);
return 0;
}
*/