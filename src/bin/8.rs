#![feature(str_split_once)]

const INPUT: &'static str = include_str!("inputs/8.txt");

#[derive(Copy, Clone, Debug)]
enum Insn {
    Nop, Acc, Jmp
}

impl Insn {
    fn parse(s: &str) -> Option<Insn> {
        match s {
            "nop" => Some(Insn::Nop),
            "acc" => Some(Insn::Acc),
            "jmp" => Some(Insn::Jmp),
            _ => None
        }
    }

    fn flip(&self) -> Option<Insn> {
        match self {
            Insn::Nop => Some(Insn::Jmp),
            Insn::Jmp => Some(Insn::Nop),
            _ => None
        }
    }
}

type Program = Vec<(Insn, isize)>;

fn program(input: &str) -> Option<Program> {
    input.lines()
        .map(|s| {
            let (insn, arg) = s.split_once(" ")?;
            let insn = Insn::parse(insn)?;
            let arg = arg.parse::<isize>().ok()?;
            Some((insn, arg))
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct State {
    ip: isize,
    accum: isize
}

impl State {
    fn new() -> Self {
        Self { ip: 0, accum: 0 }
    }

    fn step(&mut self, insn: Insn, arg: isize) {
        match insn {
            Insn::Nop => {
                self.ip += 1;
            },

            Insn::Acc => {
                self.ip += 1;
                self.accum += arg;
            },

            Insn::Jmp => {
                self.ip += arg;
            }
        }
    }
}

fn find_loop(program: &Program) -> Result<State, State> {
    let mut state = State::new();
    let mut seen = vec![false; program.len()];

    while !seen.get(state.ip as usize).copied().ok_or_else(|| state)? {
        seen[state.ip as usize] = true;

        let (insn, arg) = program[state.ip as usize];
        state.step(insn, arg);
    }

    Ok(state)
}

fn accum_before_repeat(input: &str) -> Option<isize> {
    let program = program(input).expect("correct parse");
    Some(find_loop(&program).ok()?.accum)
}

fn single_insn_patch(input: &str) -> Option<isize> {
    let mut program = program(input).expect("correct parse");

    for i in 0..program.len() {
        let (insn, arg) = program[i];
        if let Some(flip_insn) = insn.flip() {
            program[i] = (flip_insn, arg);

            if let Err(state) = find_loop(&program) {
                if state.ip as usize == program.len() {
                    return Some(state.accum);
                }
            }

            program[i] = (insn, arg);
        }
    }

    None
}

fn part_one() {
    println!("{}", accum_before_repeat(INPUT).expect("exists"));
}

fn part_two() {
    println!("{}", single_insn_patch(INPUT).expect("exists"));
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn example() {
    let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    assert_eq!(accum_before_repeat(&input), Some(5));
    assert_eq!(single_insn_patch(&input), Some(8));
}
