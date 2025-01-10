
///
/// Retrieve LDAP format domain name from FQDN
/// 
/// Arguments:
/// * `domain_fqdn`: domain FQDN format
/// 
/// Returns:
/// * string with LDAP-format domain
/// 
pub fn get_domain_dc_from_fqdn(domain_fqdn: &String) -> String {
    domain_fqdn
        .split('.')
        .map(|part| format!("DC={}", part))
        .collect::<Vec<String>>()
        .join(",")
}