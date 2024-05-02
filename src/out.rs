//! How to print OS info

// TODO: Move print code and functions from main to here

use std::{
    sync::Arc,
    collections::HashMap
};
use dlopen::wrapper::Container;
use tokio::task::JoinSet;
use os_info::Type;
use crate::{
    cfg::Config,
    plugin::Plugin
};

/// Supported distros
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Distro {
    NixOS,
    Fedora
}

/// Map known distros to os_info crate's OS type
const OS_MAP: [(Type, Distro); 2] = [
    (Type::NixOS, Distro::NixOS),
    (Type::Fedora, Distro::Fedora)
];

/// Map known distros to their logos
const LOGO_MAP: [(Distro, &'static str); 2] = [
    (Distro::NixOS, include_str!("../distro-logos/nixos.txt")),
    (Distro::Fedora, include_str!("../distro-logos/fedora.txt"))
];

/// Types of things to output
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum OutputType {
    /// Draw the distro logo
    Distro,

    /// Print OS in the format: DistroName Version (Nickname) Architecture
    OS
}

/// Replace the color name in a logo text with the unicode color escape
const COLORS: [(&'static str, &'static str); 16] = [
    ("NBLK",    "\x1B[30m"), // "Normal" Black
    ("NRED",    "\x1B[31m"), // Red
    ("NGRN",    "\x1B[32m"), // Green
    ("NYLW",    "\x1B[33m"), // Yellow
    ("NBLU",    "\x1B[34m"), // Blue
    ("NMAG",    "\x1B[35m"), // Magenta
    ("NCYN",    "\x1B[36m"), // Cyan
    ("NWHT",    "\x1B[37m"), // White

    ("BBLK",    "\x1B[30m"), // "Bright" Black
    ("BRED",    "\x1B[31m"), // Bright Red
    ("BGRN",    "\x1B[32m"),
    ("BYLW",    "\x1B[33m"),
    ("BBLU",    "\x1B[34m"),
    ("BMAG",    "\x1B[35m"),
    ("BCYN",    "\x1B[36m"),
    ("BWHT",    "\x1B[37m"),
];

pub async fn out_lines(cfg: &Config, plugins: &Vec<Arc<Container<Plugin>>>) -> Vec<String> {
// Asynchronously collect os information
    let mut set = JoinSet::new();
    if cfg.show_distro {
        set.spawn(distro_logo());
    }
    if cfg.show_os {
        set.spawn(os());
    }
    let mut plugin_set = JoinSet::new();
    for plugin in plugins.iter() {
        plugin_set.spawn(plugin_output(plugin.clone()));
    }

    // Collect information
    let mut outputs = HashMap::new();
    while let Some(result) = set.join_next().await {
        match result {
            Err(_) => {},
            Ok((output_type, value)) => { outputs.insert(output_type, value); }
        }
    }
    let mut plugin_outputs = Vec::new();
    while let Some(result) = plugin_set.join_next().await {
        match result {
            Err(_) => {},
            Ok(value) => { plugin_outputs.push(value); }
        }
    }

    // Create a buffer with all the built-in outputs
    let mut lines = Vec::new();
    let mut line_ind = 0;
    if cfg.show_distro {
        // If drawing the distro, seed lines with data
        let distro_lines = outputs[&OutputType::Distro].lines()
            .map(|s| format!("{}{}", s, COLORS[7].1))
            .collect::<Vec<_>>();
        lines = distro_lines;
    }
    if cfg.show_os {
        // You'll see this pattern throughout
        // We may or may not have the lines seeded with the logo data
        // If we do, print to the right, else just push a new line
        if line_ind < lines.len() {
            lines[line_ind].push_str(outputs[&OutputType::OS].as_str());
        } else {
            lines.push(outputs[&OutputType::OS].clone());
        }
    }

    // Then add plugin outputs
    for plugin_output in plugin_outputs.iter() {
        if line_ind < lines.len() {
            lines[line_ind].push_str(plugin_output.as_str());
        } else {
            lines.push(plugin_output.clone());
        }
        line_ind += 1;
    }

    lines
}

async fn distro_logo() -> (OutputType, String) {
    let oss = HashMap::from(OS_MAP);
    let logos = HashMap::from(LOGO_MAP);
    let colors = HashMap::from(COLORS);

    let os = os_info::get().os_type();
    let distro = oss[&os];
    let mut logo = logos[&distro].to_string().clone();
    for (c_name, c_code) in colors.iter() {
        logo = logo.replace(*c_name, c_code);
    }
    (OutputType::Distro, logo)
}

async fn os() -> (OutputType, String) {
    let os = os_info::get();
    (
        OutputType::OS,
        format!(
            "OS: {} {} ({}) {}",
            os.os_type(), os.version(),
            match os.edition() {
                None => "Unknown Edition",
                Some(edition) => edition
            }, match os.architecture() {
                None => "Unknown Architecture",
                Some(architecture) => architecture
            }
        )
    )
}

async fn plugin_output(plugin: Arc<Container<Plugin>>) -> String {
    plugin.output()
}


