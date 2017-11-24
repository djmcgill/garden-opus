pub mod draw_board;

use graphics::*;
use ::model::board::CELL_COUNT;
use ::model::{hex_coords_offset, IX_TO_COORDS};
use ::controller::BoardController;
use graphics::character::CharacterCache;
use std::fmt::Debug;
use graphics::math::translate;

/// Contains the information used to display the board on the screen
pub struct BoardView;

impl BoardView {
    pub fn new() -> Self {
        BoardView
    }

    pub fn draw<G: Graphics, E: Debug, C>(&self, controller: &BoardController, c: &Context, g: &mut G, glyphs: &mut C)
    where C: CharacterCache<Texture=G::Texture, Error=E> {
        const ORANGE: [f32; 4] = [189.0/255.0, 148.0/255.0, 49.0/255.0, 1.0];
        const DARK_ORANGE: [f32; 4] = [139.0/255.0, 118.0/255.0, 19.0/255.0, 1.0];
        let base_c = c.reset().scale(controller.hex_scale, controller.hex_scale);
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
            for ref atom in controller.board.board.0[i] {
                let (x, y) = IX_TO_COORDS[i];
                let xy_offset = hex_coords_offset(x, y);
                let hex_transformation = translate([xy_offset[0] * controller.hex_scale, xy_offset[1] * controller.hex_scale]);
                const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
                const GREY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
                let ref label = atom.print();
                let transform =
                    c.transform
                    .trans(400.,400.) // TODO: Is there not some better way to go to the center?
                    .trans(-6.,4.)
                    .prepend_transform(hex_transformation);
                let colour = if controller.board.board.is_available(x as isize, y as isize) { BLACK } else { GREY };
                text(colour, 20, label, glyphs, transform, g).unwrap();
            }
        }
    }
}
