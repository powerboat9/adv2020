use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::iter::{FromIterator, once};
use std::mem::swap;
use std::rc::Rc;
use std::sync::Once;
use std::ops::Range;

const INPUT: &str = include_str!("../test.txt");

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum MatchEntry {
    Id(u32),
    A,
    B,
}

impl MatchEntry {
    fn unwrap_id(&self) -> u32 {
        if let MatchEntry::Id(i) = self {
            *i
        } else {
            panic!("could not unwrap id")
        }
    }
}

#[derive(Clone)]
struct Match {
    pattern: Vec<MatchEntry>,
    result: u32,
}

#[derive(Copy, Clone)]
enum CNFPattern {
    A,
    B,
    Pair(u32, u32)
}

fn to_cnf(m_in: &[Match]) -> HashMap<u32, Vec<CNFPattern>> {
    // START, TERM, and DEL are already done
    // do BIN and UNIT
    let mut next_id = m_in.iter().map(|v| v.result).max().unwrap() + 1;
    let mut to_sub = Vec::new();
    let mut pairs = Vec::new();
    for m in m_in {
        match m.pattern.len() {
            0 => unreachable!(),
            1 => {
                match m.pattern[0] {
                    MatchEntry::Id(id) => to_sub.push((id, m.result)),
                    MatchEntry::A => pairs.push((m.result, CNFPattern::A)),
                    MatchEntry::B => pairs.push((m.result, CNFPattern::B))
                }
            },
            2 => {
                pairs.push((m.result, CNFPattern::Pair(m.pattern[0].unwrap_id(), m.pattern[1].unwrap_id())));
            },
            _ => {
                let mut publish = m.result;
                for i in 2..m.pattern.len() {
                    pairs.push((publish, CNFPattern::Pair(next_id, m.pattern[m.pattern.len() - i + 1].unwrap_id())));
                    publish = next_id;
                    next_id += 1;
                }
                pairs.push((publish, CNFPattern::Pair(m.pattern[0].unwrap_id(), m.pattern[1].unwrap_id())));
            }
        }
    }
    to_sub.retain(|&(a, b)| a != b);
    let mut has_change = true;
    while has_change {
        has_change = false;
        for i in 0..to_sub.len() {
            for j in 0..to_sub.len() {
                if i == j {
                    continue;
                }
                if to_sub[i].1 == to_sub[j].0 {
                    to_sub[i].1 = to_sub[j].1;
                    has_change = true;
                }
            }
        }
    }
    let to_sub = to_sub.into_iter().collect::<HashMap<_, _>>();
    let mut ret = HashMap::new();
    pairs.into_iter().flat_map(|v| {
        let second = to_sub.get(&v.0).map(|vv| (*vv, v.1));
        once(v).chain(second.into_iter())
    }).for_each(|v| {
        ret.entry(v.0).or_insert_with(Vec::new).push(v.1)
    });
    ret
}

fn is_match_r<'a>(exp: &HashMap<u32, Vec<CNFPattern>>, id: u32, s: &'a str, cache: &mut HashMap<(&'a str, u32), bool>) -> bool {
    if let Some(ret) = cache.get(&(s, id)).copied() {
        return ret;
    }
    for pat in exp.get(&id).unwrap().iter() {
        match pat {
            CNFPattern::A => {
                if s == "a" {
                    cache.insert((s, id), true);
                    return true;
                }
            },
            CNFPattern::B => {
                if s == "b" {
                    cache.insert((s, id), true);
                    return true;
                }
            },
            CNFPattern::Pair(a, b) => {
                for i in 1..s.len() {
                    let split = s.split_at(i);
                    if is_match_r(exp, *a, split.0, cache) && is_match_r(exp, *b, split.1, cache) {
                        cache.insert((s, id), true);
                        return true;
                    }
                }
            }
        }
    }
    cache.insert((s, id), false);
    return false;
}

fn is_match(exp: &HashMap<u32, Vec<CNFPattern>>, s: &str) -> bool {
    is_match_r(exp, 0, s, &mut HashMap::new())
}

fn main() {
    let mut matches_list = Vec::new();
    let mut lines = INPUT.lines();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let (id, rest) = {
            let idx = line.find(':').unwrap();
            let id: u32 = line.split_at(idx).0.parse().unwrap();
            (id, line.split_at(idx + 1).1)
        };
        let mut ors = Vec::new();
        let mut ands = Vec::new();
        for tk in rest.split_ascii_whitespace() {
            if tk == "|" {
                let mut new_ands = Vec::new();
                swap(&mut ands, &mut new_ands);
                ors.push(new_ands);
            } else if tk == "\"a\"" {
                ands.push(MatchEntry::A);
            } else if tk == "\"b\"" {
                ands.push(MatchEntry::B);
            } else {
                ands.push(MatchEntry::Id(tk.parse().unwrap()));
            }
        }
        for ent in ors.into_iter().chain(once(ands)) {
            matches_list.push(Match {
                pattern: ent,
                result: id,
            });
        }
    }
    // p1 and p2
    let p1_map = to_cnf(matches_list.as_slice());
    matches_list.push(Match {
        pattern: vec![MatchEntry::Id(42), MatchEntry::Id(8)],
        result: 8
    });
    matches_list.push(Match {
        pattern: vec![MatchEntry::Id(42), MatchEntry::Id(11), MatchEntry::Id(31)],
        result: 11
    });
    let p2_map = to_cnf(matches_list.as_slice());
    let mut p1_acc = 0;
    let mut p2_acc = 0;
    for line in lines {
        if is_match(&p1_map, line) {
            p1_acc += 1;
        }
        if is_match(&p2_map, line) {
            p2_acc += 1;
        }
    }
    println!("P1: {}", p1_acc);
    println!("P2: {}", p2_acc);
}
