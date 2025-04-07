use crate::config::generator_config::{GeneratorConfig, DEFAULT_SQL_PORT};
use std::str::{self, Bytes};
use mysql::prelude::*;
use mysql::*;

///
/// MySQL database domain user
/// 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MySQLDomainUser {
    pub login: String,
    pub last_name: String,
    pub first_name: String,
    pub class: String,
    pub password: String,
    pub tax_code: String,
    pub group: String,
    pub birth_date: String,
    pub date_modified: Option<String>
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

pub fn get_login_data(conn: &mut PooledConn) -> Vec<MySQLDomainUser> {
    let users: Vec<MySQLDomainUser>;

    const QUERY: &str = "SELECT login, cognome, nome, classe, password, CF, gruppo, data_nascita, data_modifica FROM ALUNNO";

    // Query and get each row as a Vec<Value>
    let result = conn.query_map(
        QUERY,
        |(login, cognome, nome, classe, password, cf, gruppo, nascita, modifica): (
            String, String, String, String, String, String, String, Value, Value
        )| {
            let birth_date = match nascita {
                Value::Date(y, m, d, _, _, _, _) => format!("{:04}-{:02}-{:02}", y, m, d),
                Value::Bytes(v) => String::from_utf8(v).expect("Valid date data"),
                _ => "NULL".to_string()
            };

            let date_modified = match modifica {
                Value::Date(y, m, d, _, _, _, _) => Some(format!("{:04}-{:02}-{:02}", y, m, d)),
                Value::Bytes(vx) => Some(String::from_utf8(vx).expect("Valid date data")),
                _ => None
            };

            MySQLDomainUser {
                login,
                last_name: cognome,
                first_name: nome,
                class: classe,
                password,
                tax_code: cf,
                group: gruppo,
                birth_date,
                date_modified,
            }
        },
    );

    if result.is_err() {
        return Vec::new();
    }

    users = result.unwrap();
    users
}
