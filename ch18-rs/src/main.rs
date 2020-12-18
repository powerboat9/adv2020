use std::iter::from_fn;
use itertools::Itertools;

const INPUT: &str = include_str!("../test.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
enum MathOp {
    Plus,
    Mul
}

impl MathOp {
    fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            MathOp::Plus => a + b,
            MathOp::Mul => a * b
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum MathToken {
    Int(u64),
    Op(MathOp),
    OParen,
    CParen
}

fn tokenise<'a>(s: &'a str) -> impl 'a + Iterator<Item = MathToken> {
    let mut s_iter = s.chars().peekable();
    from_fn(move || {
        loop {
            match s_iter.next() {
                Some('+') => return Some(MathToken::Op(MathOp::Plus)),
                Some('*') => return Some(MathToken::Op(MathOp::Mul)),
                Some('(') => return Some(MathToken::OParen),
                Some(')') => return Some(MathToken::CParen),
                Some(c) if c.is_digit(10) => {
                    return Some(MathToken::Int(s_iter
                        .peeking_take_while(|v| v.is_digit(10))
                        .map(|cc| (cc as u64) - ('0' as u64))
                        .fold((c as u64) - ('0' as u64), |acc, i| acc + i)))
                },
                Some(c) if c.is_whitespace() => continue,
                Some(_) => panic!("unknown char"),
                None => return None
            }
        };
    })
}

fn calc_with_rpol(mut tk_iter: impl Iterator<Item = MathToken>, is_p2: bool) -> u64 {
    fn apply_to_stack(op: MathOp, st: &mut Vec<u64>) {
        let a = st.pop().expect("too many operators");
        let b = st.pop().expect("too many operators");
        st.push(op.apply(a, b))
    }
    let mut op_ls = Vec::new();
    let mut stack = Vec::new();
    #[derive(Copy, Clone, Eq, PartialEq)]
    enum OpStT {
        Op(MathOp),
        OParen
    }
    while let Some(tk) = tk_iter.next() {
        match tk {
            MathToken::Int(n) => stack.push(n),
            MathToken::OParen => op_ls.push(OpStT::OParen),
            MathToken::CParen => {
                loop {
                    match op_ls.pop() {
                        None => panic!("unmatched parenthesis"),
                        Some(OpStT::OParen) => break,
                        Some(OpStT::Op(op)) => apply_to_stack(op, &mut stack)
                    }
                }
            }
            MathToken::Op(o) => {
                if op_ls.last().is_none() || op_ls.last() == Some(&OpStT::OParen) {
                    op_ls.push(OpStT::Op(o))
                } else {
                    loop {
                        match op_ls.last() {
                            Some(OpStT::Op(MathOp::Plus)) => {
                                op_ls.pop();
                                apply_to_stack(MathOp::Plus, &mut stack);
                            },
                            Some(OpStT::Op(MathOp::Mul)) if !is_p2 => {
                                op_ls.pop();
                                apply_to_stack(MathOp::Mul, &mut stack);
                            },
                            _ => {
                                op_ls.push(OpStT::Op(o));
                                break;
                            }
                        }
                    }
                }
            }
        }
    };
    while let Some(v) = op_ls.pop() {
        match v {
            OpStT::Op(o) => apply_to_stack(o, &mut stack),
            OpStT::OParen => panic!("unmatched parenthesis")
        }
    }
    if stack.len() != 1 {
        panic!("parse fail")
    }
    stack.pop().unwrap()
}

fn calc(s: &str, is_p1: bool) -> u64 {
    calc_with_rpol(&mut tokenise(s), is_p1)
}

fn main() {
    let p1: u64 = INPUT.lines().map(|s| calc(s, false)).sum();
    println!("P1: {}", p1);
    let p2: u64 = INPUT.lines().map(|s| calc(s, true)).sum();
    println!("P2: {}", p2);
}
