use piston::input::GenericEvent;
use ::model::GameState;

pub struct BoardController {
    pub board: GameState,
}

impl BoardController {
    /// Creates a new board controller.
    pub fn new(board: GameState) -> Self {
        BoardController {board}
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}
