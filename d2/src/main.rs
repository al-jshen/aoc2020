use parse_display::{Display, FromStr};
use std::fs::read_to_string;

#[derive(Display, FromStr)]
#[display("{low}-{high} {letter}: {pass}")]
struct PassPolicy {
    low: usize,
    high: usize,
    letter: char,
    pass: String,
}

fn main() {
    println!("{}", pt2("./input.txt"));
}

fn pt1(fp: &str) -> u32 {
    let inp = read_to_string(fp).unwrap();
    inp.lines()
        .map(|x| {
            let y: PassPolicy = x.parse().unwrap();
            if y.low <= y.pass.matches(y.letter).count()
                && y.pass.matches(y.letter).count() <= y.high
            {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}

fn pt2(fp: &str) -> u32 {
    let inp = read_to_string(fp).unwrap();
    inp.lines()
        .map(|x| {
            let y: PassPolicy = x.parse().unwrap();
            if (y.pass.chars().nth(y.low - 1).unwrap() == y.letter)
                ^ (y.pass.chars().nth(y.high - 1).unwrap() == y.letter)
            {
                return 1;
            } else {
                return 0;
            }
        })
        .sum()
}
