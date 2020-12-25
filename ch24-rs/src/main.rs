use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

const INPUT: &'static str = include_str!("../test.txt");

fn get_adj(x: i32, y: i32) -> impl 'static + Iterator<Item = (i32, i32)> {
    // E        W       SE       NW      NE       SW
    vec![(1, 0), (-1, 0), (0, -1), (0, 1), (1, 1), (-1, -1)]
        .into_iter().map(move |(dx, dy)| (x + dx, y + dy))
}

fn main() {
    let mut flips = INPUT.lines().map(|line| {
        let mut x = 0;
        let mut y = 0;
        let mut c = line.chars();
        while let Some(c1) = c.next() {
            match c1 {
                'e' => x += 1,
                'w' => x -= 1,
                _ => {
                    let c2 = c.next().unwrap();
                    match (c1, c2) {
                        ('s', 'e') => y -= 1,
                        ('n', 'w') => y += 1,
                        ('s', 'w') => {
                            y -= 1;
                            x -= 1;
                        },
                        ('n', 'e') => {
                            y += 1;
                            x += 1;
                        },
                        _ => panic!("unexpected sequence")
                    }
                }
            }
        }
        (x, y)
    }).collect::<Vec<_>>();
    let mut flips_dedup = HashSet::new();
    for ent in flips {
        if flips_dedup.contains(&ent) {
            flips_dedup.remove(&ent);
        } else {
            flips_dedup.insert(ent);
        }
    }
    let mut flips = flips_dedup.into_iter().collect::<HashSet<_>>();
    println!("P1: {}", flips.len());
    for _ in 0..100 {
        let mut adj_cnts = HashMap::new();
        for adj in flips.iter().flat_map(|&(x, y)| get_adj(x, y)) {
            match adj_cnts.entry(adj) {
                Entry::Vacant(v) => {
                    v.insert(1);
                },
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
            }
        }
        let mut new_flips = HashSet::new();
        for &ent in flips.iter() {
            if let Some(&oc) = adj_cnts.get(&ent) {
                if oc <= 2 {
                    new_flips.insert(ent);
                }
            }
        }
        for ent in adj_cnts.iter() {
            if (*ent.1 == 2) && !flips.contains(ent.0) {
                new_flips.insert(*ent.0);
            }
        }
        flips = new_flips;
    }
    println!("P2: {}", flips.len());
}
