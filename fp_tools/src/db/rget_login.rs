use crate::config::generator_config::{GeneratorConfig, DEFAULT_SQL_PORT};
use mysql::*;
use mysql::prelude::*;

///
/// MySQL database domain user
/// 
pub struct MySQLDomainUser {
    pub login: String,
    pub last_name: String,
    pub first_name: String,
    pub class: String,
    pub password: String,
    pub tax_code: String,
    pub group: String,
    pub birth_date: String,
    pub date_modified: String
}

///
/// Generate a new connection string from config data
/// 
/// Arguments:
/// * `config`: GeneratorConfig struct with parsed configuration data
pub fn generate_connection_string(config: &GeneratorConfig) -> String {
    format!(
        "mysql://{}:{}@{}:{}/{}",
        config.get_db_user(),
        config.get_db_pass(),
        config.get_db_host(),
        DEFAULT_SQL_PORT,
        config.get_db_name())
}

///
/// Create a database connection with the DB server
/// 
/// Arguments:
/// * `conn_string`: connection URL string generated with the proper function
pub fn get_db_connection(conn_string: &String) -> Option<PooledConn> {
    // Get connection pool from SQL API
    let url: &str = &conn_string;
    let pool = Pool::new(url);

    // Check for pool creation
    if pool.is_err() {
        return None
    }

    // Get connection from pool and check errors
    let conn = pool.unwrap().get_conn();

    if conn.is_err() {
        eprintln!("{}", conn.err().unwrap());
        return None
    }

    Some(conn.unwrap())
}

pub fn get_login_data(conn: &mut PooledConn, dir_name: &String, file_name: &String) -> Vec<MySQLDomainUser> {
    let mut users = Vec::<MySQLDomainUser>::new();

    let result = conn.query_map(
        "SELECT login, cognome, nome, classe, password, CF, gruppo, data_nascita, data_modifica FROM ALUNNO",
        |(login, last_name, first_name, class, password, tax_code, group, birth_date, date_modified)| {
            MySQLDomainUser {
                login,
                last_name,
                first_name,
                class,
                password,
                tax_code,
                group,
                birth_date,
                date_modified
            }
        }
    );

    users
}
