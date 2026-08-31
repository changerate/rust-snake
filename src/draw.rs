use piston_window::graphics::{Context, rectangle};
use piston_window::graphics::types::Color;
use piston_window::graphics::Graphics;

// use piston_window::*;
// use piston_window::types::Color;
//
// use rand::{thread_rng, Rng};
//
// use snake::{Direction, Snake};
// use draw::{draw_block, draw_rectangle};
//
// const FOOD_COLOR: Colo = [0.80, 0.00, 0.00, 1.0];
// const BORDER_COLOR: Colo = [0.00, 0.00, 0.00, 1.0];
// const GAMEOVER_COLOR: Colo = [0.90, 0.00, 0.00, 0.5];
//
// const MOVING_PERIOD: f64 = 0.1;
// const RESTART_TIME: f64 = 1.0;
const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block<G: Graphics>(color: Color, x: i32, y: i32, con: &Context, g: &mut G) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle<G: Graphics>(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
        x,
        y,
        BLOCK_SIZE * (width as f64),
        BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
