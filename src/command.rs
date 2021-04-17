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
}
