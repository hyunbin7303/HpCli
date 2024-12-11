mod commands;


use clap::{Arg, Args, Parser, Subcommand};
use commands::Commands;

pub struct Credentials {
    api_key: String,
    secret: String,
}
impl Credentials {
    pub fn new(api_key: String, secret: String) -> Self {
        Credentials { api_key, secret }
    }
    pub fn to_string(&self) -> String {
        format!("Credentials({})", &self.api_key)
    }
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "stringer - a simple CLI to transform and inspect strings", long_about = "stringer is a super fancy CLI (kidding)

One can use stringer to modify or inspect strings straight from the terminal")]
struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}




fn main() {
    let creds = Credentials::new(String::from("SOME_API_KEY"), String::from("SOME_SECRET"));
    println!("{}", creds.to_string());
    // let config = match config::Config::new() {
    //     Ok(config) => config,
    //     Err(err) => {
    //         eprintln!("{}", err);
    //         std::process::exit(1);
    //     }
    // };
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Encrypting(name)) => {
            match name.string {
                Some(ref _name) => {

                }
                None => {
                    // Testing
                }
            }
        }
        Some(Commands::Zipping(name)) => {
            match name.string {
                Some(ref _name) => {

                }None => {}
            }
        }
        // Some(Commands::Projects(name, test)) => {
        //     match name.string {
        //         Some(ref _name) => {

        //         }None => {}
        //     }
        // }
        None => {}
    }


    // let match_result = command!()
    //     .arg(
    //         Arg::new("firstname")
    //             .short('f')
    //             .long("first-name")
    //             .aliases(["fname", "firstname"]),
    //     )
    //     .arg(Arg::new("lastname").short('l').long("last-name"))
    //     .arg(Arg::new("fluffy").long("fluffy"))
    //     .get_matches();
}
