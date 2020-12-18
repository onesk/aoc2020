#![feature(str_split_once)]

const INPUT: &'static str = include_str!("inputs/18.txt");

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Op {
    Add,
    Mul
}

fn parse_number(input: &str) -> Option<(usize, &str)> {
    let last_digit_pos = input.find(|c: char| !c.is_ascii_digit()).unwrap_or(input.len());
    let (number, tail) = input.split_at(last_digit_pos);
    let number = number.parse::<usize>().ok()?;
    Some((number, tail))
}

fn parse_parexp(input: &str) -> Option<(usize, &str)> {
    if input.chars().nth(0)? != '(' {
        return None;
    }

    let (number, tail) = parse_lassoc(input.get(1..)?)?;

    let tail = tail.trim_start();
    if tail.chars().nth(0)? != ')' {
        return None;
    }

    Some((number, tail.get(1..)?))
}

fn parse_operand(input: &str) -> Option<(usize, &str)> {
    let input = input.trim_start();
    parse_number(input).or_else(|| parse_parexp(input))
}

fn parse_parexp_advanced(input: &str) -> Option<(usize, &str)> {
    if input.chars().nth(0)? != '(' {
        return None;
    }

    let (number, tail) = parse_mul(input.get(1..)?)?;

    let tail = tail.trim_start();
    if tail.chars().nth(0)? != ')' {
        return None;
    }

    Some((number, tail.get(1..)?))
}

fn parse_operand_advanced(input: &str) -> Option<(usize, &str)> {
    let input = input.trim_start();
    parse_number(input).or_else(|| parse_parexp_advanced(input))
}

fn parse_op(input: &str) -> Option<(Op, &str)> {
    let input = input.trim_start();
    let op = match input.chars().nth(0)? {
        '+' => Op::Add,
        '*' => Op::Mul,
        _ => { return None; }
    };

    Some((op, input.get(1..)?))
}

fn parse_op_operand(input: &str) -> Option<(Op, usize, &str)> {
    let (op, tail) = parse_op(input)?;
    let (number, tail) = parse_operand(tail)?;
    Some((op, number, tail))
}

fn parse_lassoc(input: &str) -> Option<(usize, &str)> {
    let (mut cur, mut tail) = parse_operand(input)?;

    while let Some((op, number, ntail)) = parse_op_operand(tail) {
        match op {
            Op::Add => { cur += number; },
            Op::Mul => { cur *= number; }
        }

        tail = ntail;
    }

    Some((cur, tail))
}

fn parse_add(input: &str) -> Option<(usize, &str)> {
    let (lhs, tail) = parse_operand_advanced(input)?;
    let sum = parse_op(tail).filter(|&(op, _)| op == Op::Add).and_then(|(_, tail)| {
        let (rhs, tail) = parse_add(tail)?;
        Some((lhs + rhs, tail))
    });

    Some(sum.unwrap_or((lhs, tail)))
}

fn parse_mul(input: &str) -> Option<(usize, &str)> {
    let (lhs, tail) = parse_add(input)?;
    let product = parse_op(tail).filter(|&(op, _)| op == Op::Mul).and_then(|(_, tail)| {
        let (rhs, tail) = parse_mul(tail)?;
        Some((lhs * rhs, tail))
    });

    Some(product.unwrap_or((lhs, tail)))
}

fn eval_line_simple(line: &str) -> usize {
    let (val, _) = parse_lassoc(line).expect("correct parse");
    val
}

fn eval_line_advanced(line: &str) -> usize {
    let (val, _) = parse_mul(line).expect("correct parse");
    val
}

fn part_one() -> usize {
    INPUT.lines().map(eval_line_simple).sum()
}

fn part_two() -> usize {
    INPUT.lines().map(eval_line_advanced).sum()
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

#[test]
fn example_1() {
    assert_eq!(eval_line_simple("2 * 3 + (4 * 5)"), 26);
    assert_eq!(eval_line_simple("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(eval_line_simple("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(eval_line_simple("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632);
}

#[test]
fn example_2() {
    assert_eq!(eval_line_advanced("1 + 2 * 3 + 4 * 5 + 6"), 231);
    assert_eq!(eval_line_advanced("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(eval_line_advanced("2 * 3 + (4 * 5)"), 46);
    assert_eq!(eval_line_advanced("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(eval_line_advanced("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
}
