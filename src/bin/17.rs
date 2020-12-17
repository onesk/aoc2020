use std::iter::once;
use std::collections::HashSet;
use itertools::Itertools;
use smallvec::SmallVec;

const INPUT: &'static str = include_str!("inputs/17.txt");
const CYCLES: usize = 6;

type Point = SmallVec<[isize; 4]>;
type Limits = SmallVec<[(isize, isize); 4]>;

fn parse(input: &str, dims: usize) -> HashSet<Point> {
    input.lines().map(str::trim).enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(y, _)| {
                    let mut pt = Point::from_slice(&[x as isize, y as isize]);
                    pt.resize(dims, 0);
                    pt
                })
        })
        .collect()
}

fn add_pts(pt1: &Point, pt2: &Point) -> Point {
    pt1.iter().zip(pt2.iter()).map(|(&c1, &c2)| c1 + c2).collect()
}

fn iter_pts(limits: Limits) -> Box<dyn Iterator<Item=Point>> {
    if let Some((&(min, max), limits)) = limits.split_last() {
        let outer = iter_pts(Limits::from_slice(limits)).flat_map(move |head| {
            (min..=max).map(move |c| {
                let mut pt = head.clone();
                pt.push(c);
                pt
            })
        });

        Box::new(outer)
    } else {
        Box::new(once(Point::new()))
    }
}

fn neighbors(pts: &HashSet<Point>, pt: &Point, dims: usize) -> usize {
    let mut limits = Limits::new();
    limits.resize(dims, (-1, 1));

    iter_pts(limits)
        .map(|dpt| add_pts(&dpt, pt))
        .filter(|npt| npt != pt)
        .map(|npt| pts.contains(&npt) as usize)
        .sum()
}

fn generation(pts: &HashSet<Point>, dims: usize) -> HashSet<Point> {
    let limits: Limits = (0..dims).map(|idx| {
        pts.iter()
            .map(|p| p[idx])
            .minmax()
            .into_option()
            .map_or((1, 0), |(min, max)| (min-1, max+1))
    }).collect();

    let mut ret = HashSet::new();

    for pt in iter_pts(limits) {
        let active = match (pts.contains(&pt), neighbors(pts, &pt, dims)) {
            (true, 2..=3) => true,
            (false, 3) => true,
            _ => false
        };

        if active {
            ret.insert(pt);
        }
    }

    ret
}

fn part_both(input: &str, dims: usize) -> usize {
    let mut points = parse(input, dims);

    for _ in 0..CYCLES {
        points = generation(&points, dims);
    }

    points.len()
}

fn main() {
    println!("{}", part_both(INPUT, 3));
    println!("{}", part_both(INPUT, 4));
}

#[test]
fn example() {
    let input = r".#.
..#
###";

    assert_eq!(part_both(&input, 3), 112);
    assert_eq!(part_both(&input, 4), 848);
}
