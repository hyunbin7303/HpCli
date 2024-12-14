mod commands;
mod credentials;
mod hash_helper;
mod cmd_handler;
mod file_helper;

use clap::{ Parser, };
use commands::Commands;
use credentials::Credentials;
use sha2::{Digest, Sha256};

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

fn main() {
    match file_helper::get_files_in_folder("./target/debug") {
        Ok(file_names) => {
            for file_name in file_names {
                println!("{}", file_name.display());
            }
        }
        Err(e) => println!("Error: {}", e),
    }


    let creds = Credentials::new(String::from("SOME_API_KEY"), String::from("SOME_SECRET"));
    println!("{}", creds.to_string());

    let cli = CliApp::parse();
    match &cli.command {
        Some(Commands::Encrypting(encrypting)) => {
            cmd_handler::crypto_handler(&encrypting);
        }
        Some(Commands::Zipping(name)) => {
            match name.file_path {
                Some(ref _name) => {

                }None => {}
            }
        }
        Some(Commands::StringSearch(stringsearch)) => {
            cmd_handler::string_handler(&stringsearch);
            // TODO Receive Input files from the user.
            // Search specific string from the txt file or csv file.


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


