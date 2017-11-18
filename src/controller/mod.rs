use piston::input::GenericEvent;
use piston::window::Window;
use ::model::GameState;
use ::model::model_to_hex;

pub struct BoardController {
    pub board: GameState,
}

impl BoardController {
    /// Creates a new board controller.
    pub fn new(board: GameState) -> Self {
        BoardController {board}
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) { // TODO: static dispatch on Window
        if let Some(xy) = e.mouse_cursor_args() {
            let (x, y) = Self::cursor(xy[0], xy[1]);
            if x >= 0 && y >= 0 && self.board.board.get(x as usize, y as usize).is_some() {
                self.board.selected_tile = Some((x as usize, y as usize));
            }
            // self.board.current_mouse_pos = Vector2::from(xy);
        }


    }

    /// Given the coordinates in pixels, select the hex coords
    fn cursor(x: f64, y: f64) -> (isize, isize) {
        const WIDTH: f64 = 800.; // TODO :use real values
        const HEIGHT: f64 = 800.;
        let m_x = (x as f64 / WIDTH) * 2. - 1.;
        let m_y = -(y as f64 / HEIGHT) * 2. +1.;
        model_to_hex(m_x*10., m_y*10.)
    }
}
