use itertools::Itertools;

const INPUT: &'static str = include_str!("inputs/5.txt");

fn strbin(one: char, s: &str) -> usize {
    s.chars().fold(0, |acc, c| acc + acc + (one == c) as usize)
}

fn seatdecode(s: &str) -> (usize, usize) {
    let (row_str, column_str) = s.split_at(7);
    (strbin('B', row_str), strbin('R', column_str))
}

fn seat_ids(input: &str) -> impl Iterator<Item = usize> + Clone + '_ {
    input.lines().map(seatdecode).map(|(r, c)| 8*r + c)
}

fn part_one() {
    println!("{}", seat_ids(INPUT).max().expect("exists"));
}

fn intsum(upto: usize) -> usize {
    upto * (upto + 1) / 2
}

fn part_two() {
    let iter = seat_ids(INPUT);
    let (min, max) = iter.clone().minmax().into_option().expect("exists");
    let sum: usize = iter.sum();
    println!("{}", intsum(max) - intsum(min-1) - sum);
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn examples() {
    assert_eq!(seatdecode("FBFBBFFRLR"), (44, 5));
    assert_eq!(seatdecode("BFFFBBFRRR"), (70, 7));
}
