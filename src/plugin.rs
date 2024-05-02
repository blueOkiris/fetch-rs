//! Allow users to incorporate their own functionality dynamically

use std::{
    ffi::CStr, path::PathBuf, sync::Arc
};
use dlopen::wrapper::{
    Container, WrapperApi
};
use dlopen_derive::WrapperApi;

const CONF_SUBDIR: &'static str = "fetch-rs";
const CONF_SUBSUBDIR: &'static str = "plugins";

#[derive(WrapperApi)]
pub struct Plugin {
    int_output: extern fn() -> * const i8
}

impl Plugin {
    pub fn output(&self) -> String {
        let int_output = (self.int_output)();
        let out = unsafe { CStr::from_ptr(int_output) };
        out.to_str()
            .expect("An error occurred in a plugin output function!")
            .to_string()
    }

    fn from_file(fname: PathBuf) -> Option<Arc<Container<Self>>> {
        match unsafe { Container::load(fname.to_str().unwrap()) } {
            Ok(plugin) => Some(Arc::new(plugin)),
            Err(_) => None
        }
    }
}

// Something like  ~/.config/fetch-rs/plugins
fn plugins_dir() -> PathBuf {
    let mut conf = dirs::config_dir()
        .expect("Err: Your system contains no config directory!");
    conf.push(CONF_SUBDIR);
    conf.push(CONF_SUBSUBDIR);
    conf
}

pub fn load_plugins() -> Vec<Arc<Container<Plugin>>> {
    let mut plugins = Vec::new();
    let paths = std::fs::read_dir(plugins_dir());
    if paths.is_err() {
        // If we can't read from config dir, assume no plugins to keep output clean
        return plugins;
    }
    let paths = paths.unwrap();
    for path in paths {
        match path {
            Err(_) => {}, // Ignore if we can't read something to preserve clean output
            Ok(path) => {
                match Plugin::from_file(path.path()) {
                    None => {
                        // Invalid plugin installed, so user knowing valued more than clean output
                        eprintln!("Failed to load plugin from path '{:?}'", path.path());
                    }, Some(plugin) => {
                        plugins.push(plugin);
                    }
                }
            }
        }
    }
    plugins
}

