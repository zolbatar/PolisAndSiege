use raylib::ffi::{BeginTextureMode, EndTextureMode, LoadRenderTexture, RenderTexture2D};
use skia_safe::gpu::backend_render_targets;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::SurfaceOrigin::TopLeft;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::{colors, Canvas, ColorType, Surface};

pub struct MySurface {
    texture: RenderTexture2D,
    pub skia_surface: Surface,
}

pub struct Skia {
    interface: Interface,
    context: DirectContext,
    surface: Option<MySurface>,
}

impl Skia {
    pub fn new() -> Self {
        let interface = Interface::new_native().expect("Can't get GL interface");
        let options = ContextOptions::new();
        let context = make_gl(&interface, &options).expect("Can't create Skia context");

        Skia {
            interface,
            context,
            surface: None,
        }
    }

    pub fn init(&mut self, width: i32, height: i32) {
        self.surface = Option::from(Self::make_surface(self, width, height));
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

    fn get_raw_surface(&mut self) -> &mut MySurface {
        self.surface.as_mut().unwrap()
    }

    pub fn get_canvas(&mut self) -> &Canvas {
        self.get_raw_surface().skia_surface.canvas()
    }

    pub unsafe fn render(&mut self) {
        BeginTextureMode(self.surface.as_ref().unwrap().texture);
        self.context.reset(None);
        self.get_canvas().surface().unwrap().image_snapshot();
        self.context.flush_and_submit();
        EndTextureMode();
        self.get_canvas().clear(colors::TRANSPARENT);
    }
}
