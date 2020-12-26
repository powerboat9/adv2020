use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::fmt::{Display, Formatter, Write};
use std::iter::once;

const DATA: &'static str = include_str!("../test.txt");
/*
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

 */

fn to_once<T>(mut i: impl Iterator<Item=T>) -> T {
    let v = i.next().expect("expected one item");
    if i.next().is_some() {
        panic!("expected only one item");
    }
    v
}

#[derive(Copy, Clone)]
struct Tile {
    idx: u32,
    tiles: [[bool; 10]; 10],
}

impl Tile {
    fn rot(&self) -> Self {
        let mut ret = Tile {
            idx: self.idx,
            tiles: [[false; 10]; 10],
        };
        for y in 0..10 {
            for x in 0..10 {
                ret.tiles[x][9 - y] = self.tiles[y][x];
            }
        }
        ret
    }

    fn flip_h_ax(&self) -> Self {
        let mut tiles = [[false; 10]; 10];
        for y in 0..10 {
            tiles[9 - y] = self.tiles[y]
        }
        Tile {
            idx: self.idx,
            tiles,
        }
    }

    fn flip_v_ax(&self) -> Self {
        let mut tiles = [[false; 10]; 10];
        for y in 0..10 {
            for x in 0..10 {
                tiles[y][9 - x] = self.tiles[y][x];
            }
        }
        Tile {
            idx: self.idx,
            tiles,
        }
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

fn can_align_h(a: &Tile, b: &Tile) -> bool {
    for y in 0..10 {
        if a.tiles[y][9] ^ b.tiles[y][0] {
            return false;
        }
    }
    true
}

fn can_align_v(a: &Tile, b: &Tile) -> bool {
    for x in 0..10 {
        if a.tiles[9][x] ^ b.tiles[0][x] {
            return false;
        }
    }
    true
}

fn read_tile(lines: &mut impl Iterator<Item=&'static str>) -> Option<Tile> {
    let mut head;
    loop {
        head = lines.next()?;
        if head != "" {
            break;
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
        tiles,
    })
}

#[derive(Copy, Clone)]
enum Flip {
    None,
    HozAxis,
    VerAxis,
}

impl Flip {
    fn apply_flip(&self, tile: &Tile) -> Tile {
        match self {
            Flip::None => tile.clone(),
            Flip::HozAxis => tile.flip_h_ax(),
            Flip::VerAxis => tile.flip_v_ax()
        }
    }

    fn iter_flips() -> impl 'static + Iterator<Item=Flip> {
        let ls = vec![Flip::None, Flip::HozAxis, Flip::VerAxis];
        ls.into_iter()
    }
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
        for f in Flip::iter_flips() {
            let mut cur_tile = f.apply_flip(tile);
            for rot in 0..4 {
                for (k, v) in self.tiles.iter() {
                    if can_align_v(&cur_tile, v) {
                        return Some(self.gen_embed(rot, f, k.0, k.1 - 1));
                    } else if can_align_h(v, &cur_tile) {
                        return Some(self.gen_embed(rot, f, k.0 + 1, k.1));
                    } else if can_align_v(v, &cur_tile) {
                        return Some(self.gen_embed(rot, f, k.0, k.1 + 1));
                    } else if can_align_h(&cur_tile, v) {
                        return Some(self.gen_embed(rot, f, k.0 - 1, k.1));
                    }
                }
                cur_tile = cur_tile.rot()
            }
        }
        None
    }

    fn gen_embed<'a>(&'a mut self, rot: i32, flip: Flip, x: i32, y: i32) -> impl 'a + FnOnce(Tile) {
        move |mut t: Tile| {
            t = flip.apply_flip(&t);
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

impl Display for TileEnv {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_x = self.tiles.iter().map(|(&v, _)| v.0).max().unwrap();
        let max_y = self.tiles.iter().map(|(&v, _)| v.1).max().unwrap();
        let min_x = self.tiles.iter().map(|(&v, _)| v.0).min().unwrap();
        let min_y = self.tiles.iter().map(|(&v, _)| v.1).min().unwrap();
        for row_n in min_y..=max_y {
            if row_n != min_y {
                println!();
            }
            let mut row = vec![None; (max_x - min_x + 1) as usize];
            for tile in self.tiles.iter().filter_map(|(&(x, y), t)| {
                if y == row_n {
                    Some((x - min_x, t))
                } else {
                    None
                }
            }) {
                row[tile.0 as usize] = Some(tile.1);
            }
            for i in 0..10 {
                for col in row.iter().enumerate() {
                    if col.0 != 0 {
                        print!(" ");
                    }
                    match col.1 {
                        None => print!("          "),
                        Some(v) => {
                            for j in 0..10 {
                                if v.tiles[i][j] {
                                    print!("#");
                                } else {
                                    print!(".");
                                }
                            }
                        }
                    }
                }
                println!();
            }
        }
        Ok(())
    }
}

struct TileMatrix {
    inner: Vec<bool>,
    side_len: usize,
}

impl TileMatrix {
    fn from_env(env: TileEnv) -> Self {
        let max_x = env.tiles.iter().map(|(&v, _)| v.0).max().unwrap();
        let max_y = env.tiles.iter().map(|(&v, _)| v.1).max().unwrap();
        let min_x = env.tiles.iter().map(|(&v, _)| v.0).min().unwrap();
        let min_y = env.tiles.iter().map(|(&v, _)| v.1).min().unwrap();
        if (max_x - min_x) != (max_y - min_y) {
            panic!("uneven sides");
        }
        let side_len = (max_x - min_x + 1) as usize;
        if (side_len * side_len) != env.tiles.len() {
            panic!("not full");
        }
        let side_len = side_len * 8;
        let mut inner = vec![false; side_len * side_len];
        for (map_coord, tile) in env.tiles {
            let coord = ((map_coord.0 - min_x) as usize, (map_coord.1 - min_y) as usize);
            for y in 0..8 {
                for x in 0..8 {
                    inner[(coord.1 * 8 + y) * side_len + (coord.0 * 8 + x)] = tile.tiles[y + 1][x + 1];
                }
            }
        }
        TileMatrix {
            inner,
            side_len,
        }
    }

    fn count_filled(&self) -> usize {
        self.inner.iter().filter(|&&v| v).count()
    }

    fn search_sea(&self) -> usize {
        const OFFSETS: [(usize, usize); 15] = [(0, 1), (1, 2), (4, 2), (5, 1), (6, 1), (7, 2), (10, 2), (11, 1), (12, 1), (13, 2), (16, 2), (17, 1), (18, 0), (18, 1), (19, 1)];
        const WIDTH: usize = 17;
        const HEIGHT: usize = 3;
        const AS_FUNCTIONS: [fn(&TileMatrix, usize, usize) -> bool; 12] = [
            // No flip
            |s, x, y| s.inner[x + y * s.side_len],
            |s, x, y| s.inner[(s.side_len - 1 - y) + x * s.side_len],
            |s, x, y| s.inner[(s.side_len - 1 - x) + (s.side_len - 1 - y) * s.side_len],
            |s, x, y| s.inner[y + (s.side_len - 1 - x) * s.side_len],
            // horizontal axis flip
            |s, x, y| s.inner[x + (s.side_len - 1 - y) * s.side_len],
            |s, x, y| s.inner[y + x * s.side_len],
            |s, x, y| s.inner[(s.side_len - 1 - x) + y * s.side_len],
            |s, x, y| s.inner[(s.side_len - 1 - y) + (s.side_len - 1 - x) * s.side_len],
            // vertical axis flip
            |s, x, y| s.inner[(s.side_len - 1 - x) + y * s.side_len],
            |s, x, y| s.inner[(s.side_len - 1 - y) + (s.side_len - 1 - x) * s.side_len],
            |s, x, y| s.inner[x + (s.side_len - 1 - y) * s.side_len],
            |s, x, y| s.inner[y + x * s.side_len]
        ];
        for f in AS_FUNCTIONS.iter() {
            print!(">> ");
            for x in 0..self.side_len {
                print!("{}", if (f)(self, x, 0) {
                    '#'
                } else {
                    '.'
                });
            }
            println!();
            let mut cnt = 0;
            for y in 0..(self.side_len - (HEIGHT - 1)) {
                for x in 0..(self.side_len - (WIDTH - 1)) {
                    let mut found = true;
                    for check_offset in OFFSETS.iter() {
                        let check = (x + check_offset.0, y + check_offset.1);
                        if !(f)(self, check.0, check.1) {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        cnt += 1;
                    }
                }
            }
            if cnt != 0 {
                return cnt;
            }
        }
        panic!("failed to find monsters")
    }
}

impl Display for TileMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.side_len {
            if x != 0 {
                f.write_char('\n')?;
            }
            for y in 0..self.side_len {
                f.write_char(if self.inner[y * self.side_len + x] {
                    '#'
                } else {
                    '.'
                })?;
            }
        }
        Ok(())
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
    let mut env = TileEnv::create(tiles.pop().unwrap());
    'big_loop: while tiles.len() != 0 {
        for idx in 0..tiles.len() {
            match env.try_fit(&mut tiles[idx]) {
                Some(f) => {
                    (f)(tiles.remove(idx));
                    continue 'big_loop;
                }
                None => {}
            }
        }
        panic!("failed to fit")
    }
    let max_x = env.tiles.iter().map(|(&v, _)| v.0).max().unwrap();
    let max_y = env.tiles.iter().map(|(&v, _)| v.1).max().unwrap();
    let min_x = env.tiles.iter().map(|(&v, _)| v.0).min().unwrap();
    let min_y = env.tiles.iter().map(|(&v, _)| v.1).min().unwrap();
    let p1 = [(max_x, max_y), (max_x, min_y), (min_x, max_y), (min_x, min_y)]
        .iter()
        .map(|coord| env.tiles.get(coord).unwrap().idx as u64)
        .fold(1, |a, b| a * b);
    println!("{}", &env);
    println!("P1: {}", p1);
    let mat = TileMatrix::from_env(env);
    println!("{}", &mat);
    println!("P2: {}", mat.count_filled() - mat.search_sea() * 15);
}