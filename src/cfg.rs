//! Read in a config file

use std::{
    fs::{create_dir_all, File},
    io::{BufReader, BufWriter},
    path::PathBuf
};
use serde::{
    Serialize, Deserialize
};

const CONF_SUBDIR: &'static str = "fetch-rs";
const CONF_SUBDIR_FNAME: &'static str = "config.json";

/// Represent all the options to configure info displayed to the user
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Config {
    /// If true and the program can identify the current distro, it will display a graphic.
    /// Otherwise it will show nothing
    pub show_distro: bool,

    /// If true, show OS information if possible
    pub show_os: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_distro: true,
            show_os: true
        }
    }
}

impl Config {
    fn from_file(fname: &PathBuf) -> Self {
        let file = File::open(fname);
        if file.is_err() {
            // Couldn't find config. Use default and be silent
            return Config::default();
        }
        let file = file.unwrap();
        let reader = BufReader::new(&file);
        match serde_json::from_reader(reader) {
            Err(e) => {
                // Configured incorrectly. Worth not being silent over
                eprintln!("Config file is incorrect: {}. Using default", e.to_string());
                Config::default()
            }, Ok(cfg) => cfg
        }
    }

    fn to_file(&self, fname: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(fname)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self)?;
        Ok(())
    }
}

fn config_file_path() -> PathBuf {
    // Create the directory if it doesn't exist
    let mut conf = dirs::config_dir()
        .expect("Err: Your system contains no config directory!");
    conf.push(CONF_SUBDIR);
    match create_dir_all(conf.clone()) {
        Err(_) => {}, // Ignore. It's okay. There are other checks for no folder/file
        Ok(_) => {}
    }
    conf.push(CONF_SUBDIR_FNAME);
    conf
}

pub fn load_config() -> Config {
    let path = config_file_path();
    let cfg = Config::from_file(&path);
    
    // In case it's default, let's try to resave it
    match cfg.to_file(&path) {
        Err(_) => {}, // Ignore errors
        Ok(_) => {}
    }

    cfg
}

