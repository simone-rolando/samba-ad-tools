use serde::Deserialize;
use sudo::RunningAs;
use crate::commands::user::DomainUser;
use csv::ReaderBuilder;

///
/// Raw user structure
///
#[derive(Deserialize, Debug)]
struct RawUser {
    pub login: String,
    pub last_name: String,
    pub first_name: String,
    pub groups: String,
    pub class: String,
    pub cf: String,
    pub password: String
}

#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            println!($($arg)*);
        }
    }
}

///
/// Returns if the process is running in privileged mode.
/// 
/// Returns:
/// * `true` if process is privileged, `false` otherwise
/// 
pub fn has_privileges() -> bool {
    let result = sudo::check();
    result == RunningAs::Suid || result == RunningAs::Root
}

///
/// Reads a login CSV file into a vector of Domain Users
///
/// Arguments:
/// * `file_path`: `&str` value with complete file path
///
/// Returns:
/// * a `Vec<DomainUser>` with all the domain users to process
///
pub fn read_login_csv(file_path: &str) -> Vec<DomainUser> {
    let reader = ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_path(file_path);

    if let Ok(mut reader) = reader {
        let mut users = Vec::new();

        for raw_user in reader.deserialize::<RawUser>() {
            let raw_user = raw_user.expect("Could not read user");

            // Retrieve groups from raw string
            let mut groups = raw_user.groups
                .split(',')
                .map(|s| s.trim().to_owned())
                .collect::<Vec<String>>();

            // If there is a class, add it to the groups
            if !raw_user.class.is_empty() {
                groups.push(raw_user.class.clone());
            }

            users.push(DomainUser {
                common_name: raw_user.login.clone(),
                last_name: raw_user.last_name.clone(),
                first_name: raw_user.last_name.clone(),
                groups: groups.clone(),
                password:  raw_user.password.clone()
            });
        }

        return users;
    }

    Vec::new()
}
