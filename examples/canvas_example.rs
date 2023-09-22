use division_engine_rust::{
    canvas::{rect::Rect, rect_drawer::{SolidRect, RectDrawSystem}},
    core::{Context, LifecycleManager},
};
use division_math::{Matrix4x4, Vector2, Vector4};

struct MyDelegate {
    rect_drawer: RectDrawSystem
}

fn main() {
    let mut delegate = MyDelegate {
        rect_drawer: RectDrawSystem::new()
    };
    let mut context = Context::builder()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .build(&mut delegate)
        .unwrap();

    context.run();
}

impl LifecycleManager for MyDelegate {
    fn init(&mut self, context: &mut Context ) {
        let view_matrix = Matrix4x4::ortho(0., 1024., 0., 1024.);
        self.rect_drawer.init(context, view_matrix);

        self.rect_drawer
            .draw_rect(
                context,
                SolidRect {
                rect: Rect::from_center_and_size(
                    Vector2::new(100., 100.),
                    Vector2::new(1024., 1024.),
                ),
                color: Vector4::one(),
            })
            .unwrap();

        self.rect_drawer
            .draw_rect(
                context,
                SolidRect {
                rect: Rect::from_center_and_size(
                    Vector2::new(1., 1.),
                    Vector2::new(50., 50.),
                ),
                color: Vector4::new(1., 0., 0., 1.),
            })
            .unwrap();
    }

    fn update(&mut self, _context: &mut Context) {}

    fn error(&mut self, _context: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_drawer.cleanup(context);
    }
}