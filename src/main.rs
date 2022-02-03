mod game_state;
mod resources;

use kayak::Viewport;
use crate::game_state::GameState;
use kayak::start;
use kayak::Master;
use crate::resources::Resources;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Kayak Template".to_string(),
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

    let viewport = Viewport::new(vec2(960.0, 600.0));

    start(&mut master, viewport).await;
}