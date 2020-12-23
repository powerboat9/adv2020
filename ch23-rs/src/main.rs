const INPUT: &'static str = "942387615";
const DATA: [u32; 9] = [9, 4, 2, 3, 8, 7, 6, 1, 5];
const IDX_MAP: [u32; 9] = [7, 2, 3, 1, 8, 6, 5, 4, 0];

fn shl(a: &mut [u32; 9]) {
    let tmp = a[0];
    for i in 1..9 {
        a[i - 1] = a[i];
    }
    a[8] = tmp;
}

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

fn round(a: &mut [u32; 9]) {
    let mut dest = a[0];
    let mut dest_idx = None;
    loop {
        if dest == 0 {
            dest = 9;
        } else {
            dest -= 1;
        }
        for i in 4..9 {
            if a[i] == dest {
                dest_idx = Some(i);
                break;
            }
        }
        if let Some(idx) = dest_idx {
            break;
        }
    }
    let dest_idx = dest_idx.unwrap();
    let a1 = a[1];
    let a2 = a[2];
    let a3 = a[3];
    for i in 4..=dest_idx {
        a[i - 3] = a[i];
    }
    a[dest_idx - 2] = a1;
    a[dest_idx - 1] = a2;
    a[dest_idx] = a3;
    shl(a);
}

fn main() {
    let mut cups = [0u32; 9];
    for n in INPUT.chars().map(|c| {
        c.to_string().parse().unwrap()
    }).enumerate() {
        cups[n.0] = n.1;
    }
    for _ in 0..100 {
        round(&mut cups);
    }
    while cups[0] != 1 {
        shl(&mut cups);
    }
    let p1 = cups.iter().skip(1).copied().map(|n| ((n as u8) + ('0' as u8)) as char).collect::<String>();
    println!("P1: {}", p1);
}
