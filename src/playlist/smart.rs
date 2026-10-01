#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmartRule {
    Resolution4kOnly,
    HdrContentOnly,
    UnwatchedOnly,
    DurationMin(u64), // in minutes
    RecentFiles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartPlaylist {
    pub name: String,
    pub rules: Vec<SmartRule>,
    pub items: Vec<PathBuf>,
}

impl SmartPlaylist {
    pub fn new(name: &str, rules: Vec<SmartRule>) -> Self {
        Self {
            name: name.to_string(),
            rules,
            items: Vec::new(),
        }
    }
}
