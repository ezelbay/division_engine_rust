use crate::core::{Context, RenderPassInstance};

use super::color::Color32;

pub struct RenderQueue {
    pub clear_color: Color32,
    pub data: Vec<RenderPassInstance>,
}

pub trait Renderer {
    type RenderableData;

    fn before_render_frame(&mut self, context: &mut Context);
    fn enqueue_render_passes(
        &mut self,
        context: &mut Context,
        data: &[Self::RenderableData],
        render_queue: &mut RenderQueue,
    );
    fn after_render_frame(&mut self, context: &mut Context);
}

impl RenderQueue {
    pub fn new(clear_color: Color32) -> RenderQueue {
        RenderQueue { data: Vec::new(), clear_color }
    }

    pub fn enqueue_render_pass(&mut self, render_pass: RenderPassInstance) {
        self.data.push(render_pass);
    }

    pub fn draw(&mut self, context: &mut Context) {
        context.draw_render_passes(*self.clear_color, &self.data);
        self.data.clear();
    }
}