pub mod draw_board;

use graphics::*;
use ::model::board::{CELL_COUNT, IX_TO_COORDS};
use ::controller::BoardController;
use graphics::character::CharacterCache;
use std::fmt::Debug;
pub struct BoardView;
use graphics::math::translate;

impl BoardView {
    pub fn new() -> Self {
        BoardView
    }

    fn hex_coords_offset(x: usize, y: usize) -> (f64, f64) {
        let yf = y as f64;
        let xf = x as f64;
        let x_x = 1.0 * draw_board::HEX_WIDTH;
        let x_y = 0.0 * draw_board::HEX_HEIGHT;
        let y_x = -0.5 * draw_board::HEX_WIDTH;
        let y_y = -0.75 * draw_board::HEX_HEIGHT;
        (xf * x_x + yf * y_x, xf * x_y + yf * y_y) // TODO: replace with linalg operations
    }

    pub fn draw<G: Graphics, E: Debug, C>(&self, controller: &BoardController, c: &Context, g: &mut G, glyphs: &mut C)
    where C: CharacterCache<Texture=G::Texture, Error=E> {
        const ORANGE: [f32; 4] = [189.0/255.0, 148.0/255.0, 49.0/255.0, 1.0];
        const DARK_ORANGE: [f32; 4] = [139.0/255.0, 118.0/255.0, 19.0/255.0, 1.0];
        let base_c = c.reset().scale(0.1, 0.1);
        for i in 0..CELL_COUNT {
            let (x, y) = IX_TO_COORDS[i];
            let (x_offset, y_offset) = Self::hex_coords_offset(x, y);
            let new_c = base_c.trans(x_offset, y_offset);
            draw_board::draw_hex(ORANGE, new_c, g);            
        }
        for (x, y) in controller.board.selected_tile {
            let (x_offset, y_offset) = Self::hex_coords_offset(x, y);
            let new_c = base_c.trans(x_offset, y_offset);
            draw_board::draw_hex(DARK_ORANGE, new_c, g); 
        }

        for i in 0..CELL_COUNT {
            let (x, y) = IX_TO_COORDS[i];
            let (x_offset, y_offset) = Self::hex_coords_offset(x, y);
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            text(BLACK, 20, &"a", glyphs, 
                c.transform.trans(400.,400.).trans(-6.,4.).prepend_transform(translate([x_offset/10.0, y_offset/10.0]))
                , g).unwrap();
        }
    }
}
