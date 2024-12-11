use core::fmt;

use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum Commands {
    Encrypting(Encrypting),
    Zipping(Zipping),
    StringSearch(StringSearch)
    // Projects(Project)
}
// How to implement Debug trait


impl fmt::Debug for Commands {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Commands")
        // .field("x", &self)
        .finish()
    }
}


#[derive(Args)]
pub struct Project {
    #[arg(short= 'e', long= "encryption", default_value_t = String::from("."))]
    /// directory to start exploring from
    start_path: String,
    #[arg(short, long, value_delimiter = ':')]
    /// paths to exclude when searching
    exclude: Vec<String>,
}


#[derive(Args)]
pub struct Encrypting {
    #[arg(long = "algorithm", short= 'a')]
    pub algorithm: Option<String>,

    #[arg(long = "file-path", short= 'f')]
    pub file_path: Option<String>,
    pub path: std::path::PathBuf,
}
#[derive(Args)]
pub struct Zipping {
    // #[arg(long = "file-path", short= 'f')]
    // pub path: std::path::PathBuf,


    /// The file path
    pub file_path: Option<String>,


    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}
#[derive(Args)]
pub struct StringSearch {

    /// the whole input string you want to process
    pub input_string: Option<String>,

    /// the word you try to find
    pub search: Option<String>,
}


