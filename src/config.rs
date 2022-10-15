use std::{
    fmt::{self, Display, Formatter},
    fs,
    io::ErrorKind,
};

use serde_derive::{Deserialize, Serialize};
use toml::de::Error as SyntaxError;

const CFG_PATH: &str = "shina.ini";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub patches: Patches,
    pub experimental: Option<Experimental>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize)]
pub struct Patches {
    pub fix_bsl: bool,
    pub two_guns: bool,
    pub keep_guns: bool,
    pub manual_reload: bool,
    pub hypo_anytime: bool,
    pub unlock_doors: bool,
    pub always_dev: bool,
    pub fast_cutscenes: bool,
    pub no_black_bars: bool,
    pub shut_up: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Experimental {
    pub three_guns: Option<bool>,
}

impl Config {
    pub fn load() -> Result<Self, LoadError> {
        let source = fs::read_to_string(CFG_PATH).map_err(|err| {
            if matches!(err.kind(), ErrorKind::NotFound) {
                LoadError::NotFound
            } else {
                LoadError::InputOutput(err)
            }
        })?;
        toml::from_str(&source).map_err(LoadError::Deserialize)
    }

    pub fn validate(&mut self) -> Result<(), ValidateError> {
        if let Some(e) = &self.experimental {
            if e.three_guns == Some(true) && !self.patches.two_guns {
                return Err(ValidateError::Requires("three_guns", "two_guns"));
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum LoadError {
    NotFound,
    InputOutput(std::io::Error),
    Deserialize(SyntaxError),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "Configuration file not found"),
            Self::InputOutput(err) => write!(f, "I/O error: {}", err),
            Self::Deserialize(err) => write!(f, "Syntax error: {}", err),
        }
    }
}

impl std::error::Error for LoadError {}

#[derive(Debug)]
pub enum ValidateError {
    Requires(&'static str, &'static str),
}

impl Display for ValidateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requires(a, b) => write!(f, "Patch {:?} requires patch {:?}", a, b),
        }
    }
}

impl std::error::Error for ValidateError {}
