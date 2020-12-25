use std::collections::{HashMap, HashSet};
use std::iter::once;
use std::collections::hash_map::Entry;
use std::fmt::{Display, Formatter, Write};

//const DATA: &'static str = include_str!("../test.txt");

const DATA: &'static str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

fn to_once<T>(mut i: impl Iterator<Item = T>) -> T {
    let v = i.next().expect("expected one item");
    if i.next().is_some() {
        panic!("expected only one item");
    }
    v
}

#[derive(Copy, Clone)]
struct Tile {
    idx: u32,
    tiles: [[bool; 10]; 10]
}

impl Tile {
    fn rot(&self) -> Self {
        let mut ret = Tile {
            idx: self.idx,
            tiles: [[false; 10]; 10]
        };
        for y in 0..10 {
            for x in 0..10 {
                ret.tiles[x][9 - y] = self.tiles[y][x];
            }
        }
        ret
    }

    fn edges(&self) -> [[bool; 10]; 8] {
        let mut ret = [[false; 10]; 8];
        for x in 0..10 {
            ret[0][x] = self.tiles[0][x];
            ret[2][9 - x] = self.tiles[9][x];
        }
        for y in 0..10 {
            ret[1][y] = self.tiles[y][9];
            ret[3][9 - y] = self.tiles[y][0];
        }
        // handles flips
        for i in 0..4 {
            for j in 0..10 {
                ret[i + 4][9 - j] = ret[i][j];
            }
        }
        ret
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..10 {
            for j in 0..10 {
                f.write_char(if self.tiles[i][j] {
                    '#'
                } else {
                    '.'
                })?;
            }
            if i != 9 {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

fn orient_tiles_r(side_len: usize, to_place: &mut Vec<Tile>, placed: &mut Vec<Tile>) -> bool {
    if to_place.len() == 0 {
        return true
    }
    for i in 0..to_place.len() {
        let tile = to_place[i];

    }
}

fn can_align_h(a: &Tile, b: &Tile) -> bool {
    for y in 0..10 {
        if a.tiles[y][9] ^ b.tiles[y][0] {
            return false
        }
    }
    true
}

fn can_align_v(a: &Tile, b: &Tile) -> bool {
    for x in 0..10 {
        if a.tiles[9][x] ^ b.tiles[0][x] {
            return false
        }
    }
    true
}

fn read_tile(lines: &mut impl Iterator<Item = &'static str>) -> Option<Tile> {
    let mut head;
    loop {
        head = lines.next()?;
        if head != "" {
            break
        }
    }
    let idx = head.split_at(5).1.split_at(head.len() - 6).0.parse().unwrap();
    let mut tiles = [[false; 10]; 10];
    for row in lines.take(10).map(|line| {
        let mut ret = [false; 10];
        for b in line.chars().map(|c| c == '#').enumerate() {
            ret[b.0] = b.1;
        }
        ret
    }).enumerate() {
        tiles[row.0] = row.1
    }
    Some(Tile {
        idx,
        tiles
    })
}

struct TileEnv {
    tiles: HashMap<(i32, i32), Tile>
}

impl TileEnv {
    fn is_occupied(&self, x: i32, y: i32) -> bool {
        self.tiles.get(&(x, y)).is_some()
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tiles.get(&(x, y))
    }

    fn try_fit<'a>(&'a mut self, tile: &Tile) -> Option<impl 'a + FnOnce(Tile)> {
        let mut cur_tile = tile.clone();
        for rot in 0..4 {
            for (k, v) in self.tiles.iter() {
                if can_align_v(&cur_tile, v) {
                    return Some(self.gen_embed(rot, k.0, k.1 + 1))
                } else if can_align_h(v, &cur_tile) {
                    return Some(self.gen_embed(rot, k.0 + 1, k.1))
                } else if can_align_v(v, &cur_tile) {
                    return Some(self.gen_embed(rot, k.0, k.1 - 1))
                } else if can_align_h(&cur_tile, v) {
                    return Some(self.gen_embed(rot, k.0 - 1, k.1))
                }
            }
            cur_tile = cur_tile.rot()
        }
        None
    }

    fn gen_embed<'a>(&'a mut self, rot: i32, x: i32, y: i32) -> impl 'a + FnOnce(Tile) {
        move |mut t: Tile| {
            for _ in 0..rot {
                t = t.rot();
            }
            self.tiles.insert((x, y), t).map(|_| panic!("overlap"));
        }
    }

    fn create(root: Tile) -> Self {
        TileEnv {
            tiles: once(((0, 0), root)).collect()
        }
    }
}

fn main() {
    let mut tiles = Vec::new();
    {
        let mut lines = DATA.lines();
        while let Some(t) = read_tile(&mut lines) {
            tiles.push(t);
        }
    }
    /*
    let mut env = TileEnv::create(tiles.pop().unwrap());
    println!("ILEN: {}", tiles.len());
    'big_loop: while tiles.len() != 0 {
        for idx in 0..tiles.len() {
            match env.try_fit(&mut tiles[idx]) {
                Some(f) => {
                    println!("INS");
                    (f)(tiles.remove(idx));
                    continue 'big_loop;
                },
                None => {}
            }
        }
        println!("LEN: {}", tiles.len());
        panic!("ERR");
    }
    println!("OK");
    */
    let mut edge_map = HashMap::new();
    for t in tiles.iter() {
        let edges = t.edges();
        for edge in edges.iter().enumerate() {
            match edge_map.entry(*edge.1) {
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                },
                Entry::Vacant(v) => {
                    v.insert(1);
                }
            }
        }
    }
    if edge_map.iter().any(|(_, &n)| n > 2) {
        panic!("multiple edges")
    }
    println!("ID:{}\n{}", tiles[0].idx, tiles[0]);
    for e in tiles[0].edges().iter().enumerate() {
        print!("R{}: ", e.0);
        for b in e.1 {
            print!("{}", if *b {
                '#'
            } else {
                '.'
            });
        }
        println!("    {}", *edge_map.get(e.1).unwrap());
    }
    let p1 = tiles
        .iter()
        .filter_map(|t| {
            let m_cnt = t.edges()
                .iter()
                .filter(|e| {
                    *edge_map.get(*e).unwrap() == 2
                }).count();
            println!(">> {}: {}", t.idx, m_cnt);
            if m_cnt > 4 {
                println!("TILE: {}", t.idx);
                panic!("fail att");
            } else if m_cnt == 2 {
                println!("TIE");
                Some(t.idx)
            } else {
                None
            }
        }).fold(1u128, |acc, v| acc * (v as u128));
    println!("P1: {}", p1)
}