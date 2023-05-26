use super::DivisionCore;

pub trait DivisionCoreDelegate {
    fn init(&self, core: &mut DivisionCore);
    fn update(&self, core: &mut DivisionCore);
}