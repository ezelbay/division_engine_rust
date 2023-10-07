use std::path::Path;

use division_engine_rust::{
    canvas::{
        border_radius::BorderRadius, color::Color32, decoration::Decoration, rect::Rect,
        rect_draw_system::RectDrawSystem, text_draw_system::TextDrawSystem,
    },
    core::{Context, LifecycleManager},
};

use division_math::Vector2;

struct MyLifecycleManager {
    rect_draw_system: RectDrawSystem,
    text_draw_system: Option<TextDrawSystem>,
}

fn main() {
    let mut lifecycle_manager = MyLifecycleManager {
        rect_draw_system: RectDrawSystem::new(),
        text_draw_system: None,
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

        self.rect_draw_system.init(context);
        let mut sys = TextDrawSystem::new(
            context,
            &Path::new("resources")
                .join("fonts")
                .join("Roboto-Medium.ttf"),
        );

        sys.draw_text(
            context,
            "Lorem ipsum",
            64.,
            Vector2::new(256., 256.),
            Color32::from_rgb_hex(0x007192),
        );

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

        self.text_draw_system = Some(sys);
    }

    fn update(&mut self, context: &mut Context) {
        let size = context.get_window_size();
        self.rect_draw_system.set_canvas_size(context, size);

        if let Some(ref mut text_sys) = self.text_draw_system {
            text_sys.set_canvas_size(context, size)
        }
    }

    fn error(&mut self, _context: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_draw_system.cleanup(context);
        if let Some(ref mut text_sys) = self.text_draw_system {
            text_sys.cleanup(context);
        }
    }
}
