use super::context::DivisionContext;

#[repr(C)]
#[derive(Debug)]
pub enum DivisionInputState
{
    Up = 0,
    Down = 1,
}

#[repr(C)]
pub struct DivisionMouseInput
{
    pub pos_x: i32,
    pub pos_y: i32,

    pub left_button: DivisionInputState,
    pub right_button: DivisionInputState,
    pub middle_button: DivisionInputState,
}

#[repr(C)]
pub struct DivisionInput
{
    pub mouse: DivisionMouseInput,
}

extern "C" {
    pub fn division_engine_input_get_input(
        context: *const DivisionContext, out_input: *mut DivisionInput
    );
}