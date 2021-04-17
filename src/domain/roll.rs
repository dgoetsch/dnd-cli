use crate::domain::ability_score::Ability;
use crate::domain::character::Character;
use crate::domain::effect::{Effect, RollBonus};
use itertools::structs::GroupBy;
use itertools::Itertools;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::Result;
use std::io::Write;
use crate::render::Render;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Roll {
    dice: Vec<Dice>,
}

impl Roll {
    pub fn calculate(path: &Vec<String>, character: &Character) -> RollResult {
        let effects = character
            .all_effects()
            .iter()
            .flat_map(|(source, effect)| match effect.clone() {
                Effect::Roll { bonus, scope } => {
                    if (scope.matches(path)) {
                        Some((source.clone(), bonus))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<(Vec<String>, RollBonus)>>();

        let mut applicable_effects = effects
            .iter()
            .flat_map(|(path, bonus)|  match bonus {
                RollBonus::Roll(roll) => Some(EffectResult {
                    path: path.clone(),
                    rolled_dice: roll.dice.iter().map(|d| d.roll()).collect(),
                    bonus: 0,
                }),
                RollBonus::Modifier(bonus) => Some(EffectResult {
                    path: path.clone(),
                    rolled_dice: vec![],
                    bonus: bonus.clone(),
                }),
                RollBonus::Ability(ability) => {
                    let mut path = path.clone();
                    path.push(ability.to_string());
                    Some(EffectResult {
                        path,
                        rolled_dice: vec![],
                        bonus: character.get_ability_score(ability.clone()).modifier()
                    })
                },
                RollBonus::Proficiency => None,
            })
            .collect::<Vec<EffectResult>>();

        let proficiencies = effects
            .iter()
            .filter(|(path, bonus)| match bonus {
                RollBonus::Proficiency => true,
                _ => false,
            })
            .take(1)
            .flat_map(|(path, bonus)| match bonus {
                RollBonus::Proficiency => Some(EffectResult {
                    path: path.clone(),
                    rolled_dice: vec!(),
                    bonus: character.proficiency_bonus()
                }),
                _ => None,
            });

        applicable_effects.extend(proficiencies);

        applicable_effects.sort_by(|a, b| a.path.cmp(&b.path));
        RollResult {
            effects: applicable_effects,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RollResult {
    effects: Vec<EffectResult>,
}

impl Render for RollResult {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        out.write_fmt(format_args!("{}Results\n", (0..indent).map(|_| '\t').collect::<String>()))?;
        for effect in &self.effects {
            effect.render(indent + 1, out)?;
        }
        let total_result: isize = self.effects.iter().map(|e| e.total_bonus()).sum::<isize>();
        out.write_fmt(format_args!("Total: {}\n", total_result))?;

        Ok(())
    }
}



#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct EffectResult {
    path: Vec<String>,
    rolled_dice: Vec<RolledDice>,
    bonus: isize,
}

impl EffectResult {
    fn total_bonus(&self) -> isize {
        self.bonus + self.rolled_dice.iter().map(|r| r.results.iter().sum::<isize>()).sum::<isize>()
    }
}

impl Render for EffectResult {
    fn render(&self, indent: usize, out: &mut Write) -> Result<()> {
        let rolls = self.rolled_dice.iter().flat_map(|r| text(r)).join(" + ");

        let bonus = if self.bonus > 0 {
            format!("{} + {}", rolls, self.bonus)
        } else if self.bonus < 0 {
            format!("{} - {}", rolls, self.bonus)
        } else {
            rolls
        };


        out.write_fmt(format_args!("{}{}: {}\n",
                                   (0..indent).map(|_| '\t').collect::<String>(),
                                   self.path.iter().join(" / "),
                                   bonus))?;
        Ok(())
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Dice {
    count: isize,
    sides: isize,
}

impl Dice {
    pub fn roll(&self) -> RolledDice {
        let mut rand = rand::thread_rng();
        let results = (0..self.count)
            .map(|_| rand.gen_range(1..=self.sides))
            .collect();

        RolledDice {
            dice: self.clone(),
            results,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct RolledDice {
    dice: Dice,
    results: Vec<isize>,
}

fn text(rolled_dice: &RolledDice) -> Vec<String> {
    rolled_dice.results.iter().map(|result| format!("[{} / {}]", result, rolled_dice.dice.sides)).collect()
}

impl Render for RolledDice {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        out.write_fmt(format_args!("{}{}\n",
                                   (0..indent).map(|_| '\t').collect::<String>(),
                                   self.results.iter().map(|result| format!("[{} / {}]", result, self.dice.sides)).join(", ")))?;

        Ok(())
    }
}

impl Display for Dice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}d{}", self.count, self.sides)
    }
}