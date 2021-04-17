use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use crate::render::Render;
use std::io::Write;
use itertools::Itertools;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Default)]
pub struct Inventory {
    #[serde(default)]
    items: HashMap<String, InventoryItem>
}

pub enum RemoveItemResult {
    InsufficientInventory { name: String, requested: isize, available: isize },
    Success { name: String, requested: isize, available: isize }
}

impl Render for Inventory {
    fn render(&self, indent: usize, out: &mut Write) -> Result<()> {
        out.write_fmt(format_args!("{}Inventory\n", Render::tab(indent)))?;
        for (name, item) in self.items.iter().sorted_by_key(|(n, i)| n.clone()) {
            out.write_fmt(format_args!("{}x{} ... {}\n", Render::tab(indent + 1), item.count, name))?;
        }
        out.flush();
        Ok(())
    }
}

impl Render for RemoveItemResult {
    fn render(&self, indent: usize, out: &mut dyn Write) -> Result<()> {
        match self {
            RemoveItemResult::InsufficientInventory { name, requested, available} => {
                if available.clone() <= 0 {
                    out.write_fmt(format_args!("{}Tried to use {} {}, but I don't have any\n", Render::tab(indent), requested, name))?;
                } else {
                    out.write_fmt(format_args!("{}Tried to use {} {}, but I only have {}\n", Render::tab(indent), requested, name, available))?;
                }
            },
            RemoveItemResult::Success { name, requested, available } => {
                out.write_fmt(format_args!("{}I used {} {}, and now have {} remaining\n", Render::tab(indent), requested, name, available))?;
            }
        }
        Ok(())
    }
}
impl Inventory {
    pub fn add_item(&mut self, name: String, count: isize) -> Result<InventoryItem> {
        let existing = self.items.get_mut(&name);
        if let Some(existing) = existing {
            existing.add(count)?;
            Ok(existing.clone())
        } else {
            let item = InventoryItem { count };
            self.items
                .insert(name, item.clone());
            Ok(item)

        }
    }

    pub fn remove_item(&mut self, name: String, count: isize) -> Result<RemoveItemResult> {
        let result = if let Some(existing) = self.items.get_mut(&name) {
            if(existing.count >= count) {
                existing.count -= count;
                RemoveItemResult::Success { name, requested: count, available: existing.count }
            } else {
                RemoveItemResult::InsufficientInventory { name, requested: count, available: existing.count }
            }
        } else {
            RemoveItemResult::InsufficientInventory { name, requested: count, available: 0 }
        };

        match &result {
            RemoveItemResult::Success { name, requested, available } => {
                if(available == &0) {
                    self.items.remove(name);
                }
            },
            RemoveItemResult::InsufficientInventory { name, requested, available } => {
                if(available == &0) {
                    self.items.remove(name);
                }
            }
        };

        Ok(result)


    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct InventoryItem {
    count: isize
}

impl InventoryItem {
    fn add(&mut self, count: isize) -> Result<()> {
        self.count += count;
        Ok(())
    }

    pub fn count(&self) -> isize {
        self.count
    }
}

