use std::io::{self, Read};

fn init_recipes() -> Vec<usize> {
    let mut recipes = Vec::new();
    recipes.push(3);
    recipes.push(7);
    recipes
}

fn step(recipes: &mut Vec<usize>, pos: &mut [usize; 2]) {
    let new_recipe = recipes[pos[0]] + recipes[pos[1]];
    if new_recipe >= 10 {
        recipes.push(new_recipe / 10);
    }
    recipes.push(new_recipe % 10);
    for p in pos.iter_mut() {
        *p = (*p + recipes[*p] + 1) % recipes.len();
    }
}

fn part1(input: &str) -> String {
    let start_after = input.trim_end().parse().unwrap();
    let mut recipes = init_recipes();
    let mut pos = [0, 1];
    while recipes.len() < start_after+10 {
        step(&mut recipes, &mut pos);
    }

    let mut ret = String::new();
    for recipe in recipes[start_after..start_after+10].iter() {
        ret.push_str(&format!("{}", recipe));
    }
    ret
}

fn part2(input: &str) -> usize {
    let seq = input.trim_end().chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let mut recipes = init_recipes();
    let mut pos = [0, 1];
    let mut checked_through = 0;
    loop {
        step(&mut recipes, &mut pos);
        if recipes.len() >= seq.len() {
            while checked_through < recipes.len() - seq.len() {
                if seq.as_slice() == &recipes[checked_through .. checked_through+seq.len()] {
                    return checked_through;
                }
                checked_through += 1;
            }
        }
    }
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

    #[test]
    fn test_part1() {
        assert_eq!(part1("9"), "5158916779".to_string());
        assert_eq!(part1("5"), "0124515891".to_string());
        assert_eq!(part1("18"), "9251071085".to_string());
        assert_eq!(part1("2018"), "5941429882".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }

}
