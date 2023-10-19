/*
   TODO: Now it works incorrectly, because screen clears for every system! 
         There is a need to make a global store for render passes,
         rather than for each system
*/

use std::path::Path;

use division_engine_rust::{
    canvas::{
        border_radius::BorderRadius,
        color::Color32,
        decoration::Decoration,
        rect::Rect,
        rect_draw_system::{DrawableRect, RectDrawSystem},
        text_draw_system::TextDrawSystem,
    },
    core::{
        Context, CoreRunner, DivisionId, Image, ImageSettings, LifecycleManager,
        LifecycleManagerBuilder,
    },
    EngineState,
};

use division_math::{Vector2, Vector4};

struct MyLifecycleManagerBuilder {}

struct RectInfo {
    index: usize,
    id: DivisionId,
}

struct MyLifecycleManager {
    rects: Vec<DivisionId>,
    rects_to_remove: Vec<RectInfo>,
    animated_borders_rect: DivisionId,
    borders_animation_incr: i32,

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

    fn build(&mut self, state: &mut EngineState) -> Self::LifecycleManager {
        let context = &mut state.context;

        let mut manager = MyLifecycleManager {
            rect_draw_system: RectDrawSystem::new(context),
            text_draw_system: TextDrawSystem::new(
                context,
                &Path::new("resources")
                    .join("fonts")
                    .join("Roboto-Medium.ttf"),
            ),
            rects: Vec::new(),
            rects_to_remove: Vec::new(),

            borders_animation_incr: 1,
            animated_borders_rect: 0,
        };
        manager.draw(context);

        manager
    }
}

impl LifecycleManager for MyLifecycleManager {
    fn update(&mut self, core_state: &mut EngineState) {
        let context = &mut core_state.context;
        let size = context.get_window_size();
        self.rect_draw_system.update(context, size);

        self.text_draw_system.set_canvas_size(context, size);
        self.text_draw_system.update(context);

        for (index, id) in self.rects.iter().enumerate() {
            let id = *id;
            let r = self.rect_draw_system.get_rect_mut(id);
            r.rect.center.x += 0.5;
            r.rect.center.y += (r.rect.center.x * 0.1).sin();

            if id == self.animated_borders_rect {
                let v: &mut Vector4 = &mut r.decoration.border_radius;
                let r = *v + Vector4::one() * self.borders_animation_incr as f32;
                *v = r;

                if v.x > 90. || v.x < 0. {
                    self.borders_animation_incr *= -1;
                }
            }

            if r.rect.center.x >= 512. {
                self.rects_to_remove.push(RectInfo { index, id });
            }
        }

        for remove_rect in &self.rects_to_remove {
            self.rect_draw_system.remove_rect(remove_rect.id);
            self.rects.remove(remove_rect.index);
        }
        self.rects_to_remove.clear();
    }

    fn error(&mut self, _: &mut EngineState, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, core_state: &mut EngineState) {
        let context = &mut core_state.context;
        self.rect_draw_system.cleanup(context);
        self.text_draw_system.cleanup(context);
    }
}

impl MyLifecycleManager {
    fn draw(&mut self, context: &mut Context) {
        context.set_clear_color(Color32::white().into());

        let nevsky = Image::create_bundled_image(
            &Path::new("resources").join("images").join("nevsky.jpg"),
            ImageSettings::with_vertical_flip(true),
        )
        .unwrap();
        let nevsky = context.create_texture_buffer_from_image(&nevsky).unwrap();

        let red_brush = Decoration {
            color: Color32::red(),
            border_radius: BorderRadius::all(1.),
            texture: nevsky,
        };
        let purple_brush = Decoration {
            color: Color32::purple(),
            border_radius: BorderRadius::all(0.),
            texture: self.rect_draw_system.white_texture_id(),
        };

        let red_rects = [
            Rect::from_bottom_left(Vector2::new(100., 100.), Vector2::new(100., 100.)),
            Rect::from_bottom_left(Vector2::new(0., 0.), Vector2::new(50., 50.)),
        ];

        let purple_rects = [Rect::from_center(
            Vector2::new(0., 512.),
            Vector2::new(200., 200.),
        )];

        for rect in red_rects {
            self.rects.push(self.rect_draw_system.add_rect(
                context,
                DrawableRect {
                    rect,
                    decoration: red_brush,
                },
            ));
        }

        for rect in purple_rects {
            self.rects.push(self.rect_draw_system.add_rect(
                context,
                DrawableRect {
                    rect,
                    decoration: purple_brush,
                },
            ));
        }

        self.animated_borders_rect = *self.rects.last().unwrap();

        self.text_draw_system
            .draw_text_line(
                context,
                // Uncomment this to get error
                // "qwertyuiop[]asdfghjkl;'\\zxcvnm,./QWERTYUIOP{}ASDFGHJKL:\"|ZXCVBNM<>?",
                "New text",
                64.,
                Vector2::new(256., 128.),
                Color32::from_rgb_hex(0x757575),
            )
            .unwrap();
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
