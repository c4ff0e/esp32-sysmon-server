use simplelog::*;
use std::{fs::{File, create_dir_all}, path::PathBuf};

use directories::{ProjectDirs};

pub fn log_dir() -> Result<PathBuf, std::io::Error> {
    let project_dir = match ProjectDirs::from("com", "c4ff0e", "esp32-sysmon-server") {
        Some(project_dir) => project_dir,
        None => {
            panic!("Failed to get project directory!")
        }
    };
    let log_dir = project_dir.data_dir().to_path_buf();
    match create_dir_all(&log_dir){
        Ok(_) => {
            return Ok(log_dir)
        }
        Err(e) => {
            return Err(e)
        }
    }
}

//WriteLogger::init(LevelFilter::Info, Config::default(), File::create(log_dir).unwrap());
