use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug)]
struct Reaction {
    inputs: HashMap<String, u64>,
    output_amount: u64
}

fn parse(puzzle_input: &str) -> HashMap<String, Reaction> {
    puzzle_input.trim().lines().map(|line| {
        let mut sp = line.split("=>");
        let inputs = sp.next().unwrap();
        let output = sp.next().unwrap();
        let mut sp = output.trim().split(" ");
        let output_amount = sp.next().unwrap().parse().unwrap();
        let output_chemical = sp.next().unwrap().to_string();
        let inputs = inputs.trim().split(",").map(|input| {
            let mut sp = input.trim().split(" ");
            let input_amount = sp.next().unwrap().parse().unwrap();
            let input_chemical = sp.next().unwrap().to_string();
            (input_chemical, input_amount)
        }).collect();
        (output_chemical, Reaction { inputs: inputs, output_amount: output_amount })
    }).collect()
}

fn resolve_one_reaction(reaction: &Reaction,
        needed_chemicals: &mut HashMap<String, i64>,
        chemical: &str,
        amount: u64) {
    let times = ((amount + reaction.output_amount - 1) / reaction.output_amount) as i64;
    debug_assert!(times > 0);
    for (input_chemical, input_amount) in reaction.inputs.iter() {
        *needed_chemicals.entry(input_chemical.to_string()).or_insert(0) += times * *input_amount as i64;
    }
    *needed_chemicals.get_mut(chemical).unwrap() -= times * reaction.output_amount as i64;
}

fn resolve_next_reaction(reactions: &HashMap<String, Reaction>, needed_chemicals: &mut HashMap<String, i64>) -> bool {
    for (needed_chemical, needed_amount) in needed_chemicals.clone() {
        if needed_chemical != "ORE" && needed_amount > 0 {
            resolve_one_reaction(&reactions[&needed_chemical], needed_chemicals, &needed_chemical, needed_amount as u64);
            return true;
        }
    }
    false
}

fn resolve_reactions(reactions: &HashMap<String, Reaction>, needed_chemicals: &mut HashMap<String, i64>) {
    while resolve_next_reaction(reactions, needed_chemicals) { }
}

fn ore_needed_for_fuel(reactions: &HashMap<String, Reaction>, fuel: u64) -> u64 {
    let mut needed_chemicals = HashMap::new();
    needed_chemicals.insert("FUEL".to_string(), fuel as i64);
    resolve_reactions(reactions, &mut needed_chemicals);
    needed_chemicals["ORE"] as u64
}

fn fuel_for_available_ore_upper_bound(reactions: &HashMap<String, Reaction>, ore: u64) -> u64 {
    for fuel_exp in 1.. {
        let fuel = 1 << fuel_exp;
        if ore_needed_for_fuel(reactions, fuel) > ore {
            return fuel
        }
    }
    panic!()
}

fn fuel_for_available_ore(reactions: &HashMap<String, Reaction>, ore: u64) -> u64 {
    let mut max = fuel_for_available_ore_upper_bound(reactions, ore);
    let mut min = max / 2;
    while max - min > 1 {
        let mid = min + (max - min) / 2;
        match ore_needed_for_fuel(reactions, mid).cmp(&ore) {
            Ordering::Equal => return mid,
            Ordering::Less => min = mid,
            Ordering::Greater => max = mid
        }
    }
    min
}

fn part1(puzzle_input: &str) -> u64 {
    let reactions = parse(puzzle_input);
    ore_needed_for_fuel(&reactions, 1)
}

fn part2(puzzle_input: &str) -> u64 {
    let reactions = parse(puzzle_input);
    fuel_for_available_ore(&reactions, 1_000_000_000_000)
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

    const EX1: &str = "
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const EX2: &str = "
9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const EX3: &str = "
157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const EX4: &str = "
2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const EX5: &str = "
171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX1), 31);
        assert_eq!(part1(EX2), 165);
        assert_eq!(part1(EX3), 13312);
        assert_eq!(part1(EX4), 180697);
        assert_eq!(part1(EX5), 2210736);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX3), 82892753);
        assert_eq!(part2(EX4), 5586022);
        assert_eq!(part2(EX5), 460664);
    }
}
