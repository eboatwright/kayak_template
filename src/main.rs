use macroquad::prelude::*;
use rutoe::*;

#[macroquad::main(|| {
    Conf {
        window_title: "RUTOE Template".to_string(),
        window_width: 960,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
})]
async fn main() {
    let mut master = Master::default();
        state: Box::new(GameState {
        }),
        resources: Box::new(Resources {
        }),
    };

    start(&mut master).await;
}

struct GameState {
}

impl State for GameState {
    fn initialize(&mut self) {
    }

    fn update(&mut self) {
    }

    fn render(&self, resources: &Box<dyn ResourceContainer>) {
        let resources: &Resources = convert_resources(resources);
    }
}

struct Resources {
}
impl ResourceContainer for Resources {}