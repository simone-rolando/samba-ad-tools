use base64::encode;

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

///
/// Returns the Base 64 UTF-16 LE encoded password from plain text
/// 
/// Arguments:
/// * `clear_text`: clear text password string
/// 
/// Returns:
/// * UTF-16 LE Base64 encoded password as string
pub fn get_base64_password(clear_text: &String) -> String {
    let pwd = format!("\"{}\"", clear_text);

    // Encode password in UTF-16 base 16 little-endian
    let utf16_pwd: Vec<u8> = pwd.encode_utf16()
        .flat_map(|x| x.to_le_bytes())
        .collect();

    // Encode the UTF-16 LE byte buffer in Base64
    let base64_pwd = encode(&utf16_pwd);

    base64_pwd
}