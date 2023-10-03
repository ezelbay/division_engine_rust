use std::path::Path;

use division_engine_rust::{
    canvas::{
        border_radius::BorderRadius, color::Color32, decoration::Decoration, rect::Rect,
        rect_draw_system::RectDrawSystem,
    },
    core::{Context, LifecycleManager, TextureFormat},
};

use division_math::Vector2;

struct MyLifecycleManager {
    rect_draw_system: RectDrawSystem,
    text_draw_system: RectDrawSystem,
}

fn main() {
    let mut lifecycle_manager = MyLifecycleManager {
        rect_draw_system: RectDrawSystem::new(),
        text_draw_system: RectDrawSystem::new(),
    };
    let mut context = Context::builder()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .build(&mut lifecycle_manager)
        .unwrap();

    context.run();
}

impl LifecycleManager for MyLifecycleManager {
    fn init(&mut self, context: &mut Context) {
        context.set_clear_color(Color32::white().into());

        let font_path = Path::new("resources")
            .join("fonts")
            .join("Roboto-Medium.ttf");

        let font = context.create_font(&font_path, 16).unwrap();

        let glyph = context.get_font_glyph(font, 'X').unwrap();
        let font_bitmap = context.rasterize_glyph(font, glyph).unwrap();

        context.delete_font(font);

        let texture = context
            .create_texture_buffer_from_data(
                glyph.width,
                glyph.height,
                TextureFormat::R8Uint,
                &font_bitmap,
            )
            .unwrap();

        self.rect_draw_system.init(context);
        self.text_draw_system.init_with_texture(context, texture);

        let red_brush = Decoration {
            color: Color32::red(),
            border_radius: BorderRadius::all(1.),
        };
        let purple_brush = Decoration {
            color: Color32::purple(),
            border_radius: BorderRadius::top_bottom(50., 30.),
        };

        let red_rects = [
            Rect::from_bottom_left(Vector2::new(100., 100.), Vector2::new(100., 100.)),
            Rect::from_bottom_left(Vector2::new(0., 0.), Vector2::new(50., 50.)),
        ];

        let purple_rects = [Rect::from_center(
            Vector2::new(512., 512.),
            Vector2::new(200., 100.),
        )];

        for r in red_rects {
            self.rect_draw_system.draw_rect(context, r, red_brush);
        }

        for r in purple_rects {
            self.rect_draw_system.draw_rect(context, r, purple_brush);
        }

        self.text_draw_system.draw_rect(
            context,
            Rect::from_center(Vector2::new(200., 200.), Vector2::new(100., 100.)),
            Decoration {
                color: Color32::white(),
                border_radius: BorderRadius::all(15.),
            },
        );
    }

    fn update(&mut self, context: &mut Context) {
        let size = context.get_window_size();
        self.rect_draw_system.set_canvas_size(context, size);
        self.text_draw_system.set_canvas_size(context, size);
    }

    fn error(&mut self, _context: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_draw_system.cleanup(context);
        self.text_draw_system.cleanup(context);
    }
}
