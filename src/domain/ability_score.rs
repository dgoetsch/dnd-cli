use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Ability {
    pub fn of(value: String) -> Option<Ability> {
        match value.to_lowercase().as_str() {
            "strength" | "str" => Some(Ability::Strength),
            "dexterity" | "dex" => Some(Ability::Dexterity),
            "constitution" | "con" => Some(Ability::Constitution),
            "intelligence" | "int" => Some(Ability::Intelligence),
            "wisdom" | "wis" => Some(Ability::Wisdom),
            "charisma" | "cha" => Some(Ability::Charisma),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct AbilityScores {
    strength: AbilityScore,
    dexterity: AbilityScore,
    constitution: AbilityScore,
    intelligence: AbilityScore,
    wisdom: AbilityScore,
    charisma: AbilityScore,
}

impl Display for Ability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl AbilityScores {
    pub fn default() -> AbilityScores {
        AbilityScores {
            strength: AbilityScore::default(),
            dexterity: AbilityScore::default(),
            constitution: AbilityScore::default(),
            intelligence: AbilityScore::default(),
            wisdom: AbilityScore::default(),
            charisma: AbilityScore::default(),
        }
    }

    pub fn get(&self, ability: Ability) -> AbilityScore {
        match ability {
            Ability::Strength => self.strength.clone(),
            Ability::Dexterity => self.dexterity.clone(),
            Ability::Constitution => self.constitution.clone(),
            Ability::Intelligence => self.intelligence.clone(),
            Ability::Wisdom => self.wisdom.clone(),
            Ability::Charisma => self.charisma.clone(),
        }
    }

    pub fn with(&self, ability: Ability, score: AbilityScore) -> AbilityScores {
        let mut new = self.clone();
        match ability {
            Ability::Strength => new.strength = score,
            Ability::Dexterity => new.dexterity = score,
            Ability::Constitution => new.constitution = score,
            Ability::Intelligence => new.intelligence = score,
            Ability::Wisdom => new.wisdom = score,
            Ability::Charisma => new.charisma = score,
        };
        new
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AbilityScore {
    value: isize,
}

impl Default for AbilityScore {
    fn default() -> Self {
        AbilityScore::of(10)
    }
}

impl AbilityScore {
    pub fn of(value: isize) -> AbilityScore {
        AbilityScore { value }
    }
    pub fn modifier(&self) -> isize {
        if self.value < 10 {
            (self.value - 11) / 2
        } else {
            (self.value - 10) / 2
        }
    }
}

#[cfg(test)]
mod test {
    use super::AbilityScore;

    fn assert_modfier(value: isize, modifier: isize) {
        assert_eq!(
            AbilityScore::of(value).modifier(),
            modifier,
            "Expected modifier for ability score {} to be {}",
            value,
            modifier
        )
    }
    #[test]
    fn test_modifier() {
        vec![
            (4, -3),
            (5, -3),
            (6, -2),
            (7, -2),
            (8, -1),
            (9, -1),
            (10, 0),
            (11, 0),
            (12, 1),
            (13, 1),
            (14, 2),
            (15, 2),
            (16, 3),
            (17, 3),
            (18, 4),
            (19, 4),
            (20, 5),
            (21, 5),
            (22, 6),
            (23, 6),
        ]
        .into_iter()
        .for_each(|(value, modifier)| assert_modfier(value, modifier))
    }
}
