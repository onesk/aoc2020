#[macro_use]
extern crate lazy_static;

use std::collections::{HashSet, HashMap, hash_map::Entry};

use regex::Regex;

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"([^(]+)\(contains ([^)]+)\)").expect("compiles");
}

const INPUT: &'static str = include_str!("inputs/21.txt");

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>
}

fn foods(input: &str) -> Option<Vec<Food>> {
    input.lines().map(str::trim).map(|line| {
        let captures = LINE_RE.captures(line)?;

        let ingredients = captures.get(1)?.as_str();
        let allergens = captures.get(2)?.as_str();

        let ingredients = ingredients.split_whitespace().map(str::to_string).collect();
        let allergens = allergens.split(", ").map(str::to_string).collect();

        Some(Food { ingredients, allergens })
    }).collect()
}

fn part_both(input: &str) -> (usize, String) {
    let foods = foods(input).expect("correct parse");

    let all_ingredients: HashSet<String> = foods.iter().flat_map(|f| f.ingredients.iter()).cloned().collect();

    let mut allergen_source: HashMap<String, HashSet<String>> = HashMap::new();

    for food in foods.iter() {
        let cur_ingredients: HashSet<String> = food.ingredients.iter().cloned().collect();
        for allergen in food.allergens.iter().cloned() {
            match allergen_source.entry(allergen) {
                Entry::Occupied(mut oe) => {
                    let vr = oe.get_mut();
                    *vr = vr.intersection(&cur_ingredients).cloned().collect();
                },

                Entry::Vacant(ve) => {
                    ve.insert(cur_ingredients.clone());
                }
            }
        }
    }

    let mut identified_allergens = HashSet::new();
    let mut identified_ingredients = HashSet::new();
    let mut identifieds = Vec::<(String, String)>::new();

    while !allergen_source.is_empty() {
        for (allergen, ingredients) in allergen_source.clone() {
            if ingredients.len() == 1 {
                let ingredient: String = ingredients.iter().next().cloned().expect("exactly one");
                identified_allergens.insert(allergen.clone());
                identified_ingredients.insert(ingredient.clone());

                identifieds.push((allergen, ingredient));

            } else {
                let less_ingredients = allergen_source
                    .remove(&allergen).expect("iterator invariants")
                    .difference(&identified_ingredients)
                    .cloned()
                    .collect();

                allergen_source.insert(allergen, less_ingredients);
            }
        }

        for allergen in &identified_allergens {
            allergen_source.remove(allergen);
        }
    }

    let impossibru_ingredients: HashSet<String> = all_ingredients.difference(&identified_ingredients).cloned().collect();

    identifieds.sort();
    let canonical_list = identifieds.into_iter().map(|(_, v)| v).collect::<Vec<_>>().join(",");
    let impossibru_occ = foods.iter().flat_map(|f| f.ingredients.iter()).filter(|&i| impossibru_ingredients.contains(i)).count();

    (impossibru_occ, canonical_list)
}

fn main() {
    println!("{:?}", part_both(INPUT));
}

#[test]
fn example() {
    let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    assert_eq!(part_both(&input), (5, "mxmxvkd,sqjhc,fvjkl".to_string()));
}
