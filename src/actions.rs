use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
pub enum Action {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    Swap,
    RotateCw,
    RotateCcw,
}

pub struct ActionsHeld(HashMap<Action, usize>);

impl ActionsHeld {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn just_pressed(&self, ticks: usize, control: &Action) -> bool {
        self.held_for(ticks, control, |held_for| held_for == 0)
    }

    pub fn held_for<F: Fn(usize) -> bool>(
        &self,
        ticks: usize,
        control: &Action,
        functor: F,
    ) -> bool {
        self.get(control)
            .map(|&held_since| ticks - held_since)
            .is_some_and(functor)
    }
}

impl std::ops::Deref for ActionsHeld {
    type Target = HashMap<Action, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ActionsHeld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
