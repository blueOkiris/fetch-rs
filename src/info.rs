//! Functions for getting information to print

use std::{
    collections::HashMap, fs::File, io::{BufReader, Read}, process::Command
};
use procfs::{Current, Uptime};

use crate::out::{
    OutputType, OS_MAP, LOGO_MAP, COLORS
};

pub async fn distro_logo() -> (OutputType, String) {
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

pub async fn os() -> (OutputType, String) {
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

pub async fn host() -> (OutputType, String) {
    if let Ok(board_vendor) = File::open("/sys/devices/virtual/dmi/id/board_vendor") {
        let mut reader = BufReader::new(board_vendor);
        let mut vendor = "".to_string();
        match reader.read_to_string(&mut vendor) {
            Ok(_) => {
                if let Ok(board_name) = File::open("/sys/devices/virtual/dmi/id/board_name") {
                    reader = BufReader::new(board_name);
                    let mut board = "".to_string();
                    match reader.read_to_string(&mut board) {
                        Ok(_) => return (
                            OutputType::Host,
                            format!(
                                "Host: {} {}",
                                &vendor[0..vendor.len() - 1],
                                &board[0..board.len() - 1]
                            )
                        ), _ => {}
                    }
                }
            }, _ => {}
        }
    }
    (OutputType::Host, "".to_string())
}

pub async fn kernel() -> (OutputType, String) {
    match Command::new("uname").arg("-r").output() {
        Ok(kernel_version) => {
            let kernel_version = String::from_utf8_lossy(&kernel_version.stdout);
            (
                OutputType::Kernel,
                format!("Kernel: {}", &kernel_version[0..kernel_version.len() - 1])
            )
        }, Err(_) => (OutputType::Kernel, "".to_string())
    }
}

pub async fn uptime() -> (OutputType, String) {
    let uptime_s = Uptime::current();
    if uptime_s.is_err() {
        return (OutputType::Uptime, "".to_string());
    }
    let uptime_s = uptime_s.unwrap().uptime as u64;

    let uptime_min = uptime_s / 60;
    let uptime_s_left = uptime_s % 60;

    let uptime_hr = uptime_min / 60;
    let uptime_min_left = uptime_min % 60;

    (
        OutputType::Uptime,
        format!("Uptime: {}hr {}m {}s", uptime_hr, uptime_min_left, uptime_s_left)
    )
}

// TODO: Finish implementing output functions

