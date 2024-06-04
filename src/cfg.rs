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
    /// Otherwise it will show nothing. Defaults to true
    pub show_distro: bool,

    /// If true, show OS information if possible. Defaults to true
    pub show_os: bool,

    /// If true, shows the host of the machine, i.e. motherboard
    pub show_host: bool,

    /// If true, show the kernel version. Defaults to true
    pub show_kernel: bool,

    /// If true, show the uptime in hours and minutes. Defaults to true
    pub show_uptime: bool,

    /// If true, show the number of packages and what package managers. Defaults to true
    pub show_packages: bool,

    /// If true, show the shell. Defaults to true
    pub show_shell: bool,

    /// If true, show the screen resolution(s). Defaults to true
    pub show_resolution: bool,

    /// If true, show the desktop environment. Defaults to true
    pub show_de: bool,

    /// If true, show the window manager. Defaults to true
    pub show_wm: bool,

    /// If true, show the window manager's theme. Defaults to true
    pub show_wm_theme: bool,

    /// If true, show the theme. Defaults to true
    pub show_theme: bool,

    /// If true, show the icon set. Defaults to true
    pub show_icons: bool,

    /// If true, show the terminal. Defaults to true
    pub show_terminal: bool,

    /// If true, show the terminal's font. Defaults to true
    pub show_terminal_font: bool,

    /// If true, show the cpu. Defaults to true
    pub show_cpu: bool,

    /// If true, show the GPU(s). Defaults to true
    pub show_gpu: bool,

    /// If true, show the memory (used / total). Defaults to true
    pub show_memory: bool,

    /// If true, show the CPU usage. Defaults to false
    pub show_cpu_usage: bool,

    /// If true, show the Disk usage. Defaults to false
    pub show_disk_usage: bool,

    /// If true, show the Battery usage. Defaults to false
    pub show_battery: bool,

    /// If true, show the font. Defaults to false
    pub show_font: bool,

    /// If true, show the song playing. Defaults to false
    pub show_song: bool,

    /// If true, show the local up. Defaults to false
    pub show_local_ip: bool,

    /// If true, show the public ip. Defaults to false
    pub show_public_ip: bool,

    /// If true, show the users. Defaults to false
    pub show_users: bool,

    /// If true, show the current user's birthday. Defaults to false
    pub show_birthday: bool,

    /// If true, show the terminal colors. Defaults to true
    pub show_colors: bool
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_distro: true,
            show_os: true,
            show_host: true,
            show_kernel: true,
            show_uptime: true,
            show_packages: true,
            show_shell: true,
            show_resolution: true,
            show_de: true,
            show_wm: true,
            show_wm_theme: true,
            show_theme: true,
            show_icons: true,
            show_terminal: true,
            show_terminal_font: true,
            show_cpu: true,
            show_gpu: true,
            show_memory: true,
            show_cpu_usage: false,
            show_disk_usage: false,
            show_battery: false,
            show_font: false,
            show_song: false,
            show_local_ip: false,
            show_public_ip: false,
            show_users: false,
            show_birthday: false,
            show_colors: true
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

