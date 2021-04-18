use super::ability_score::AbilityScores;
use crate::domain::ability_score::{Ability, AbilityScore};
use crate::domain::effect::Effect;
use crate::domain::inventory::Inventory;
use crate::domain::roll::Roll;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type FeatureName = String;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Character {
    #[serde(default)]
    ability_scores: AbilityScores,
    #[serde(default)]
    classes: Vec<CharacterClass>,
    #[serde(default)]
    features: HashMap<FeatureName, Feature>,
    #[serde(default)]
    inventory: Inventory,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CharacterClass {
    name: String,
    level: isize,
}

impl Character {
    pub fn all_effects(&self) -> Vec<(Vec<String>, Effect)> {
        self.features
            .iter()
            .flat_map(|(name, feature)| feature.all_effects(vec![name.clone()]))
            .collect()
    }

    pub fn get_ability_score(&self, ability: Ability) -> AbilityScore {
        self.ability_scores.get(ability)
    }

    pub fn inventory(&mut self) -> &mut Inventory {
        &mut self.inventory
    }

    pub fn with_inventory(&self, inventory: Inventory) -> Character {
        Character {
            inventory,
            ..self.clone()
        }
    }
    pub fn get_feature(&self, name: &String) -> Option<&Feature> {
        self.features.get(name)
    }

    pub fn total_level(&self) -> isize {
        self.classes.iter().fold(0, |acc, class| acc + class.level)
    }

    pub fn proficiency_bonus(&self) -> isize {
        let total_level = self.total_level();
        if total_level < 5 {
            2
        } else if total_level < 9 {
            3
        } else if total_level < 13 {
            4
        } else if total_level < 17 {
            5
        } else if total_level <= 20 {
            6
        } else {
            total_level / 4 + 2
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Feature {
    #[serde(default)]
    pub children: HashMap<FeatureName, Feature>,
    #[serde(default)]
    pub roll: Option<Roll>,
    #[serde(default)]
    pub effects: Vec<Effect>,
}

impl Feature {
    pub fn all_effects(&self, path: Vec<String>) -> Vec<(Vec<String>, Effect)> {
        let mut effects: Vec<(Vec<String>, Effect)> = self
            .effects
            .iter()
            .map(|e| (path.clone(), e.clone()))
            .collect();

        effects.extend(self.children.iter().flat_map(|(name, feature)| {
            let mut path = path.clone();
            path.push(name.clone());
            feature.all_effects(path)
        }));

        effects
    }
}
