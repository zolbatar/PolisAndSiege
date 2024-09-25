use crate::app_state::AppState;
use rand::Rng;
use raylib::ffi::{BeginTextureMode, EndTextureMode, LoadRenderTexture, RenderTexture2D};
use skia_safe::gpu::backend_render_targets;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::SurfaceOrigin::TopLeft;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle, TypefaceFontProvider};
use skia_safe::{colors, Canvas, ColorType, FontMgr, ImageFilter, Paint, PaintStyle, Point, Surface};

static EBGARAMOND_REGULAR_TTF: &[u8] = include_bytes!("../assets/EBGaramond-Regular.ttf");

pub struct MySurface {
    pub texture: RenderTexture2D,
    pub skia_surface: Surface,
}

pub struct Skia {
    context: DirectContext,
    font_collection: FontCollection,
    pub blur: Option<ImageFilter>,
    pub drop_shadow: Option<ImageFilter>,
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

        // Filters
        /*        let blur = blur((1.0, 1.0), TileMode::default(), None, None);
                let drop_shadow = drop_shadow_only(
                    Vector::new(1.5, -1.5),
                    (1.5, 1.5),
                    Color::BLACK,
                    None,
                    None,
                    None);*/

        Skia {
            context,
            font_collection,
            drop_shadow: None,
            blur: None,
        }
    }

    pub fn test(&self, canvas: &Canvas, width: i32, height: i32) {
        let mut rng = rand::thread_rng();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        for _ in 1..=10000 {
            canvas.draw_line(Point { x: rng.gen_range(0..=width) as f32, y: rng.gen_range(0..=height) as f32 }, Point { x: rng.gen_range(0..=width) as f32, y: rng.gen_range(0..=height) as f32 }, &paint);
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

    pub fn set_matrix(&mut self, canvas: &Canvas, app_state: &AppState) {
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.dpi, app_state.dpi));
    }

    pub fn set_matrix_centre(&mut self, canvas: &Canvas, app_state: &AppState) {
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.dpi, app_state.dpi));
        canvas.translate((app_state.half_width, app_state.half_height));
    }

    pub fn set_matrix_camera(&mut self, canvas: &Canvas, app_state: &AppState) {
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.dpi, app_state.dpi));
        canvas.translate((app_state.camera.offset.x, app_state.camera.offset.y));
        canvas.scale((app_state.camera.zoom, app_state.camera.zoom));
        canvas.translate((-app_state.camera.target.x, -app_state.camera.target.y));
        canvas.scale((1.0, -1.0)); // Flip vertically
    }

    pub fn clear_matrix(&mut self, canvas: &Canvas) {
        canvas.restore();
    }

    fn create_paragraph_builder(&mut self, font_size: f32, paint: &Paint, text: &str) -> ParagraphBuilder {
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
        builder
    }

    pub fn text_dimensions(&mut self, font_size: f32, paint: &Paint, text: &str) -> f32 {
        let mut builder = self.create_paragraph_builder(font_size, paint, text);
        let mut paragraph = builder.build();
        paragraph.layout(10000.0);
        paragraph.max_intrinsic_width()
    }

    pub fn write_text(&mut self, canvas: &Canvas, font_size: f32, paint: &Paint, text: &str, x: f32, y: f32, width: f32) {
        let mut builder = self.create_paragraph_builder(font_size, paint, text);
        let mut paragraph = builder.build();
        paragraph.layout(if width == 0.0 { canvas.base_layer_size().width as f32 } else { width });
        paragraph.paint(canvas, Point::new(x, y));
    }
}
