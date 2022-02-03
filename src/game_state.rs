use kayak::Viewport;
use kayak::convert_resources;
use crate::Resources;
use kayak::ResourceContainer;
use kayak::State;

pub struct GameState {
}

impl State for GameState {
    fn initialize(&mut self, viewport: &mut Viewport) {
    }

    fn update(&mut self, _viewport: &mut Viewport) {
    }

    fn render(&self, _viewport: &Viewport, resources: &Box<dyn ResourceContainer>) {
        let _resources: &Resources = convert_resources(resources.as_any());
    }
}