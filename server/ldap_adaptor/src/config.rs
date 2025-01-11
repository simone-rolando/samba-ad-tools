use serde::Deserialize;
use std::fs;

///
/// LDAP configuration structure
/// 
#[derive(Deserialize, Debug)]
pub struct LdapConfig {
    pub ad_domain: String,
    pub server_fqdn: String,
    pub home_dirs_path: String,
    pub home_dirs_share: String,
    pub home_drive_letter: String,
    pub nt_domain_name: String,
    pub winbind_separator: String
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

///
/// Print current configuration
/// 
pub fn print_config(config: &LdapConfig) {
    println!("AD domain: {}", config.ad_domain);
    println!("Server FQDN: {}", config.server_fqdn);
    println!("Home directories: {}", config.home_dirs_path);
    println!("NT Domain name: {}", config.nt_domain_name);
    println!("Winbind separator: {}", config.winbind_separator);
}
