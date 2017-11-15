pub mod draw_board;

use graphics::*;
use ::model::board::{CELL_COUNT, IX_TO_COORDS};
use ::controller::BoardController;
use graphics::character::CharacterCache;
use std::fmt::Debug;
pub struct BoardView;

impl BoardView {
    pub fn new() -> Self {
        BoardView
    }

    fn hex_ix_offset(ix: usize) -> (f64, f64) {
        let (x, y) = IX_TO_COORDS[ix];
        let yf = y as f64;
        let xf = x as f64;
        let x_x = 1.0 * draw_board::HEX_WIDTH;
        let x_y = 0.0 * draw_board::HEX_HEIGHT;
        let y_x = -0.5 * draw_board::HEX_WIDTH;
        let y_y = 0.75 * draw_board::HEX_HEIGHT;
        (xf * x_x + yf * y_x, xf * x_y + yf * y_y) // TODO: replace with linalg operations
    }

    pub fn draw<G: Graphics, E: Debug, C>(&self, controller: &BoardController, c: &Context, g: &mut G, glyphs: &mut C)
    where C: CharacterCache<Texture=G::Texture, Error=E> {
        let base_c = c.reset().scale(0.1, 0.1);
        for i in 0..CELL_COUNT {
            
            let (x_offset, y_offset) = Self::hex_ix_offset(i);
            let new_c = base_c.trans(x_offset, y_offset);
            draw_board::draw_hex(new_c, g);            
        }

        let transform = c.transform.trans(400.0, 400.0);
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        text(BLACK, 50, draw_board::QUICKSILVER_SYMBOL, glyphs, transform, g).unwrap();
    }
}
