use division_engine_rust::{
    canvas::{
        color::Color32, decoration::Decoration, rect::Rect,
        rect_draw_system::RectDrawSystem, border_radius::BorderRadius,
    },
    core::{
        Context, LifecycleManager,
    },
};

use division_math::Vector2;

struct MyLifecycleManager {
    rect_draw_system: RectDrawSystem,
}

fn main() {
    let mut lifecycle_manager = MyLifecycleManager {
        rect_draw_system: RectDrawSystem::new(),
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

        let red_brush = Decoration {
            color: Color32::red(),
            border_radius: BorderRadius::without(),
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
    }

    fn update(&mut self, context: &mut Context) {
        let size = context.get_window_size();
        self.rect_draw_system.set_canvas_size(context, size);
    }

    fn error(&mut self, _context: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_draw_system.cleanup(context);
    }
}
