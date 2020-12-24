use std::collections::{HashSet, HashMap};

const INPUT: &'static str = include_str!("inputs/24.txt");

const P2_DAYS: usize = 100;

type Dir = (isize, isize);

const DIRS: [(&'static str, Dir); 6] = [
    ("e", (1, 0)),
    ("w", (-1, 0)),
    ("sw", (0, -1)),
    ("se", (1, -1)),
    ("nw", (-1, 1)),
    ("ne", (0, 1))
];

fn consume_dir(s: &str) -> Option<(Dir, &str)> {
    DIRS.iter().find(|(pat, _)| s.starts_with(pat)).map(|(pat, dir)| (dir.clone(), &s[pat.len()..]))
}

fn tiles(input: &str) -> Option<Vec<Vec<Dir>>> {
    input.lines().map(str::trim).map(|mut l| {
        let mut dirs = Vec::new();

        while !l.is_empty() {
            let (dir, l_) = consume_dir(l)?;

            l = l_;
            dirs.push(dir);
        }

        Some(dirs)
    }).collect()
}

fn sum_dirs(dirs: &[Dir]) -> Dir {
    dirs.iter().fold((0, 0), |(ax, ay), (x, y)| (ax+x, ay+y))
}

fn adjacent((x, y): Dir) -> impl Iterator<Item=Dir> {
    DIRS.iter().map(move |(_, (dx, dy))| (x+dx, y+dy))
}

#[derive(Debug)]
struct Tileset(HashSet<Dir>);

impl Tileset {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn black_tiles(&self) -> usize {
        self.0.len()
    }

    fn toggle(&mut self, tile: Dir) {
        if !self.0.remove(&tile) {
            self.0.insert(tile);
        }
    }

    fn next_generation(&self) -> Self {
        let mut next = self.0.clone();

        let mut adj_blacks = HashMap::<Dir, usize>::new();

        for black in self.0.iter().flat_map(|&d| adjacent(d)) {
            *adj_blacks.entry(black).or_insert(0) += 1;
        }

        for (&dir, &cnt) in adj_blacks.iter() {
            if cnt == 2 && !self.0.contains(&dir) {
                next.insert(dir);
            }
        }

        for &white in self.0.iter() {
            let cnt = adjacent(white).filter(|d| self.0.contains(d)).count();
            if cnt == 0 || cnt > 2 {
                next.remove(&white);
            }
        }

        Self(next)
    }
}

fn starting_tileset(input: &str) -> Tileset {
    let tiles = tiles(input).expect("correct parse");

    let mut tileset = Tileset::new();

    for tile in tiles {
        let sum = sum_dirs(&tile);
        tileset.toggle(sum);
    }

    tileset
}

fn part_one(input: &str) -> usize {
    starting_tileset(input).black_tiles()
}

fn part_two(input: &str) -> usize {
    let mut tileset = starting_tileset(input);

    for _ in 0..P2_DAYS {
        tileset = tileset.next_generation();
    }

    tileset.black_tiles()
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example() {
    let input = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    assert_eq!(part_one(&input), 10);
    assert_eq!(part_two(&input), 2208);
}
