use std::{io, process::Command};

///
/// Run a command getting its output and splitting it into multiple lines
/// 
/// Arguments:
/// * `command`: command path string
/// * `args`: array of strings with command arguments
/// 
/// Returns:
/// * `io::Result<Vec<String>>` containing the separated lines
/// 
pub fn run_command_with_output(command: &str, args: &[&str]) -> io::Result<Vec<String>> {
    let output = Command::new(command)
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command failed with status: {}", output.status)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout
        .lines()
        .map(|line| line.trim_end().to_string())
        .collect();

    Ok(lines)
}