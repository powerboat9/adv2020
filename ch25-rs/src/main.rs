use std::intrinsics::transmute;

const CARD_PUB_KEY: u64 = 18356117;
const DOOR_PUB_KEY: u64 = 5909654;

fn transform(n: u64, l_size: u64) -> u64 {
    let mut acc = 1;
    for _ in 0..l_size {
        acc *= n;
        acc %= 20201227;
    }
    acc
}

fn inv_transform(n: u64, res: u64) -> u64 {
    let mut acc = 1;
    let mut cnt = 0;
    while acc != res {
        acc *= n;
        acc %= 20201227;
        cnt += 1;
    }
    cnt
}

fn main() {
    let card_loop = inv_transform(7, CARD_PUB_KEY);
    let door_loop = inv_transform(7, DOOR_PUB_KEY);
    let p1 = if card_loop > door_loop {
        transform(CARD_PUB_KEY, door_loop)
    } else {
        transform(DOOR_PUB_KEY, card_loop)
    };
    println!("P1: {}", p1)
}
