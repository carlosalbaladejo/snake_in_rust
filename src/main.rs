mod draw;
mod game;
mod snake;

use piston_window::types::Color;
use piston_window::*;

use crate::draw::to_coord_u32;
use crate::game::Game;

const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const WIDTH: i32 = 20;
const HEIGHT: i32 = 20;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake by Carlos Albaladejo",
        [to_coord_u32(WIDTH), to_coord_u32(HEIGHT)],
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(WIDTH, HEIGHT);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        window.draw_2d(&event, |c, g, _d| {
            clear(BACKGROUND_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
