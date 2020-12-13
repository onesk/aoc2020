use num::integer::Integer;
use num::bigint::BigInt;

const INPUT: &'static str = include_str!("inputs/13.txt");

#[derive(Debug)]
struct Req {
    earliest: usize,
    periods: Vec<Option<usize>>
}

fn req(input: &str) -> Option<Req> {
    let mut lines = input.lines();

    let earliest = lines.next()?;
    let periods = lines.next()?;

    let earliest = earliest.parse::<usize>().ok()?;
    let periods: Option<Vec<Option<usize>>> = periods.split(",").map(|part| {
        let ret = if part == "x" {
            None
        } else {
            Some(part.parse::<usize>().ok()?)
        };

        Some(ret)
    }).collect();

    let periods = periods?;

    Some(Req { earliest, periods })
}

fn part_one(input: &str) -> usize {
    let reqs = req(input).expect("correct parse");
    let (wait, id) = reqs.periods.iter()
        .filter_map(|oid| oid.map(|id| (id - reqs.earliest % id, id))).min()
        .expect("at least one");

    id * wait
}

fn crt(r1: BigInt, m1: BigInt, r2: BigInt, m2: BigInt) -> (BigInt, BigInt) {
    let e = BigInt::extended_gcd(&m1, &m2);
    assert!(e.gcd == 1.into());

    let m = m1.clone() * m2.clone();
    let r = (r2 * e.x * m1 + r1 * e.y * m2) % m.clone();
    let r = (r + m.clone()) % m.clone();

    (r, m)
}

fn part_two(input: &str) -> BigInt {
    let reqs = req(input).expect("correct parse");
    let gcds: Vec<(BigInt, BigInt)> = reqs.periods.iter().enumerate()
        .filter_map(|(index, id)| id.map(|id| (BigInt::from(index), id.into())))
        .collect();

    let (mut r1, mut m1) = gcds.get(0).expect("at least one").clone();

    for (r2, m2) in gcds[1..].iter() {
        let (r1_n, m1_n) = crt(r1, m1, -r2.clone(), m2.clone());
        r1 = r1_n; m1 = m1_n;
    }

    r1
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example_1() {
    let input = r"939
7,13,x,x,59,x,31,19";

    assert_eq!(part_one(&input), 295);
    assert_eq!(part_two(&input), 1068781.into());
}

#[test]
fn example_2() {
    let input = r"0
17,x,13,19";
    assert_eq!(part_two(&input), 3417.into());

    let input = r"0
67,7,59,61";
    assert_eq!(part_two(&input), 754018.into());

    let input = r"0
67,x,7,59,61";
    assert_eq!(part_two(&input), 779210.into());

    let input = r"0
67,7,x,59,61";
    assert_eq!(part_two(&input), 1261476.into());

    let input = r"0
1789,37,47,1889";
    assert_eq!(part_two(&input), 1202161486.into());
}
