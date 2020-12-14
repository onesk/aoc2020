#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use itertools::iterate;
use regex::Regex;

const INPUT: &'static str = include_str!("inputs/14.txt");

lazy_static! {
    static ref MASK_OP: Regex = Regex::new(r"^mask = ([01X]{36})$").expect("compiles");
    static ref MEM_OP: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").expect("compiles");
}

#[derive(Clone, Copy, Debug)]
struct Masks {
    and_mask: u64,
    or_mask: u64
}

impl Masks {
    fn new() -> Self {
        Self { and_mask: 0, or_mask: 0 }
    }

    fn append_bits(&self, and_bit: u64, or_bit: u64) -> Self {
        let and_mask = self.and_mask * 2 + and_bit;
        let or_mask = self.or_mask * 2 + or_bit;
        Self { and_mask, or_mask }
    }
}

#[derive(Debug)]
enum Op {
    Mask(Masks),
    Mem { dest: u64, value: u64 }
}

fn program(input: &str) -> Option<Vec<Op>> {
    input.lines().map(|line| {
        if let Some(mask) = MASK_OP.captures(line) {
            let masks = mask.get(1)?.as_str().chars().fold(
                Masks::new(),
                |masks, c| {
                    let (and_bit, or_bit) = match c {
                        'X' => (1, 0),
                        '1' => (0, 1),
                        '0' => (0, 0),
                        _ => unreachable!()
                    };

                    masks.append_bits(and_bit, or_bit)
                });

            Some(Op::Mask(masks))
        } else if let Some(mem) = MEM_OP.captures(line) {
            let dest = mem.get(1)?.as_str().parse::<u64>().ok()?;
            let value = mem.get(2)?.as_str().parse::<u64>().ok()?;

            Some(Op::Mem { dest, value })
        } else {
            None
        }
    }).collect()
}

fn part_one(input: &str) -> u64 {
    let program = program(input).expect("correct parse");

    let mut memory = HashMap::<u64, u64>::new();
    let mut last_mask = None;

    for op in program {
        match op {
            Op::Mask(mask) => { last_mask = Some(mask); },
            Op::Mem { dest, value } => {
                let mask = last_mask.expect("must be set");
                memory.insert(dest, (value & mask.and_mask) | mask.or_mask);
            }
        }
    }

    memory.values().sum()
}

fn subset_masks(mask: u64) -> impl Iterator<Item=u64> {
    iterate(mask, move |&m| (m.wrapping_sub(1)) & mask).take_while(|&m| m != 0)
}

fn part_two(input: &str) -> u64 {
    let program = program(input).expect("correct parse");

    let mut memory = HashMap::<u64, u64>::new();
    let mut last_mask = None;

    for op in program {
        match op {
            Op::Mask(mask) => { last_mask = Some(mask); },
            Op::Mem { dest, value } => {
                let mask = last_mask.expect("must be set");
                let dest = (dest | mask.or_mask) & !mask.and_mask;

                memory.insert(dest, value);

                for or_mask in subset_masks(mask.and_mask) {
                    memory.insert(dest | or_mask, value);
                }
            }
        }
    }

    memory.values().sum()
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example_1() {
    let input = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    assert_eq!(part_one(&input), 165);
}

#[test]
fn example_2() {
    let input = r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    assert_eq!(part_two(&input), 208);
}
