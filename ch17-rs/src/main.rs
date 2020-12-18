use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::mem::swap;
use std::hash::Hash;

const INPUT: &str = include_str!("../test.txt");

trait Coord: Copy + Eq + Hash {
    fn get_all_neighbors(self) -> Box<dyn 'static + Iterator<Item=Self>>;
    fn get_filled_neighbors<'a>(self, e: &'a Environ<Self>) -> Box<dyn 'a + Iterator<Item=Self>> {
        Box::new(self.get_all_neighbors().filter(move |v| e.is_occupied(v)))
    }
    fn get_neighbor_cnt(self, e: &Environ<Self>) -> u32 {
        self.get_filled_neighbors(e).count() as u32
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Coord for Coord3 {
    fn get_all_neighbors(self) -> Box<dyn 'static + Iterator<Item=Self>> {
        Box::new((-1..=1)
            .into_iter()
            .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v, vv)))
            .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v.0, v.1, vv)))
            .filter(|v| (v.0.clone() != 0) || (v.1.clone() != 0) || (v.2.clone() != 0))
            .map(move |v| Coord3 {x: self.x + v.0, y: self.y + v.1, z: self.z + v.2}))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Coord for Coord4 {
    fn get_all_neighbors(self) -> Box<dyn Iterator<Item=Self>> {
        Box::new((-1..=1)
            .into_iter()
            .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v, vv)))
            .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v.0, v.1, vv)))
            .flat_map(|v| (-1..=1).into_iter().map(move |vv| (v.0, v.1, v.2, vv)))
            .filter(|v| (v.0.clone() != 0) || (v.1.clone() != 0) || (v.2.clone() != 0) || (v.3.clone() != 0))
            .map(move |v| Coord4 {x: self.x + v.0, y: self.y + v.1, z: self.z + v.2, w: self.w + v.3}))
    }
}

struct Environ<C: Coord> {
    data: HashSet<C>
}

impl<C: Coord> Environ<C> {
    fn is_occupied(&self, pos: &C) -> bool {
        self.data.contains(pos)
    }

    fn step(&mut self) {
        let mut adj_cnts = HashMap::new();
        self.data.iter().cloned().flat_map(|v| v.get_all_neighbors()).for_each(|ent| {
            match adj_cnts.entry(ent) {
                Entry::Vacant(v) => {
                    v.insert(1);
                }
                Entry::Occupied(o) => {
                    *o.into_mut() += 1;
                }
            };
        });
        let mut ret = HashSet::new();
        for cnt in adj_cnts {
            if (cnt.1 == 3) || ((cnt.1 == 2) && self.data.contains(&cnt.0)) {
                ret.insert(cnt.0);
            }
        }
        swap(&mut self.data, &mut ret)
    }

    fn new(h: HashSet<C>) -> Self {
        Environ {
            data: h
        }
    }

    fn count(self) -> u32 {
        self.data.len() as u32
    }
}

fn main() {
    let mut p1_set = HashSet::new();
    INPUT.split('\n')
        .enumerate()
        .flat_map(|v| v.1
            .chars()
            .enumerate()
            .map(move |vv| (vv.0 as isize, v.0 as isize, vv.1))
        )
        .filter_map(|v| if v.2 == '#' { Some((v.0, v.1, 0isize)) } else { None })
        .for_each(|v| {
            p1_set.insert(Coord3 { x: v.0, y: v.1, z: v.2 });
        });
    let mut p2_set = p1_set
        .iter()
        .cloned()
        .map(|v| Coord4 {x: v.x, y: v.y, z: v.z, w: 0})
        .collect();
    let mut p1_env = Environ::new(p1_set);
    for _ in 0..6 {
        p1_env.step();
    }
    println!("P1: {}", p1_env.count());
    let mut p2_env = Environ::new(p2_set);
    for _ in 0..6 {
        p2_env.step();
    }
    println!("P2: {}", p2_env.count());
}
