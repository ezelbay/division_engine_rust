/*
   TODO: Now it works incorrectly, because screen clears for every system!
         There is a need to make a global store for render passes,
         rather than for each system
*/

use std::{path::Path, time::Instant};

use division_engine_rust::{
    canvas::{
        border_radius::BorderRadius,
        color::Color32,
        decoration::Decoration,
        rect::Rect,
        rect_renderer::RectRenderer,
        renderable_rect::RenderableRect,
        renderable_text::RenderableText,
        renderer::{RenderQueue, Renderer},
        text_renderer::TextRenderer,
    },
    core::{
        Context, CoreRunner, DivisionId, Image, ImageSettings, LifecycleManager,
        LifecycleManagerBuilder, TextureDescriptor, TextureFormat,
    },
};

use division_math::Vector2;

#[repr(transparent)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct UniformData {
    size: Vector2,
}

struct MyLifecycleManagerBuilder;

struct MyLifecycleManager {
    rects: Vec<RenderableRect>,
    texts: Vec<RenderableText>,
    render_queue: RenderQueue,
    screen_size_uniform: DivisionId,
    render_draw_time: Instant,

    rect_draw_system: RectRenderer,
    text_draw_system: TextRenderer,

    _white_texture: DivisionId,
}

fn main() {
    CoreRunner::new()
        .window_size(1024, 1024)
        .window_title("Hello rect drawer")
        .run(MyLifecycleManagerBuilder)
        .unwrap();
}

impl LifecycleManagerBuilder for MyLifecycleManagerBuilder {
    type LifecycleManager = MyLifecycleManager;

    fn build(&mut self, context: &mut Context) -> Self::LifecycleManager {
        let nevsky_texture = {
            let image = Image::create_bundled_image(
                &Path::new("resources").join("images").join("nevsky.jpg"),
                ImageSettings::with_vertical_flip(true),
            )
            .unwrap();
            context.create_texture_buffer_from_image(&image).unwrap()
        };

        let white_texture = context
            .create_texture_buffer_from_data(
                &TextureDescriptor::new(1, 1, TextureFormat::RGBA32Uint),
                &[255u8; 4],
            )
            .unwrap();

        let screen_size_uniform = context
            .create_uniform_buffer_with_size_of::<UniformData>()
            .unwrap();

        let manager = MyLifecycleManager {
            rect_draw_system: RectRenderer::new(context, screen_size_uniform),
            text_draw_system: TextRenderer::new(
                context,
                screen_size_uniform,
                &Path::new("resources")
                    .join("fonts")
                    .join("Roboto-Medium.ttf"),
            ),
            render_queue: RenderQueue::new(Color32::white()),
            screen_size_uniform,
            rects: create_rects(nevsky_texture, white_texture),
            texts: create_texts(),
            render_draw_time: Instant::now(),
            _white_texture: white_texture,
        };

        manager
    }
}

impl LifecycleManager for MyLifecycleManager {
    fn draw(&mut self, context: &mut Context) {
        let now = Instant::now();
        let render_time_diff = (now - self.render_draw_time).as_millis();

        let last_text = self.texts.last_mut().unwrap();
        // TODO: fix spaces issue
        last_text.text = format!("The overall render time is:{render_time_diff}ms");

        self.update_window_size(context);

        self.rect_draw_system.before_render_frame(context);
        self.text_draw_system.before_render_frame(context);

        self.rect_draw_system.enqueue_render_passes(
            context,
            &mut self.rects,
            &mut self.render_queue,
        );
        self.text_draw_system.enqueue_render_passes(
            context,
            &mut self.texts,
            &mut self.render_queue,
        );

        self.render_queue.draw(context);

        self.rect_draw_system.after_render_frame(context);
        self.text_draw_system.after_render_frame(context);

        self.render_draw_time = Instant::now();
    }

    fn error(&mut self, _: &mut Context, _error_code: i32, message: &str) {
        panic!("{message}");
    }

    fn cleanup(&mut self, context: &mut Context) {
        self.rect_draw_system.cleanup(context);
        self.text_draw_system.cleanup(context);
    }
}

impl MyLifecycleManager {
    fn update_window_size(&mut self, context: &mut Context) {
        let window_size = context.get_window_size();
        let screen_size =
            context.uniform_buffer_data::<UniformData>(self.screen_size_uniform);
        screen_size.data.size = window_size;
    }
}

fn create_rects(
    nevsky_texture: DivisionId,
    white_texture: DivisionId,
) -> Vec<RenderableRect> {
    vec![
        RenderableRect::new(
            Rect::from_bottom_left(Vector2::new(100., 100.), Vector2::new(100., 100.)),
            Decoration {
                color: Color32::red(),
                border_radius: BorderRadius::all(0.),
                texture_id: white_texture,
            },
        ),
        RenderableRect::new(
            Rect::from_bottom_left(Vector2::new(0., 0.), Vector2::new(50., 50.)),
            Decoration {
                color: Color32::purple(),
                border_radius: BorderRadius::all(10.),
                texture_id: nevsky_texture,
            },
        ),
    ]
}

fn create_texts() -> Vec<RenderableText> {
    vec![
        RenderableText {
            color: Color32::black(),
            position: Vector2::new(256., 256.),
            font_size: 16.,
            text: String::from("There is a text!"),
        },
        RenderableText {
            color: Color32::red(),
            position: Vector2::new(512., 512.),
            font_size: 20.,
            text: String::from("Another one!"),
        },
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
