#![feature(str_split_once)]

use std::collections::{VecDeque, HashSet};

const INPUT: &'static str = include_str!("inputs/7.txt");

const CONTAIN: &'static str = " bags contain ";

type Rules = Vec<(String, Vec<(usize, String)>)>;

fn rules(input: &str) -> Option<Rules> {
    input.lines()
        .map(|line| {
            let contain_pos = line.find(CONTAIN)?;
            let outer = line.get(..contain_pos)?;
            let inner = line.get((contain_pos + CONTAIN.len())..)?;

            let inner = if inner == "no other bags." {
                Vec::new()
            } else {
                let inner: Option<Vec<_>> = inner.strip_suffix(".")?.split(", ")
                    .map(|bag| {
                        let pair = bag.strip_suffix(" bag").or_else(|| bag.strip_suffix(" bags"))?;
                        let (cnt, name) = pair.split_once(" ")?;
                        let cnt = cnt.parse::<usize>().ok()?;
                        Some((cnt, name.to_string()))
                    })
                    .collect();

                inner?
            };

            Some((outer.to_string(), inner))
        })
        .collect()
}

fn outer_for<'a>(rules: &'a Rules, inner: &'a str) -> impl Iterator<Item=&'a String> {
    rules.iter().filter_map(move |(outer, inners)| {
        inners.iter().find(|(_, some_inner)| some_inner == inner).map(|_| outer)
    })
}

fn bfs_outer(rules: &Rules, from: &str) -> HashSet<String> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(from.to_string());

    while let Some(current_inner) = queue.pop_front() {
        for outer in outer_for(rules, &current_inner) {
            if !seen.contains(outer) {
                seen.insert(outer.clone());
                queue.push_back(outer.clone());
            }
        }
    }

    seen
}

fn total_bags(rules: &Rules, outer: &str) -> Option<usize> {
    let (_, inners) = rules.iter().find(|(bag, _)| bag == outer)?;
    inners.iter()
        .map(|(cnt, inner)| Some(cnt * total_bags(rules, inner)?))
        .sum::<Option<usize>>()
        .map(|total| total + 1)
}

fn eventual_outers(input: &str) -> usize {
    let rules = rules(input).expect("correct parse");
    let seen = bfs_outer(&rules, "shiny gold");
    seen.len()
}

fn total_inners(input: &str) -> usize {
    let rules = rules(input).expect("correct parse");
    total_bags(&rules, "shiny gold").expect("bag exists") - 1
}

fn part_one() {
    println!("{}", eventual_outers(INPUT));
}

fn part_two() {
    println!("{}", total_inners(INPUT));
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn example_1() {
    let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    assert_eq!(eventual_outers(&input), 4);
    assert_eq!(total_inners(&input), 32);
}

#[test]
fn example_2() {
    let input = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    assert_eq!(total_inners(&input), 126);
}
