use std::collections::{BTreeSet, BinaryHeap};
use std::cmp::min;
use std::io::{self, Read};

use regex::Regex;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Player {
    hp: u16,
    armor: u16,
    mana: u16
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Boss {
    hp: u16,
    damage: u16
}

fn parse_boss(input: &str) -> Boss {
    let hp_re = Regex::new("^Hit Points: (\\d+)$").unwrap();
    let damage_re = Regex::new("^Damage: (\\d+)$").unwrap();
    let mut lines = input.lines();
    let hp = hp_re.captures(lines.next().unwrap()).unwrap()[1].parse().unwrap();
    let damage = damage_re.captures(lines.next().unwrap()).unwrap()[1].parse().unwrap();
    Boss { hp, damage }
}

fn step_effects(effects: &mut [u16; 3], player: &mut Player, boss: &mut Boss) {
    if effects[0] == 1 {
        player.armor -= 7;
    }
    if effects[1] > 0 {
        boss.hp -= min(boss.hp, 3);
    }
    if effects[2] > 0 {
        player.mana += 101;
    }
    for effect in effects.iter_mut() {
        if *effect > 0 { *effect -= 1; }
    }
}

fn take_boss_turn(player: &mut Player, boss: &Boss) {
    let damage_dealt = if player.armor >= boss.damage { 1 } else { boss.damage - player.armor };
    player.hp -= min(player.hp, damage_dealt);
}

// Rust doesn't have a good way to reverse the ordering for a binary heap
fn rev_u16(x: u16) -> u16 {
    u16::max_value() - x
}

fn take_player_turn<F>(player: &Player, boss: &Boss, effects: &[u16; 3], spent_rev: u16, spell_mana: u16,
                       queue: &mut BinaryHeap<(u16, Player, Boss, [u16; 3], bool)>, f: F)
where F: FnOnce(&mut Player, &mut Boss, &mut [u16; 3])
{
    if player.mana >= spell_mana {
        let mut player = player.clone();
        let mut boss = boss.clone();
        let mut effects = effects.clone();
        player.mana -= spell_mana;
        f(&mut player, &mut boss, &mut effects);
        queue.push((spent_rev - spell_mana, player, boss, effects, false));
    }
}

fn least_mana_spent_outcome(player: Player, boss: Boss, hard_mode: bool) -> u16 {
    let mut seen = BTreeSet::new();
    let mut queue = BinaryHeap::new();
    queue.push((rev_u16(0), player, boss, [0,0,0], true));
    while let Some((spent_rev, mut player, mut boss, mut effects, player_turn)) = queue.pop() {
        if !seen.insert((player.clone(), boss.clone(), effects.clone(), player_turn)) { continue }
        if hard_mode && player_turn {
            player.hp -= min(player.hp, 1);
            if player.hp == 0 { continue }
        }
        step_effects(&mut effects, &mut player, &mut boss);
        if boss.hp == 0 { return rev_u16(spent_rev); }
        if player_turn {
            take_player_turn(&player, &boss, &effects, spent_rev, 53, &mut queue, |_, boss, _| {
                boss.hp -= min(boss.hp, 4);
            });
            take_player_turn(&player, &boss, &effects, spent_rev, 73, &mut queue, |player, boss, _| {
                boss.hp -= min(boss.hp, 2);
                player.hp += 2;
            });
            if effects[0] == 0 {
                take_player_turn(&player, &boss, &effects, spent_rev, 113, &mut queue, |player, _, effects| {
                    player.armor += 7;
                    effects[0] = 6;
                });
            }
            if effects[1] == 0 {
                take_player_turn(&player, &boss, &effects, spent_rev, 173, &mut queue, |_, _, effects| {
                    effects[1] = 6;
                });
            }
            if effects[2] == 0 {
                take_player_turn(&player, &boss, &effects, spent_rev, 229, &mut queue, |_, _, effects| {
                    effects[2] = 5;
                });
            }
        } else {
            take_boss_turn(&mut player, &boss);
            if player.hp == 0 { continue }
            queue.push((spent_rev, player, boss, effects, true));
        }
    }
    panic!()
}

fn part1(input: &str) -> u16 {
    let player = Player { hp: 50, armor: 0, mana: 500 };
    let boss = parse_boss(input);
    least_mana_spent_outcome(player, boss, false)
}

fn part2(input: &str) -> u16 {
    let player = Player { hp: 50, armor: 0, mana: 500 };
    let boss = parse_boss(input);
    least_mana_spent_outcome(player, boss, true)
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
        let player = Player { hp: 10, armor: 0, mana: 250 };
        //assert_eq!(least_mana_spent_outcome(player.clone(), Boss { hp: 13, damage: 8 }, false), 226);
        assert_eq!(least_mana_spent_outcome(player.clone(), Boss { hp: 14, damage: 8 }, false), 641);
    }
}
