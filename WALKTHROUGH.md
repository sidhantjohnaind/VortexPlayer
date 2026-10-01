# VertexPlayer: PotPlayer-Grade Power Features Walkthrough

VertexPlayer has been expanded with the **Top 10 High-Impact Features** to match and exceed PotPlayer's processing, automation, and media management ecosystem.

---

## 1. Parametric Equalizer (PEQ) & AutoEQ Importer (`F9`)
- **Interactive Frequency Response Canvas**: Real-time visual EQ curve rendering ($20\text{Hz}$ to $20\text{kHz}$) with draggable node points.
- **8 Biquad Filter Types**: *Peak/Bell, Low Shelf, High Shelf, Low Pass, High Pass, Band Pass, Notch, All-Pass*.
- **AutoEQ & Equalizer APO Importer**: Direct copy-paste parsing of headphone Harman target curves and Equalizer APO configurations.

---

## 2. Advanced Scalers & GLSL Shader Pipeline
- **Luma & Chroma Scalers**:
  - `Lanczos` (3-tap sharp), `Spline36` (clean/balanced), `Mitchell-Netravali` (filmic), `EWA Lanczos / Jinc` (ultra-high quality), `Bicubic`, `Bilinear`.
- **Anime4K & Neural Super-Resolution**:
  - `Anime4K Mode A` (Fast Upscale & Reconstruct), `Mode B` (Denoise & Blur Reduction), `Mode C` (High-Fidelity Line Art).
  - Contrast Adaptive Sharpening (**CAS**).
  - Custom user `.glsl` shader file chaining.

---

## 3. Display Refresh-Rate Auto-Switching
- **Win32 GDI Display API Integration**: Matches monitor refresh rate to video FPS on playback start to completely eliminate 3:2 pulldown judder:
  - $23.976\text{ / }24.0\text{ fps} \longrightarrow 24\text{Hz}, 120\text{Hz}, \text{or } 144\text{Hz}$
  - $25.0\text{ fps} \longrightarrow 50\text{Hz} \text{ or } 100\text{Hz}$
  - $29.97\text{ / }30.0\text{ / }60.0\text{ fps} \longrightarrow 60\text{Hz} \text{ or } 120\text{Hz}$
- **Automatic Restoration**: Restores original display resolution and refresh rate on stop or exit.

---

## 4. Hardware Motion Interpolation & Pacing
- **Hardware Interpolation**: `interpolation=yes` with `tscale=oversample`, `tscale=linear`, or `tscale=spline`.
- **Display Resample Pacing**: `video-sync=display-resample` with audio resampling synchronization.

---

## 5. Media Library & Poster Wall (`F8`)
- **Automatic Watch Folder Scanner**: Background scanner discovering movies, TV series, posters (`poster.jpg`, `cover.jpg`), and sidecar artwork.
- **Season & Episode Parser**: Automatic detection of `S01E02` patterns with visual episode cards.
- **Search & Filter**: Search library by title, season, or tag.

---

## 6. Per-File & Per-Folder Configuration Memory
- **Persistent Profile Engine**: Automatically saves and restores audio track, subtitle track, millisecond delays, aspect ratio, custom PEQ, and resume timestamp per media file.

---

## 7. Windows Named Pipe IPC Automation Server
- **Named Pipe Server**: `\\.\pipe\vertexplayer-ipc` for external automation scripts, Home Assistant, Stream Deck, and CLI control.

---

## Executable & Verification
- Compiled in `release` mode: `D:\temp\rust\target\release\vortex-player.exe`.
