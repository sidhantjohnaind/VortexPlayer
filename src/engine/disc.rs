#![allow(dead_code)]

use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscType {
    Dvd,
    BluRay,
    IsoImage,
}

pub struct DiscEngine;

impl DiscEngine {
    pub fn detect_disc_type(path: &Path) -> Option<DiscType> {
        let path_str = path.to_string_lossy().to_lowercase();
        if path_str.ends_with(".iso") {
            return Some(DiscType::IsoImage);
        }
        if path_str.contains("video_ts") || path.join("VIDEO_TS").exists() {
            return Some(DiscType::Dvd);
        }
        if path_str.contains("bdmv") || path.join("BDMV").exists() {
            return Some(DiscType::BluRay);
        }
        None
    }

    pub fn build_disc_url(path: &Path, title: u32) -> String {
        let disc_type = Self::detect_disc_type(path);
        match disc_type {
            Some(DiscType::Dvd) => {
                format!("dvd://{} --dvd-device=\"{}\"", title, path.to_string_lossy())
            }
            Some(DiscType::BluRay) => {
                format!("bd://{} --bluray-device=\"{}\"", title, path.to_string_lossy())
            }
            Some(DiscType::IsoImage) => {
                format!("dvd://{} --dvd-device=\"{}\"", title, path.to_string_lossy())
            }
            None => path.to_string_lossy().to_string(),
        }
    }
}
