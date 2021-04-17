use crate::domain::ability_score::Ability;
use crate::domain::roll::Roll;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Effect {
    // Ability {
    //     bonus: AbilityScoreBonus,
    //     ability: Ability,
    // },
    Roll { bonus: RollBonus, scope: RollScope },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum AbilityScoreBonus {
    Modifier { modifier: isize },
    Become { value: isize },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum RollBonus {
    // Reroll(HashSet<isize>),
    // Advantage(Advantage),
    Modifier(isize),
    Roll(Roll),
    Proficiency,
    Ability(Ability)
}

// #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
// pub enum Advantage {
//     Advantage,
//     Disadvantage,
// }
//
// impl Display for Advantage {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Advantage::Advantage => write!(f, "Advantage"),
//             Advantage::Disadvantage => write!(f, "Disadvantage"),
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct RollScope {
    pub path: Option<Vec<String>>,
    pub ability: Option<Ability>,
    pub range: Option<Range>,
}

impl RollScope {
    pub fn matches(&self, path: &Vec<String>) -> bool {
        match &self.path {
            Some(scope) => {
                &path
                    .clone()
                    .into_iter()
                    .take(scope.len())
                    .collect::<Vec<String>>()
                    == scope
            }
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum Range {
    Melee,
    Ranged { normal: isize, long: isize },
}

impl Display for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Range::Melee => write!(f, "Melee"),
            Range::Ranged { normal, long } => write!(f, "Ranged ({}/{})", normal, long),
        }
    }
}
