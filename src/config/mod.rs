use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::layouts::KeyboardLayoutType;
use serde::Deserialize;

mod constants;

#[derive(Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub layout: KeyboardLayoutType,
}

impl Config {
    pub fn parse(cwd: &Path) -> Self {
        if let Some(rc_file) = find_rc_file(cwd) {
            let buf = fs::read_to_string(rc_file)
                .expect("Failed to read config file");
            toml::from_str(&buf).expect("Failed to parse config file")
        } else {
            Self::default()
        }
    }
}

fn find_rc_file(path: &Path) -> Option<PathBuf> {
    let mut path: PathBuf = path.into();
    let file = Path::new(constants::RC_FILENAME);

    // Remove filename if it exists. This happens if the user specifies a path
    // to a single file.
    if path.is_file() {
        path.pop();
    }

    loop {
        path.push(file);

        // If the path exists, we've found it!
        if path.is_file() {
            break Some(path);
        }

        // remove file && remove parent
        if !(path.pop() && path.pop()) {
            break None;
        }
    }
}
