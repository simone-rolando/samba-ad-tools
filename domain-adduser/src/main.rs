use ldap_adaptor::{config, ldif::{generate_add_member_ldif, generate_adduser_ldif, generate_sam_ldif, Groups, User}};

fn main() {
    let config = config::load_config_from_file("/etc/ad/settings.toml");
    config::print_config(&config);

    let user = User {
        username: "testuser".to_string(),
        first_name: "Test User".to_string(),
        last_name: "LDAP User".to_string(),
        groups: Groups {
            first_group: "1a".to_string(),
            second_group: "alunni".to_string()
        }
    };

    let ldif = generate_adduser_ldif(&user, &config);
    let sam_ldif = generate_sam_ldif(&user, &config);
    let chgrp_ldif = generate_add_member_ldif(&user, &config, &"Domain Admins".to_string());

    println!("\n{}", ldif);
    println!("\n{}", sam_ldif);
    println!("\n{}", chgrp_ldif);
}