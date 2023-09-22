use division_engine_rust::{
    canvas::{decoration::Decoration, rect::Rect, rect_draw_system::RectDrawSystem, color::Color32},
    core::{Context, LifecycleManager},
};
use division_math::{Matrix4x4, Vector2};

struct MyDelegate {
    rect_draw_system: RectDrawSystem,
}

fn main() {
    let mut delegate = MyDelegate {
        rect_draw_system: RectDrawSystem::new(),
    };
    let mut context = Context::builder()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .build(&mut delegate)
        .unwrap();

    context.run();
}

impl LifecycleManager for MyDelegate {
    fn init(&mut self, context: &mut Context) {
        self.rect_draw_system.init(context);

        let red_brush = Decoration { color: Color32::red() };
        let purple_brush = Decoration { color: Color32::purple() };

        let red_rects = [
            Rect::from_center_and_size(Vector2::new(100., 100.), Vector2::new(100., 100.)),
            Rect::from_center_and_size(Vector2::new(0., 0.), Vector2::new(50., 50.)),
        ];

        let purple_rects = [
            Rect::from_center_and_size(Vector2::new(512., 512.), Vector2::new(200., 200.))
        ];

        for r in red_rects {
            self.rect_draw_system.draw_rect(context, r, red_brush);
        }

        for r in purple_rects {
            self.rect_draw_system.draw_rect(context, r, purple_brush);
        }
    }

    fn update(&mut self, context: &mut Context) {
        let size = context.get_window_size();
        self.rect_draw_system
            .set_view_matrix(context, Matrix4x4::ortho(0., size.x, 0., size.y));
    }

    fn error(&mut self, _context: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_draw_system.cleanup(context);
    }
}
