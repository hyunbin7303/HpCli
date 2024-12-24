use core::fmt;

use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum Commands {
    Encrypting(Encrypting),
    Decrypting(Decrypting),
    Zipping(Zipping),
    StringSearch(StringSearch),
    Inspect(Inspect),
    Random(Random)
    // Projects(Project)
}
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


#[derive(Args, Debug)]
pub struct Encrypting {
    // #[arg(required=false)]
    // pub path: std::path::PathBuf,
    #[arg(long = "type", short ='t', required=true)]
    pub input_type: Option<String>,

    #[arg(long = "algorithm", short = 'a', required=true)]
    pub algorithm: Option<String>,

    #[arg(required=true)]
    pub input_string: Option<String>,

    #[arg(long= "output", short= 'o', required=false)]
    pub output_path: Option<String>,

    #[arg(long = "file-path", short= 'f', required=false)]
    pub file_path: Option<String>,

    #[arg(short='p', long="password", required=false)]
    pub password: Option<String>,
}

#[derive(Args,Debug)]
pub struct Decrypting {
    #[arg(long = "type", short ='t', required=true)]
    pub input_type: Option<String>,

    #[arg(long = "algorithm", short = 'a', required=true)]
    pub algorithm: Option<String>,

    #[arg(required=true)]
    pub input_string: Option<String>,

    #[arg(long= "output", short= 'o', required=false)]
    pub output_path: Option<String>,

    #[arg(long = "file-path", short= 'f', required=false)]
    pub file_path: Option<String>,

    #[arg(short='p', long="password", required=false)]
    pub password: Option<String>,
}

#[derive(Args)]
pub struct Zipping {
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

#[derive(Args)]
pub struct Inspect {
    /// The string to inspect
    pub string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}

#[derive(Args)]
pub struct Random {

    /// Type of the random output.
    #[arg(short='o', long= "output")]
    pub output_type: Option<String>,

    /// The string to inspect
    pub string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    pub only_digits: bool,
}

