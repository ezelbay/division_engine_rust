use division_engine_rust::{
    canvas::{rect::Rect, rect_drawer::SolidRect},
    core::{Core, CoreDelegate},
};
use division_math::{Matrix4x4, Vector2, Vector4};

struct MyDelegate {
}

fn main() {
    let delegate = Box::new(MyDelegate {});
    let core = Core::builder()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .build(delegate)
        .unwrap();

    core.run();
}

impl CoreDelegate for MyDelegate {
    fn init(&mut self, core: &mut Core) {
        let view_matrix = Matrix4x4::ortho(0., 1024., 0., 1024.);
        let mut rect_drawer = Box::new(core.create_rect_drawer(view_matrix));

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
                rect: Rect::from_center_and_size(Vector2::new(1., 1.), Vector2::new(50., 50.)),
                color: Vector4::new(1., 0., 0., 1.),
            })
            .unwrap()
    }

    fn update(&mut self, _core: &mut Core) {}
}