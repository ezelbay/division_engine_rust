use division_engine_rust::{
    canvas::{rect::Rect, rect_drawer::SolidRect},
    core::{Context, LifecycleManager, PinnedContext},
};
use division_math::{Matrix4x4, Vector2, Vector4};

struct MyDelegate {
    context: PinnedContext,
}

fn main() {
    let context = Context::builder()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .build()
        .unwrap();

    let mut delegate = MyDelegate { context };
    delegate.run();
}

impl LifecycleManager for MyDelegate {
    fn init(&mut self) {
        let context = unsafe { self.context_mut() };
        let view_matrix = Matrix4x4::ortho(0., 1024., 0., 1024.);
        let mut rect_drawer = Box::new(context.create_rect_drawer(view_matrix));

        rect_drawer
            .draw_rect(SolidRect {
                rect: Rect::from_center_and_size(
                    Vector2::new(100., 100.),
                    Vector2::new(1024., 1024.),
                ),
                color: Vector4::one(),
            })
            .unwrap();

        rect_drawer
            .draw_rect(SolidRect {
                rect: Rect::from_center_and_size(
                    Vector2::new(1., 1.),
                    Vector2::new(50., 50.),
                ),
                color: Vector4::new(1., 0., 0., 1.),
            })
            .unwrap()
    }

    fn update(&mut self) {}

    fn error(&mut self, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn pinned_context_mut(&mut self) -> &mut PinnedContext {
        &mut self.context
    }
}
