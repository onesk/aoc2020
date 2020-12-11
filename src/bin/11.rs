use std::fmt;

const INPUT: &'static str = include_str!("inputs/11.txt");

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Seat {
    Floor,
    Empty,
    Occupied
}

impl Seat {
    fn from_char(c: char) -> Option<Seat> {
        match c {
            'L' => Some(Seat::Empty),
            '.' => Some(Seat::Floor),
            _ => None
        }
    }
}

#[derive(Eq, PartialEq)]
struct Grid {
    w: usize,
    h: usize,
    seats: Vec<Seat>
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.seats.chunks(self.w) {
            for seat in row {
                let s = match seat {
                    Seat::Occupied => "#",
                    Seat::Empty => "L",
                    Seat::Floor => ".",
                };

                write!(f, "{}", s)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn from_vecs(vecs: Vec<Vec<Seat>>) -> Option<Self> {
        let w = vecs.get(0).map_or(0, |v| v.len());

        if vecs.iter().any(|v| v.len() != w) {
            return None;
        }

        let h = vecs.len();
        let seats = vecs.concat();

        Some(Self { w, h, seats })
    }

    fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.w && y < self.h {
            Some(y * self.w + x)
        } else {
            None
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Seat> {
        Some(self.seats[self.index(x, y)?])
    }

    fn near_occ_neighbors(&self, x: usize, y: usize) -> usize {
        let mut occ = 0;

        for sx in (x.max(1)-1)..=(x+1).min(self.w-1) {
            for sy in (y.max(1)-1)..=(y+1).min(self.h-1) {
                if let Some(Seat::Occupied) = self.get(sx, sy) {
                    occ += (sx != x || sy != y) as usize;
                }
            }
        }

        occ
    }

    fn update_los_occ(&self, los: &mut Vec<Vec<usize>>, pos: impl Iterator<Item=(usize, usize)>) {
        let seats: Vec<(Seat, usize, usize)> = pos
            .map(|(x, y)| (self.get(x, y).unwrap(), x, y))
            .filter(|&(s, _, _)| s != Seat::Floor)
            .collect();

        for pair in seats.windows(2) {
            let (s1, x1, y1) = pair[0];
            let (s2, x2, y2) = pair[1];

            if s2 == Seat::Occupied {
                los[x1][y1] += 1;
            }

            if s1 == Seat::Occupied {
                los[x2][y2] += 1;
            }
        }
    }

    fn los_occ_neighbors(&self) -> Vec<Vec<usize>> {
        let mut los = vec![vec![0; self.h]; self.w];

        // -

        for y in 0..self.h {
            self.update_los_occ(&mut los, (0..self.w).map(|x| (x, y)));
        }

        // |

        for x in 0..self.w {
            self.update_los_occ(&mut los, (0..self.h).map(|y| (x, y)));
        }

        // \ /

        for x in 0..self.w {
            self.update_los_occ(&mut los, (0..self.h.min(self.w - x)).map(|y| (x+y, y)));
            self.update_los_occ(&mut los, (0..self.h.min(self.w - x)).map(|y| (self.w-1-x-y, y)));
        }

        for y in 1..self.h {
            self.update_los_occ(&mut los, (0..self.w.min(self.h - y)).map(|x| (x, x+y)));
            self.update_los_occ(&mut los, (0..self.w.min(self.h - y)).map(|x| (self.w-1-x, x+y)));
        }

        los
    }

    fn step_near(&self) -> Grid {
        let vecs: Vec<Vec<Seat>> = (0..self.h).map(|y| (0..self.w).map(|x| {
            let occ = self.near_occ_neighbors(x, y);
            let seat = self.get(x, y).unwrap();

            if seat == Seat::Empty && occ == 0 {
                Seat::Occupied
            } else if seat == Seat::Occupied && occ >= 4 {
                Seat::Empty
            } else {
                seat
            }

        }).collect()).collect();

        Self::from_vecs(vecs).unwrap()
    }

    fn step_los(&self) -> Grid {
        let occs = self.los_occ_neighbors();

        let vecs: Vec<Vec<Seat>> = (0..self.h).map(|y| (0..self.w).map(|x| {
            let occ = occs[x][y];
            let seat = self.get(x, y).unwrap();

            if seat == Seat::Empty && occ == 0 {
                Seat::Occupied
            } else if seat == Seat::Occupied && occ >= 5 {
                Seat::Empty
            } else {
                seat
            }

        }).collect()).collect();

        Self::from_vecs(vecs).unwrap()
    }
}

type Seats = Vec<Vec<Seat>>;

fn grid(input: &str) -> Option<Grid> {
    let vecs: Option<Vec<Vec<Seat>>> = input.lines()
        .map(|s| s.trim().chars().map(Seat::from_char).collect())
        .collect();

    Grid::from_vecs(vecs?)
}

fn part_one(input: &str) -> usize {
    let mut grid = grid(input).expect("correct parse");

    let mut cnt = 0;
    while true {
        let new_grid = grid.step_near();

        if new_grid == grid {
            break;
        }

        grid = new_grid;
    }

    grid.seats.iter().filter(|&&s| s == Seat::Occupied).count()
}

fn part_two(input: &str) -> usize {
    let mut grid = grid(input).expect("correct parse");

    let mut cnt = 0;
    while true {
        let new_grid = grid.step_los();

        if new_grid == grid {
            break;
        }

        grid = new_grid;
    }

    grid.seats.iter().filter(|&&s| s == Seat::Occupied).count()
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example() {
    let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    assert_eq!(part_one(&input), 37);
    assert_eq!(part_two(&input), 26);
}
