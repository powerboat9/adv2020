use std::collections::{HashSet, HashMap};
use std::collections::hash_map::Entry;

const DATA: &'static str = include_str!("../test.txt");

fn main() {
    let mut ingredients = HashMap::new();
    let mut allergens: HashMap<&'static str, HashSet<&'static str>> = HashMap::new();
    for (ings, als) in DATA
        .lines()
        .map(|line| {
            let sep = line.find('(').unwrap();
            (line.split_at(sep).0.split_ascii_whitespace().collect::<HashSet<_>>(),
            line.split_at(line.len() - 1).0
                .split_at(sep + 10).1
                .split_ascii_whitespace()
                .map(|s| s.trim_end_matches(','))
                .collect::<Vec<_>>())
        }) {
        for ing in ings.iter().copied() {
            match ingredients.entry(ing) {
                Entry::Vacant(v) => {
                    v.insert(1);
                },
                Entry::Occupied(mut o) => {
                    *o.get_mut() += 1;
                }
            }
        }
        for al in als {
            match allergens.entry(al) {
                Entry::Occupied(mut o) => {
                    o.get_mut().retain(|&ent| ings.contains(ent));
                },
                Entry::Vacant(v) => {
                    v.insert(ings.iter().copied().collect::<HashSet<_>>());
                }
            }
        }
    }
    // p1
    let mut possibly_bad = HashSet::new();
    for (_, ings) in allergens.iter() {
        ings.iter().for_each(|&i| {
            possibly_bad.insert(i);
        });
    }
    let mut p1_acc = 0;
    for (ing, cnt) in ingredients {
        if !possibly_bad.contains(ing) {
            p1_acc += cnt;
        }
    }
    println!("P1: {}", p1_acc);
    // p2
    let mut comp_als = Vec::new();
    let mut todo = allergens.into_iter().collect::<Vec<_>>();
    while todo.len() != 0 {
        let mut i = 0;
        while i < todo.len() {
            let mut table = todo[i].1.clone();
            for j in 0..todo.len() {
                if i == j {
                    continue
                }
                for &s in todo[j].1.iter() {
                    table.remove(s);
                }
            }
            if table.len() == 1 {
                let name = todo[i].0;
                let ing = table.into_iter().next().unwrap();
                comp_als.push((name, ing));
                todo.remove(i);
                for j in 0..todo.len() {
                    todo[j].1.remove(name);
                }
            } else {
                i += 1;
            }
        }
    }
    comp_als.sort_by(|a, b| a.0.cmp(b.0));
    let p2_ls = comp_als
        .into_iter()
        .map(|v| v.1)
        .collect::<Vec<_>>();
    println!("P1: \"{}\"", p2_ls.join(","))
}
