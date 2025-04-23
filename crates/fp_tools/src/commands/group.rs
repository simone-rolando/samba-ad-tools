use std::process;
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
    process::exit(1);
}

///
/// Add a new group to the Samba domain
/// 
/// Arguments:
/// * `config`: system configuration
/// * `group`: group name to add
/// 
/// Returns:
/// * boolean `true` on success, `false` otherwise
/// 
pub fn add_group(config: &ToolsConfiguration, group: &String) -> bool {
    let result = common::run_command_with_output(
        &config.samba_path,
    &[
        "group",
        "add",
        &format!("\"{}\"", group)
    ]);

    if let Ok(_) = result {
        return true;
    }

    eprintln!("{:?}", result.err());
    process::exit(1);
}

///
/// Add member to group
/// 
/// Arguments:
/// * `config`: system configuration
/// * `group`: group common name
/// * `username`: user common name
/// 
/// Returns:
/// * boolean `true` on success, `false` otherwise
/// 
pub fn add_member(config: &ToolsConfiguration, group: &String, username: &String) -> bool {
    let result = common::run_command_with_output(
        &config.samba_path,
        &[
            "group",
            "addmembers",
            &format!("\"{}\"", group),
            &format!("\"{}\"", username)
        ]
    );

    if let Ok(_) = result {
        return true;
    }

    eprintln!("{:?}", result.err());
    process::exit(1);
}

///
/// Checks the membership of a user in a specific group
/// 
/// Arguments:
/// * `config`: system configuration
/// * `group`: group name
/// * `username`: user common name
/// 
/// Returns:
/// * `true` when the user is member, `false` on non-membership or operational error
/// 
pub fn check_membership(config: &ToolsConfiguration, group: &String, username: &String) -> bool {
    let result = common::run_command_with_output(
        &config.samba_path,
        &[
            "group",
            "listmembers",
            &format!("\"{}\"", group)
        ]
    );

    if let Ok(result) = result {
        return result.contains(username);
    }

    eprintln!("{:?}", result.err());
    process::exit(1);
}