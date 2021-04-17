mod command;
pub mod domain;
mod store;

use crate::domain::ability_score::{Ability, AbilityScore};
use command::*;

use anyhow::Result;

fn main() {
    let cmd = command::RootCmd::parse();
    handle(cmd).unwrap();
}
use domain::roll::Render;
use std::io::Write;
use crate::domain::roll::Roll;

fn handle(cmd: RootCmd) -> Result<()> {
    let store = store::Store::new("./")?;
    match cmd {
        RootCmd::Character { name, cmd } => {
            let character = store.load_character(name)?;
            match cmd {
                CharacterCmd::Roll { name } => match name.first().map(|n| n.as_str()) {
                    Some("ability") => name
                        .get(1)
                        .map(|s| s.clone())
                        .and_then(Ability::of)
                        .map(|ability| {
                            let modifier = character.get_ability_score(ability).modifier();
                            println!("1d20+{}", modifier)
                        })
                        .unwrap_or_else(|| println!("Invalid ability path: {:?}", name)),
                    Some(first) => {
                        let calc_result = Roll::calculate(&name, &character);
                        let mut out = std::io::stdout();
                        calc_result.render(0, &mut out).unwrap();
                        out.flush().unwrap();
                    },


                    None => println!("Nothing to roll for {:?}", name),
                },
            }
        }
    };
    Ok(())
}
