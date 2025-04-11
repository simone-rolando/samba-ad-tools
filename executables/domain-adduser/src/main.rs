use std::{io::{self, Write}, process};
use clap::{CommandFactory, Parser};
use fp_tools::{commands::user::is_existing_user, config::tools_config, debug_println};
use rpassword::read_password;

/// 
/// Command line arguments
/// 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file name
    #[arg(short, long)]
    filename: Option<String>,

    /// Interactive option
    #[arg(short, long)]
    interactive: Option<bool>,

    /// Update
    #[arg(short, long)]
    update: Option<bool>
}

fn main() {
    // Parse command line arguments
    let cli = Args::parse();

    // Load configuration
    let config = tools_config::read_config_from_file("/etc/ad/settings.json");
    if config.is_none() {
        eprintln!("domain-adduser: cannot load config file '/etc/ad/settings.json'");
        process::exit(1)
    }

    debug_println!("{:#?}", &config.as_ref().unwrap());

    // If no command has been provided, exit
    if cli.filename.is_none() && cli.interactive.unwrap_or(false) == false && cli.update.unwrap_or(false) == false {
        eprintln!("{}", Args::command().render_usage());
        process::exit(1)
    }

    // Interactive mode handling
    if cli.interactive.unwrap_or(false) {
        print!("Enter user name: ");
        io::stdout().flush().unwrap();

        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();

        let username = username.trim();
        
        if is_existing_user(&config.unwrap(), &username.to_string()) {
            eprintln!("domain-adduser: user {} already exists in the domain. Aborting!", username);
            process::exit(1)
        }

        // Additional user information
        let mut last_name = String::new();
        print!("\tLast name []: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut last_name).unwrap();
        let last_name = last_name.trim();

        let mut first_name = String::new();
        print!("\tFirst name []: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut first_name).unwrap();
        let first_name = first_name.trim();

        let mut groups_input = String::new();
        print!("\tGroups [] (comma separated): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut groups_input).unwrap();
        let groups: Vec<&str> = groups_input
            .trim()
            .split(',')
            .map(|s| s.trim()).filter(|s| !s.is_empty())
            .collect();

        let mut password: String;
        match read_password() {
            Ok(pw) => password = pw.clone(),
            Err(err) => eprintln!("domain-adduser: error reading password. {:#?}", err)
        };

        println!("Collected user information: ");
        println!("User common name: {}", username);
        println!("Last name: {}", last_name);
        println!("First name: {}", first_name);
        for group in groups {
            println!("Member of: {}", group);
        }
    }
}
