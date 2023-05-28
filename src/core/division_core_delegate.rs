use super::DivisionCore;

pub trait DivisionCoreDelegate {
    fn init(&mut self, core: &mut DivisionCore);
    fn update(&mut self, core: &mut DivisionCore);
}