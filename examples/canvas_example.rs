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
        rect_draw_system::{RectDrawSystem, RectInstanceData},
        text_draw_system::TextDrawSystem,
    },
    core::{
        Context, CoreRunner, DivisionId, LifecycleManager, LifecycleManagerBuilder,
        TextureDescriptor, TextureFormat,
    },
};

use division_math::Vector2;

#[repr(transparent)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct UniformData {
    size: Vector2,
}

struct MyLifecycleManagerBuilder {}

struct MyLifecycleManager {
    rects: Vec<RectInstanceData>,
    white_texture: DivisionId,
    screen_size_uniform: DivisionId,

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

    fn build(&mut self, context: &mut Context) -> Self::LifecycleManager {
        let white_texture = context
            .create_texture_buffer_from_data(
                &TextureDescriptor::new(1, 1, TextureFormat::RGBA32Uint),
                &[255; 4],
            )
            .unwrap();

        let screen_size_uniform = context
            .create_uniform_buffer_with_size_of::<UniformData>()
            .unwrap();

        let manager = MyLifecycleManager {
            rect_draw_system: RectDrawSystem::new(context, screen_size_uniform),
            text_draw_system: TextDrawSystem::new(
                context,
                &Path::new("resources")
                    .join("fonts")
                    .join("Roboto-Medium.ttf"),
            ),
            screen_size_uniform,
            rects: create_rects(),
            white_texture,
        };

        manager
    }
}

impl LifecycleManager for MyLifecycleManager {
    fn update(&mut self, context: &mut Context) {
        {
            let window_size = context.get_window_size();
            let screen_size =
                context.uniform_buffer_data::<UniformData>(self.screen_size_uniform);
            screen_size.data.size = window_size;
        }

        context.set_clear_color(Color32::white().into());

        self.rect_draw_system.begin_frame_render();
        let mut passes = Vec::new();
        passes.push(self.rect_draw_system.create_new_pass(
            context,
            self.white_texture,
            &self.rects,
        ));

        context.draw_render_passes(unsafe { std::mem::transmute(passes.as_slice()) });
    }

    fn error(&mut self, _: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_draw_system.cleanup(context);
        self.text_draw_system.cleanup(context);
    }
}

fn create_rects() -> Vec<RectInstanceData> {
    vec![
        RectInstanceData::new(
            Rect::from_bottom_left(Vector2::new(100., 100.), Vector2::new(100., 100.)),
            Decoration {
                color: Color32::red(),
                border_radius: BorderRadius::all(0.),
            },
        ),
        RectInstanceData::new(
            Rect::from_bottom_left(Vector2::new(0., 0.), Vector2::new(50., 50.)),
            Decoration {
                color: Color32::purple(),
                border_radius: BorderRadius::all(10.),
            },
        ),
    ]
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
