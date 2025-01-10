use ldap_adaptor::ldap::ldap_config;

fn main() {
    let config = ldap_config::load_config_from_file("/etc/ad/settings.toml");
    ldap_config::print_config(&config);
}