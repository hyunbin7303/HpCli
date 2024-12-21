mod commands;
mod credentials;
mod cmd_handler;
mod files;
mod strings;
mod cliapp;
mod rand_generator;
mod cryptography;

use cliapp::CliApp;
use commands::Commands;
use credentials::Credentials;
use clap::{Command, Parser};
use files::file_helper;
use strings::string_handler::inspect;



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
        Some(Commands::Inspect(input)) => {
            match input.string {
                Some(ref _name) => {
                    let (res, kind) = inspect(_name, input.only_digits);

                    let mut plural_s = "s";
                    if res == 1 {
                        plural_s = "";
                    }

                    println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
                }
                None => {
                    println!("Type any string.")
                }
            }
        }

        Some(Commands::Random(input)) => {

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


