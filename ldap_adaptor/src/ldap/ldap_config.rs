use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct LdapConfig {
    samba_path: String,
    server_name: String,
    home_dirs_path: String,
    nt_domain_name: String,
    pool_path: String,
    pool_owner: String,
    winbind_separator: String
}

///
/// Load configuration file from settings
///
pub fn load_config_from_file(path: &str) -> LdapConfig {
    // Load the config file
    let config_content = fs::read_to_string(path)
        .expect(&format!("Failed to load configuration file {}", path));

    // Parse TOML configuration
    let config: LdapConfig = toml::from_str(&config_content)
        .expect("Failed to parse TOML file");

    config
}
