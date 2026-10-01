#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureDevice {
    pub name: String,
    pub is_video: bool,
    pub is_audio: bool,
}

pub struct CaptureDeviceManager;

impl CaptureDeviceManager {
    pub fn list_directshow_devices() -> Vec<CaptureDevice> {
        vec![
            CaptureDevice {
                name: "Integrated Camera".to_string(),
                is_video: true,
                is_audio: false,
            },
            CaptureDevice {
                name: "USB Video Capture Card (HDMI)".to_string(),
                is_video: true,
                is_audio: true,
            },
            CaptureDevice {
                name: "Microphone Array".to_string(),
                is_video: false,
                is_audio: true,
            },
        ]
    }

    pub fn build_dshow_url(video_device: Option<&str>, audio_device: Option<&str>) -> String {
        match (video_device, audio_device) {
            (Some(v), Some(a)) => format!("dshow://video=\"{}\":audio=\"{}\"", v, a),
            (Some(v), None) => format!("dshow://video=\"{}\"", v),
            (None, Some(a)) => format!("dshow://audio=\"{}\"", a),
            (None, None) => "dshow://".to_string(),
        }
    }
}
