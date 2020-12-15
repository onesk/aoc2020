use std::collections::HashMap;

const INPUT: &'static str = "11,18,0,20,1,7,16";

const P1_CNT: usize = 2020;
const P2_CNT: usize = 30000000;

fn parse(input: &str) -> Option<Vec<usize>> {
    input.split(",").map(|p| p.parse::<usize>().ok()).collect()
}

fn part_both(input: &str, index: usize) -> usize {
    let mut memory = parse(input).expect("correct parse");
    let mut last_pos: HashMap<usize, usize> = memory[..memory.len()-1].iter()
        .enumerate()
        .map(|(i, &v)| (v, i))
        .collect();

    loop {
        let (&last, head) = memory.split_last().expect("more than one");

        if memory.len() == index {
            return last;
        }

        let next = last_pos.insert(last, head.len()).map_or(0, |pos| head.len() - pos);
        memory.push(next);
    }
}

fn main() {
    println!("{}", part_both(INPUT, P1_CNT));
    println!("{}", part_both(INPUT, P2_CNT));
}

#[test]
fn example_1() {
    assert_eq!(part_both("0,3,6", P1_CNT), 436);
    assert_eq!(part_both("1,3,2", P1_CNT), 1);
    assert_eq!(part_both("2,1,3", P1_CNT), 10);
    assert_eq!(part_both("1,2,3", P1_CNT), 27);
    assert_eq!(part_both("2,3,1", P1_CNT), 78);
    assert_eq!(part_both("3,2,1", P1_CNT), 438);
    assert_eq!(part_both("3,1,2", P1_CNT), 1836);
}

#[test]
fn example_2() {
    assert_eq!(part_both("0,3,6", P2_CNT), 175594);
    assert_eq!(part_both("1,3,2", P2_CNT), 2578);
    assert_eq!(part_both("2,1,3", P2_CNT), 3544142);
    assert_eq!(part_both("1,2,3", P2_CNT), 261214);
    assert_eq!(part_both("2,3,1", P2_CNT), 6895259);
    assert_eq!(part_both("3,2,1", P2_CNT), 18);
    assert_eq!(part_both("3,1,2", P2_CNT), 362);
}
