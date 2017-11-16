pub mod draw_board;

use graphics::*;
use ::model::board::CELL_COUNT;
use ::model::{hex_coords_offset, IX_TO_COORDS};
use ::controller::BoardController;
use graphics::character::CharacterCache;
use std::fmt::Debug;
pub struct BoardView;
use graphics::math::translate;

impl BoardView {
    pub fn new() -> Self {
        BoardView
    }

    pub fn draw<G: Graphics, E: Debug, C>(&self, controller: &BoardController, c: &Context, g: &mut G, glyphs: &mut C)
    where C: CharacterCache<Texture=G::Texture, Error=E> {
        const SCALE: f64 = 0.1;
        const ORANGE: [f32; 4] = [189.0/255.0, 148.0/255.0, 49.0/255.0, 1.0];
        const DARK_ORANGE: [f32; 4] = [139.0/255.0, 118.0/255.0, 19.0/255.0, 1.0];
        let base_c = c.reset().scale(SCALE, SCALE);
        // Draw background hexes
        for i in 0..CELL_COUNT {
            let (x, y) = IX_TO_COORDS[i];
            let xy_offset = hex_coords_offset(x, y);
            let new_c = base_c.trans(xy_offset[0], xy_offset[1]);
            draw_board::draw_hex(ORANGE, new_c, g);            
        }
        // Draw selected tile
        for (x, y) in controller.board.selected_tile {
            let xy_offset = hex_coords_offset(x, y);
            let new_c = base_c.trans(xy_offset[0], xy_offset[1]);
            draw_board::draw_hex(DARK_ORANGE, new_c, g); 
        }
        // Draw labels
        for i in 0..CELL_COUNT {
            let (x, y) = IX_TO_COORDS[i];
            let xy_offset = hex_coords_offset(x, y);
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            const GREY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
            let transform =
                c.transform
                 .trans(400.,400.) // TODO: Is there not some better way to go to the center?
                 .trans(-6.,4.)
                 .prepend_transform(
                     translate([xy_offset[0] * SCALE, xy_offset[1] * SCALE])
                 );
            text(BLACK, 20, &"a", glyphs, transform, g).unwrap();
        }
    }
}
