mod snake;
mod game;
mod drawing;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use drawing::to_gui_coord_u32;

const BACK_COLOR: Color = [0.204, 0.286, 0.369, 1.0];

fn main() {
    let (width, height) = (30, 30);

    // Create a window
    let mut window: PistonWindow = WindowSettings::new("贪吃蛇",
        [to_gui_coord_u32(width), to_gui_coord_u32(height)]).exit_on_esc(true).build().unwrap();

    // Create a snake
    let mut game = Game::new(width, height);

    // Event loop
    while let Some(event) = window.next() {

        // Catch the events of the keyboard
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Draw all of them
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // Update the state of the game
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}