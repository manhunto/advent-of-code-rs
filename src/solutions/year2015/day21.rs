use crate::solutions::Solution;
use itertools::{iproduct, Itertools};
use std::str::FromStr;

const SHOP_DATA: &str = r#"Weapons:    Cost  Damage  Armor
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
Damage +1     25     1       0
Damage +2     50     2       0
Damage +3    100     3       0
Defense +1    20     0       1
Defense +2    40     0       2
Defense +3    80     0       3"#;

const MIN_DAMAGE: u64 = 1;
const DEFAULT_PLAYER_HP: u64 = 100;

pub struct Day21;

impl Solution for Day21 {
    fn part_one(&self, input: &str) -> String {
        let boss: Mob = input.parse().unwrap();

        self.all_player_loadouts()
            .filter(|set| {
                let player: Mob = set.into();

                player.wins(&boss)
            })
            .map(|set| set.cost())
            .min()
            .unwrap_or(0)
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let boss: Mob = input.parse().unwrap();

        self.all_player_loadouts()
            .filter(|set| {
                let player: Mob = set.into();

                !player.wins(&boss)
            })
            .map(|set| set.cost())
            .max()
            .unwrap_or(0)
            .to_string()
    }
}

impl Day21 {
    fn all_player_loadouts(&self) -> impl Iterator<Item = Set> {
        let (weapons, armors, rings) = self.parse_shop();

        iproduct!(weapons, armors, rings.into_iter().tuple_combinations())
            .map(|(w, a, (r1, r2))| Set::new(w, a, vec![r1, r2]))
    }
    fn parse_shop(&self) -> (Vec<Item>, Vec<Item>, Vec<Item>) {
        let (weapons_str, armor_str, rings_str) = SHOP_DATA.split("\n\n").collect_tuple().unwrap();

        let weapons = self.parse_items(weapons_str);

        let mut armors = self.parse_items(armor_str);
        armors.push(Item::default());

        let mut rings = self.parse_items(rings_str);
        rings.push(Item::default());
        rings.push(Item::default());

        (weapons, armors, rings)
    }

    fn parse_items(&self, items: &str) -> Vec<Item> {
        items
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()
    }
}

struct Mob {
    hit_points: u64,
    damage: u64,
    armor: u64,
}

impl Mob {
    /// It indicates that self has first turn
    fn wins(&self, other: &Mob) -> bool {
        let my_damage = self.damage.saturating_sub(other.armor).max(MIN_DAMAGE);
        let other_damage = other.damage.saturating_sub(self.armor).max(MIN_DAMAGE);

        let hits_to_kill_other = other.hit_points.div_ceil(my_damage);
        let hits_to_kill_self = self.hit_points.div_ceil(other_damage);

        hits_to_kill_other <= hits_to_kill_self
    }
}

impl From<&Set> for Mob {
    fn from(value: &Set) -> Self {
        Self {
            hit_points: DEFAULT_PLAYER_HP,
            damage: value.damage(),
            armor: value.armor(),
        }
    }
}

impl FromStr for Mob {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u64> = s
            .lines()
            .map(|l| l.split(": ").last().unwrap().parse().unwrap())
            .collect();

        Ok(Self {
            hit_points: nums[0],
            damage: nums[1],
            armor: nums[2],
        })
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct Item {
    cost: u64,
    damage: u64,
    armor: u64,
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        let len = parts.len();
        if len < 3 {
            return Err(());
        }

        Ok(Item {
            cost: parts[len - 3].parse().unwrap(),
            damage: parts[len - 2].parse().unwrap(),
            armor: parts[len - 1].parse().unwrap(),
        })
    }
}

#[derive(Clone)]
struct Set {
    weapon: Item,
    armor: Item,
    rings: Vec<Item>,
}

impl Set {
    fn new(weapon: Item, armor: Item, rings: Vec<Item>) -> Self {
        assert!(rings.len() <= 2);

        Self {
            weapon,
            armor,
            rings,
        }
    }

    fn cost(&self) -> u64 {
        self.weapon.cost + self.armor.cost + self.sum_rings(|item| item.cost)
    }

    fn damage(&self) -> u64 {
        self.weapon.damage + self.sum_rings(|item| item.damage)
    }

    fn armor(&self) -> u64 {
        self.armor.armor + self.sum_rings(|item| item.armor)
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

    impl Mob {
        fn new(hit_points: u64, damage: u64, armor: u64) -> Self {
            Self {
                hit_points,
                damage,
                armor,
            }
        }
    }

    #[test]
    fn wins_equal_rounds() {
        let player = Mob::new(8, 5, 5);
        let boss = Mob::new(12, 7, 2);

        assert!(player.wins(&boss));
    }

    #[test]
    fn wins_boss_more_hp() {
        let player = Mob::new(8, 5, 5);
        let boss = Mob::new(13, 7, 2);

        assert!(!player.wins(&boss));
    }

    #[test]
    fn wins_boss_more_armor_than_damage_decreases_1hp() {
        let player = Mob::new(100, 20, 2);
        let boss = Mob::new(2, 4, 100);

        assert!(player.wins(&boss));
    }

    #[test]
    fn wins_when_fractional_rounds_disfavor_player_but_integer_rounds_are_equal() {
        // Player deals 5 dmg to 14 HP -> 14/5 = 2.8 "rounds" (needs 3 hits)
        // Boss deals 5 dmg to 12 HP   -> 12/5 = 2.4 "rounds" (needs 3 hits)

        // Since both need 3 hits and Player goes first:
        // Turn 1: Player hits (Boss 9), Boss hits (Player 7)
        // Turn 2: Player hits (Boss 4), Boss hits (Player 2)
        // Turn 3: Player hits (Boss -1/Dead). Player wins.

        // BUG: Current logic compares 2.8 <= 2.4 (False), so it thinks Player loses.
        let player = Mob::new(12, 5, 0);
        let boss = Mob::new(14, 5, 0);

        assert!(
            player.wins(&boss),
            "Player should win because they strike first and need the same number of turns"
        );
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

        let set = Set::new(weapon, armor, vec![ring1, ring2]);

        assert_eq!(128, set.cost());
        assert_eq!(6, set.damage());
        assert_eq!(5, set.armor());

        let player: Mob = (&set).into();

        assert_eq!(100, player.hit_points);
        assert_eq!(6, player.damage);
        assert_eq!(5, player.armor);
    }
}
