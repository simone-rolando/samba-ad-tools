use std::process;
use crate::config::tools_config::ToolsConfiguration;
use crate::commands::common;

const SRV_DELIM: &'static str = "\\\\";
const PATH_DELIM: &'static str = "\\";

///
/// Temporary user data
/// 
#[derive(Clone, PartialEq, Eq)]
pub struct DomainUser {
    pub common_name: String,
    pub last_name: String,
    pub first_name: String,
    pub groups: Vec<String>,
    pub password: String
}

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

    eprintln!("{:?}", result.err());
    process::exit(1)
}

///
/// Add a domain user to the current Samba domain using 'samba-tool'
/// 
/// Arguments:
/// * `config`: system configuration
/// * `user`: domain user data
/// 
/// Returns:
/// * `true` if successful, `false` otherwise
/// 
pub fn add_user(config: &ToolsConfiguration, user: &DomainUser) -> bool {

    let last_name = format!("--surname=\"{}\"", &user.last_name);
    let first_name = format!("--given-name=\"{}\"", &user.first_name);

    let share = format!(
        "--home-directory=\"{}{}{}{}{}{}\"",
        SRV_DELIM,
        &config.srv_name,
        PATH_DELIM,
        &config.home_dirs_share,
        PATH_DELIM,
        &user.common_name
    );

    let profile = format!(
        "--profile-path=\"{}{}{}{}{}{}{}.profiles{}{}",
        SRV_DELIM,
        &config.srv_name,
        PATH_DELIM,
        &config.home_dirs_share,
        PATH_DELIM,
        &user.common_name,
        PATH_DELIM,
        PATH_DELIM,
        &user.common_name
    );

    let result = common::run_command_with_output(
        &config.samba_path,
    &[
        "user",
        "create",
        &format!("\"{}\"", &user.common_name),
        &user.password,
        "--use-username-as-cn",
        &last_name,
        &first_name,
        "--home-drive:H:",
        &share,
        &profile
    ]);

    if let Ok(_) = result {
        return true;
    }

    eprintln!("domain-adduser: {:?}", result.err());
    process::exit(1)
}

///
/// Changes a user password
/// 
/// Arguments:
/// * `config`: system configuration
/// * `username`: domain user common name
/// * `password`: new password
/// 
/// Returns:
/// * `true` if successful, exit on error
/// 
pub fn change_password(config: &ToolsConfiguration, username: &String, password: &String) -> bool {
    let result = common::run_command_with_output(
        &config.samba_path,
        &[
            "user",
            "setpassword",
            &format!("--newpassword=\"{}\"", password),
            &format!("\"{}\"", username)
        ]
    );

    if let Ok(_) = result {
        return true;
    }

    eprintln!("{:?}", result.err());
    process::exit(1)
}

///
/// Gets the list of groups associated with this user
/// 
/// Arguments:
/// * `config`: system configuration
/// * `username`: domain user common name
/// 
/// Returns:
/// * `true` if successful, exit on error
/// 
pub fn get_groups(config:& ToolsConfiguration, username: &String) -> Vec<String> {
    let result = common::run_command_with_output(
        &config.samba_path,
        &[
            "user",
            "getgroups",
            &format!("\"{}\"", username)
        ]
    );

    if let Ok(result) = result {
        return result;
    }

    eprintln!("{:?}", result.err());
    process::exit(1)
}
