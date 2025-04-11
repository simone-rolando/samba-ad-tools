use crate::config::tools_config::ToolsConfiguration;
use crate::commands::common;

///
/// Check if a user exists in the samba domain
/// 
/// Arguments:
/// * `config`: system configuration
/// * `username`: username to find
/// 
/// Returns:
/// * a boolean value - `true` if the user exists, `false` otherwise
/// 
pub fn is_existing_user(config: &ToolsConfiguration, username: &String) -> bool {
    let result = common::run_command_with_output(
        &config.samba_path,
        &[
            "user",
            "list"
        ]
    );

    if let Ok(result) = result {
        return result.contains(username);
    }

    false
}