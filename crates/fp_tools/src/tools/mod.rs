use sudo::RunningAs;


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
