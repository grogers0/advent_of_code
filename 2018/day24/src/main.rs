use std::collections::BTreeSet;
use std::fmt::Write;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

const IMMUNE_SYSTEM: &str = "Immune System";
const INFECTION: &str = "Infection";

#[derive(Clone, Debug)]
struct Group {
    army: String,
    units: usize,
    hp: usize,
    weaknesses: BTreeSet<String>,
    immunities: BTreeSet<String>,
    damage: usize,
    attack_type: String,
    initiative: usize
}

impl Group {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }
}

// Returns (weaknesses, immunities)
fn parse_modifiers(modifiers_input: &str) -> (BTreeSet<String>, BTreeSet<String>) {
    lazy_static!{
        static ref MODIFIER_RE: Regex = Regex::new("^(weak|immune) to ([a-z, ]+)$").unwrap();
    }
    let mut weaknesses = BTreeSet::new();
    let mut immunities = BTreeSet::new();
    for modifier_input in modifiers_input.split("; ") {
        let cap = MODIFIER_RE.captures(modifier_input).unwrap();
        let modifier = match &cap[1] {
            "weak" => &mut weaknesses,
            "immune" => &mut immunities,
            _ => unreachable!()
        };
        for damage_type in cap[2].split(", ") {
            modifier.insert(damage_type.to_string());
        }
    }
    (weaknesses, immunities)
}

fn parse(input: &str) -> Vec<Group> {
    lazy_static!{
        static ref GROUP_RE: Regex = Regex::new("^(\\d+) units each with (\\d+) hit points (?:\\((.*)\\) )?with an attack that does (\\d+) ([a-z]+) damage at initiative (\\d+)$").unwrap();
    }
    let mut groups = Vec::new();
    let mut army = IMMUNE_SYSTEM;
    for line in input.lines() {
        if line.starts_with(IMMUNE_SYSTEM) {
            army = IMMUNE_SYSTEM;
            continue
        } else if line.starts_with(INFECTION) {
            army = INFECTION;
        } else if line == "" {
            continue
        } else if let Some(cap) = GROUP_RE.captures(line) {
            let (weaknesses, immunities) = if let Some(modifiers_cap) = cap.get(3) {
                parse_modifiers(modifiers_cap.as_str())
            } else {
                (BTreeSet::new(), BTreeSet::new())
            };
            let group = Group {
                army: army.to_string(),
                units: cap[1].parse().unwrap(),
                hp: cap[2].parse().unwrap(),
                weaknesses: weaknesses,
                immunities: immunities,
                damage: cap[4].parse().unwrap(),
                attack_type: cap[5].to_string(),
                initiative: cap[6].parse().unwrap()
            };
            groups.push(group);
        } else {
            unreachable!()
        }
    }
    groups
}

fn damage_dealt(attacker: &Group, defender: &Group) -> usize {
    if defender.immunities.contains(&attacker.attack_type) {
        0
    } else if defender.weaknesses.contains(&attacker.attack_type) {
        attacker.effective_power() * 2
    } else {
        attacker.effective_power()
    }
}

fn decide_targets(groups: &Vec<Group>) -> Vec<Option<usize>> {
    let mut chosen = BTreeSet::new();
    let mut targets = vec![None; groups.len()];
    let mut selection_order: Vec<_> = (0..groups.len()).collect();
    selection_order.sort_by_key(|idx| {
        let group = &groups[*idx];
        (std::usize::MAX - group.effective_power(),
        std::usize::MAX - group.initiative)
    });
    for attacker_idx in selection_order {
        let attacker = &groups[attacker_idx];
        let mut enemies: Vec<_> = (0..groups.len())
            .filter(|idx| attacker.army != groups[*idx].army)
            .filter(|idx| !chosen.contains(idx))
            .filter(|idx| damage_dealt(attacker, &groups[*idx]) != 0)
            .collect();
        enemies.sort_by_key(|idx| {
            (std::usize::MAX - damage_dealt(attacker, &groups[*idx]),
            std::usize::MAX - groups[*idx].effective_power(),
            std::usize::MAX - groups[*idx].initiative)
        });
        let maybe_target = enemies.into_iter().next();
        maybe_target.map(|target| chosen.insert(target));
        targets[attacker_idx] = maybe_target;
    }
    targets
}

fn resolve_attacks(groups: &mut Vec<Group>, targets: Vec<Option<usize>>) {
    let mut initiative_order: Vec<_> = (0..groups.len()).collect();
    initiative_order.sort_by_key(|idx| std::usize::MAX - groups[*idx].initiative);
    for attacker_idx in initiative_order {
        if let Some(defender_idx) = targets[attacker_idx] {
            let dmg = damage_dealt(&groups[attacker_idx], &groups[defender_idx]);
            let defender = &mut groups[defender_idx];
            if dmg / defender.hp >= defender.units {
                defender.units = 0;
            } else {
                defender.units -= dmg / defender.hp;
            }
        }
    }

    groups.retain(|group| group.units > 0);
}

fn total_units(groups: &Vec<Group>) -> usize {
    groups.iter().map(|group| group.units).sum()
}

fn resolve_combat(groups: &mut Vec<Group>) {
    // We can get stuck where neither side does enough damage to kill any units in an iteration of
    // combat, so treat that as finishing the combat
    let mut last_units = 0;
    while armies_remaining(groups).len() > 1 && last_units != total_units(groups) {
        last_units = total_units(groups);
        let targets = decide_targets(groups);
        resolve_attacks(groups, targets);
    }
}

fn armies_remaining(groups: &Vec<Group>) -> BTreeSet<&str> {
    let mut armies = BTreeSet::new();
    for group in groups.iter() {
        armies.insert(group.army.as_str());
    }
    armies
}

#[allow(dead_code)]
fn army_summary(groups: &Vec<Group>, army: &str) -> String {
    let mut out = String::new();
    writeln!(out, "{}", army).unwrap();
    for group in groups.iter().filter(|group| group.army == army) {
        writeln!(out, "Group contains {} units", group.units).unwrap();
    }
    out
}

fn part1(input: &str) -> usize {
    let mut groups = parse(input);
    resolve_combat(&mut groups);
    total_units(&groups)
}

fn boost(groups: &mut Vec<Group>, amount: usize) {
    for group in groups.iter_mut() {
        if group.army == IMMUNE_SYSTEM {
            group.damage += amount;
        }
    }
}

fn immune_system_wins(groups: &Vec<Group>, boost_amount: usize) -> bool {
    let mut groups = groups.clone();
    boost(&mut groups, boost_amount);
    resolve_combat(&mut groups);
    let remaining = armies_remaining(&groups);
    remaining.contains(IMMUNE_SYSTEM) && remaining.len() == 1
}

fn binary_search_boost_amount(groups: &Vec<Group>, mut min_boost: usize, mut max_boost: usize) -> usize {
    while min_boost + 1 != max_boost {
        let mid_boost = min_boost + (max_boost - min_boost)/2;
        if immune_system_wins(groups, mid_boost) {
            max_boost = mid_boost;
        } else {
            min_boost = mid_boost;
        }
    }
    max_boost
}

fn part2(input: &str) -> usize {
    let mut groups = parse(input);

    let mut boost_amount = 1;
    while !immune_system_wins(&groups, boost_amount) {
        boost_amount *= 2;
    }
    let boost_amount = binary_search_boost_amount(&groups, boost_amount/2, boost_amount);

    boost(&mut groups, boost_amount);
    resolve_combat(&mut groups);
    total_units(&groups)
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
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EX), 5216);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EX), 51);
    }
}
