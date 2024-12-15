use clap::Parser;

use crate::commands::Commands;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Hp Cli - a simple CLI for multi purpose application.",
long_about = "Built by Kevin Park but not sure how I am going to use it yet hehe.")]
pub struct CliApp {
    // #[clap(long, short = 'k')]
    // key: String,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(global=true)]
    global_testing: Option<String>
}