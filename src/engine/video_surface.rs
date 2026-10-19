use egui_glow::glow;
use egui_glow::glow::HasContext;

pub struct VideoSurface {
    pub fbo: Option<glow::Framebuffer>,
    pub texture: Option<glow::Texture>,
    pub width: i32,
    pub height: i32,
    pub has_rendered_any_frame: bool,
}

impl Default for VideoSurface {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoSurface {
    pub fn new() -> Self {
        Self {
            fbo: None,
            texture: None,
            width: 0,
            height: 0,
            has_rendered_any_frame: false,
        }
    }

    pub fn destroy(&mut self, gl: &glow::Context) {
        unsafe {
            if let Some(fbo) = self.fbo.take() {
                gl.delete_framebuffer(fbo);
            }
            if let Some(tex) = self.texture.take() {
                gl.delete_texture(tex);
            }
        }
        self.width = 0;
        self.height = 0;
        self.has_rendered_any_frame = false;
    }

    pub fn ensure_surface(&mut self, gl: &glow::Context, w: i32, h: i32) -> Result<i32, String> {
        if w <= 0 || h <= 0 {
            return Err("Invalid surface dimensions".to_string());
        }
        if self.fbo.is_some() && self.width == w && self.height == h {
            if let Some(fbo) = self.fbo {
                return Ok(Self::fbo_to_i32(fbo));
            }
        }

        self.destroy(gl);

        unsafe {
            let tex = gl.create_texture().map_err(|e| format!("glCreateTexture failed: {}", e))?;
            let fbo = gl.create_framebuffer().map_err(|e| format!("glCreateFramebuffer failed: {}", e))?;

            gl.bind_texture(glow::TEXTURE_2D, Some(tex));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                w,
                h,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(None),
            );
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);

            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(tex),
                0,
            );

            let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            gl.bind_texture(glow::TEXTURE_2D, None);

            if status != glow::FRAMEBUFFER_COMPLETE {
                gl.delete_framebuffer(fbo);
                gl.delete_texture(tex);
                return Err(format!("Framebuffer incomplete: status 0x{:X}", status));
            }

            let raw_id = Self::fbo_to_i32(fbo);
            self.fbo = Some(fbo);
            self.texture = Some(tex);
            self.width = w;
            self.height = h;
            Ok(raw_id)
        }
    }

    #[inline]
    pub fn fbo_to_i32(fbo: glow::Framebuffer) -> i32 {
        fbo.0.get() as i32
    }

    pub fn blit_to_screen(&self, gl: &glow::Context, target_w: i32, target_h: i32) {
        if let Some(fbo) = self.fbo {
            unsafe {
                gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(fbo));
                gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None);
                gl.blit_framebuffer(
                    0, 0, self.width, self.height,
                    0, 0, target_w, target_h,
                    glow::COLOR_BUFFER_BIT,
                    glow::NEAREST,
                );
                gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            }
        }
    }

    pub fn render_video(
        &mut self,
        gl: &glow::Context,
        player: &crate::engine::Player,
        w: i32,
        h: i32,
    ) {
        if w <= 0 || h <= 0 {
            return;
        }
        let size_changed = self.width != w || self.height != h;
        let had_rendered = self.has_rendered_any_frame;
        match self.ensure_surface(gl, w, h) {
            Ok(fbo_id) => {
                let force = (size_changed && had_rendered) || !self.has_rendered_any_frame;
                if player.render_to_surface(fbo_id, w, h, force) {
                    self.has_rendered_any_frame = true;
                } else if had_rendered {
                    self.has_rendered_any_frame = true;
                }
                if self.has_rendered_any_frame {
                    self.blit_to_screen(gl, w, h);
                }
            }
            Err(_) => {
                player.render_to_surface(0, w, h, true);
            }
        }
    }
}
