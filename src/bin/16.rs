#![feature(str_split_once)]

use std::collections::HashSet;

const INPUT: &'static str = include_str!("inputs/16.txt");

#[derive(Debug)]
struct Range {
    low: usize,
    high: usize
}

impl Range {
    fn within(&self, n: usize) -> bool {
        self.low <= n && n <= self.high
    }
}

type Ticket = Vec<usize>;
type RuleRef = (String, usize);

#[derive(Debug)]
struct Stmt {
    rules: Vec<(String, Vec<Range>)>,
    your: Ticket,
    nearby: Vec<Ticket>
}

impl Stmt {
    fn invalid_all(&self, n: usize) -> bool {
        self.rules.iter().all(|(_, rs)| rs.iter().all(|r| !r.within(n)))
    }

    fn eligible(&self, tickets: &[Ticket]) -> Vec<Vec<RuleRef>> {
        (0..tickets.get(0).map_or(0, |t| t.len())).map(|col| {
            self.rules.iter().enumerate().filter_map(|(i, (name, rs))| {
                if tickets.iter().all(|t| rs.iter().any(|r| r.within(t[col]))) {
                    Some((name.clone(), i))
                } else {
                    None
                }
            }).collect()
        }).collect()
    }
}

fn parse_ticket(line: &str) -> Option<Vec<usize>> {
    line.split(",").map(|s| s.parse::<usize>().ok()).collect()
}

fn parse(input: &str) -> Option<Stmt> {
    let lines: Vec<String> = input.lines().map(str::trim).map(str::to_string).collect();

    let mut parts = lines.split(|line| line.is_empty());

    let rules = parts.next()?.iter().map(|line| {
        let (name, ranges) = line.split_once(": ")?;

        let ranges: Option<Vec<Range>> = ranges.split(" or ").map(|range| {
            let (low, high) = range.split_once("-")?;

            let low = low.parse::<usize>().ok()?;
            let high = high.parse::<usize>().ok()?;

            Some(Range { low, high })
        }).collect();

        Some((name.to_string(), ranges?))
    }).collect::<Option<Vec<(String, Vec<Range>)>>>()?;

    let your = parse_ticket(parts.next()?.get(1)?)?;

    let nearby = parts.next()?[1..].iter().map(|s| parse_ticket(s)).collect::<Option<Vec<Vec<usize>>>>()?;

    Some(Stmt { rules, your, nearby })
}

fn part_one(input: &str) -> usize {
    let stmt = parse(input).expect("correct parse");
    stmt.nearby.iter().flat_map(|nt| nt).filter(|&&n| stmt.invalid_all(n)).sum()
}

fn tsp(columns: &[Vec<RuleRef>]) -> Option<Vec<RuleRef>> {
    fn tsp_rec(
        used_rules: &mut HashSet<usize>,
        path: &mut Vec<RuleRef>,
        columns: &[Vec<RuleRef>]) -> Result<(), Vec<RuleRef>>
    {
        if let Some((rule_refs, columns)) = columns.split_last() {
            for (name, rule_idx) in rule_refs.iter() {
                if used_rules.insert(*rule_idx) {
                    path.push((name.clone(), *rule_idx));
                    tsp_rec(used_rules, path, columns)?;
                    path.pop();
                    used_rules.remove(&rule_idx);
                }
            }
            Ok(())
        } else {
            path.reverse();
            Err(path.clone())
        }
    }

    tsp_rec(&mut HashSet::new(), &mut Vec::new(), columns).err()
}

fn part_two(input: &str) -> usize {
    let stmt = parse(input).expect("correct parse");
    let correct: Vec<Vec<usize>> = stmt.nearby.iter()
        .filter(|&nt| nt.iter().all(|&n| !stmt.invalid_all(n)))
        .cloned()
        .collect();

    let eligible = stmt.eligible(&correct);
    let path = tsp(&eligible).expect("exists");

    let ans = path.into_iter()
        .map(|(name, _)| name)
        .zip(stmt.your.iter())
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, &value)| value)
        .product();

    ans
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example_1() {
    let input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    assert_eq!(part_one(&input), 71);
}

#[test]
fn example_2() {
    let input = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    assert_eq!(part_two(&input), 71);
}
