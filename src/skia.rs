use raylib::ffi::{BeginTextureMode, EndTextureMode, LoadRenderTexture, RenderTexture2D};
use skia_safe::gpu::backend_render_targets;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::SurfaceOrigin::TopLeft;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::{colors, Canvas, ColorType, Surface};

pub struct MySurface {
    pub texture: RenderTexture2D,
    pub skia_surface: Surface,
}

pub struct Skia {
    context: DirectContext,
}

impl Skia {
    pub fn new() -> Self {
        let interface = Interface::new_native().expect("Can't get GL interface");
        let options = ContextOptions::new();
        let context = make_gl(&interface, &options).expect("Can't create Skia context");

        Skia {
            context,
        }
    }

    pub fn make_surface(&mut self, width: i32, height: i32) -> MySurface {

        // Create raylib texture
        let texture: RenderTexture2D;
        unsafe {
            texture = LoadRenderTexture(width, height);
        }

        // 0x8058 is GL_RGBA8
        let fb_info = FramebufferInfo { fboid: texture.texture.id, format: 0x8058, ..Default::default() };
        let target = backend_render_targets::make_gl((width, height), 1, 8, fb_info);
        let surface = wrap_backend_render_target(&mut self.context, &target, TopLeft, ColorType::RGBA8888, None, None).expect("Can't create GL surface");
        MySurface {
            skia_surface: surface,
            texture,
        }
    }

    pub fn get_canvas(surface: &mut MySurface) -> &Canvas {
        surface.skia_surface.canvas()
    }

    pub unsafe fn flush(&mut self, surface: &mut MySurface) {
        BeginTextureMode(surface.texture);
        self.context.reset(None);
        surface.skia_surface.image_snapshot();
        self.context.flush_and_submit();
        EndTextureMode();
        surface.skia_surface.canvas().clear(colors::TRANSPARENT);
    }

    pub fn set_matrix(&mut self, canvas: &Canvas, dpi: f32) {
        canvas.save();
        canvas.scale((dpi, dpi));
    }

    pub fn clear_matrix(&mut self, canvas: &Canvas) {
        canvas.restore();
    }
}
