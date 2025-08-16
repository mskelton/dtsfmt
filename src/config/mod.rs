use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use typed_builder::TypedBuilder;

use crate::layouts::KeyboardLayoutType;

mod constants;

#[derive(Default, Deserialize, TypedBuilder)]
pub struct Config {
    #[builder(default)]
    #[serde(default)]
    pub layout: KeyboardLayoutType,

    #[builder(default_code = "Config::default_indent_str()")]
    #[serde(default = "Config::default_indent_str")]
    pub indent_str: String,

    #[builder(default)]
    #[serde(default)]
    pub warn_on_unhandled_tokens: bool,
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

    pub fn default_indent_str() -> String {
        "  ".to_owned()
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
