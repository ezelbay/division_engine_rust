use super::context::DivisionContext;
use super::keycode::Keycode;

#[repr(u32)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Other(u32),
}

#[repr(C)]
pub struct DivisionMouseInput
{
    pub pos_x: i32,
    pub pos_y: i32,

    pub mouse_button_state_mask: u32
}

#[repr(C)]
pub struct DivisionKeyboardInput
{
    key_state_mask: [u32; 4]
}

#[repr(C)]
pub struct DivisionInput
{
    pub mouse: DivisionMouseInput,
    pub keyboard: DivisionKeyboardInput,
}

extern "C" {
    pub fn division_engine_input_get_input(
        context: *const DivisionContext, out_input: *mut DivisionInput
    );
}

impl DivisionKeyboardInput {
    pub fn is_key_pressed(&self, keycode: Keycode) -> bool {
        let keycode = keycode as u32;
        let mask_index = keycode / 32;
        let mask_offset = keycode % 32;

        (self.key_state_mask[mask_index as usize] & (1 << mask_offset)) != 0
    }
}

impl DivisionMouseInput {
    pub fn is_button_pressed(&self, mouse_button: MouseButton) -> bool {
        let mouse_button: u32 = mouse_button.into();
        (self.mouse_button_state_mask & (1 << mouse_button)) != 0
    }
}

impl From<MouseButton> for u32 {
    fn from(value: MouseButton) -> Self {
        let ptr = &value as *const MouseButton as *const u32;
        unsafe {
            *ptr
        }
    }
}