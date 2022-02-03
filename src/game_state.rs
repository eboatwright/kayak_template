use kayak::Context;
use crate::Resources;
use kayak::State;

pub struct GameState {
}

impl State for GameState {
    fn initialize(&mut self, _context: &mut Context) {
    }

    fn update(&mut self, _context: &mut Context) {
    }

    fn render(&self, context: &Context) {
        let _resources: &Resources = context.get_resources();
    }
}