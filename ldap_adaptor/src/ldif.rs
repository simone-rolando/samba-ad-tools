use crate::{config::LdapConfig, tools};

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub groups: [String; 2],
}

///
/// Generates a user LDIF given its user properties and domain configuration.
/// This generates LDIF text to add a user to the domain.
/// 
/// Arguments:
/// * `user`: immutable reference to User
/// * `config`: immutable reference to LdapConfig
/// 
/// Returns:
/// * a string containing the LDIF generated text
pub fn generate_adduser_ldif(user: &User, config: &LdapConfig) -> String {
    // Find DC domain
    let dc_domain = tools::get_domain_dc_from_fqdn(&config.ad_domain);

    // Generate DN string
    let dn = format!("{},CN=Users,{}", &user.username, dc_domain);

    // CN is username
    let cn = user.username.clone();

    // Home directory
    let home = format!("{}\\{}", &config.home_dirs_share, &user.username);

    // Profiles directory
    let profiles = format!("{}\\.profiles\\{}", &home, &user.username);

    // Display name
    let display_name = format!("{} {}", &user.first_name, &user.last_name);

    format!(
r"dn: {}
objectClass: top
objectClass: person
objectClass: organizationalPerson
objectClass: user
cn: {}
sn: {}
givenName: {}
instanceType: 4
name: {}
codePage: 0
countryCode: 0
homeDirectory: {}
homeDrive: {}
profilePath: {}
objectCategory: CN=Person,CN=Schema,CN=Configuration,{}
displayName: {}
distinguishedName: {}",
    dn,
    &cn,
    &user.last_name,
    &user.first_name,
    &cn,
    &home,
    &config.home_drive_letter,
    &profiles,
    &dc_domain,
    &display_name,
    &dn
    )
}