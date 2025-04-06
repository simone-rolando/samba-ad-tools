use serde::Deserialize;
use serde_json::Result;

///
/// Generator configuration data
/// 
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct GeneratorConfig {
    #[serde(rename = "databaseHost")]
    db_host: String,

    #[serde(rename = "databaseName")]
    db_name: String,

    #[serde(rename = "databaseUser")]
    db_user: String,

    #[serde(rename = "databasePass")]
    db_pass: String
}

impl GeneratorConfig {
    /// 
    /// Create a new GeneratorConfig struct
    /// 
    /// Arguments:
    /// * `db_host`: database host IP address or FQDN
    /// * `db_name`: SQL database name
    /// * `db_user`: database user for connection
    /// * `db_pass`: database user password for connection
    /// 
    pub fn new(db_host: &String, db_name: &String, db_user: &String, db_pass: &String) -> GeneratorConfig {
        GeneratorConfig {
            db_host: db_host.clone(),
            db_name: db_name.clone(),
            db_user: db_user.clone(),
            db_pass: db_pass.clone()
        }
    }

    ///
    /// Get database host IP address or FQDN
    /// 
    pub fn get_db_host(&self) -> &String {
        &self.db_host
    }

    ///
    /// Get database name
    /// 
    pub fn get_db_name(&self) -> &String {
        &self.db_name
    }

    ///
    /// Get database user
    pub fn get_db_user(&self) -> &String {
        &self.db_user
    }

    /// Get database password
    pub fn get_db_pass(&self) -> &String {
        &self.db_pass
    }
}

///
/// Read a configuration from a specific file
/// 
/// Arguments:
/// * file_path: file path to configuration
/// 
pub fn read_config_from_file(file_path: &String) -> Option<GeneratorConfig> {
    let content = read_file(file_path);

    if let Some(content) = content {
        let config = read_config(&content);
        return config;
    }

    None
}

fn read_config(config_text: &String) -> Option<GeneratorConfig> {
    let config: Result<GeneratorConfig> = serde_json::from_str(&config_text);

    if config.is_ok() {
        Some(config.unwrap())
    } else {
        None
    }
}

fn read_file(file_path: &String) -> Option<String> {
    // Read the content
    let contents = std::fs::read_to_string(&file_path);

    if contents.is_ok() {
        Some(contents.unwrap())
    } else {
        None
    }
}