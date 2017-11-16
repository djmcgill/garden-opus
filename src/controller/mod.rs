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

    /// Given the coordinates in pixels, select the hex coords
    fn on_click(x: usize, y: usize) -> (usize, usize) {
        unimplemented!()
    }
}
