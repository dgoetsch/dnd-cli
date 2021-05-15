mod command;
pub mod domain;
pub mod render;
mod store;

use crate::domain::ability_score::{Ability, AbilityScore};
use command::*;

use anyhow::Result;
use clap::{Clap, ValueHint};
fn main() {
    let cmd = command::RootCmd::parse();

    handle(cmd).unwrap();
}
use crate::domain::character::Character;
use crate::domain::roll::Roll;
use render::Render;
use std::io::Write;

fn handle(cmd: RootCmd) -> Result<()> {
    let store = store::Store::new("./")?;
    match cmd {
        RootCmd::Character { name, cmd } => {
            let mut character = store.load_character(&name)?.clone();
            match cmd {
                CharacterCmd::Roll { name } => {
                    handle_roll_cmd(name, &character)?;
                }
                CharacterCmd::Inventory { cmd } => {
                    handle_inventory_cmd(cmd, &mut character)?;
                    store.update_inventory(&name, character.inventory().clone())?;
                }
                CharacterCmd::HitPoints { cmd } => {
                    handle_hitpoints_cmd(cmd, &mut character)?;
                    store.update_hit_points(&name, character.hit_points().clone())?;
                }
            }
        }
    };
    Ok(())
}

fn handle_hitpoints_cmd(cmd: HitPointsCmd, character: &mut Character) -> Result<()> {
    match cmd {
        HitPointsCmd::Show => { }
        HitPointsCmd::IncreaseMax { hit_points } => {
            character.hit_points().increase_max(hit_points);
        }
        HitPointsCmd::Add { hit_points } => {
            character.hit_points().add_current(hit_points);
        }
        HitPointsCmd::Remove { hit_points } => {
            character.hit_points().add_current(-hit_points);
        }
        HitPointsCmd::AddTemporary { hit_points } => {
            character.hit_points().add_temporary(hit_points);
        }
        HitPointsCmd::ResetTemporary => {
            character.hit_points().reset_temporary();
        }
        HitPointsCmd::Reset => {
            character.hit_points().reset();
        }
    }

    render(character.hit_points())?;
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
        }
        None => println!("Nothing to roll for {:?}", name),
    };
    Ok(())
}

fn handle_inventory_cmd(cmd: InventoryCmd, character: &mut Character) -> Result<()> {
    match cmd {
        InventoryCmd::Add { name, count } => {
            let result = character.inventory().add_item(name.clone(), count)?;
            render(character.inventory())?;
            render(&result)?;
            Ok(())
        }
        InventoryCmd::Remove { name, count } => {
            let result = character.inventory().add_item(name, -count)?;
            render(character.inventory())?;
            render(&result)?;
            Ok(())
        }
        InventoryCmd::Show => {
            render(character.inventory())?;
            Ok(())
        },
        InventoryCmd::Container { cmd } => {
            match cmd {
                InventoryContainerCmd::Add { name } => {
                    let result = character.inventory().add_container(name)?;
                    render(&result)?;
                    Ok(())
                },
                InventoryContainerCmd::Remove { name } => {

                    Ok(())
                }
            }
        }
    }
}

fn render(renderable: &Render) -> Result<()> {
    let mut out = std::io::stdout();
    renderable.render(0, &mut out)?;
    out.flush()?;
    Ok(())
}
