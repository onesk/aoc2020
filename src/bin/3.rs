const INPUT: &'static str = include_str!("inputs/3.txt");

fn tree_map() -> Option<Vec<Vec<bool>>> {
    let rows: Vec<Vec<_>> = INPUT.lines()
        .map(str::trim).map(|s| s.chars().map(|c| c == '#').collect())
        .collect();

    if let Some((first, rest)) = rows.split_first() {
        if rest.iter().any(|r| r.len() != first.len()) {
            return None;
        }
    }

    Some(rows)
}

fn part_one() {
    let tree_count: usize = tree_map().expect("correct parse")
        .into_iter()
        .enumerate()
        .map(|(i, row)| row[(3*i) % row.len()] as usize)
        .sum();

    println!("{}", tree_count);
}

fn part_two() {
    let tree_map = tree_map().expect("correct_parse");
    let steps = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    let count_product: usize = steps.iter()
        .map(|&(i_step, j_step)| {
            tree_map.iter().enumerate().step_by(i_step)
                .map(|(i, row)| row[(j_step * i) % row.len()] as usize)
                .sum::<usize>()
        }).product();

    println!("{}", count_product);
}

fn main() {
    part_one();
    part_two();
}
