use std::io::{self, Read};

use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Item {
    cost: u16,
    damage: u16,
    armor: u16
}

impl Item {
    fn new(cost: u16, damage: u16, armor: u16) -> Item {
        Item { cost, damage, armor }
    }
}

#[derive(Copy, Clone, Debug)]
struct Player {
    hp: u16,
    damage: u16,
    armor: u16
}

fn parse_boss(input: &str) -> Player {
    let hp_re = Regex::new("^Hit Points: (\\d+)$").unwrap();
    let damage_re = Regex::new("^Damage: (\\d+)$").unwrap();
    let armor_re = Regex::new("^Armor: (\\d+)$").unwrap();
    let mut lines = input.lines();
    let hp = hp_re.captures(lines.next().unwrap()).unwrap()[1].parse().unwrap();
    let damage = damage_re.captures(lines.next().unwrap()).unwrap()[1].parse().unwrap();
    let armor = armor_re.captures(lines.next().unwrap()).unwrap()[1].parse().unwrap();
    Player { hp: hp, damage: damage, armor: armor }
}

struct Store {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
}

fn store() -> Store {
    let weapons = vec![
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0)
    ];
    let armor = vec![
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5)
    ];
    let rings = vec![
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3)
    ];
    Store { weapons, armor, rings }
}

fn damage_dealt(damage: u16, armor: u16) -> u16 {
    if damage > armor {
        damage - armor
    } else {
        1
    }
}

fn wins_combat(mut player: Player, mut boss: Player) -> bool {
    loop {
        let boss_taken = damage_dealt(player.damage, boss.armor);
        if boss.hp <= boss_taken { return true }
        boss.hp -= boss_taken;

        let player_taken = damage_dealt(boss.damage, player.armor);
        if player.hp <= player_taken { return false }
        player.hp -= player_taken;
    }
}

fn item_combinations(store: &Store) -> Vec<Vec<Item>> {
    fn select_weapon(store: &Store, items_list: Vec<Vec<Item>>) -> Vec<Vec<Item>> {
        let mut ret = Vec::new();
        for mut items in items_list {
            for weapon in store.weapons.iter() {
                items.push(*weapon);
                ret.push(items.clone()); // 1 weapon
                items.pop();
            }
        }
        ret
    }
    fn select_armor(store: &Store, items_list: Vec<Vec<Item>>) -> Vec<Vec<Item>> {
        let mut ret = Vec::new();
        for mut items in items_list {
            ret.push(items.clone()); // 0 armor
            for armor in store.armor.iter() {
                items.push(*armor);
                ret.push(items.clone()); // 1 armor
                items.pop();
            }
        }
        ret
    }
    fn select_rings(store: &Store) -> Vec<Vec<Item>> {
        let mut ret = Vec::new();
        let mut items = Vec::new();
        ret.push(items.clone()); // 0 rings
        for (i, ring1) in store.rings.iter().enumerate() {
            items.push(*ring1);
            ret.push(items.clone()); // 1 ring
            for ring2 in store.rings[i+1..].iter() { // Can only buy one of the same item
                items.push(*ring2);
                ret.push(items.clone()); // 2 rings
                items.pop();
            }
            items.pop();
        }
        ret
    }
    select_weapon(store, select_armor(store, select_rings(store)))
}

fn equip(player: Player, items: &Vec<Item>) -> Player {
    Player {
        hp: player.hp,
        damage: player.damage + items.iter().map(|item| item.damage).sum::<u16>(),
        armor: player.armor + items.iter().map(|item| item.armor).sum::<u16>()
    }
}

fn part1(input: &str) -> u16 {
    let boss = parse_boss(input);
    let player = Player { hp: 100, damage: 0, armor: 0 };
    item_combinations(&store()).iter()
        .filter(|items| wins_combat(equip(player, items), boss))
        .map(|items| items.iter().map(|item| item.cost).sum::<u16>())
        .min().unwrap()
}

fn part2(input: &str) -> u16 {
    let boss = parse_boss(input);
    let player = Player { hp: 100, damage: 0, armor: 0 };
    item_combinations(&store()).iter()
        .filter(|items| !wins_combat(equip(player, items), boss))
        .map(|items| items.iter().map(|item| item.cost).sum::<u16>())
        .max().unwrap()
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
        assert!(wins_combat(Player { hp: 8, damage: 5, armor: 5 },
                            Player { hp: 12, damage: 7, armor: 2 }));
    }
}
