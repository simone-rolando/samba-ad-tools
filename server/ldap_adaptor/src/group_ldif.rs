use crate::{config::LdapConfig, tools};

///
/// Groups struct for LDIF generation
/// 
#[derive(Clone)]
pub struct Groups {
    pub first_group: String,
    pub second_group: String
}

/// LDAP AD security group magic value
pub const SECURITY_GROUP: i32 = -2147483646;

///
/// Generates a user LDIF given its user properties and domain configuration.
/// This generates LDIF text to add a user to the domain.
/// 
/// Arguments:
/// * `new_grp`: immutable reference to group name
/// * `config`: immutable reference to LdapConfig
/// 
/// Returns:
/// * a string containing the LDIF generated text
/// 
pub fn generate_addgroup_ldif(new_grp: &String, config: &LdapConfig) -> String {
    // Find DC domain
    let dc_domain = tools::get_domain_dc_from_fqdn(&config.ad_domain);

    // Generate group DN
    let dn = format!("CN={},CN=Users,{}", new_grp, &dc_domain);

    // Generate LDIF text
    format!(
r"dn: {}
objectClass: top
objectClass: group
cn: {}
sAMAccountName: {}
groupType: {}",
    &dn,
    new_grp,
    new_grp,
    SECURITY_GROUP)
}