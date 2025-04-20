use crate::config::tools_config::ToolsConfiguration;

use super::common;

///
/// Check if the group exists in the Samba domain
/// 
/// Arguments:
/// * `config`: system configuration
/// * `group`: group to find
/// 
/// Returns:
/// * a boolean value - `true` if the group exists, `false` otherwise
/// 
pub fn is_existing_group(config: &ToolsConfiguration, group: &String) -> bool {
    let result = common::run_command_with_output(
        &config.samba_path, 
        &[
            "group",
            "list"
        ]);

    if let Ok(result) = result {
        return result.contains(group);
    }

    eprintln!("{:?}", result.err());

    false
}