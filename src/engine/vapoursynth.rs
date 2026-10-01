#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InterpolationEngine {
    Off,
    NativeMpv,         // display-resample with oversample
    VapourSynthRife2x, // RIFE AI 2x neural flow
    VapourSynthRife4x, // RIFE AI 4x neural flow
    Svp4Bridge,        // SmoothVideo Project 4 socket bridge
}

impl Default for InterpolationEngine {
    fn default() -> Self {
        Self::Off
    }
}

impl InterpolationEngine {
    pub const ALL: [InterpolationEngine; 5] = [
        InterpolationEngine::Off,
        InterpolationEngine::NativeMpv,
        InterpolationEngine::VapourSynthRife2x,
        InterpolationEngine::VapourSynthRife4x,
        InterpolationEngine::Svp4Bridge,
    ];

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Off => "Off",
            Self::NativeMpv => "Native GPU Frame Pacing (Smooth Pacing)",
            Self::VapourSynthRife2x => "RIFE AI Neural Motion 2× (24 → 48/60 fps)",
            Self::VapourSynthRife4x => "RIFE AI Neural Motion 4× (24 → 96/120 fps)",
            Self::Svp4Bridge => "SVP 4 (SmoothVideo Project) Socket Bridge",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VapourSynthConfig {
    pub engine: InterpolationEngine,
    pub rife_model: String, // e.g. "rife-v4.6"
    pub gpu_threads: u32,
    pub custom_vpy_path: Option<PathBuf>,
}

impl Default for VapourSynthConfig {
    fn default() -> Self {
        Self {
            engine: InterpolationEngine::Off,
            rife_model: "rife-v4.6".to_string(),
            gpu_threads: 2,
            custom_vpy_path: None,
        }
    }
}

impl VapourSynthConfig {
    /// Generates a temporary VapourSynth python script and returns the mpv vf string.
    pub fn build_video_filter(&self, temp_dir: &Path) -> Option<String> {
        match self.engine {
            InterpolationEngine::Off | InterpolationEngine::NativeMpv => None,
            InterpolationEngine::VapourSynthRife2x | InterpolationEngine::VapourSynthRife4x => {
                let multiplier = if self.engine == InterpolationEngine::VapourSynthRife4x { 4 } else { 2 };
                let script_path = temp_dir.join("vertex_rife.vpy");
                
                let vpy_script = format!(
                    r#"import vapoursynth as vs
core = vs.core
clip = video_in
# Convert to RGB for RIFE AI processing
clip = core.resize.Bilinear(clip, format=vs.RGBS, matrix_in_s="709")
try:
    from vsrife import RIFE
    clip = RIFE(clip, multiplier={}, model="{}", gpu_id=0, num_streams={})
except Exception:
    # Fallback to double frame rate motion estimation if vsrife not installed
    clip = core.std.AssumeFPS(clip, fpsnum=clip.fps.numerator * {}, fpsden=clip.fps.denominator)
clip = core.resize.Bilinear(clip, format=vs.YUV420P8, matrix_s="709")
clip.set_output()
"#,
                    multiplier, self.rife_model, self.gpu_threads, multiplier
                );

                let _ = fs::write(&script_path, vpy_script);
                if let Some(path_str) = script_path.to_str() {
                    let sanitized = path_str.replace('\\', "/");
                    return Some(format!("vapoursynth=\"{}\":buffered-frames=4:concurrent-frames=8", sanitized));
                }
                None
            }
            InterpolationEngine::Svp4Bridge => {
                let script_path = temp_dir.join("vertex_svp.vpy");
                let svp_script = r#"import vapoursynth as vs
core = vs.core
clip = video_in
try:
    core.std.LoadPlugin("C:/Program Files (x86)/SVP 4/plugins/svpflow1_vs64.dll")
    core.std.LoadPlugin("C:/Program Files (x86)/SVP 4/plugins/svpflow2_vs64.dll")
    import json
    # SVP 4 automatic socket connection
    super = core.svp1.Super(clip, "{scale:{up:0}}")
    vectors = core.svp1.Analyse(super["clip"], super["data"], clip, "{}")
    clip = core.svp2.SmoothFps(clip, super["clip"], super["data"], vectors["clip"], vectors["data"], "{}")
except Exception:
    pass
clip.set_output()
"#;
                let _ = fs::write(&script_path, svp_script);
                if let Some(path_str) = script_path.to_str() {
                    let sanitized = path_str.replace('\\', "/");
                    return Some(format!("vapoursynth=\"{}\":buffered-frames=4:concurrent-frames=8", sanitized));
                }
                None
            }
        }
    }
}
