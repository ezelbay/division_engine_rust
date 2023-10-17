use std::path::Path;

use division_engine_rust::{
    canvas::{
        border_radius::BorderRadius, color::Color32, decoration::Decoration, rect::Rect,
        rect_draw_system::RectDrawSystem, text_draw_system::TextDrawSystem,
    },
    core::{LifecycleManager, CoreRunner, CoreState, LifecycleManagerBuilder, Context},
};

use division_math::Vector2;

struct MyLifecycleManagerBuilder {

}

struct MyLifecycleManager {
    rect_draw_system: RectDrawSystem,
    text_draw_system: TextDrawSystem,
}

fn main() {
    CoreRunner::new()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .run(MyLifecycleManagerBuilder {})
        .unwrap();
}

impl LifecycleManagerBuilder for MyLifecycleManagerBuilder {
    type LifecycleManager = MyLifecycleManager;

    fn build(&mut self, state: &mut CoreState) -> Self::LifecycleManager {
        let context = &mut state.context;

        let mut manager = MyLifecycleManager {
            rect_draw_system: RectDrawSystem::new(context),
            text_draw_system: TextDrawSystem::new(context, &Path::new("resources")
            .join("fonts")
            .join("Roboto-Medium.ttf"))
        };
        manager.draw(context);

        manager
    }
}

impl LifecycleManager for MyLifecycleManager {
    fn update(&mut self, core_state: &mut CoreState) {
        let context = &mut core_state.context;
        let size = context.get_window_size();
        self.rect_draw_system.set_canvas_size(context, size);

        self.text_draw_system.set_canvas_size(context, size);
        self.text_draw_system.update(context);
    }

    fn error(&mut self, _: &mut CoreState, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, core_state: &mut CoreState) {
        let context = &mut core_state.context;
        self.rect_draw_system.cleanup(context);
        self.text_draw_system.cleanup(context);
    }
}

impl MyLifecycleManager {
    fn draw(&mut self, context: &mut Context) {
        context.set_clear_color(Color32::white().into());

        self.text_draw_system.draw_text_line(
            context,
            // Uncomment this to get error
            // "qwertyuiop[]asdfghjkl;'\\zxcvnm,./QWERTYUIOP{}ASDFGHJKL:\"|ZXCVBNM<>?",
            "New text",
            64.,
            Vector2::new(256., 128.),
            Color32::from_rgb_hex(0x757575),
        ).unwrap();

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
    }
}

impl Drop for MyLifecycleManagerBuilder {
    fn drop(&mut self) {
        println!("Builder was dropped")
    }
}

impl Drop for MyLifecycleManager {
    fn drop(&mut self) {
        println!("Manager was dropped")
    }
}
