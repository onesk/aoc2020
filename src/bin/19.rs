#![feature(str_split_once)]

use std::collections::HashMap;

const INPUT: &'static str = include_str!("inputs/19.txt");
const MAX_REPEAT: usize = 6;

#[derive(Debug)]
enum Rule {
    Letter(char),
    Recurse(Vec<Vec<usize>>)
}

type Rules = HashMap<usize, Rule>;

#[derive(Debug)]
struct Stmt {
    rules: Rules,
    messages: Vec<String>
}

impl Stmt {
    fn is_correct(&self, msg: &str, alt_rules: bool) -> bool {
        fn rec<'a>(rules: &'a Rules, index: usize, msg: &'a str) -> Option<&'a str>
        {
            let rule = rules.get(&index)?;

            match rule {
                &Rule::Letter(first_char) => {
                    if msg.chars().nth(0)? != first_char {
                        None
                    } else {
                        Some(msg.get(1..)?)
                    }
                },

                Rule::Recurse(options) => {
                    options.iter().filter_map(|option| {
                        let mut msg = msg;
                        for &index in option {
                            msg = rec(rules, index, msg)?;
                        }

                        Some(msg)
                    }).nth(0)
                }
            }
        }

        if !alt_rules {
            rec(&self.rules, 0, msg).map(str::is_empty).unwrap_or(false)

        } else {
            let counts = (2..=MAX_REPEAT).flat_map(|n| (1..n).rev().map(move |m| (n, m)));
            let mut leftovers = counts.filter_map(|(n, m)| {
                let mut msg = msg;
                for _ in 0..n { msg = rec(&self.rules, 42, msg)?; }
                for _ in 0..m { msg = rec(&self.rules, 31, msg)?; }
                Some(msg)
            });

            leftovers.any(str::is_empty)
        }
    }
}

fn parse_rule(line: &str) -> Option<(usize, Rule)> {
    let (index, rule) = line.split_once(": ")?;

    let index = index.trim().parse::<usize>().ok()?;

    let rule = rule.trim();
    let rule = if rule.contains('"') {
        Rule::Letter(rule.chars().nth(1)?)

    } else {
        let options = rule.split("|").map(|part| {
            part.trim()
                .split_whitespace()
                .map(|i| i.parse::<usize>().ok())
                .collect::<Option<_>>()
        }).collect::<Option<_>>();

        Rule::Recurse(options?)
    };

    Some((index, rule))
}

fn stmt(input: &str) -> Option<Stmt> {
    let lines: Vec<&str> = input.lines().map(str::trim).collect();

    let mut parts = lines.split(|l| l.is_empty());

    let rules: Option<_> = parts.next()?.iter().map(|l| parse_rule(l)).collect();
    let rules = rules?;

    let messages = parts.next()?.iter().map(|l| l.to_string()).collect();

    Some(Stmt{ rules, messages })
}

fn part_both(input: &str, alt_rules: bool) -> usize {
    let stmt = stmt(input).expect("correct parse");
    stmt.messages.iter().filter(|s| stmt.is_correct(s, alt_rules)).count()
}

fn main() {
    println!("{}", part_both(INPUT, false));
    println!("{}", part_both(INPUT, true));
}

#[test]
fn example_1() {
    let input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    assert_eq!(part_both(&input, false), 2);
}

#[test]
fn example_2() {
    let input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

    assert_eq!(part_both(&input, false), 3);
    assert_eq!(part_both(&input, true), 12);
}
