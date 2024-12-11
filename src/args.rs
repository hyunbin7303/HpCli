

#[derive(Debug,Parser)]
#[clap(author,version,about, long_about = None)]
pub struct CliArgs {
    #[clap(subcommand)]
    pub userinput : Option<ActionType>,
    // #[clap(global = false, long)]
    // json: bool,
}
