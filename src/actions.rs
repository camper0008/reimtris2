use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum Controls {
    Left,
    Right,
    SoftDrop,
    HardDrop,
}

pub struct ControlsHeld(HashMap<Controls, usize>);

impl std::ops::Deref for ControlsHeld {
    type Target = HashMap<Controls, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ControlsHeld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
