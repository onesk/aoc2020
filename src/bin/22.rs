use std::collections::{VecDeque, HashSet, HashMap};

const INPUT: &'static str = include_str!("inputs/22.txt");

type Deck = VecDeque<usize>;

fn decks(input: &str) -> Option<(Deck, Deck)> {
    let lines: Vec<_> = input.lines().map(str::trim).collect();

    let players = lines.split(|s| s.is_empty()).enumerate().map(|(i, part)| {
        let (header, cards) = part.split_first()?;

        if header != &format!("Player {}:", i+1) {
            return None;
        }

        cards.iter().map(|c| c.parse::<usize>().ok()).collect()
    }).collect::<Option<Vec<Deck>>>()?;

    if players.len() != 2 {
        return None;
    }

    Some((players[0].clone(), players[1].clone()))
}

#[derive(Copy, Clone)]
enum Winner {
    First,
    Second
}

fn score(deck_1: Deck, deck_2: Deck, winner: Winner) -> usize {
    let deck = match winner {
        Winner::First => deck_1,
        Winner::Second => deck_2
    };

    deck.into_iter().rev().enumerate().map(|(i, v)| (i+1) * v).sum()
}

fn play_combat(deck_1: &mut Deck, deck_2: &mut Deck) -> Winner {
    while !deck_1.is_empty() && !deck_2.is_empty() {
        let front_1 = deck_1.pop_front().expect("both nonempty");
        let front_2 = deck_2.pop_front().expect("both nonempty");

        if front_1 > front_2 {
            deck_1.push_back(front_1);
            deck_1.push_back(front_2);

        } else {
            deck_2.push_back(front_2);
            deck_2.push_back(front_1);

        }
    }

    if deck_1.is_empty() { Winner::Second } else { Winner::First }
}

type Visited = HashSet<(Deck, Deck)>;

fn play_recursive_combat(deck_1: &mut Deck, deck_2: &mut Deck) -> Winner {
    let mut visited = Visited::new();

    loop {
        if !visited.insert((deck_1.clone(), deck_2.clone())) {
            return Winner::First;
        }

        if deck_1.is_empty() || deck_2.is_empty() {
            break;
        }

        let front_1 = deck_1.pop_front().expect("both nonempty");
        let front_2 = deck_2.pop_front().expect("both nonempty");

        let winner = if deck_1.len() >= front_1 && deck_2.len() >= front_2 {
            let mut deck_1 = deck_1.iter().take(front_1).cloned().collect();
            let mut deck_2 = deck_2.iter().take(front_2).cloned().collect();
            play_recursive_combat(&mut deck_1, &mut deck_2)
        } else {
            if front_1 > front_2 { Winner::First } else { Winner::Second }
        };

        match winner {
            Winner::First => {
                deck_1.push_back(front_1);
                deck_1.push_back(front_2);
            },

            Winner::Second => {
                deck_2.push_back(front_2);
                deck_2.push_back(front_1);
            }
        }
    }

    if deck_1.is_empty() { Winner::Second } else { Winner::First }
}

fn part_one(input: &str) -> usize {
    let (mut deck_1, mut deck_2) = decks(input).expect("correct parse");
    let winner = play_combat(&mut deck_1, &mut deck_2);
    score(deck_1, deck_2, winner)
}

fn part_two(input: &str) -> usize {
    let (mut deck_1, mut deck_2) = decks(input).expect("correct parse");
    let winner = play_recursive_combat(&mut deck_1, &mut deck_2);
    score(deck_1, deck_2, winner)
}

fn main() {
    println!("{}", part_one(INPUT));
    println!("{}", part_two(INPUT));
}

#[test]
fn example() {
    let input = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";

    assert_eq!(part_one(&input), 306);
    assert_eq!(part_two(&input), 291);
}
