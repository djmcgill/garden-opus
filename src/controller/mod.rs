use piston::input::GenericEvent;
use piston::window::Window;
use ::model::GameState;
use ::model::model_to_hex;

pub struct BoardController {
    pub board: GameState,
    pub window_width: u32,
    pub window_height: u32,
    pub hex_scale: f64,
}

impl BoardController {
    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(xy) = e.mouse_cursor_args() {
            let (x, y) = self.cursor(xy[0], xy[1]);
            if x >= 0 && y >= 0 && self.board.board.get(x as usize, y as usize).is_some() {
                self.board.selected_tile = Some((x as usize, y as usize));
            }
            // self.board.current_mouse_pos = Vector2::from(xy);
        }


    }

    /// Given the coordinates in pixels, select the hex coords
    fn cursor(&self, x: f64, y: f64) -> (isize, isize) {
        let m_x = (x as f64 / self.window_width as f64) * 2. - 1.;
        let m_y = -(y as f64 / self.window_height as f64) * 2. +1.;
        model_to_hex(m_x/self.hex_scale, m_y/self.hex_scale)
    }
}
