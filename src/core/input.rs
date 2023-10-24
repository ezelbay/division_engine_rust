use std::mem::MaybeUninit;

use super::{Context, ffi::input::{division_engine_input_get_input, DivisionInput}};

impl Context {
    pub fn get_input(&mut self) -> DivisionInput {
        let mut input = MaybeUninit::uninit();
        unsafe {
            division_engine_input_get_input(self, input.as_mut_ptr());
            input.assume_init()
        }
    }
}