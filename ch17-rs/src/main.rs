use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::mem::swap;

const INPUT: &str = include_str!("../test.txt");

fn get_neighbor_pos_list(x: isize, y: isize, z: isize) -> impl 'static + Iterator<Item = (isize, isize, isize)> {
    (-1..=1)
        .into_iter()
        .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v, vv)))
        .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v.0, v.1, vv)))
        .filter(|v| (v.0.clone() != 0) || (v.1.clone() != 0) || (v.2.clone() != 0))
        .map(move |v| (x + v.0, y + v.1, z + v.2))
}

fn get_neighbors<'a>(hs: &'a HashSet<(isize, isize, isize)>, x: isize, y: isize, z: isize) -> impl 'a + Iterator<Item = (isize, isize, isize)> {
    get_neighbor_pos_list(x, y, z).filter(move |v| hs.contains(v))
}

fn get_neighbors_cnt(hs: &HashSet<(isize, isize, isize)>, x: isize, y: isize, z: isize) -> u32 {
    get_neighbors(hs, x, y, z).count() as u32
}

fn step(hs: &HashSet<(isize, isize, isize)>) -> HashSet<(isize, isize, isize)> {
    let mut adj_cnts = HashMap::new();
    hs.iter().cloned().flat_map(|v| get_neighbor_pos_list(v.0, v.1, v.2)).for_each(|ent| {
        match adj_cnts.entry(ent) {
            Entry::Vacant(v) => {
                v.insert(1);
            },
            Entry::Occupied(o) => {
                *o.into_mut() += 1;
            }
        };
    });
    let mut ret = HashSet::new();
    for cnt in adj_cnts {
        if (cnt.1 == 3) || ((cnt.1 == 2) && hs.contains(&cnt.0)) {
            ret.insert(cnt.0);
        }
    }
    ret
}

fn get_neighbor_pos_list_4(x: isize, y: isize, z: isize, w: isize) -> impl 'static + Iterator<Item = (isize, isize, isize, isize)> {
    (-1..=1)
        .into_iter()
        .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v, vv)))
        .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v.0, v.1, vv)))
        .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v.0, v.1, v.2, vv)))
        .filter(|v| (v.0.clone() != 0) || (v.1.clone() != 0) || (v.2.clone() != 0) || (v.3.clone() != 0))
        .map(move |v| (x + v.0, y + v.1, z + v.2, w + v.3))
}

fn get_neighbors_4<'a>(hs: &'a HashSet<(isize, isize, isize, isize)>, x: isize, y: isize, z: isize, w: isize) -> impl 'a + Iterator<Item = (isize, isize, isize, isize)> {
    get_neighbor_pos_list_4(x, y, z, w).filter(move |v| hs.contains(v))
}

fn get_neighbors_cnt_4(hs: &HashSet<(isize, isize, isize, isize)>, x: isize, y: isize, z: isize, w: isize) -> u32 {
    get_neighbors_4(hs, x, y, z, w).count() as u32
}

fn step_4(hs: &HashSet<(isize, isize, isize, isize)>) -> HashSet<(isize, isize, isize, isize)> {
    let mut adj_cnts = HashMap::new();
    hs.iter().cloned().flat_map(|v| get_neighbor_pos_list_4(v.0, v.1, v.2, v.3)).for_each(|ent| {
        match adj_cnts.entry(ent) {
            Entry::Vacant(v) => {
                v.insert(1);
            },
            Entry::Occupied(o) => {
                *o.into_mut() += 1;
            }
        };
    });
    let mut ret = HashSet::new();
    for cnt in adj_cnts {
        if (cnt.1 == 3) || ((cnt.1 == 2) && hs.contains(&cnt.0)) {
            ret.insert(cnt.0);
        }
    }
    ret
}

fn main() {
    let mut hset = HashSet::new();
    INPUT.split('\n')
        .enumerate()
        .flat_map(|v| v.1
            .chars()
            .enumerate()
            .map(move |vv| (vv.0 as isize, v.0 as isize, vv.1))
        )
        .filter_map(|v| if v.2 == '#' {Some((v.0, v.1, 0isize))} else {None})
        .for_each(|v| {
            hset.insert(v);
        });
    let mut p2_hset = hset.iter().cloned().map(|v| (v.0, v.1, v.2, 0)).collect();
    for _ in 0..6 {
        let mut nhset = step(&hset);
        swap(&mut hset, &mut nhset);
    }
    println!("P1: {}", hset.into_iter().count());
    for _ in 0..6 {
        let mut nhset = step_4(&p2_hset);
        swap(&mut p2_hset, &mut nhset);
    }
    println!("P2: {}", p2_hset.into_iter().count());
}
