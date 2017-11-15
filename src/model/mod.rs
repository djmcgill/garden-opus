pub mod board;
use self::board::*;

pub mod elements;
use self::elements::*;

use cgmath::*;

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub board: Board,
    pub selected_tile: Option<(usize, usize)>, // The hex x, y index from the top left.
    pub active_metal: Option<BaseMetal>, // The metal that can be combined with quicksilver, if any.
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::empty(),
            selected_tile: None,
            active_metal: Some(BaseMetal::Lead),
        }
    }

    pub fn select_tile(&mut self, x: usize, y: usize) {
        if let Some(tile) = self.board.get_mut(x as isize, y as isize) {
            if let Some(atom@Atom::BaseMetal(BaseMetal::Gold)) = *tile {
                // Gold was selected
                if self.active_metal == Some(BaseMetal::Gold) {
                    self.active_metal = None;
                    *tile = None;
                }
            } else {
                // Gold was not selected
                if let Some(xy) = self.selected_tile {
                    unimplemented!(); // Check for matches
                } else {
                    // TODO: check for valid selectable tile
                    self.selected_tile = Some((x, y))
                }
            }
        }
    }
}
