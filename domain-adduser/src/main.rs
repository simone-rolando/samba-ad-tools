use ldap_adaptor::{ldap_config, ldif::{generate_adduser_ldif, User}};

fn main() {
    let config = ldap_config::load_config_from_file("/etc/ad/settings.toml");
    ldap_config::print_config(&config);

    let user = User {
        username: "testuser".to_string(),
        first_name: "Test User".to_string(),
        last_name: "LDAP User".to_string(),
        groups: [
            "1a".to_string(),
            "alunni".to_string(),
        ]
    };

    let ldif = generate_adduser_ldif(&user, &config);

    println!("\nGenerated LDIF:\n{}", ldif);
}