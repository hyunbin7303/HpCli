mod commands;
mod credentials;

use clap::{ Parser, };
use commands::Commands;
use credentials::Credentials;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Hp Cli - a simple CLI for multi purpose application.",
long_about = "Built by Kevin Park but not sure how I am going to use it yet hehe.")]
pub struct CliApp {
    // #[clap(long, short = 'k')]
    // key: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

fn main() {

    let creds = Credentials::new(String::from("SOME_API_KEY"), String::from("SOME_SECRET"));
    println!("{}", creds.to_string());
    let cli = CliApp::parse();

    match &cli.command {
        Some(Commands::Encrypting(name)) => {

            println!("Encryption Option. Choose algorithm.");
            println!("1. SHA256");
            println!("2. MD5");
            // match name.algorithm {
            //     Some(ref _name) => {

            //     }
            //     None => {
            //         // Testing
            //     }
            // }
        }
        Some(Commands::Zipping(name)) => {
            match name.file_path {
                Some(ref _name) => {

                }None => {}
            }
        }
        Some(Commands::StringSearch(check)) => {
            match check.input_string {
                Some(ref _str) => {

                }None => {}
            }

            match check.search {
                Some(ref _search) => {

                }None => {}
            }
        }

        // Some(Commands::Projects(name)) =>
        // {
        //     start_path;
        //     exclude;
        // }) => match projects(&start_path, args.max_depth, &exclude, &logger) {
        //     Ok(_) => {}
        //     Err(e) => eprintln!("error in processing : {}", e),
        // },
        None => {}
    }
}
