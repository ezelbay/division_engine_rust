use std::mem::ManuallyDrop;

use super::Context;

pub struct CoreState {
    pub context: ManuallyDrop<Box<Context>>,
}