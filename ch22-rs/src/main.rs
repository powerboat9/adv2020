use std::collections::{VecDeque, HashSet, HashMap};
use std::option::Option::Some;
use crate::Winner::{PlayerTwo, PlayerOne};

#[macro_use]
extern crate cached;

use cached::proc_macro::cached;

const DATA: &'static str = include_str!("../test.txt");

#[derive(Clone)]
struct Deck {
    cards: VecDeque<u32>
}

impl Deck {
    fn get_score(&self) -> u64 {
        self.cards
            .iter()
            .enumerate()
            .map(|ent| {
                (*ent.1 as u64) * ((self.cards.len() - ent.0) as u64)
            })
            .sum()
    }
}

fn game_tick(p1: &mut Deck, p2: &mut Deck) {
    let p1_play = p1.cards.pop_front().unwrap();
    let p2_play = p2.cards.pop_front().unwrap();
    if p1_play > p2_play {
        p1.cards.push_back(p1_play);
        p1.cards.push_back(p2_play);
    } else if p2_play > p1_play {
        p2.cards.push_back(p2_play);
        p2.cards.push_back(p1_play);
    } else {
        panic!("tie")
    }
}

#[derive(Copy, Clone)]
enum Winner {
    PlayerOne,
    PlayerTwo
}

#[cached]
fn game_p2(p1: VecDeque<u32>, p2: VecDeque<u32>) -> Winner {
    let mut p1 = p1;
    let mut p2 = p2;
    let mut old_rounds = HashSet::new();
    loop {
        if p1.len() == 0 {
            return PlayerTwo
        } else if p2.len() == 0 {
            return PlayerOne
        } else if !old_rounds.insert((p1.clone(), p2.clone())) {
            // player one wins
            let p1_card = p1.pop_front().unwrap();
            let p2_card = p2.pop_front().unwrap();
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            let p1_card = p1.pop_front().unwrap();
            let p2_card = p2.pop_front().unwrap();
            if (p1_card <= (p1.len() as u32)) && (p2_card <= (p2.len() as u32)) {
                match game_p2(
                    p1.iter().copied().take(p1_card as usize).collect(),
                    p2.iter().copied().take(p2_card as usize).collect()) {
                    Winner::PlayerOne => {
                        p1.push_back(p1_card);
                        p1.push_back(p2_card);
                    },
                    Winner::PlayerTwo => {
                        p2.push_back(p2_card);
                        p2.push_back(p1_card);
                    }
                }
            } else if p1_card > p2_card {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            } else if p2_card > p1_card {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            } else {
                panic!("tie");
            }
        }
    }
}

fn main() {
    let mut player_1;
    let mut player_2;
    {
        let mut lines = DATA.lines();
        lines.next().unwrap();
        let mut ls_1 = VecDeque::new();
        loop {
            let line = lines.next().unwrap();
            if line == "" {
                break
            }
            ls_1.push_back(line.parse().unwrap())
        }
        player_1 = Deck {
            cards: ls_1
        };
        lines.next().unwrap();
        let mut ls_2 = VecDeque::new();
        for line in lines {
            ls_2.push_back(line.parse().unwrap())
        }
        player_2 = Deck {
            cards: ls_2
        }
    }
    let mut player_1_p2 = player_1.clone();
    let mut player_2_p2 = player_2.clone();
    let mut score = 0;
    loop {
        if player_2.cards.len() == 0 {
            // player 1 win
            score = player_1.get_score();
            break;
        } else if player_1.cards.len() == 0 {
            // player 2 win
            score = player_2.get_score();
            break;
        } else {
            game_tick(&mut player_1, &mut player_2);
        }
    }
    println!("P1: {}", score);
    let score_p2 = match game_p2(player_1_p2.cards.clone(), player_2_p2.cards.clone()) {
        PlayerOne => {
            player_1_p2.get_score()
        }
        PlayerTwo => {
            player_2_p2.get_score()
        }
    };
    println!("P2: {}", score_p2);
}
