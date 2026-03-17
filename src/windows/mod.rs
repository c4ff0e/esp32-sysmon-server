#[cfg(target_os = "windows")]
pub mod tray;

#[cfg(target_os = "linux")]
pub mod linux;
