use macroquad::prelude::*;
use rutoe::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "RUTOE Template".to_string(),
        window_width: 960,
        window_height: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut master = Master {
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
        let resources = convert_resources::<Resources>(resources);
    }
}

#[derive(Clone)]
struct Resources {
}
impl ResourceContainer for Resources {}