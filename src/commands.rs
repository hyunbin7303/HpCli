use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum Commands {
    /// Reverses a string
    Encrypting(Encrypting),
    // Inspect(Inspect),
    Zipping(Zipping),
    // Projects {
    //     #[arg(short, long, default_value_t = String::from("."))]
    //     /// directory to start exploring from
    //     start_path: String,
    //     #[arg(short, long, value_delimiter = ':')]
    //     /// paths to exclude when searching
    //     exclude: Vec<String>,
    // },
}


#[derive(Args)]
pub struct Encrypting {
    /// The string to reverse
    pub string: Option<String>,
}
#[derive(Args)]
pub struct Zipping {
    pub string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}
