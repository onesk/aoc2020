use num_bigint::BigUint;

const INPUT: &'static str = include_str!("inputs/10.txt");

fn joltages(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|s| s.trim().parse::<usize>().ok()).collect()
}

fn chain_distribution(jolts: &[usize]) -> Option<usize> {
    let mut all = jolts.to_vec();

    all.push(jolts.iter().copied().max()? + 3);
    all.push(0usize);
    all.sort();

    let mut counts = [0usize; 3];
    for i in 1..all.len() {
        let diff = all[i] - all[i-1];
        if 1 <= diff && diff <= 3 {
            counts[diff-1] += 1;

        } else {
            return None;

        }
    }

    Some(counts[0] * counts[2])
}

fn arrangements(jolts: &[usize]) -> BigUint {
    let mut all = jolts.to_vec();

    all.push(0usize);
    all.sort();

    let mut dp: Vec<BigUint> = Vec::new();
    dp.push(1usize.into());

    for i in 1..all.len() {
        let mut total = 0usize.into();

        for j in (0..i).rev() {
            let diff = all[i] - all[j];
            if diff > 3 {
                break;
            }

            total += dp[j].clone();
        }

        dp.push(total);
    }

    dp.last().cloned().unwrap()
}

fn part_one(input: &str) -> usize {
    let jolts = joltages(input).expect("correct parse");
    chain_distribution(&jolts).expect("exists")
}

fn part_two(input: &str) -> BigUint {
    let jolts = joltages(input).expect("correct parse");
    arrangements(&jolts)
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example() {
    let input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    assert_eq!(part_one(&input), 220);
    assert_eq!(part_two(&input), BigUint::from(19208u32))
}
