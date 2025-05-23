use crate::app_state::{AppState, GFXState, NOISE_MIX};
use rand::Rng;
use skia_safe::font_style::{Slant, Weight, Width};
use skia_safe::gpu::direct_contexts::make_gl;
use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::surfaces::wrap_backend_render_target;
use skia_safe::gpu::{ContextOptions, DirectContext};
use skia_safe::image_filters::drop_shadow_only;
use skia_safe::paint::Style;
use skia_safe::textlayout::{
    FontCollection, ParagraphBuilder, ParagraphStyle, TextAlign, TextStyle, TypefaceFontProvider,
};
use skia_safe::{
    gpu, Canvas, Color, Color4f, Contains, Data, FontMgr, FontStyle, ImageFilter, Paint, PaintStyle, Point, Rect,
    RuntimeEffect, Shader, Surface, Vector,
};

static EBGARAMOND_TTF: &[u8] = include_bytes!("../../assets/EBGaramond-VariableFont_wght.ttf");
static NOTO_SANS_SYMBOLS_TTF: &[u8] = include_bytes!("../../assets/NotoSansSymbols-VariableFont_wght.ttf");
const NOISE_SKSL: &str = include_str!("../../assets/noise.sksl");
const HALFTONE_SKSL: &str = include_str!("../../assets/halftone.sksl");
pub const ELLIPSIS: &str = "\u{2026}";

pub enum FontFamily {
    EbGaramond,
    EbGaramondBold,
    NotoSansSymbols,
}

pub struct Skia {
    context: DirectContext,
    font_collection: FontCollection,
    pub drop_shadow: Option<ImageFilter>,
    pub drop_shadow_white: Option<ImageFilter>,
    noise_shader: RuntimeEffect,
    _halftone_shader: RuntimeEffect,
    pub surface: Surface,
    pub colour_background: Color,
    pub colour_popup: Color,
    pub colour_outline: Color,
    pub button: Color,
    pub button_hover: Color,
}

impl Skia {
    fn make_surface(context: &mut DirectContext, width: i32, height: i32) -> Surface {
        // Get window size and create a Skia surface from the OpenGL framebuffer
        let fb_info = FramebufferInfo {
            fboid: 0,
            format: gl::RGBA8,
            ..Default::default()
        };
        let backend_render_target = gpu::backend_render_targets::make_gl(
            (width, height),
            0, // Sample count
            8, // Stencil bits
            fb_info,
        );

        // Create the Skia surface for rendering
        wrap_backend_render_target(
            context,
            &backend_render_target,
            gpu::SurfaceOrigin::BottomLeft,
            skia_safe::ColorType::RGBA8888,
            None,
            None,
        )
        .expect("Could not create Skia surface")
    }

    pub fn new(app_state: &AppState) -> Self {
        let interface = Interface::new_native().expect("Can't get GL interface");
        let options = ContextOptions::new();
        let mut context = make_gl(&interface, &options).expect("Can't create Skia context");

        // Fonts
        let typeface_font_provider = {
            let mut typeface_font_provider = TypefaceFontProvider::new();
            let font_mgr = FontMgr::new();

            let typeface = font_mgr.new_from_data(EBGARAMOND_TTF, None).expect("Failed to load font");
            typeface_font_provider.register_typeface(typeface, "EB Garamond");

            let typeface = font_mgr.new_from_data(NOTO_SANS_SYMBOLS_TTF, None).expect("Failed to load font");
            typeface_font_provider.register_typeface(typeface, "Noto Sans Symbols");

            typeface_font_provider
        };

        // Font collection
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(Some(typeface_font_provider.into()), "EB Garamond");

        // Shaders
        let noise_shader = RuntimeEffect::make_for_shader(NOISE_SKSL, None).expect("Failed to make runtime effect");

        // Create the runtime effect from the SkSL code
        let halftone_shader =
            RuntimeEffect::make_for_shader(HALFTONE_SKSL, None).expect("Failed to make runtime effect");

        // Filters
        let drop_shadow =
            drop_shadow_only(Vector::new(1.5, 1.5), (0.5, 0.5), Color::from_argb(64, 0, 0, 0), None, None, None);
        let drop_shadow_white =
            drop_shadow_only(Vector::new(1.5, 1.5), (2.0, 2.0), Color::from_argb(64, 255, 255, 255), None, None, None);

        // Surface
        let surface = Skia::make_surface(
            &mut context,
            app_state.gfx.width * app_state.gfx.dpi as i32,
            app_state.gfx.height * app_state.gfx.dpi as i32,
        );

        Skia {
            context,
            surface,
            font_collection,
            drop_shadow,
            drop_shadow_white,
            noise_shader,
            _halftone_shader: halftone_shader,
            colour_background: Color::from_argb(255, 53, 53, 53),
            colour_popup: Color::from_argb(255, 80, 80, 80),
            colour_outline: Color::from_argb(255, 209, 185, 120),
            button: Color::from_argb(255, 80, 80, 80),
            button_hover: Color::from_argb(255, 100, 100, 100),
        }
    }

    pub fn _test(&self, canvas: &Canvas, width: i32, height: i32) {
        let mut rng = rand::rng();
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        for _ in 1..=10000 {
            canvas.draw_line(
                Point {
                    x: rng.random_range(0..=width) as f32,
                    y: rng.random_range(0..=height) as f32,
                },
                Point {
                    x: rng.random_range(0..=width) as f32,
                    y: rng.random_range(0..=height) as f32,
                },
                &paint,
            );
        }
    }

    pub fn get_canvas(&mut self) -> &Canvas {
        self.surface.canvas()
    }

    pub unsafe fn flush(&mut self) {
        self.surface.image_snapshot();
        self.context.flush_and_submit();

        // Clear
        let w = self.surface.width();
        let h = self.surface.height();
        self.get_canvas().clear(skia_safe::Color::TRANSPARENT);
        let mut paint_background = Paint::default();
        paint_background.set_style(PaintStyle::Fill);
        paint_background.set_shader(self.create_noise_shader(self.colour_background, NOISE_MIX));
        self.get_canvas().draw_rect(Rect::from_xywh(0.0, 0.0, w as f32, h as f32), &paint_background);
    }

    pub fn set_matrix(&mut self, gfx: &GFXState) {
        let canvas = self.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((gfx.dpi, gfx.dpi));
    }

    pub fn _set_matrix_centre(&mut self, gfx: &GFXState) {
        let canvas = self.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((gfx.dpi, gfx.dpi));
        canvas.translate((gfx.half_width, gfx.half_height));
    }

    pub fn _set_matrix_camera(&mut self, app_state: &AppState) {
        let canvas = self.get_canvas();
        canvas.save();
        canvas.reset_matrix();
        canvas.scale((app_state.gfx.dpi, app_state.gfx.dpi));
        canvas.translate((app_state.gfx.half_width, app_state.gfx.half_height));
        canvas.scale((app_state.zoom, app_state.zoom));
        canvas.translate((-app_state.target.x, -app_state.target.y));
    }

    pub fn clear_matrix(&mut self) {
        let canvas = self.get_canvas();
        canvas.restore();
    }

    fn create_paragraph_builder(
        &self,
        font_size: f32,
        paint: &Paint,
        text: &str,
        family: &FontFamily,
        align: TextAlign,
    ) -> ParagraphBuilder {
        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_align(align);
        paragraph_style.set_max_lines(5);
        paragraph_style.set_ellipsis(ELLIPSIS);

        // Use the Make method to create a ParagraphBuilder
        let mut builder = ParagraphBuilder::new(&paragraph_style, &self.font_collection);

        let weight = match family {
            FontFamily::EbGaramond => Weight::NORMAL,
            FontFamily::EbGaramondBold => Weight::MEDIUM,
            FontFamily::NotoSansSymbols => Weight::NORMAL,
        };
        let font_style = FontStyle::new(weight, Width::NORMAL, Slant::Upright);

        // Text style
        let mut text_style = TextStyle::new();
        text_style.set_font_size(font_size);
        match family {
            FontFamily::EbGaramond => text_style.set_font_families(&["EB Garamond"]),
            FontFamily::EbGaramondBold => text_style.set_font_families(&["EB Garamond"]),
            FontFamily::NotoSansSymbols => text_style.set_font_families(&["Noto Sans Symbols"]),
        };
        text_style.set_font_style(font_style);
        text_style.set_foreground_paint(paint);
        text_style.add_font_feature("kern", 1);
        text_style.add_font_feature("liga", 1);
        text_style.add_font_feature("dlig", 1);
        if text.contains('/') {
            text_style.add_font_feature("frac", 1);
        }

        // Add text style and text
        builder.push_style(&text_style);
        builder.add_text(text);
        builder
    }

    pub fn text_dimensions(
        &self,
        font_size: f32,
        paint: &Paint,
        text: &str,
        family: &FontFamily,
        align: TextAlign,
    ) -> f32 {
        let mut builder = self.create_paragraph_builder(font_size, paint, text, family, align);
        let mut paragraph = builder.build();
        paragraph.layout(10000.0);
        paragraph.max_intrinsic_width()
    }

    pub fn write_text(
        &mut self,
        font_size: f32,
        paint: &Paint,
        text: &str,
        xy: Point,
        width: f32,
        family: &FontFamily,
    ) {
        let mut builder = self.create_paragraph_builder(font_size, paint, text, family, TextAlign::Left);
        let mut paragraph = builder.build();
        paragraph.layout(if width == 0.0 {
            self.get_canvas().base_layer_size().width as f32
        } else {
            width
        });
        paragraph.paint(self.get_canvas(), xy);
    }

    pub fn write_text_centre(
        &mut self,
        font_size: f32,
        paint: &Paint,
        text: &str,
        xy: Point,
        width: f32,
        family: &FontFamily,
    ) {
        let mut builder = self.create_paragraph_builder(font_size, paint, text, family, TextAlign::Center);
        let mut paragraph = builder.build();
        paragraph.layout(if width == 0.0 {
            self.get_canvas().base_layer_size().width as f32
        } else {
            width
        });
        //        let aa = paragraph.get_
        paragraph.paint(self.get_canvas(), xy);
    }

    pub fn write_text_right(
        &mut self,
        font_size: f32,
        paint: &Paint,
        text: &str,
        xy: Point,
        width: f32,
        family: &FontFamily,
    ) {
        let mut builder = self.create_paragraph_builder(font_size, paint, text, family, TextAlign::Right);
        let mut paragraph = builder.build();
        paragraph.layout(if width == 0.0 {
            self.get_canvas().base_layer_size().width as f32
        } else {
            width
        });
        paragraph.paint(self.get_canvas(), xy);
    }

    pub fn reset_context(&mut self) {
        self.context.reset(None);
    }

    pub fn create_noise_shader(&mut self, base_color: Color, mix: f32) -> Shader {
        let uniforms = {
            let mut data = vec![];

            // Mix
            data.extend_from_slice(&mix.to_ne_bytes());

            // Colour
            let d = Color4f::from(base_color).as_array().iter().map(|&f| f.to_ne_bytes()).flatten().collect::<Vec<_>>();
            data.extend_from_slice(&d);

            Data::new_copy(&data)
        };
        self.noise_shader.clone().make_shader(uniforms, &[], None).expect("Make shader failed")
    }

    fn _create_halftone_shader(&mut self, dot_colour: Color, bg_color: Color, dot_radius: f32) -> Shader {
        let uniforms = {
            let mut data = vec![];

            // Dot radius
            data.extend_from_slice(&dot_radius.to_ne_bytes());

            let d = Color4f::from(dot_colour).as_array().iter().map(|&f| f.to_ne_bytes()).flatten().collect::<Vec<_>>();
            data.extend_from_slice(&d);

            let d = Color4f::from(bg_color).as_array().iter().map(|&f| f.to_ne_bytes()).flatten().collect::<Vec<_>>();
            data.extend_from_slice(&d);

            Data::new_copy(&data)
        };

        self._halftone_shader.clone().make_shader(uniforms, &[], None).expect("Make shader failed")
    }

    pub fn _halftone(&mut self, dot_color: Color) -> Shader {
        self._apply_halftone_shader(0.1, dot_color, Color::BLACK)
    }

    fn _apply_halftone_shader(&mut self, dot_radius: f32, dot_color: Color, bg_color: Color) -> Shader {
        self._create_halftone_shader(dot_color, bg_color, dot_radius)
    }

    pub fn button(&mut self, text: &str, app_state: &AppState, xy: Vector) {
        let width = 328.0 * 0.4 / 2.0;
        let height = 196.0 * 0.4 / 2.0;

        {
            let canvas = self.get_canvas();
            canvas.save();
            canvas.reset_matrix();
            canvas.scale((app_state.gfx.dpi, app_state.gfx.dpi));
            canvas.translate(Vector::new(xy.x - width, xy.y));
            canvas.scale((0.4, 0.4));
            app_state.res.button_path.render(canvas);
            canvas.restore();
        }

        // Background Colour
        let mut paint_click = Paint::default();
        paint_click.set_style(Style::Fill);
        let rect = skia_safe::Rect::from_xywh(xy.x - width + 1.0, xy.y + 21.4, width * 2.0 - 1.0, height - 3.0);
        if rect.contains(app_state.hover) {
            paint_click.set_color(self.button_hover);
        } else {
            paint_click.set_color(self.button);
        }
        self.get_canvas().draw_rect(rect, &paint_click);

        // Label
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_color(Color::WHITE);
        paint.set_image_filter(self.drop_shadow_white.clone());
        self.write_text_centre(
            30.0,
            &paint,
            text,
            Point::new(xy.x - width, xy.y + (196.0 * 0.4 / 2.0 / 2.0)),
            width * 2.0,
            &FontFamily::EbGaramondBold,
        );
        paint.set_image_filter(None);
        self.write_text_centre(
            30.0,
            &paint,
            text,
            Point::new(xy.x - width, xy.y + (height / 2.0)),
            width * 2.0,
            &FontFamily::EbGaramondBold,
        );
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
