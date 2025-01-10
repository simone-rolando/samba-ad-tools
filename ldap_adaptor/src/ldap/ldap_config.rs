use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub struct LdapConfig {
    pub ldap_domain: String,
    pub server_fqdn: String,
    pub home_dirs_path: String,
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
    println!("LDAP domain: {}", config.ldap_domain);
    println!("Server FQDN: {}", config.server_fqdn);
    println!("Home directories: {}", config.home_dirs_path);
    println!("NT Domain name: {}", config.nt_domain_name);
    println!("Winbind separator: {}", config.winbind_separator);
}
