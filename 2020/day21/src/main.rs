use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::io::{self, Read};

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

fn parse(puzzle_input: &str) -> Vec<Food> {
    fn parse_line(line: &str) -> Food {
        let mut parts = line.split(" (contains ");
        let ingredients = parts.next().unwrap()
            .split(" ").map(|x| x.to_string()).collect();
        let allergens = parts.next().unwrap().trim_end_matches(")")
            .split(", ").map(|x| x.to_string()).collect();
        Food { ingredients, allergens }
    }

    puzzle_input.lines().map(|line| parse_line(line)).collect()
}

fn calculate_ingredient_allergens(foods: &[Food]) -> HashMap<String, String> {
    let mut possible_allergen_ingredients = HashMap::new();
    for food in foods {
        for allergen in &food.allergens {
            let mut ingredients = possible_allergen_ingredients.remove(allergen).unwrap_or_else(|| food.ingredients.clone());
            ingredients = &ingredients & &food.ingredients;
            possible_allergen_ingredients.insert(allergen.clone(), ingredients);
        }
    }

    let mut ingredient_allergens = HashMap::new();
    while !possible_allergen_ingredients.is_empty() {
        let allergen = possible_allergen_ingredients.iter()
            .filter(|(_,ingredients)| ingredients.len() == 1)
            .map(|(allergen,_)| allergen.clone())
            .next().unwrap();
        let ingredient = possible_allergen_ingredients.remove(&allergen).unwrap().into_iter().next().unwrap();
        for ingredients in possible_allergen_ingredients.values_mut() {
            ingredients.remove(&ingredient);
        }
        ingredient_allergens.insert(ingredient, allergen);
    }

    ingredient_allergens
}

fn part1(puzzle_input: &str) -> usize {
    let foods = parse(puzzle_input);
    let ingredient_allergens = calculate_ingredient_allergens(&foods);

    foods.iter().flat_map(|food| food.ingredients.iter())
        .filter(|ingredient| !ingredient_allergens.contains_key(ingredient.as_str()))
        .count()
}

fn part2(puzzle_input: &str) -> String {
    let foods = parse(puzzle_input);
    let mut ingredient_allergens: Vec<_> = calculate_ingredient_allergens(&foods).into_iter().collect();
    ingredient_allergens.sort_by_key(|(_,allergen)| allergen.clone());

    let mut ret = String::new();
    for (i, ingredient) in ingredient_allergens.into_iter().map(|(ingredient,_)| ingredient).enumerate() {
        if i != 0 { ret.push(',') }
        write!(ret, "{}", ingredient).unwrap();
    }
    ret
}

fn main() {
    let mut puzzle_input = String::new();
    io::stdin().read_to_string(&mut puzzle_input).unwrap();

    println!("{}", part1(&puzzle_input));
    println!("{}", part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!("mxmxvkd,sqjhc,fvjkl".to_string(), part2(EX));
    }
}
