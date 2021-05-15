use crate::domain::character::Character;
use anyhow::Result;

pub struct Store {
    storage_dir: String,
}

impl Store {
    pub fn new(storage_dir: &str) -> Result<Store> {
        let store = std::fs::create_dir_all(storage_dir).map(|_| Store {
            storage_dir: storage_dir.to_string(),
        })?;
        Ok(store)
    }

    fn path_for(&self, key: String) -> String {
        self.storage_dir
            .strip_suffix('/')
            .map(|s| s.to_string())
            .unwrap_or(self.storage_dir.clone())
            + "/"
            + key.strip_prefix('/').map(|s| s).unwrap_or(key.as_str())
    }
}

use crate::domain::inventory::Inventory;
use itertools::Itertools;
use serde_json::Value;
use crate::domain::hit_points::HitPoints;

fn merge(a: Value, b: Value) -> Value {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => Value::Object(
            a.keys()
                .chain(b.keys())
                .dedup()
                .flat_map(|key| match (a.get(key), b.get(key)) {
                    (Some(a), Some(b)) => Some((key.clone(), merge(a.clone(), b.clone()))),
                    (Some(a), None) => Some((key.clone(), a.clone())),
                    (None, Some(b)) => Some((key.clone(), b.clone())),
                    _ => None,
                })
                .collect(),
        ),
        (Value::Array(a), Value::Array(b)) => {
            let mut result = vec![];
            result.extend(a);
            result.extend(b);
            Value::Array(result)
        }
        (a, _) => a,
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Store {
    pub fn load_character(&self, name: &String) -> Result<Character> {
        let template =
            std::fs::read_to_string(self.path_for("characters/template.json".to_string()))?;
        let template: Value = serde_json::from_str(template.as_str())?;
        let content = std::fs::read_to_string(self.path_for(format!("characters/{}.json", name)))?;
        let character: Value = serde_json::from_str(content.as_str())?;
        let merged = merge(character, template);
        let character = serde_json::value::from_value(merged)?;
        Ok(character)
    }

    pub fn update_inventory(&self, name: &String, inventory: Inventory) -> Result<()> {
        let content = std::fs::read_to_string(self.path_for(format!("characters/{}.json", name)))?;
        let mut character: Character = serde_json::from_str(content.as_str())?;
        let updated = serde_json::to_string(&character.with_inventory(inventory))?;
        std::fs::write(self.path_for(format!("characters/{}.json", name)), updated)?;
        Ok(())
    }

    pub fn update_hit_points(&self, name: &String, hit_points: HitPoints) -> Result<()> {
        let content = std::fs::read_to_string(self.path_for(format!("characters/{}.json", name)))?;
        let mut character: Character = serde_json::from_str(content.as_str())?;
        let updated = serde_json::to_string(&character.with_hit_points(hit_points))?;
        std::fs::write(self.path_for(format!("characters/{}.json", name)), updated)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::domain::ability_score::Ability;
    use crate::domain::effect::{Effect, RollBonus, RollScope};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Skill {
        name: String,
        ability: Ability,
    }

    fn skills() -> Vec<Skill> {
        serde_json::from_str::<Vec<Skill>>(
            &std::fs::read_to_string("./characters/skills.json").unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn create_skills() {
        let skills = skills();
        let effects = skills
            .iter()
            .map(|skill| Effect::Roll {
                bonus: RollBonus::Ability(skill.ability.clone()),
                scope: RollScope {
                    path: Some(vec!["skill".to_string(), skill.name.to_lowercase()]),
                    ..RollScope::default()
                },
            })
            .collect::<Vec<Effect>>();

        println!("{}", serde_json::to_string(&effects).unwrap());
    }
}
