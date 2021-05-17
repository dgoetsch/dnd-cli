use crate::domain::character::Character;
use anyhow::Result;
use dirs::home_dir;
pub struct Store {
    storage_dir: PathBuf,
}

impl Store {
    pub fn new(storage_dir: PathBuf) -> Result<Store> {
        let store = std::fs::create_dir_all(storage_dir.clone()).map(|_| Store {
            storage_dir,
        })?;
        Ok(store)
    }

    fn path_for(&self, key: String) -> String {
        let dir = self.storage_dir
            .to_str()
            .unwrap_or("");

        let dir = dir.strip_suffix("/").unwrap_or(dir);
        let key = key.strip_prefix("/").unwrap_or(key.as_str());
        format!("{}/{}", dir, key)
    }
}

use crate::domain::inventory::{Inventory, InventoryItem};
use itertools::Itertools;
use serde_json::Value;
use crate::domain::hit_points::HitPoints;
use std::fs::FileType;
use std::path::PathBuf;
use std::collections::HashMap;

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
    pub fn source_files(&self) -> Vec<String> {
        let home = home_dir().and_then(|p| p.to_str().map(|s| s.to_string())).unwrap_or("".to_string());
        vec![format!(
            "{}/.dnd-cli/template.json", home),
             self.path_for("character.json".to_string())]
    }

    pub fn load_file(template: Value, path: String) -> Result<Value> {
        let content = std::fs::read_to_string(path)?;
        let value: Value = serde_json::from_str(content.as_str())?;
        Ok(merge(template, value))
    }

    pub fn load_character(&self) -> Result<Character> {
        let value = self.source_files().into_iter()

            .fold(Ok(Value::Object(serde_json::Map::new())), |r, b| {
                r.and_then(|template| Store::load_file(template, b))
        })?;
        let character: Character = serde_json::value::from_value(value)?;
        let inventory = Store::load_inventory(self.storage_dir.clone())?;
        let inventory = Inventory::new(inventory);
        let character = character.with_inventory(inventory);
        Ok(character)


        // let template =
        //     std::fs::read_to_string(self.path_for("characters/template.json".to_string()))?;
        // let template: Value = serde_json::from_str(template.as_str())?;
        // let content = std::fs::read_to_string(self.path_for(format!("characters/{}.json", name)))?;
        // let character: Value = serde_json::from_str(content.as_str())?;
        // let merged = merge(character, template);
        // let character = serde_json::value::from_value(merged)?;
        // Ok(character)
    }

    pub fn update_inventory(&self, inventory: Inventory) -> Result<()> {
        Store::write_inventory(self.storage_dir.clone(), inventory.items())?;
        // let content = std::fs::read_to_string(self.path_for(format!("characters/{}.json", name)))?;
        // let mut character: Character = serde_json::from_str(content.as_str())?;
        // let updated = serde_json::to_string(&character.with_inventory(inventory))?;
        // std::fs::write(self.path_for(format!("characters/{}.json", name)), updated)?;
        Ok(())
    }

    pub fn update_hit_points(&self, hit_points: HitPoints) -> Result<()> {
        let file_name = self.path_for("character.json".to_string());
        let content = std::fs::read_to_string(file_name.clone())?;
        let mut character: Character = serde_json::from_str(content.as_str())?;
        let updated = serde_json::to_string(&character.with_hit_points(hit_points))?;
        std::fs::write(file_name, updated)?;
        Ok(())
    }

    fn write_inventory(path: PathBuf, items: &HashMap<String, InventoryItem>) -> Result<()> {
        for (name, item) in items.iter() {
            let child_path =path.join(name);
            match item {
                InventoryItem::Object {
                    count
                } => {
                    std::fs::write(child_path, serde_json::to_string_pretty(item)?)?;
                },
                InventoryItem::Container { items } => {
                    std::fs::create_dir_all(child_path.clone())?;
                    Store::write_inventory(child_path, items)?;
                }
            };
        }
        Ok(())

    }

    fn load_inventory(path: PathBuf) -> Result<HashMap<String, InventoryItem>> {
        let dir = std::fs::read_dir(path)?;

        let result = dir
            .flat_map(|r| r.ok())
            .flat_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                Store::load_inventory(path.clone()).ok().and_then(|items|
                    path.file_name().map(|name| (name.to_str().unwrap_or("").to_string(), InventoryItem::Container { items })))

            } else {
                Store::load_item(path.clone()).ok().and_then(|item|
                path.file_name().map(|name| (name.to_str().unwrap_or("").to_string(), item)))
            }
        }).collect();

        Ok(result)
    }

    fn load_item(path: PathBuf) -> Result<InventoryItem> {
        let contents = std::fs::read_to_string(path)?;
        let item = serde_json::from_str(&contents)?;
        Ok(item)
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

    #[test]
    pub fn test() {
        let dir = std::fs::read_dir("./src").unwrap();
        let result: Vec<String> = dir
            .flat_map(|r| r.ok())
            .flat_map(|entry| {
                let path = entry.path();
                if path.is_dir() {

                        path.file_name().map(|name| name.to_str().unwrap_or("").to_string())

                } else {
                        path.file_name().map(|name| name.to_str().unwrap_or("").to_string())
                }
            }).collect();

        println!("{:?}", result);
    }
}
