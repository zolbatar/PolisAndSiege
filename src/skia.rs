use raylib::ffi::{BeginTextureMode, EndTextureMode, LoadRenderTexture, RenderTexture2D};
use skia_safe::gpu::backend_render_targets;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::SurfaceOrigin::TopLeft;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::{colors, Canvas, ColorType, FontMgr, Paint, Point, Surface};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle, TypefaceFontProvider};

static EBGARAMOND_REGULAR_TTF: &[u8] = include_bytes!("../assets/EBGaramond-Regular.ttf");

pub struct MySurface {
    pub texture: RenderTexture2D,
    pub skia_surface: Surface,
}

pub struct Skia {
    context: DirectContext,
    font_collection: FontCollection,
}

impl Skia {
    pub fn new() -> Self {
        let interface = Interface::new_native().expect("Can't get GL interface");
        let options = ContextOptions::new();
        let context = make_gl(&interface, &options).expect("Can't create Skia context");

        // Fonts
        let typeface_font_provider = {
            let mut typeface_font_provider = TypefaceFontProvider::new();
            let font_mgr = FontMgr::new();
            let typeface = font_mgr
                .new_from_data(EBGARAMOND_REGULAR_TTF, None)
                .expect("Failed to load font");
            typeface_font_provider.register_typeface(typeface, "EB Garamond");
            typeface_font_provider
        };

        // Font collection
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(Some(typeface_font_provider.into()), "EB Garamond");

        Skia {
            context,
            font_collection,
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

    pub fn write_text(&mut self, canvas: &Canvas, font_size: f32, paint: &Paint, text: &str, x: f32, y: f32) {

        // Paragraph style
        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_align(TextAlign::Left);
        paragraph_style.set_max_lines(1);

        // Use the Make method to create a ParagraphBuilder
        let mut builder = ParagraphBuilder::new(&paragraph_style, &self.font_collection);

        // Text style
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font_size);
        text_style.set_foreground_paint(paint);
        text_style.add_font_feature("kern", 1);
        text_style.add_font_feature("liga", 1);
        text_style.add_font_feature("dlig", 1);
        //text_style.add_font_feature("frac", 1);

        // Add text style and text
        builder.push_style(&text_style);
        builder.add_text(text);

        // Render
        let mut paragraph = builder.build();
        paragraph.layout(256.0);
        paragraph.paint(canvas, Point::new(x, y));
    }
}
