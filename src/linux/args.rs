#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::{env, fs, path::PathBuf};
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_args(log_path: &PathBuf) {
    let args: Vec<String> = env::args().collect();

    if args.len() !=2 {
        eprintln!("Expected 1 argument. Available arguments: run, logs");
        std::process::exit(1);
    }
    match args[1].as_str() {
        "run" => println!("Started..."),
        "logs" => {
            print_logs(log_path);
        }
        _ => {
            eprintln!("Unknown argument. Accepted arguments: run, logs");
            std::process::exit(1)
        }
    };
}
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn print_logs(log_path: &PathBuf) {
    let content = fs::read_to_string(log_path);
    match content {
        Ok(content) => {
            println!("Log file path:{}\n{}",&log_path.to_str().unwrap(), content);
            std::process::exit(0)
        }
        Err(e) => {
            panic!("Failed to read logs: {}", e)
        }
    }
}
