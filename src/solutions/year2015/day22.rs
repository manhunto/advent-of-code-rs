use crate::solutions::Solution;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day22;

const INIT_PLAYER_HP: u32 = 50;
const INIT_PLAYER_MANA: u32 = 500;

impl Solution for Day22 {
    fn part_one(&self, input: &str) -> String {
        let player = Player::new(INIT_PLAYER_HP, INIT_PLAYER_MANA);
        let boss: Boss = input.parse().unwrap();

        self.solve(player, boss)
    }

    fn part_two(&self, input: &str) -> String {
        let mut player = Player::new(INIT_PLAYER_HP, INIT_PLAYER_MANA);
        player.enable_hard_mode();
        let boss: Boss = input.parse().unwrap();

        self.solve(player, boss)
    }
}

impl Day22 {
    fn solve(&self, player: Player, boss: Boss) -> String {
        const SPELLS: [Spell; 5] = [
            Spell::MagicMissile,
            Spell::Drain,
            Spell::Poison,
            Spell::Shield,
            Spell::Recharge,
        ];

        let mut queue = VecDeque::new();
        queue.push_back((player, boss));

        let mut best = u32::MAX;

        while let Some((player, boss)) = queue.pop_front() {
            for spell in SPELLS.iter() {
                let mut current_player = player.clone();
                let mut current_boss = boss.clone();

                let result = self.make_player_turn(&mut current_player, &mut current_boss, spell);

                if current_player.mana_spent > best {
                    continue;
                }

                match result {
                    Ok(status) => match status {
                        FightStatus::PlayerWin => {
                            best = best.min(current_player.mana_spent);

                            continue;
                        }
                        FightStatus::BossWin => continue,
                        _ => {}
                    },
                    Err(_) => continue,
                }

                let result = self.make_boss_turn(&mut current_player, &mut current_boss);

                match result {
                    Ok(status) => match status {
                        FightStatus::PlayerWin => {
                            best = best.min(current_player.mana_spent);

                            continue;
                        }
                        FightStatus::BossWin => continue,
                        FightStatus::Pending => {
                            queue.push_back((current_player.clone(), current_boss.clone()))
                        }
                    },
                    Err(_) => continue,
                }
            }
        }

        best.to_string()
    }

    fn make_player_turn(
        &self,
        player: &mut Player,
        boss: &mut Boss,
        spell: &Spell,
    ) -> Result<FightStatus, AttackFail> {
        player.apply_before_player_turn();

        self.make_turn(player, boss, |player, boss| player.cast_spell(boss, spell))
    }

    fn make_boss_turn(
        &self,
        player: &mut Player,
        boss: &mut Boss,
    ) -> Result<FightStatus, AttackFail> {
        self.make_turn(player, boss, |player, boss| boss.attack(player))
    }

    fn make_turn<F>(
        &self,
        player: &mut Player,
        boss: &mut Boss,
        func: F,
    ) -> Result<FightStatus, AttackFail>
    where
        F: Fn(&mut Player, &mut Boss) -> Result<(), AttackFail>,
    {
        if let Some(status) = self.check_result(player, boss) {
            return Ok(status);
        }

        player.apply_recharge_effect();
        boss.apply_poison_effect();
        player.apply_shield_effect();

        if let Some(status) = self.check_result(player, boss) {
            return Ok(status);
        }

        func(player, boss)?;

        Ok(self
            .check_result(player, boss)
            .unwrap_or(FightStatus::Pending))
    }

    fn check_result(&self, player: &Player, boss: &Boss) -> Option<FightStatus> {
        if boss.hit_points == 0 {
            return Some(FightStatus::PlayerWin);
        }

        if player.hit_points == 0 {
            return Some(FightStatus::BossWin);
        }

        None
    }
}

#[derive(Debug, PartialEq)]
enum FightStatus {
    Pending,
    PlayerWin,
    BossWin,
}

#[derive(Debug)]
enum AttackFail {
    #[allow(dead_code)]
    PlayerAttackFail(PlayerAttackFail),
}

#[derive(Debug)]
enum PlayerAttackFail {
    NotEnoughMana,
    EffectCurrentlyApplied,
}

#[derive(Clone)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone)]
struct EffectDuration {
    left: u8,
}

impl EffectDuration {
    fn new(left: u8) -> Option<Self> {
        Some(Self { left })
    }

    fn drain(&self) -> Option<EffectDuration> {
        let left = self.left.saturating_sub(1);

        if left == 0 {
            None
        } else {
            Some(Self { left })
        }
    }
}

#[derive(Debug, Clone)]
struct Player {
    hit_points: u32,
    current_mana: u32,
    armor: u32,
    mana_spent: u32,
    shield_effect: Option<EffectDuration>,
    recharge_effect: Option<EffectDuration>,
    hard_mode: bool,
}

impl Player {
    fn new(hit_points: u32, mana: u32) -> Self {
        Self {
            hit_points,
            current_mana: mana,
            armor: 0,
            mana_spent: 0,
            shield_effect: None,
            recharge_effect: None,
            hard_mode: false,
        }
    }

    fn cast_spell(&mut self, boss: &mut Boss, spell: &Spell) -> Result<(), AttackFail> {
        self.check_can_cast(boss, spell)?;

        self.apply_spell(boss, spell);
        self.reduce_mana(spell);

        Ok(())
    }

    fn reduce_mana(&mut self, spell: &Spell) {
        let mana_cost = Self::mana_cost(spell);

        self.current_mana -= mana_cost;
        self.mana_spent += mana_cost;
    }

    fn apply_spell(&mut self, boss: &mut Boss, spell: &Spell) {
        match spell {
            Spell::MagicMissile => boss.damage(4),
            Spell::Drain => {
                boss.damage(2);
                self.hit_points += 2;
            }
            Spell::Shield => {
                self.shield_effect = EffectDuration::new(7);
                self.apply_shield_effect()
            }
            Spell::Poison => boss.poison_effect = EffectDuration::new(6),
            Spell::Recharge => self.recharge_effect = EffectDuration::new(5),
        }
    }

    fn mana_cost(spell: &Spell) -> u32 {
        match spell {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }

    fn check_can_cast(&self, boss: &Boss, spell: &Spell) -> Result<(), AttackFail> {
        let mana_cost = Self::mana_cost(spell);

        if self.current_mana < mana_cost {
            return Err(AttackFail::PlayerAttackFail(
                PlayerAttackFail::NotEnoughMana,
            ));
        }

        match spell {
            Spell::MagicMissile => Ok(()),
            Spell::Drain => Ok(()),
            Spell::Shield => self.shield_effect.as_ref().map_or(Ok(()), |_| {
                Err(AttackFail::PlayerAttackFail(
                    PlayerAttackFail::EffectCurrentlyApplied,
                ))
            }),
            Spell::Poison => boss.poison_effect.as_ref().map_or(Ok(()), |_| {
                Err(AttackFail::PlayerAttackFail(
                    PlayerAttackFail::EffectCurrentlyApplied,
                ))
            }),
            Spell::Recharge => self.recharge_effect.as_ref().map_or(Ok(()), |_| {
                Err(AttackFail::PlayerAttackFail(
                    PlayerAttackFail::EffectCurrentlyApplied,
                ))
            }),
        }
    }

    fn apply_recharge_effect(&mut self) {
        if let Some(effect) = &self.recharge_effect {
            self.current_mana += 101;
            self.recharge_effect = effect.drain();
        }
    }

    fn apply_shield_effect(&mut self) {
        if let Some(effect) = &self.shield_effect {
            self.armor = 7;
            self.shield_effect = effect.drain();

            if self.shield_effect.is_none() {
                self.armor = 0;
            }
        }
    }

    fn enable_hard_mode(&mut self) {
        self.hard_mode = true;
    }

    fn apply_before_player_turn(&mut self) {
        if self.hard_mode {
            self.hit_points = self.hit_points.saturating_sub(1);
        }
    }

    fn damage(&mut self, amount: u32) {
        let real_damage = amount.saturating_sub(self.armor).max(1);

        self.hit_points = self.hit_points.saturating_sub(real_damage);
    }
}

#[derive(Debug, Clone)]
struct Boss {
    hit_points: u32,
    damage: u32,
    poison_effect: Option<EffectDuration>,
}

impl Boss {
    fn new(hit_points: u32, damage: u32) -> Self {
        Self {
            hit_points,
            damage,
            poison_effect: None,
        }
    }

    fn damage(&mut self, amount: u32) {
        self.hit_points = self.hit_points.saturating_sub(amount);
    }

    fn attack(&self, player: &mut Player) -> Result<(), AttackFail> {
        player.damage(self.damage);

        Ok(())
    }

    fn apply_poison_effect(&mut self) {
        if let Some(effect) = &self.poison_effect {
            self.hit_points = self.hit_points.saturating_sub(3);
            self.poison_effect = effect.drain();
        }
    }
}

impl FromStr for Boss {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u32> = s
            .lines()
            .map(|l| l.split(": ").last().unwrap().parse().unwrap())
            .collect();

        Ok(Self::new(nums[0], nums[1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solutions::year2015::day22::FightStatus::{Pending, PlayerWin};

    #[test]
    fn part_one_first_fight() {
        let mut player = Player::new(10, 250);
        let mut boss = Boss::new(13, 8);

        // Turn 1
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 250);
        assert_eq!(boss.hit_points, 13);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::Poison);
        assert!(result.is_ok_and(|status| status == Pending));

        // Turn 2
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 77);
        assert_eq!(boss.hit_points, 13);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(boss.poison_effect.clone().is_some_and(|d| d.left == 5));

        // Turn 3
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 77);
        assert_eq!(boss.hit_points, 10);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::MagicMissile);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(boss.poison_effect.clone().is_some_and(|d| d.left == 4));

        // Turn 4
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 24);
        assert_eq!(boss.hit_points, 3);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == PlayerWin));
    }

    #[test]
    fn part_one_second_fight() {
        let mut player = Player::new(10, 250);
        let mut boss = Boss::new(14, 8);

        // Turn 1
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 250);
        assert_eq!(boss.hit_points, 14);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::Recharge);
        assert!(result.is_ok_and(|status| status == Pending));

        // Turn 2
        assert_eq!(player.hit_points, 10);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 21);
        assert_eq!(boss.hit_points, 14);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.recharge_effect.clone().is_some_and(|d| d.left == 4));

        // Turn 3
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 122);
        assert_eq!(boss.hit_points, 14);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::Shield);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.recharge_effect.clone().is_some_and(|d| d.left == 3));

        // Turn 4
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.current_mana, 110);
        assert_eq!(boss.hit_points, 14);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.recharge_effect.clone().is_some_and(|d| d.left == 2));
        assert!(player.shield_effect.clone().is_some_and(|d| d.left == 5));

        // Turn 5
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 7);
        assert_eq!(player.current_mana, 211);
        assert_eq!(boss.hit_points, 14);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::Drain);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.recharge_effect.clone().is_some_and(|d| d.left == 1));
        assert!(player.shield_effect.clone().is_some_and(|d| d.left == 4));

        // Turn 6
        assert_eq!(player.hit_points, 3);
        assert_eq!(player.armor, 7);
        assert_eq!(player.current_mana, 239);
        assert_eq!(boss.hit_points, 12);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.recharge_effect.clone().is_none());
        assert!(player.shield_effect.clone().is_some_and(|d| d.left == 3));

        // Turn 7
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.current_mana, 340);
        assert_eq!(boss.hit_points, 12);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::Poison);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.shield_effect.clone().is_some_and(|d| d.left == 2));

        // Turn 8
        assert_eq!(player.hit_points, 2);
        assert_eq!(player.armor, 7);
        assert_eq!(player.current_mana, 167);
        assert_eq!(boss.hit_points, 12);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.shield_effect.clone().is_some_and(|d| d.left == 1));
        assert!(boss.poison_effect.clone().is_some_and(|d| d.left == 5));

        // Turn 9
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 7);
        assert_eq!(player.current_mana, 167);
        assert_eq!(boss.hit_points, 9);

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::MagicMissile);
        assert!(result.is_ok_and(|status| status == Pending));
        assert!(player.shield_effect.clone().is_none());
        assert!(boss.poison_effect.clone().is_some_and(|d| d.left == 4));

        // Turn 8
        assert_eq!(player.hit_points, 1);
        assert_eq!(player.armor, 0);
        assert_eq!(player.current_mana, 114);
        assert_eq!(boss.hit_points, 2);

        let result = Day22.make_boss_turn(&mut player, &mut boss);
        assert!(result.is_ok_and(|status| status == PlayerWin));
    }

    #[test]
    fn shield_recast_mechanic() {
        let mut player = Player::new(50, 500);
        let mut boss = Boss::new(50, 10);

        player.shield_effect = EffectDuration::new(1);
        player.armor = 7;

        let result = Day22.make_player_turn(&mut player, &mut boss, &Spell::Shield);
        assert!(result.is_ok());
        assert!(player.shield_effect.is_some());
        assert!(player.shield_effect.as_ref().unwrap().left >= 6);
    }
}
