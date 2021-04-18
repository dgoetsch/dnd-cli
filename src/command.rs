use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Dnd Cli Utilities")]
pub enum RootCmd {
    Character {
        #[structopt()]
        name: String,
        #[structopt(subcommand)]
        cmd: CharacterCmd,
    },
}

impl RootCmd {
    pub fn parse() -> RootCmd {
        RootCmd::from_args()
    }
}

#[derive(StructOpt, Debug)]
pub enum CharacterCmd {
    Roll {
        #[structopt()]
        name: Vec<String>,
    },
    Inventory {
        #[structopt(subcommand)]
        cmd: InventoryCmd,
    },
}

#[derive(StructOpt, Debug)]
pub enum InventoryCmd {
    Add {
        #[structopt()]
        count: isize,
        #[structopt()]
        name: Vec<String>,
    },
    Remove {
        #[structopt()]
        count: isize,
        #[structopt()]
        name: Vec<String>,
    },
    Show,
    Container {
        #[structopt(subcommand)]
        cmd: InventoryContainerCmd,
    },
}
#[derive(StructOpt, Debug)]

pub enum InventoryContainerCmd {
    Add {
        #[structopt()]
        name: Vec<String>,
    },
    Remove {
        #[structopt()]
        name: Vec<String>,
    },
}
