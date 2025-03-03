use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum Controls {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    Swap,
    RotateCw,
    RotateCcw,
}

pub struct ControlsHeld(HashMap<Controls, usize>);

impl ControlsHeld {
    pub fn just_pressed(&self, ticks: usize, control: &Controls) -> bool {
        self.held_for(ticks, control, |held_for| held_for == 0)
    }

    pub fn held_for<F: Fn(usize) -> bool>(
        &self,
        ticks: usize,
        control: &Controls,
        functor: F,
    ) -> bool {
        self.get(control)
            .map(|&held_since| ticks - held_since)
            .is_some_and(functor)
    }
}

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
