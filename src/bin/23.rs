use std::char;
use std::collections::{VecDeque, HashMap};

const INPUT: &'static str = "685974213";

const P1_LABELS: u32 = 10;
const P1_MOVES: usize = 100;

const P2_LABELS: u32 = 1_000_000;
const P2_MOVES: usize = 10_000_000;

type Circle = VecDeque<u32>;

fn circle(input: &str) -> Option<Circle> {
    input.chars().map(|c| c.to_digit(10)).collect()
}

fn do_move_p1(circle: &mut Circle) {
    let current = circle.front().cloned().expect("should be present");
    circle.rotate_left(1);

    let cup_1 = circle.pop_front().expect("should be present");
    let cup_2 = circle.pop_front().expect("should be present");
    let cup_3 = circle.pop_front().expect("should be present");

    let dest_pos = (1..=10).map(|d| (current + P1_LABELS - d) % P1_LABELS)
        .find_map(|dl| circle.iter().rposition(|&cl| dl == cl))
        .expect("destination should exist") + 1;

    circle.insert(dest_pos, cup_3);
    circle.insert(dest_pos, cup_2);
    circle.insert(dest_pos, cup_1);
}

fn part_one(input: &str) -> String {
    let mut circle = circle(input).expect("correct parse");

    for _ in 0..P1_MOVES {
        do_move_p1(&mut circle);
    }

    let shift = circle.iter().position(|&c| c == 1).expect("1 present");
    circle.rotate_left(shift);
    circle.pop_front().expect("non empty");

    circle.into_iter().map(|c| char::from_digit(c, 10).expect("valid decimal")).collect()
}

#[derive(Debug)]
struct CircleList {
    current: u32,
    prev: HashMap<u32, u32>,
    next: HashMap<u32, u32>
}

impl CircleList {
    fn from_circle(mut circle: Circle) -> Self {
        let slice = circle.make_contiguous();

        let mut prev = HashMap::new();
        let mut next = HashMap::new();

        for w in slice.windows(2) {
            next.insert(w[0], w[1]);
            prev.insert(w[1], w[0]);
        }

        let first = slice.first().cloned().expect("at least one");
        let last = slice.last().cloned().expect("at least one");

        next.insert(last, first);
        prev.insert(first, last);

        Self { current: first, prev, next }
    }

    fn pop_after_current(&mut self) -> Option<u32> {
        let ret_key = self.next.get(&self.current).cloned()?;

        let ret_next = self.next.remove(&ret_key)?;
        let ret_prev = self.prev.remove(&ret_key)?;
        assert!(ret_prev == self.current);

        self.next.insert(ret_prev, ret_next);
        self.prev.insert(ret_next, ret_prev);

        Some(ret_key)
    }

    fn push_after_key(&mut self, key: u32, after_key: u32) {
        let key_next = self.next.remove(&key).expect("key should be present");

        self.next.insert(key, after_key);
        self.next.insert(after_key, key_next);

        self.prev.insert(key_next, after_key);
        self.prev.insert(after_key, key);
    }

    fn advance_current(&mut self) {
        self.current = self.next.get(&self.current).cloned().expect("current consistent");
    }
}

fn do_move_p2(circle_list: &mut CircleList) {
    let cup_1 = circle_list.pop_after_current().expect("should pop");
    let cup_2 = circle_list.pop_after_current().expect("should pop");
    let cup_3 = circle_list.pop_after_current().expect("should pop");

    let dest_key = (2..=10).map(|d| (circle_list.current + P2_LABELS - d) % P2_LABELS + 1)
        .find(|&dk| dk != cup_1 && dk != cup_2 && dk != cup_3)
        .expect("destination exists");

    circle_list.push_after_key(dest_key, cup_3);
    circle_list.push_after_key(dest_key, cup_2);
    circle_list.push_after_key(dest_key, cup_1);

    circle_list.advance_current();
}

fn part_two(input: &str) -> usize {
    let mut circle = circle(input).expect("correct parse");
    circle.extend(P1_LABELS..=P2_LABELS);

    let mut circle_list = CircleList::from_circle(circle);

    for i in 0..P2_MOVES {
        do_move_p2(&mut circle_list);
    }

    while circle_list.current != 1 {
        circle_list.advance_current();
    }

    let cup_1 = circle_list.pop_after_current().expect("should be present") as usize;
    let cup_2 = circle_list.pop_after_current().expect("should be present") as usize;

    cup_1 * cup_2
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example() {
    assert_eq!(part_one("389125467"), "67384529");
    assert_eq!(part_two("389125467"), 149245887792);
}
