use std::collections::HashSet;

const INPUT: &'static str = include_str!("inputs/1.txt");

fn entries() -> impl Iterator<Item=u32> + Clone {
    INPUT.lines().map(|s| s.parse::<u32>().expect("unsigned integer"))
}

fn summing_pair(sum: u32) -> Option<(u32, u32)> {
    let mut seen = HashSet::new();

    for entry in entries() {
        if entry <= sum && seen.contains(&(sum - entry)) {
            return Some((entry, sum - entry))
        }

        seen.insert(entry);
    }

    None
}

fn summing_triple(sum: u32) -> Option<(u32, u32, u32)> {
    let mut seen = HashSet::new();

    for third_entry in entries() {
        if third_entry <= sum {
            for &second_entry in seen.iter() {
                if second_entry <= sum - third_entry && seen.contains(&(sum - third_entry - second_entry)) {
                    return Some((sum - third_entry - second_entry, second_entry, third_entry))
                }
            }
        }

        seen.insert(third_entry);
    }

    None
}

fn part_one() {
    let (a, b) = summing_pair(2020).expect("summing pair exists");
    println!("{}", a * b);
}

fn part_two() {
    let (a, b, c) = summing_triple(2020).expect("summing triple exists");
    println!("{}", a * b * c);
}

fn main() {
    part_one();
    part_two();
}
