pub mod draw_board;

use graphics::*;
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
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        draw_board::draw_hex(c.reset(), g);
        let transform = c.transform.trans(400.0, 400.0);
        text(BLACK, 50, "a", glyphs, transform, g).unwrap();
    }
}
