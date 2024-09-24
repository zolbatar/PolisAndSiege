use skia_safe::{Canvas, Color, ImageFilter, Paint, PaintStyle, Point, Vector};
use skia_safe::colors::BLACK;
use skia_safe::image_filters::drop_shadow_only;
use crate::model::location::Location;

pub struct City {
    name: String,
    location: Location,
    population: i64,
    paint_territory: Color,
}

static MIN_SIZE: f32 = 1.0;
static MAX_SIZE: f32 = 2.5;

impl City {
    pub fn new(name: String, latitude: f32, longitude: f32, population: i64, paint_territory: Color) -> Self {
        City {
            name,
            location: Location::new(latitude, longitude),
            population,
            paint_territory,
        }
    }

    pub fn render(&self, canvas: &Canvas, drop_shadow: &ImageFilter) {
        let mut paint = Paint::default();
        paint.set_anti_alias(false);
        paint.set_style(PaintStyle::Fill);

        // Size
        let size = City::log_transform(self.population as f32).clamp(MIN_SIZE, MAX_SIZE);

        // Shadow
        paint.set_color4f(BLACK, None);
        let drop_shadow = drop_shadow_only(Vector::new(1.5, -1.5), (1.5, 1.5), BLACK, None, None, None).expect("Can't create drop shadow filter");
        paint.set_image_filter(Some(drop_shadow.clone()));
        canvas.draw_circle(Point::new(self.location.x, self.location.y), size, &paint);

        // Draw
        let mut paint_fill = Paint::default();
        paint_fill.set_style(PaintStyle::Fill);
        paint_fill.set_color(self.paint_territory);
        //paint_fill.setColor(Skia::MixColors(paint_territory, SkColors::kWhite.toSkColor(), 0.5f));
        paint_fill.set_image_filter(None);
        canvas.draw_circle(Point::new(self.location.x, self.location.y), size, &paint_fill);

        // Outline
        let mut paint_outline = Paint::default();
        paint_outline.set_anti_alias(true);
        paint_outline.set_style(PaintStyle::Stroke);
        paint_outline.set_color4f(BLACK, None);
        paint_outline.set_stroke_width(size / 8.0);
        canvas.draw_circle(Point::new(self.location.x, self.location.y), size, &paint_outline);
    }

    fn log_transform(x: f32) -> f32 {
        const MIN_INPUT: f32 = 100_000.0;
        const MAX_INPUT: f32 = 25_000_000.0;

        // Calculate log_min and log_max for normalization
        let log_min = MIN_INPUT.ln(); // ln() is the natural logarithm in Rust
        let log_max = MAX_INPUT.ln();

        // Calculate the log of the input value (no need to add 1 if x is non-zero)
        let log_x = x.ln();

        // Normalize log_x to a range between 0 and 1
        let normalized_log_x = (log_x - log_min) / (log_max - log_min);

        // Scale and shift to the desired output range
        let output = MIN_SIZE + normalized_log_x * (MAX_SIZE - MIN_SIZE);

        output
    }
}