use std::{fs::OpenOptions, process::exit};
use std::io::prelude::*;
use std::env::current_dir;

use chrono::Datelike;
use fp_tools::db::rget_login::filter_by_class;
use fp_tools::{
    config::generator_config::read_config_from_file,
    db::rget_login::{
        generate_connection_string,
        get_db_connection, get_login_data,
        MySQLDomainUser},
    debug_println
};

use clap::Parser;

/// 
/// Command line arguments
/// 
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output directory
    #[arg(short, long)]
    output_dir: Option<String>,

    /// File name prefix
    #[arg(short, long)]
    file_prefix: Option<String>
}

///
/// Output paths struct
/// 
struct Paths {
    out_dir: String,
    prefix: String
}

fn main() {
    // Standard CSV format
    const CSV_HEADER: &'static str = "login;cognome;nome;gruppo;classe;CF;password;";

    // Command line arguments
    let cli = Args::parse();

    // Get correct paths
    let paths = get_paths(cli.output_dir, cli.file_prefix);

    // Load SQL configuration from file
    let sql_config = read_config_from_file("/etc/ad/mysql_config.json");

    // Check for SQL configuration loading
    if sql_config.is_none() {
        eprintln!("get-login: error loading MySQL configuration data");
        exit(1)
    }

    // Get an SQL connection
    let conn_str = generate_connection_string(&sql_config.unwrap());
    let connection = get_db_connection(&conn_str);

    // Check the SQL connection
    if connection.is_none() {
        eprintln!("get-login: error getting a MySQL connection to the server...");
        exit(1)
    }

    // Load all users from database
    let mut users = get_login_data(&mut connection.unwrap());

    // Sort by class
    users.sort_by(|a, b| a.class.cmp(&b.class));

    // Get all classes
    let classes = get_classes(&users);

    // Print all classes (debug)
    for class in &classes {
        debug_println!("{}", class);
    }

    // Print all users (debug)
    for user in &users {
        debug_println!("{:#?}", user)
    }

    // Create files for each class
    for class in classes {
        write_class_file(CSV_HEADER, &users, &class, &paths.out_dir, &paths.prefix);
    }
}

///
/// Get all classes as strings
/// 
fn get_classes(users: &Vec<MySQLDomainUser>) -> Vec<String> {
    let mut classes: Vec<String> = users
        .iter()
        .map(|el| el.class.clone())
        .collect();

    classes.dedup();

    classes
}

///
/// Return the correct paths based on input values
/// 
fn get_paths(out_dir: Option<String>, prefix: Option<String>) -> Paths {
    let mut paths = Paths { out_dir: "".to_string(), prefix: "".to_string() };

    if let Some(out_dir) = out_dir {
        paths.out_dir = out_dir.clone();
    } else {
        let cwd_buf = current_dir().expect("get-login: cannot read current working directory");
        paths.out_dir = cwd_buf.into_os_string().into_string().expect("get-login: cannot read path buffer").clone();
    }

    if let Some(prefix) = prefix {
        paths.prefix = prefix;
    } else {
        let current_date = chrono::Utc::now();
        let year = current_date.year();
        paths.prefix = "p".to_string() + &year.to_string();
    }

    paths
}

/// 
/// Write file for the given class
/// 
fn write_class_file(
    header: &str, 
    users: &Vec<MySQLDomainUser>, 
    class: &String,
    directory: &String, 
    prefix: &String) {

    // Create the class file
    let file_path = format!("{}/{}_{}.csv", directory, prefix, class);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();

    // Write the header
    if let Err(_) = writeln!(file, "{}", header) {
        eprintln!("get-login: cannot write to file {}", &file_path);
        exit(1)
    }

    // For each user in the class, write it
    let users = filter_by_class(users, class);

    for user in users {
        if let Err(_e) = writeln!(
            file, "{};{};{};{};{};{};{};",
            user.login,
            user.last_name,
            user.first_name,
            user.group,
            user.class,
            "CF",
            user.password
        ) {
            eprintln!("get-login: cannot write to file {}", &file_path);
            exit(1)
        }
    }
}