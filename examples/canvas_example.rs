use division_engine_rust::{
    canvas::{rect::Rect, rect_drawer::{SolidRect, RectDrawer}},
    core::{Context, LifecycleManager, PinnedContext, PinnedContextGetter},
};
use division_math::{Matrix4x4, Vector2, Vector4};

struct MyDelegate {
    context: PinnedContext,
    rect_drawer: Option<Box<RectDrawer>>
}

fn main() {
    let context = Context::builder()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .build()
        .unwrap();

    let mut delegate = MyDelegate { context, rect_drawer: None };
    delegate.run();
}

impl LifecycleManager for MyDelegate {
    fn init(&mut self) {
        let context = unsafe { self.context.context_mut() };
        let view_matrix = Matrix4x4::ortho(0., 1024., 0., 1024.);
        let mut rect_drawer = Box::new(RectDrawer::new(context, view_matrix));

        rect_drawer
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

        rect_drawer
            .draw_rect(context,
                SolidRect {
                rect: Rect::from_center_and_size(
                    Vector2::new(1., 1.),
                    Vector2::new(50., 50.),
                ),
                color: Vector4::new(1., 0., 0., 1.),
            })
            .unwrap();

        self.rect_drawer = Some(rect_drawer);
    }

    fn update(&mut self) {}

    fn error(&mut self, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    #[inline(always)]
    fn pinned_context_mut(&mut self) -> &mut PinnedContext {
        &mut self.context
    }
}

impl Drop for MyDelegate {
    fn drop(&mut self) {
        if let Some(rd) = &mut self.rect_drawer {
            rd.delete(unsafe { self.context.context_mut() });
        }
    }
}
