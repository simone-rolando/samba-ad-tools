use serde::Deserialize;

///
/// Tools configuration data
/// 
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ToolsConfiguration {
    #[serde(rename = "sambaPath")] 
    pub samba_path: String,

    #[serde(rename = "srvName")]
    pub srv_name: String,

    #[serde(rename = "homeDirsPath")]
    pub home_dirs_path: String,

    #[serde(rename = "homeDirsShare")]
    pub home_dirs_share: String,

    #[serde(rename = "domainFqdn")]
    pub domain_fqdn: String,

    #[serde(rename = "ntDomainName")]
    pub nt_domain_name: String,

    #[serde(rename = "poolPath")]
    pub pool_path: String,

    #[serde(rename = "poolShare")]
    pub pool_share: String,

    #[serde(rename = "poolOwner")]
    pub pool_owner: String,

    #[serde(rename = "winbindSeparator")]
    pub winbind_separator: String
}

///
/// Load configuration from file for all tools
/// 
/// Arguments:
/// * `file_path`: file path string
/// 
/// Returns:
/// * `Option<ToolsConfiguration>` with all configurations
/// 
pub fn read_config_from_file(file_path: &str) -> Option<ToolsConfiguration> {
    let content = read_file(&file_path.to_string());

    if let Some(content) = content {
        let config = read_config(&content);
        return config;
    }

    None
}

fn read_config(config_text: &String) -> Option<ToolsConfiguration> {
    let config: Result<ToolsConfiguration, serde_json::Error> = serde_json::from_str(&config_text);

    if config.is_ok() {
        return Some(config.unwrap());
    }

    None
}

fn read_file(file_path: &String) -> Option<String> {
    let contents = std::fs::read_to_string(&file_path);

    if contents.is_ok() {
        return Some(contents.unwrap());
    }

    None
}