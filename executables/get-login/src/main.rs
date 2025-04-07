use std::env;
use std::{fs::OpenOptions, process::exit};
use std::io::prelude::*;

use fp_tools::db::rget_login::filter_by_class;
use fp_tools::{
    config::generator_config::read_config_from_file,
    db::rget_login::{
        generate_connection_string,
        get_db_connection, get_login_data,
        MySQLDomainUser},
    debug_println
};

fn main() {
    // Standard CSV format
    const CSV_HEADER: &'static str = "login;cognome;nome;gruppo;classe;CF;password;";

    // Command line arguments
    let args: Vec<String> = env::args().collect();

    // Check the number of arguments

    // Load SQL configuration from file
    let sql_config = read_config_from_file("/tmp/ad_config.json");

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
    for class in classes {
        debug_println!("{}", class);
    }

    // Print all users (debug)
    for user in users {
        debug_println!("{:#?}", user)
    }

    // Create files for each class

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
    if let Err(e) = writeln!(file, "{}", header) {
        eprintln!("get-login: cannot write to file {}", &file_path);
        exit(1)
    }

    // For each user in the class, write it
    let users = filter_by_class(users, class);

    for user in users {
        if let Err(e) = writeln!(
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
