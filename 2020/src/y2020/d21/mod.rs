use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;
use itertools::Itertools;
use crate::support::*;

const EXAMPLE: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)";
const INPUT: &str = include_str!("input.txt");

struct Food
{
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl FromStr for Food
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (i_str, a_str, _) = scan(s)
            .until(" (contains ").parse::<String>()
            .until(")").parse::<String>()
            .remaining().parse::<String>();

        let ingredients = scan(&i_str).remaining().parse_vec::<String>(" ").0.into_iter().collect();
        let allergens = scan(&a_str).remaining().parse_vec::<String>(", ").0.into_iter().collect();

        Ok(Self { ingredients, allergens })
    }
}

fn map_allergens_to_ingredients(foods: &Vec<Food>) -> HashMap<String, String>
{
    let mut remaining_ingredients: HashSet<String> = foods.iter()
        .map(|f| f.ingredients.clone())
        .flatten()
        .collect();

    let mut remaining_allergens: HashSet<String> = foods.iter()
        .map(|f| f.allergens.clone())
        .flatten()
        .collect();

    let mut result = HashMap::new();

    while !remaining_allergens.is_empty()
    {
        for allergen in remaining_allergens.clone()
        {
            let mut possible_ingredients = foods.iter()
                .filter(|f| f.allergens.contains(&allergen))
                .map(|f| f.ingredients.iter().filter(|&i| remaining_ingredients.contains(i)).cloned())
                .flatten()
                .collect::<HashSet<String>>();

            for food in foods.iter().filter(|f| f.allergens.contains(&allergen))
            {
                possible_ingredients = possible_ingredients
                    .intersection(&food.ingredients)
                    .cloned()
                    .collect();
            }

            assert!(!possible_ingredients.is_empty());

            if possible_ingredients.len() == 1
            {
                let ingredient = possible_ingredients.iter().next().unwrap().clone();

                remaining_allergens.remove(&allergen);
                remaining_ingredients.remove(&ingredient);
                result.insert(allergen, ingredient);
            }
        }
    }

    result
}

pub fn part_1(input: &str) -> usize
{
    let foods = input_to_lines_parsed::<Food>(input);

    // Find the set of "bad" ingredients - i.e. ingredients
    // that contain an allergen

    let bad_ingredients = map_allergens_to_ingredients(&foods)
        .values()
        .cloned()
        .collect::<HashSet<String>>();

    // Flatten out to all listed ingredients across all foods,
    // Filter for only ingredients that don't contain an allergen
    // Count the number of clean ingredients listed

    foods.iter()
        .map(|f| &f.ingredients)
        .flatten()
        .filter(|&i| !bad_ingredients.contains(i))
        .count()
}

fn part_2(input: &str) -> String
{
    let foods = input_to_lines_parsed::<Food>(input);

    // Map allergens to ingredients,
    // sort by allergen,
    // get ingredients in allergen order,
    // join with commas

    map_allergens_to_ingredients(&foods)
        .into_iter()
        .collect::<BTreeMap<String, String>>()
        .into_iter()
        .map(|(_a, i)| i)
        .join(",")
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(21)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 5, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 2072, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: "mxmxvkd,sqjhc,fvjkl", })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: "fdsfpg,jmvxx,lkv,cbzcgvc,kfgln,pqqks,pqrvc,lclnj", })
}
