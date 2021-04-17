mod command;
pub mod domain;
mod store;
pub mod render;

use crate::domain::ability_score::{Ability, AbilityScore};
use command::*;

use anyhow::Result;

fn main() {
    let cmd = command::RootCmd::parse();
    handle(cmd).unwrap();
}
use render::Render;
use std::io::Write;
use crate::domain::roll::Roll;
use crate::domain::character::Character;

fn handle(cmd: RootCmd) -> Result<()> {
    let store = store::Store::new("./")?;
    match cmd {
        RootCmd::Character { name, cmd } => {
            let mut character = store.load_character(&name)?.clone();
            match cmd {
                CharacterCmd::Roll { name } => {
                    handle_roll_cmd(name, &character)?;
                },
                CharacterCmd::Inventory { cmd } => {
                    handle_inventory_cmd(cmd, &mut character)?;
                    store.update(&name, character.inventory().clone())?;
                }
            }
        }
    };
    Ok(())
}

fn handle_roll_cmd(name: Vec<String>, character: &Character) -> Result<()> {
    match name.first().map(|n| n.as_str()) {
        Some("ability") => name
            .get(1)
            .map(|s| s.clone())
            .and_then(Ability::of)
            .map(|ability| {
                let modifier = character.get_ability_score(ability).modifier();
                println!("1d20+{}", modifier)
            })
            .unwrap_or_else(|| println!("Invalid ability path: {:?}", name)),
        Some(_) => {
            let calc_result = Roll::calculate(&name, &character);
            render(&calc_result)?;
        },
        None => println!("Nothing to roll for {:?}", name)
    };
    Ok(())
}

fn handle_inventory_cmd(cmd: InventoryCmd, character: &mut Character) -> Result<()> {
    match cmd {
        InventoryCmd::Add { name, count } => {
            let result = character.inventory().add_item(name.clone(), count)?;
            let mut out = std::io::stdout();
            out.write_fmt(format_args!("Added {} {}, I now have {}\n", count, name, result.count()))?;
            out.flush()?;
            render(character.inventory())?;
            Ok(())
        },
        InventoryCmd::Remove { name, count } => {
            let result = character.inventory().remove_item(name, count)?;
            render(&result)?;
            render(character.inventory())?;
            Ok(())
        }
        InventoryCmd::Show => {
            render(character.inventory())?;
            Ok(())
        }
    }
}

fn render(renderable: &Render) -> Result<()> {
    let mut out = std::io::stdout();
    renderable.render(0, &mut out)?;
    out.flush()?;
    Ok(())
}