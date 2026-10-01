#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Smb,
    WebDav,
    Ftp,
    Dlna,
    UncShare,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServer {
    pub name: String,
    pub protocol: NetworkProtocol,
    pub host: String,
    pub port: u16,
    pub share_path: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl NetworkServer {
    pub fn build_url(&self) -> String {
        match self.protocol {
            NetworkProtocol::Smb => {
                if let (Some(u), Some(p)) = (&self.username, &self.password) {
                    format!("smb://{}:{}@{}:{}/{}", u, p, self.host, self.port, self.share_path.trim_start_matches('/'))
                } else {
                    format!("smb://{}:{}/{}", self.host, self.port, self.share_path.trim_start_matches('/'))
                }
            }
            NetworkProtocol::WebDav => {
                if let (Some(u), Some(p)) = (&self.username, &self.password) {
                    format!("http://{}:{}@{}:{}/{}", u, p, self.host, self.port, self.share_path.trim_start_matches('/'))
                } else {
                    format!("http://{}:{}/{}", self.host, self.port, self.share_path.trim_start_matches('/'))
                }
            }
            NetworkProtocol::Ftp => {
                if let (Some(u), Some(p)) = (&self.username, &self.password) {
                    format!("ftp://{}:{}@{}:{}/{}", u, p, self.host, self.port, self.share_path.trim_start_matches('/'))
                } else {
                    format!("ftp://{}:{}/{}", self.host, self.port, self.share_path.trim_start_matches('/'))
                }
            }
            NetworkProtocol::Dlna | NetworkProtocol::UncShare => {
                format!("\\\\{}\\{}", self.host, self.share_path.trim_matches('/'))
            }
        }
    }
}
