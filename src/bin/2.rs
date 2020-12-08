#[macro_use]
extern crate lazy_static;

use regex::{Regex, Captures};

const INPUT: &'static str = include_str!("inputs/2.txt");

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").expect("compiles");
}

#[derive(Debug)]
struct Rule {
    low: usize,
    high: usize,
    letter: char
}

impl Rule {
    fn from_captures<'t>(c: Captures<'t>) -> Option<(Rule, String)> {
        let low = c.get(1)?.as_str().parse::<usize>().ok()?;
        let high = c.get(2)?.as_str().parse::<usize>().ok()?;
        let letter = c.get(3)?.as_str().parse::<char>().ok()?;

        let password = c.get(4)?.as_str().to_string();

        Some((Rule { low, high, letter }, password))
    }

    fn is_valid_password_part_one(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count();
        self.low <= count && count <= self.high
    }

    fn is_valid_password_part_two(&self, password: &str) -> bool {
        let low_char = password.chars().nth(self.low-1);
        let high_char = password.chars().nth(self.high-1);
        low_char.zip(high_char).map_or(false, |(l, h)| (l == self.letter) ^ (h == self.letter))
    }
}

fn pairs() -> Option<Vec<(Rule, String)>> {
    INPUT.lines()
        .map(|l| RE.captures(l).and_then(|c| Rule::from_captures(c)))
        .collect()
}

fn valid_passwords<F>(f: F) -> usize where F: Fn(&Rule, &str) -> bool {
    pairs().expect("correct parse")
        .into_iter()
        .filter(|(rule, password)| f(rule, password))
        .count()
}

fn part_one() {
    println!("{}", valid_passwords(|r, p| r.is_valid_password_part_one(p)));
}

fn part_two() {
    println!("{}", valid_passwords(|r, p| r.is_valid_password_part_two(p)));
}

fn main() {
    part_one();
    part_two();
}
