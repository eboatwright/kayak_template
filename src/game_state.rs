use rutoe::Viewport;
use rutoe::convert_resources;
use crate::Resources;
use rutoe::ResourceContainer;
use rutoe::State;

pub struct GameState {
}

impl State for GameState {
    fn initialize(&mut self) {
    }

    fn update(&mut self, _viewport: &mut Viewport) {
    }

    fn render(&self, _viewport: &Viewport, resources: &Box<dyn ResourceContainer>) {
        let _resources: &Resources = convert_resources(resources.as_any());
    }
}