# VortexPlayer: Complete PotPlayer-Parity Pro Feature Matrix

VortexPlayer has reached complete feature parity and architectural superiority over PotPlayer.

---

## 1. True AI Motion Interpolation (RIFE / SVP Bridge)
- **VapourSynth Processing Pipeline**: Executes Python `.vpy` scripts inside the playback pipeline via `vf=vapoursynth=...`.
- **RIFE Neural Flow (2× / 4×)**: Synthesizes intermediate motion-compensated frames ($24\text{fps} \rightarrow 48/60/120\text{fps}$) using optical flow neural networks.
- **SVP 4 Integration**: Direct SmoothVideo Project socket bridge and VapourSynth script handoff (`SVPMgr.vpy`).

---

## 2. Full Capture & Recording Studio (`Ctrl+S`)
- **Burst / Continuous Snapshots**:
  - Automatically captures snapshots every $N$ seconds or $N$ frames into `PNG`, `JPEG`, `WebP`, or `BMP`.
- **$4\times 4$ Contact Sheet Generator**:
  - Automatically generates structured video preview thumbnail sheets with timestamp grids and header info.
- **Live Stream & Playback Recorder**:
  - Lossless direct stream copy (`stream-record`) or hardware accelerated re-encode with NVIDIA `NVENC`, Intel `QSV`, or AMD `AMF`.

---

## 3. Bidirectional JSON-RPC 2.0 Automation Protocol
- **Transport**: Named Pipe `\\.\pipe\vortexplayer-ipc` & Localhost TCP `127.0.0.1:42120`.
- **Protocol**: JSON-RPC 2.0 (`{"jsonrpc":"2.0","id":1,"method":"...","params":{...}}`).
- **Methods**: `get_status`, `play`, `pause`, `toggle_pause`, `stop`, `seek`, `seek_relative`, `set_volume`, `set_speed`, `set_audio_track`, `set_subtitle_track`, `load_file`, `load_url`, `take_screenshot`, `start_recording`, `stop_recording`, `get_playlist`, `add_to_playlist`, `next`, `previous`.

---

## 4. Advanced Subtitle Ecosystem & Live Translation
- **OpenSubtitles 64-Bit File Hash Matching**: Exact hash calculation for automatic subtitle lookup.
- **Real-Time Translation Bridge**: Translates foreign subtitle text with thread-safe memoization caching.

---

## 5. Dynamic C-ABI / Rust Plugin Architecture
- **Dynamic `.dll` Plugin Loader**: Scans `%APPDATA%\VortexPlayer\plugins\` to load custom third-party DSP filters, decoders, and visualizers without recompilation using `libloading`.

---

## 6. Comprehensive Comparison Matrix

| Feature Area | PotPlayer | VortexPlayer (Rust) | Advantage |
| :--- | :---: | :---: | :--- |
| **Rendering Pipeline** | DirectShow / madVR | D3D11 / libplacebo / Vulkan | **VortexPlayer** (Zero-overhead modern pipeline) |
| **Graphic EQ** | 10-Band | 18-Band ($20\text{Hz}$–$20\text{kHz}$) | **VortexPlayer** (Higher resolution) |
| **Parametric EQ & AutoEQ** | ❌ Limited | ✅ Full Interactive Curve + APO Import | **VortexPlayer** (Headphone Harman Target matching) |
| **Shader Upscaling** | Pixel Shaders | Lanczos, Spline36, Anime4K, CAS | **VortexPlayer** (Native chained presets) |
| **Refresh-Rate Sync** | ✅ GDI | ✅ Automated Win32 Judder Elimination | Parity |
| **Per-File Settings** | ✅ INI | ✅ JSON Hash-Indexed Persistent Memory | Parity |
| **Media Library Wall** | ⚠️ Basic | ✅ Poster Art, TV Seasons, Watch Folders | **VortexPlayer** (Modern UI Grid) |
| **Automation & IPC** | ⚠️ Win32 MSG | ✅ Bidirectional JSON-RPC 2.0 Named Pipe & TCP | **VortexPlayer** (Modern JSON-RPC 2.0) |
| **VapourSynth / RIFE** | ✅ External | ✅ Integrated Python Script Generator | Parity |
| **Capture & Recording** | ✅ Complete | ✅ Burst Snapshot, Contact Sheet & Stream Record | Parity |
| **Subtitle Ecosystem** | ✅ Online | ✅ OpenSubtitles Hash + Dual Live Translation | Parity |
| **Plugin Subsystem** | DirectShow / Winamp | Modern C-ABI / Rust `.dll` Plugins | **VortexPlayer** (Memory-safe Rust foundation) |

---

## Executable Location
```powershell
cargo run --release
```
