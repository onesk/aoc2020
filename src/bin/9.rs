const INPUT: &'static str = include_str!("inputs/9.txt");

fn numbers(input: &str) -> Option<Vec<usize>> {
    input.lines().map(|l| l.trim().parse::<usize>().ok()).collect()
}

fn first_incorrect_slice(numbers: &[usize], preamble: usize) -> Option<usize> {
    for k in preamble..numbers.len() {
        let window = &numbers[k-preamble..k];
        let sum = numbers[k];
        let correct = window.iter().any(|&i| window.iter().any(|&j| i != j && i + j == sum));
        if !correct {
            return Some(sum)
        }
    }

    None
}

fn weakness(numbers: &[usize], sum: usize) -> Option<usize> {
    let mut sums = (0..numbers.len())
        .flat_map(|start| {
            numbers[start..].iter()
                .enumerate()
                .scan(0usize, move |state, (i, &x)| {
                    *state += x;
                    Some((*state, start, i+1))
                })
        });

    let (_, start, len) = sums.find(|&(contiguous_sum, _, _)| contiguous_sum == sum)?;
    let res = &numbers[start..(start+len)];
    Some(res.iter().min()? + res.iter().max()?)
}

fn first_incorrect(input: &str, preamble: usize) -> Option<usize> {
    let numbers = numbers(input).expect("correct parse");
    first_incorrect_slice(&numbers, preamble)
}

fn find_weakness(input: &str, preamble: usize) -> Option<usize> {
    let numbers = numbers(input).expect("correct parse");
    let invalid_number = first_incorrect_slice(&numbers, preamble)?;
    weakness(&numbers, invalid_number)
}

fn part_one() {
    println!("{}", first_incorrect(INPUT, 25).expect("exists"));
}

fn part_two() {
    println!("{}", find_weakness(INPUT, 25).expect("exists"));
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn example() {
    let input = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    assert_eq!(first_incorrect(&input, 5), Some(127));
    assert_eq!(find_weakness(&input, 5), Some(62));
}
