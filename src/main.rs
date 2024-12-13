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
        Some(Commands::Encrypting(encrypting)) => {

            let result = match encrypting.input_type.as_deref() {
                Some("string") => {
                    println!("Given String")

                }
                Some("file") => {
                    println!("Given file name")

                }
                None => {
                    println!("Invalid type.");
                }
                _ => {

                }
            };


            match encrypting.algorithm.as_deref() {
                Some("sha256") => {
                    println!("Sha256  chosen.")
                }
                Some("md5") => {
                    println!("Md5 chosen.")
                }

                Some(ref _algo) => {
                    println!("Algorithm you chose is : {}", _algo);
                }
                None => {
                    println!("algorithm is not valid.")
                }
            }
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