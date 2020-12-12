use std::f64::consts::PI;

const INPUT: &'static str = include_str!("inputs/12.txt");

#[derive(Clone, Copy, Debug)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

impl Action {
    fn from_char(c: char) -> Option<Action> {
        match c {
            'N' => Some(Action::North),
            'S' => Some(Action::South),
            'E' => Some(Action::East),
            'W' => Some(Action::West),
            'L' => Some(Action::Left),
            'R' => Some(Action::Right),
            'F' => Some(Action::Forward),
            _ => None
        }
    }
}

#[derive(Debug)]
struct State {
    e: f64,
    n: f64,
    wpe: f64,
    wpn: f64,
    dir: f64
}

impl State {
    fn new() -> Self {
        Self {
            e: 0f64, n: 0f64,
            wpe: 10f64, wpn: 1f64,
            dir: 0f64
        }
    }

    fn manhattan_distance(&self) -> f64 {
        self.e.abs() + self.n.abs()
    }

    fn rotate_waypoint(&mut self, degrees: f64) {
        let radians = degrees * PI / 180.;

        let wpe = self.wpe * radians.cos() - self.wpn * radians.sin();
        let wpn = self.wpe * radians.sin() + self.wpn * radians.cos();

        self.wpe = wpe;
        self.wpn = wpn;
    }

    fn apply_action(&mut self, action: Action, arg: usize) {
        let arg = arg as f64;

        match action {
            Action::North => { self.n += arg; },
            Action::South => { self.n -= arg; },
            Action::East => { self.e += arg; },
            Action::West => { self.e -= arg; },
            Action::Left => { self.dir += arg; },
            Action::Right => { self.dir -= arg; },
            Action::Forward => {
                let radians = self.dir * PI / 180.;
                self.e += arg * radians.cos();
                self.n += arg * radians.sin();
            }
        }
    }

    fn apply_action_waypoint(&mut self, action: Action, arg: usize) {
        let arg = arg as f64;

        match action {
            Action::North => { self.wpn += arg; },
            Action::South => { self.wpn -= arg; },
            Action::East => { self.wpe += arg; },
            Action::West => { self.wpe -= arg; },
            Action::Left => { self.rotate_waypoint(arg); },
            Action::Right => { self.rotate_waypoint(-arg); },
            Action::Forward => {
                self.n += self.wpn * arg;
                self.e += self.wpe * arg;
            }
        }
    }
}

fn actions(input: &str) -> Option<Vec<(Action, usize)>> {
    input.lines().map(|s| {
        let s = s.trim();
        let action = s.chars().nth(0)?;
        let arg = s.get(1..)?.parse::<usize>().ok()?;
        Some((Action::from_char(action)?, arg))
    }).collect()
}

fn part_one(input: &str) -> f64 {
    let actions = actions(input).expect("correct parse");

    let mut state = State::new();

    for (action, arg) in actions {
        state.apply_action(action, arg);
    }

    state.manhattan_distance()
}

fn part_two(input: &str) -> f64 {
    let actions = actions(input).expect("correct parse");

    let mut state = State::new();

    for (action, arg) in actions {
        state.apply_action_waypoint(action, arg);
    }

    state.manhattan_distance()
}

fn main() {
    println!("{:.2}", part_one(INPUT));
    println!("{:.2}", part_two(INPUT));
}

#[test]
fn example() {
    let input = r"F10
N3
F7
R90
F11";

    assert_eq!(part_one(&input), 25.);
    assert_eq!(part_two(&input), 286.);
}
