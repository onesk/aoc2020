use itertools::Itertools;

const INPUT: &'static str = include_str!("inputs/6.txt");

fn sets(input: &str) -> Vec<Vec<String>> {
    let lines: Vec<_> = input.lines().collect();

    lines.split(|line| line.trim().is_empty())
        .map(|group| group.iter().map(|&s| s.trim().to_string()).collect())
        .collect()
}

fn char_union<F: AsRef<str>>(group: &[F]) -> usize {
    let mut all: Vec<char> = group.iter().flat_map(|s| s.as_ref().chars()).collect();
    all.sort();
    all.dedup();
    all.len()
}

fn char_intersect<F: AsRef<str>>(group: &[F]) -> usize {
    let mut all: Vec<char> = group.iter().flat_map(|s| s.as_ref().chars()).collect();
    all.sort();
    all.iter().group_by(|&c| c).into_iter().map(|(_, same)| (same.count() == group.len()) as usize).sum()
}

fn sum_union_counts(input: &str) -> usize {
    sets(input).into_iter().map(|g| char_union(&g)).sum()
}

fn sum_intersect_counts(input: &str) -> usize {
    sets(input).into_iter().map(|g| char_intersect(&g)).sum()
}

fn part_one() {
    println!("{}", sum_union_counts(INPUT));
}

fn part_two() {
    println!("{}", sum_intersect_counts(INPUT));
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn example() {
    let input = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    assert_eq!(sum_union_counts(input), 11);
    assert_eq!(sum_intersect_counts(input), 6);
}
