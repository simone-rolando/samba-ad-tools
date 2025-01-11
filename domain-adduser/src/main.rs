use ldap_adaptor::{config, group_ldif::{generate_addgroup_ldif, Groups}, user_ldif::{generate_add_member_ldif, generate_adduser_ldif, generate_deluser_ldif, generate_sam_ldif, generate_setpasswd_ldif, User}};

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

    let addgroup_ldif = generate_addgroup_ldif(&"Prova".to_string(), &config);
    let pwd_ldif = generate_setpasswd_ldif(&user, &config, &"lillovilli".to_string());

    let del_ldif = generate_deluser_ldif(&user, &config);

    println!("\n{}", ldif);
    println!("\n{}", sam_ldif);
    println!("\n{}", chgrp_ldif);
    println!("\n{}", addgroup_ldif);
    println!("\n{}", pwd_ldif);
    println!("\n{}", del_ldif);
}