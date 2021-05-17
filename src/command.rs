use clap::Clap;
use std::path::{PathBuf, Path};


#[derive(Clap, Debug, PartialEq)]
#[clap(about = "Dnd Cli Utilities")]
pub enum RootCmd {
    Character {
        // #[clap(short, long)]
        // name: String,
        #[clap(subcommand)]
        cmd: CharacterCmd,
    },
    Completions {
        #[clap(arg_enum, value_name="SHELL")]
        shell: Shell,
    }
}

#[derive(Clap, Debug, PartialEq)]
pub enum CharacterCmd {
    Roll {
        #[clap(subcommand)]
        cmd: RollCmd,
    },
    Inventory {
        #[clap(subcommand)]
        cmd: InventoryCmd,
    },
    HitPoints {
        #[clap(subcommand)]
        cmd: HitPointsCmd
    }
}
#[derive(Clap, Debug, PartialEq)]
pub enum RollCmd {
    Skill {
        #[clap(subcommand)]
        skill: Skill
    },
    Ability {
        #[clap(subcommand)]
        ability: Ability
    },
    SavingThrow {
        #[clap(subcommand)]
        ability: Ability
    }
}

impl RollCmd {
    pub fn to_path(&self) -> Vec<String> {
        match self {
            RollCmd::Skill { skill } => {
                vec!["skill".to_string(), skill.to_path()]
            },
            RollCmd::Ability { ability } => {
                vec!["ability".to_string(), format!("{:?}", ability).to_lowercase()]
            },
            RollCmd::SavingThrow { ability } => {
                vec!["saving-throw".to_string(), format!("{:?}", ability).to_lowercase()]
            }
        }
    }
}

#[derive(Clap, Debug, PartialEq)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Clap, Debug, PartialEq)]
pub enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl Skill {
    pub fn to_path(&self) -> String {
        match self {
            Skill::AnimalHandling => "animal-handling".to_string(),
            Skill::SleightOfHand => "sleight-of-hand".to_string(),
            _ => format!("{:?}", self).to_lowercase()
        }
    }
}

#[derive(Clap, Debug, PartialEq)]
pub enum HitPointsCmd {
    Show,
    IncreaseMax {
        #[clap()]
        hit_points: isize
    },
    Add {
        #[clap()]
        hit_points: isize
    },
    Remove {
        #[clap()]
        hit_points: isize
    },
    AddTemporary {
        #[clap()]
        hit_points: isize
    },
    ResetTemporary,
    Reset
}

#[derive(Clap, Debug, PartialEq)]
pub enum InventoryCmd {
    Add {

        #[clap()]
        name: PathBuf,
        #[clap()]
        count: isize,
    },
    Remove {
        #[clap()]
        name: PathBuf,
        #[clap()]
        count: isize,
    },
    Show,
    Container {
        #[clap(subcommand)]
        cmd: InventoryContainerCmd,
    },
}
#[derive(Clap, Debug, PartialEq)]
pub enum InventoryContainerCmd {
    Add {
        #[clap()]
        name: PathBuf,
    },
    Remove {
        #[clap()]
        name: PathBuf,
    },
}

#[derive(Clap, Copy, Clone, Debug, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish
}