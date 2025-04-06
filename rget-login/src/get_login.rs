use fp_tools::config::generator_config::{self, GeneratorConfig, DEFAULT_SQL_PORT};
use mysql::*;
use mysql::prelude::*;

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


