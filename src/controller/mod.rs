use piston::input::GenericEvent;
use ::model::board::Board;

pub struct BoardController {
    pub board: Board,
}

impl BoardController {
    /// Creates a new board controller.
    pub fn new(board: Board) -> Self {
        BoardController {board}
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
