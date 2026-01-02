use crate::solutions::Solution;
use itertools::Itertools;
use std::num::ParseIntError;
use std::str::FromStr;

const ITEMS_LIST: &str = r#"Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage+1     25     1       0
Damage+2     50     2       0
Damage+3    100     3       0
Defense+1    20     0       1
Defense+2    40     0       2
Defense+3    80     0       3"#;

pub struct Day21;

impl Solution for Day21 {
    fn part_one(&self, input: &str) -> String {
        let (weapons, armors, rings) = self.parse_item_list();
        let ring_combinations = self.rings_combinations(rings);

        let boss = self.parse_input(input);
        let mut winning_costs: Vec<u64> = Vec::new();

        for weapon in weapons {
            for armor in armors.iter() {
                for rings_set in ring_combinations.iter() {
                    let set = Set::new(weapon, *armor, rings_set.clone());
                    let set_cost = set.cost();
                    let player: Character = set.into();

                    if player.wins(&boss) {
                        winning_costs.push(set_cost);
                    }
                }
            }
        }

        winning_costs.iter().min().unwrap_or(&0).to_string()
    }

    fn part_two(&self, _input: &str) -> String {
        String::from("0")
    }
}

impl Day21 {
    fn parse_item_list(&self) -> (Vec<Item>, Vec<Option<Item>>, Vec<Item>) {
        let (weapons_str, armor_str, rings_str) =
            ITEMS_LIST.split_terminator("\n\n").collect_tuple().unwrap();

        let mut armors: Vec<Option<Item>> = self.parse_items(armor_str).map(Some).collect();
        armors.push(None);

        (
            self.parse_items(weapons_str).collect(),
            armors,
            self.parse_items(rings_str).collect(),
        )
    }

    fn parse_items<'a>(&self, items: &'a str) -> impl Iterator<Item = Item> + 'a {
        items.lines().skip(1).map(|line| line.parse().unwrap())
    }

    fn parse_input(&self, input: &str) -> Character {
        let lines = input.lines().collect_vec();

        let hit_points = self.parse_boss_stat(lines[0]);
        let damage = self.parse_boss_stat(lines[1]);
        let armor = self.parse_boss_stat(lines[2]);

        Character::new(hit_points, damage, armor)
    }

    fn parse_boss_stat(&self, line: &str) -> u64 {
        let (_, n) = line.split_once(": ").unwrap();

        n.parse().unwrap()
    }

    fn rings_combinations(&self, rings: Vec<Item>) -> Vec<Vec<Item>> {
        let mut combinations = Vec::new();
        combinations.push(vec![]);

        for ring_count in 1..=2 {
            combinations.extend(rings.clone().into_iter().combinations(ring_count));
        }

        combinations
    }
}

struct Character {
    hit_points: u64,
    damage: u64,
    armor: u64,
}

impl Character {
    fn new(hit_points: u64, damage: u64, armor: u64) -> Self {
        Self {
            hit_points,
            damage,
            armor,
        }
    }

    /// It indicates that self has first turn
    fn wins(&self, other: &Character) -> bool {
        let my_damage = self.damage.saturating_sub(other.armor).max(1) as f64;
        let my_rounds = other.hit_points as f64 / my_damage;

        let other_damage = other.damage.saturating_sub(self.armor).max(1) as f64;
        let other_rounds = self.hit_points as f64 / other_damage;

        my_rounds <= other_rounds
    }
}

impl From<Set> for Character {
    fn from(value: Set) -> Self {
        Self {
            hit_points: 100,
            damage: value.damage(),
            armor: value.armor(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Item {
    cost: u64,
    damage: u64,
    armor: u64,
}

impl FromStr for Item {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, cost, damage, armor) = s.split_whitespace().collect_tuple().unwrap();

        Ok(Self {
            cost: cost.parse()?,
            damage: damage.parse()?,
            armor: armor.parse()?,
        })
    }
}

#[derive(Clone)]
struct Set {
    weapon: Item,
    armor: Option<Item>,
    rings: Vec<Item>,
}

impl Set {
    fn new(weapon: Item, armor: Option<Item>, rings: Vec<Item>) -> Self {
        assert!(rings.len() <= 2);

        Self {
            weapon,
            armor,
            rings,
        }
    }

    fn cost(&self) -> u64 {
        self.weapon.cost + self.sum_armor(|item| item.cost) + self.sum_rings(|item| item.cost)
    }

    fn damage(&self) -> u64 {
        self.weapon.damage + self.sum_rings(|item| item.damage)
    }

    fn armor(&self) -> u64 {
        self.sum_armor(|item| item.armor) + self.sum_rings(|item| item.armor)
    }

    fn sum_armor<F>(&self, func: F) -> u64
    where
        F: Fn(&Item) -> u64,
    {
        self.armor.as_ref().map_or(0, func)
    }

    fn sum_rings<F>(&self, func: F) -> u64
    where
        F: Fn(&Item) -> u64,
    {
        self.rings.iter().map(func).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wins_equal_rounds() {
        let player = Character::new(8, 5, 5);
        let boss = Character::new(12, 7, 2);

        assert!(player.wins(&boss));
    }

    #[test]
    fn wins_boss_more_hp() {
        let player = Character::new(8, 5, 5);
        let boss = Character::new(13, 7, 2);

        assert!(!player.wins(&boss));
    }

    #[test]
    fn wins_boss_more_armor_than_damage_decreases_1hp() {
        let player = Character::new(100, 20, 2);
        let boss = Character::new(2, 4, 100);

        assert!(player.wins(&boss));
    }

    #[test]
    fn item_parse() {
        let str = "Dagger        8     4       0";

        let item: Item = str.parse().unwrap();

        assert_eq!(item.cost, 8);
        assert_eq!(item.damage, 4);
        assert_eq!(item.armor, 0);
    }

    #[test]
    fn set_construction() {
        let weapon: Item = "Shortsword   10     5       0".parse().unwrap();
        let armor: Item = "Splintmail   53     0       3".parse().unwrap();
        let ring1: Item = "Damage+1    25     1       0".parse().unwrap();
        let ring2: Item = "Defense+2   40     0       2".parse().unwrap();

        let set = Set::new(weapon, Some(armor), vec![ring1, ring2]);

        assert_eq!(128, set.cost());
        assert_eq!(6, set.damage());
        assert_eq!(5, set.armor());

        let player: Character = set.into();

        assert_eq!(100, player.hit_points);
        assert_eq!(6, player.damage);
        assert_eq!(5, player.armor);
    }
}
