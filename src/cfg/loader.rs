use std::fs;
use std::fmt;
use anyhow::{Context,Result};

use super::spec::{
    Spec,
    Otto,
    Task,
    Param,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Loader {
}

impl Loader {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn load(&self, filename: &str) -> Result<Otto, anyhow::Error> {
        let content = fs::read_to_string(filename).context(format!("Can't load filename={:?}", filename))?;
        let spec: Spec = serde_yaml::from_str(&content).context(format!("Can't load content={:?}", content))?;
        Ok(spec.otto)
    }
}
