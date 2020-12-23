use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::mem::swap;
use std::rc::Rc;
use std::sync::Once;
use std::iter::{once, FromIterator};

const INPUT: &str = include_str!("../test.txt");

enum MatchEntry {
    Id(u32),
    OptId(u32),
    A,
    B,
}

struct Match {
    subs: Vec<Vec<MatchEntry>>
}

#[derive(Clone)]
enum NodeEntry {
    A,
    B,
    Node(Box<Node>),
    OptNode(Box<Node>)
}

#[derive(Clone)]
enum Node {
    Or(Vec<NodeEntry>),
    And(Vec<NodeEntry>),
}

fn matches_to_node(hs: &HashMap<u32, Match>) -> Node {
    matches_to_node_r(hs, 0, &mut HashMap::new())
}

fn matches_to_node_r(hs: &HashMap<u32, Match>, idx: u32, cache: &mut HashMap<u32, Node>) -> Node {
    match cache.get(&idx) {
        Some(n) => n.clone(),
        None => {
            let or_ls = hs
                .get(&idx).unwrap()
                .subs.iter()
                .map(|and_ls| {
                    let mut new_ls = and_ls.iter().map(|ent| {
                        match ent {
                            MatchEntry::A => NodeEntry::A,
                            MatchEntry::B => NodeEntry::B,
                            MatchEntry::Id(n) => NodeEntry::Node(Box::new(matches_to_node_r(hs, *n, cache))),
                            MatchEntry::OptId(n) => NodeEntry::OptNode(Box::new(matches_to_node_r(hs, *n, cache)))
                        }
                    }).collect();
                    NodeEntry::Node(Box::new(Node::And(new_ls)))
                })
                .collect();
            let ret = Node::Or(or_ls);
            cache.insert(idx, ret.clone());
            ret
        }
    }
}

#[derive(Clone)]
struct NFANode {
    a: HashSet<u32>,
    b: HashSet<u32>,
    can_end: bool
}

#[derive(Clone)]
struct NFA {
    nodes: Vec<NFANode>
}

impl NFA {
    fn check_match(&self, s: &str) -> bool {
        let mut poses = HashSet::new();
        poses.insert(0);
        for c in s.chars() {
            poses = match c {
                'a' => poses
                    .iter()
                    .flat_map(|v| {
                        self.nodes.get(*v as usize).unwrap().a.iter().copied()
                    })
                    .collect(),
                'b' => poses
                    .iter()
                    .flat_map(|v| {
                        self.nodes.get(*v as usize).unwrap().b.iter().copied()
                    })
                    .collect(),
                _ => panic!("unexpected char")
            };
            if poses.len() == 0 {
                return false
            }
        }
        poses.iter().any(|&v| self.nodes.get(v as usize).unwrap().can_end)
    }
}

fn nfa_parallel(a: &NFA, b: &NFA) -> NFA {
    let mut ret = Vec::new();
    let offset = (a.nodes.len() - 1) as u32;
    let first_a = a.nodes.first().unwrap();
    let first_b = b.nodes.first().unwrap();
    ret.push(NFANode {
        a: first_a.a.iter().copied().chain(first_b.a.iter().map(|v| *v + offset)).collect(),
        b: first_a.b.iter().copied().chain(first_b.b.iter().map(|v| *v + offset)).collect(),
        can_end: first_a.can_end || first_b.can_end
    });
    ret.extend(a.nodes.iter().skip(1).cloned().chain(b.nodes.iter().skip(1).map(|v| {
        NFANode {
            a: v.a.iter().map(|j| *j + offset).collect(),
            b: v.b.iter().map(|j| *j + offset).collect(),
            can_end: v.can_end
        }
    })));
    NFA {
        nodes: ret
    }
}

fn nfa_serial(a: &NFA, b: &NFA) -> NFA {
    let mut ret = Vec::new();
    let offset = (a.nodes.len() - 1) as u32;
    let first_b = b.nodes.first().unwrap();
    ret.extend(a.nodes.iter().map(|v| {
        if v.can_end {
            NFANode {
                a: v.a.iter().copied().chain(first_b.a.iter().map(|v| *v + offset)).collect(),
                b: v.b.iter().copied().chain(first_b.b.iter().map(|v| *v + offset)).collect(),
                can_end: first_b.can_end
            }
        } else {
            v.clone()
        }
    }).chain(b.nodes.iter().skip(1).map(|v| {
        NFANode {
            a: v.a.iter().map(|j| *j + offset).collect(),
            b: v.b.iter().map(|j| *j + offset).collect(),
            can_end: v.can_end
        }
    })));
    NFA {
        nodes: ret
    }
}

fn node_to_nfa(node: &Node) -> NFA {
    match node {
        Node::Or(ls) => {
            let mut it = ls.iter().map(node_ent_to_nfa);
            let mut acc = it.next().unwrap();
            for v in it {
                acc = nfa_parallel(&acc, &v);
            }
            acc
        },
        Node::And(ls) => {
            let mut it = ls.iter().map(node_ent_to_nfa);
            let mut acc = it.next().unwrap();
            for v in it {
                acc = nfa_serial(&acc, &v);
            }
            acc
        }
    }
}

fn node_ent_to_nfa(node_ent: &NodeEntry) -> NFA {
    match node_ent {
        NodeEntry::A => {
            NFA {
                nodes: vec![
                    NFANode {
                        a: HashSet::from_iter(once(1)),
                        b: HashSet::from_iter(once(2)),
                        can_end: false
                    },
                    NFANode {
                        a: HashSet::from_iter(once(2)),
                        b: HashSet::from_iter(once(2)),
                        can_end: true
                    },
                    NFANode {
                        a: HashSet::from_iter(once(2)),
                        b: HashSet::from_iter(once(2)),
                        can_end: false
                    }
                ]
            }
        },
        NodeEntry::B => {
            NFA {
                nodes: vec![
                    NFANode {
                        a: HashSet::from_iter(once(2)),
                        b: HashSet::from_iter(once(1)),
                        can_end: false
                    },
                    NFANode {
                        a: HashSet::from_iter(once(2)),
                        b: HashSet::from_iter(once(2)),
                        can_end: true
                    },
                    NFANode {
                        a: HashSet::from_iter(once(2)),
                        b: HashSet::from_iter(once(2)),
                        can_end: false
                    }
                ]
            }
        },
        NodeEntry::Node(n) => node_to_nfa(&*n),
        NodeEntry::OptNode(n) => {
            let mut ret = node_to_nfa(&*n);
            ret.nodes[0].can_end = true;
            ret
        }
    }
}

fn main() {
    let mut matches_list = HashMap::new();
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
        ors.push(ands);
        matches_list.insert(id, Match {
            subs: ors
        });
    }
    let node_root = matches_to_node(&matches_list);
    let nfa_root = node_to_nfa(&node_root);
    // p1
    let mut p1_acc = 0;
    for line in lines {
        if nfa_root.check_match(line) {
            p1_acc += 1;
        }
    }
    println!("P1: {}", p1_acc);
    // p2
    *matches_list.get_mut(&8) = Match {
        subs: vec![vec![42], vec![42, 8]]
    }
}
