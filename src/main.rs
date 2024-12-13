mod commands;
mod credentials;
mod hash_helper;

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

    let creds = Credentials::new(String::from("SOME_API_KEY"), String::from("SOME_SECRET"));
    println!("{}", creds.to_string());




    let cli = CliApp::parse();

    match &cli.command {
        Some(Commands::Encrypting(encrypting)) => {
            //Symmetric Encryption
            // AES Encryption
            //

            let result = match encrypting.input_type.as_deref() {
                Some("string") => {
                    println!("Given String");
                    // Should we do the match statement sgdsfddsagain in here?
                    // encrypt_sha256("");
                    let inputStr = match encrypting.input_string.as_deref() {
                        Some(input) => input,
                        None => {
                            return
                        }
                    };


                    if encrypting.algorithm.as_deref().unwrap() == "sha256".to_string() {
                        println!("Sha256  chosen.");

                        let result = hash_helper::encrypt_sha256(inputStr);
                        println!("{}", &result);
                    }
                    else if encrypting.algorithm.as_deref().unwrap() == "md5".to_string() {
                        println!("md5  chosen.")
                    }
                    else {
                        println!("Invalid hash algorithm.")
                    }

                    true;
                    return
                }
                Some("file") => {
                    false
                }
                None => {
                    println!("Invalid type.");
                    return
                }
                _ => {
                    false
                }
            };
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


