use super::Core;

pub trait CoreDelegate {
    fn init(&mut self, core: &mut Core);
    fn update(&mut self, core: &mut Core);
}
