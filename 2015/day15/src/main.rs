use std::io::{self, Read};

use regex::Regex;

struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    let re = Regex::new("^[A-Za-z]+: capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (-?\\d+)$").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let capacity = cap[1].parse().unwrap();
            let durability = cap[2].parse().unwrap();
            let flavor = cap[3].parse().unwrap();
            let texture = cap[4].parse().unwrap();
            let calories = cap[5].parse().unwrap();
            Ingredient {
                capacity: capacity,
                durability: durability,
                flavor: flavor,
                texture: texture,
                calories: calories
            }
        })
        .collect()
}

// TODO - there's probably a better way to use a combinations library to iterate through all these possibilities
fn iterate_all_possible_amounts<F>(ingredients: &[Ingredient], amounts: &mut [u64], depth: usize, amount_left: u64, f: &mut F)
    where F: FnMut(&[u64])
{
    if depth == ingredients.len() - 1 {
        amounts[depth] = amount_left;
        f(amounts);
    } else {
        for amount in 0..=amount_left {
            amounts[depth] = amount;
            iterate_all_possible_amounts(ingredients, amounts, depth + 1, amount_left - amount, f);
        }
    }
}

fn score_recipe(ingredients: &[Ingredient], amounts: &[u64]) -> u64 {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    for (ingredient, amount) in ingredients.iter().zip(amounts.iter()) {
        capacity += ingredient.capacity * *amount as i64;
        durability += ingredient.durability * *amount as i64;
        flavor += ingredient.flavor * *amount as i64;
        texture += ingredient.texture * *amount as i64;
    }
    if capacity > 0 && durability > 0 && flavor > 0 && texture > 0 {
        (capacity * durability * flavor * texture) as u64
    } else {
        0
    }
}

fn count_calories(ingredients: &[Ingredient], amounts: &[u64]) -> i64 {
    let mut calories = 0;
    for (ingredient, amount) in ingredients.iter().zip(amounts.iter()) {
        calories += ingredient.calories * *amount as i64;
    }
    calories
}

fn part1(input: &str) -> u64 {
    let ingredients = parse_ingredients(input);
    let mut amounts = vec![0; ingredients.len()];
    let mut best_score = 0;
    iterate_all_possible_amounts(&ingredients, &mut amounts, 0, 100, &mut |amounts| {
        let score = score_recipe(&ingredients, amounts);
        if score > best_score { best_score = score; }
    });
    best_score
}

fn part2(input: &str) -> u64 {
    let ingredients = parse_ingredients(input);
    let mut amounts = vec![0; ingredients.len()];
    let mut best_score = 0;
    iterate_all_possible_amounts(&ingredients, &mut amounts, 0, 100, &mut |amounts| {
        if count_calories(&ingredients, amounts) == 500 {
            let score = score_recipe(&ingredients, amounts);
            if score > best_score { best_score = score; }
        }
    });
    best_score
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 62842880);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 57600000);
    }
}
