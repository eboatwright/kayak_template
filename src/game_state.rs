use kayak::UpdateStatus;
use kayak::Context;
use crate::Resources;
use kayak::State;

pub struct GameState {
}

impl Default for GameState {
    fn default() -> Self {
        Self {
        }
    }
}

impl State for GameState {
    fn update(&mut self, _context: &mut Context) -> UpdateStatus {
        UpdateStatus::Ok
    }

    fn render(&self, context: &Context) {
        let _resources: &Resources = context.get_resources();
    }
}