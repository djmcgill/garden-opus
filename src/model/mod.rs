pub mod board;
use self::board::*;

pub mod elements;
use self::elements::*;

use cgmath::*;

#[derive(PartialEq, Debug, Default)]
pub struct GameState {
    board: Board,
    selected_tile: Option<Vector2<u8>>, // The hex x, y index from the top left.
    active_metal: Option<BaseMetal>, // The metal that can be combined with quicksilver, if any.
}
