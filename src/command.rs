use clap::Clap;

#[derive(Clap, Debug, PartialEq)]
#[clap(about = "Dnd Cli Utilities")]
pub enum RootCmd {
    Character {
        #[clap()]
        name: String,
        #[clap(subcommand)]
        cmd: CharacterCmd,
    },
}

#[derive(Clap, Debug, PartialEq)]
pub enum CharacterCmd {
    Roll {
        #[clap()]
        name: Vec<String>,
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
        count: isize,
        #[clap()]
        name: Vec<String>,
    },
    Remove {
        #[clap()]
        count: isize,
        #[clap()]
        name: Vec<String>,
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
        name: Vec<String>,
    },
    Remove {
        #[clap()]
        name: Vec<String>,
    },
}
