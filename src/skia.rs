use crate::app_state::AppState;
use rand::Rng;
use raylib::data;
use raylib::ffi::{BeginTextureMode, EndTextureMode, LoadRenderTexture, RenderTexture2D};
use skia_safe::gpu::backend_render_targets;
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::SurfaceOrigin::TopLeft;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle, TypefaceFontProvider};
use skia_safe::{Canvas, Color, ColorType, Data, FontMgr, ImageFilter, Paint, PaintStyle, Point, RuntimeEffect, Surface};
use skia_safe::runtime_effect::Uniform;

static EBGARAMOND_REGULAR_TTF: &[u8] = include_bytes!("../assets/EBGaramond-Regular.ttf");
const NOISE_SKSL: &[u8] = include_bytes!("../assets/noise.sksl");
pub const ELLIPSIS: &str = "\u{2026}";

pub struct MySurface {
    pub texture: RenderTexture2D,
    pub skia_surface: Surface,
}

pub struct Skia {
    context: DirectContext,
    font_collection: FontCollection,
    pub blur: Option<ImageFilter>,
    pub drop_shadow: Option<ImageFilter>,
    noise_shader: RuntimeEffect,
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

        // Shaders
        let noise_shader = RuntimeEffect::make_for_shader(std::str::from_utf8(NOISE_SKSL).unwrap(), None).unwrap();

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
            noise_shader,
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
        surface.skia_surface.canvas().clear(Color::from_argb(255, 63, 63, 63));
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
    }

    pub fn clear_matrix(&mut self, canvas: &Canvas) {
        canvas.restore();
    }

    fn create_paragraph_builder(&self, font_size: f32, paint: &Paint, text: &str) -> ParagraphBuilder {
        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_align(TextAlign::Left);
        paragraph_style.set_max_lines(1);
        paragraph_style.set_ellipsis(ELLIPSIS);

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

    pub fn text_dimensions(&self, font_size: f32, paint: &Paint, text: &str) -> f32 {
        let mut builder = self.create_paragraph_builder(font_size, paint, text);
        let mut paragraph = builder.build();
        paragraph.layout(10000.0);
        paragraph.max_intrinsic_width()
    }

    pub fn write_text(&self, canvas: &Canvas, font_size: f32, paint: &Paint, text: &str, xy: Point, width: f32) {
        let mut builder = self.create_paragraph_builder(font_size, paint, text);
        let mut paragraph = builder.build();
        paragraph.layout(if width == 0.0 { canvas.base_layer_size().width as f32 } else { width });
        paragraph.paint(canvas, xy);
    }

    pub fn write_text_centre(&self, canvas: &Canvas, font_size: f32, paint: &Paint, text: &str, xy: Point, width: f32) {
        let dimensions = self.text_dimensions(font_size, paint, text);
        let mut builder = self.create_paragraph_builder(font_size, paint, text);
        let mut paragraph = builder.build();
        paragraph.layout(if width == 0.0 { canvas.base_layer_size().width as f32 } else { width });
        paragraph.paint(canvas, Point::new(xy.x - dimensions / 2.0, xy.y));
    }

    fn create_noise_shader(&self, effect: &RuntimeEffect, base_color: Color, mix: f32) -> Option<Shader> {
//        let mut data = Data::
        let mut builder = self.noise_shader.

        // Set the uniform for "u_noiseMix"
        builder.set_uniform("u_noiseMix", mix);

        // Set the uniform for "u_baseColor" (convert SkColor to SkColor4f)
        let base_color_4f = Color4f::from_color(base_color);
        builder.set_uniform("u_baseColor", base_color_4f);

        // Create the shader from the builder
        builder.make_shader(None)
    }
}

pub fn mix_colors(color1: Color, color2: Color, mut ratio: f32) -> Color {
    // Clamp the ratio between 0.0 and 1.0
    ratio = ratio.clamp(0.0, 1.0);

    // Extract RGBA components from each color
    let r1 = color1.r() as f32;
    let g1 = color1.g() as f32;
    let b1 = color1.b() as f32;
    let a1 = color1.a() as f32;

    let r2 = color2.r() as f32;
    let g2 = color2.g() as f32;
    let b2 = color2.b() as f32;
    let a2 = color2.a() as f32;

    // Linearly interpolate between the two colors' components based on the ratio
    let r = (r1 * (1.0 - ratio) + r2 * ratio) as u8;
    let g = (g1 * (1.0 - ratio) + g2 * ratio) as u8;
    let b = (b1 * (1.0 - ratio) + b2 * ratio) as u8;
    let a = (a1 * (1.0 - ratio) + a2 * ratio) as u8;

    // Return the blended color
    Color::from_argb(a, r, g, b)
}