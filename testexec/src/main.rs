use fp_tools::{config::generator_config::read_config_from_file, db::rget_login::{generate_connection_string, get_db_connection, get_login_data}};

fn main() {
    let path = "/tmp/ad_config.json".to_string();
    let config = read_config_from_file(&path);

    let connection_info = generate_connection_string(&config.unwrap());
    println!("Connection string: {}", &connection_info);

    let mut conn = get_db_connection(&connection_info).unwrap();

    let users = get_login_data(&mut conn,);

    for user in users {
        println!("{:#?}", user);
    }
}
