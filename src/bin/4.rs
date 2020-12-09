#[macro_use]
extern crate lazy_static;

use regex::Regex;
use smallvec::SmallVec;

const INPUT: &'static str = include_str!("inputs/4.txt");

const MANDATORY: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const ECL_VALUES: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

lazy_static! {
    static ref HCL_RE: Regex = Regex::new("^#[0-9a-f]{6}$").expect("compiles");
    static ref PID_RE: Regex = Regex::new("^[0-9]{9}$").expect("compiles");
}

fn passports(input: &str) -> Option<Vec<Vec<(String, String)>>> {
    let lines: Vec<&str> = input.lines().collect();

    let mut passports = Vec::new();
    for passport_lines in lines.split(|line| line.trim().is_empty()) {
        let fields: Option<Vec<_>> = passport_lines.iter()
            .flat_map(|line| line.split_whitespace())
            .map(|field| {
                let fields_str: SmallVec<[&str; 2]> = field.splitn(2, ":").collect();

                if let &[field_name, field_value] = &fields_str[..] {
                    Some((field_name.to_string(), field_value.to_string()))

                } else {
                    None

                }
            })
            .collect();

        let fields = fields?;

        if !fields.is_empty() {
            passports.push(fields)
        }
    }

    Some(passports)
}

fn validate(field: &str, value: &str) -> Option<bool> {
    match field {
        "byr" => value.parse::<usize>().ok().map(|v| 1920 <= v && v <= 2002),
        "iyr" => value.parse::<usize>().ok().map(|v| 2010 <= v && v <= 2020),
        "eyr" => value.parse::<usize>().ok().map(|v| 2020 <= v && v <= 2030),
        "hgt" => {
            let pos = value.find(char::is_alphabetic)?;
            let unit = value.get(pos..)?;
            let v = value.get(..pos)?.parse::<usize>().ok()?;

            match unit {
                "in" => Some(59 <= v && v <= 76),
                "cm" => Some(150 <= v && v <= 193),
                _ => None
            }
        },
        "hcl" => Some(HCL_RE.is_match(value)),
        "ecl" => Some(ECL_VALUES.iter().any(|&ecl| ecl == value)),
        "pid" => Some(PID_RE.is_match(value)),
        _ => Some(true)
    }
}

fn valid_field_set(input: &str) -> impl Iterator<Item=Vec<(String, String)>> {
    passports(input).expect("correct parse").into_iter()
        .filter(|fields| MANDATORY.iter().all(|field| fields.iter().any(|(f, _)| f == field)))
}

fn valid_passports(input: &str) -> impl Iterator<Item=Vec<(String, String)>> {
    valid_field_set(input).filter(|fields| fields.iter().all(|(field, value)| validate(field, value).unwrap_or(false)))
}

fn part_one() {
    println!("{}", valid_field_set(INPUT).count());
}

fn part_two() {
    println!("{}", valid_passports(INPUT).count());
}

fn main() {
    part_one();
    part_two();
}

#[test]
fn example_1() {
    let input = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    assert_eq!(valid_field_set(input).count(), 2);
}

#[test]
fn example_2() {
    let input = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";

    assert_eq!(valid_passports(input).count(), 0);
}

#[test]
fn example_3() {
    let input = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    assert_eq!(valid_passports(input).count(), 4);
}
