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

    pub fn draw<G: Graphics, E: Debug, C>(&self, controller: &BoardController, c: &Context, g: &mut G, glyphs: &mut C)
    where C: CharacterCache<Texture=G::Texture, Error=E> {
        let base_c = c.reset().scale(0.1, 0.1);
        for i in 0..CELL_COUNT {
            let (y, x) = IX_TO_COORDS[i];
            let yf = y as f64;
            let xf = x as f64;
            let x_x = 1.0 * draw_board::HEX_WIDTH;
            let x_y = 0.0;
            let y_x = -0.5 * draw_board::HEX_WIDTH;
            let y_y = 0.75 * draw_board::HEX_HEIGHT;
            let new_c = base_c
                .trans(xf * x_x, xf * x_y)
                .trans(yf * y_x, yf * y_y);
            draw_board::draw_hex(new_c, g);            
        }

        let transform = c.transform.trans(400.0, 400.0);
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        text(BLACK, 50, draw_board::QUICKSILVER_SYMBOL, glyphs, transform, g).unwrap();
    }
}
