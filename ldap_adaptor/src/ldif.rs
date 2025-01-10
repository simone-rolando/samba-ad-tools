use std::collections::HashMap;

use crate::tools;

#[derive(Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub groups: [String; 2],
}

/// Default user settings
const USER_OBJECT_CLASS: &'static str = 
r"objectClass: top
objectClass: person
objectClass: organizationalPerson
objectClass: user";

pub fn generate_adduser_ldif(user: String, first_name: String, last_name: String, domain: String) -> String {
    let mut ldif = String::new();

    // Find DC domain
    let dc_domain = tools::get_domain_dc_from_fqdn(domain);

    // Generate DN string
    let dn = format!("dn: {},CN=Users,{}", user, dc_domain);

    // CN is username
    let cn = user.clone();

    "".to_string()
}