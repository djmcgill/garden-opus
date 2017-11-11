use ::model::GameState;
use graphics::types::Polygon;

pub fn draw_board(state: &GameState) {
    
}

const HEX_HEIGHT: f64 = 1.0;
const HEX_WIDTH: f64 = 0.86602540378 * HEX_HEIGHT; // sqrt(3)/2
pub fn unit_hex() -> Polygon<'static> {
    &[
         [HEX_HEIGHT/2.0, 0.0],
         [HEX_HEIGHT/4.0, HEX_WIDTH/2.0],
                    [0.0, HEX_WIDTH/2.0],
        [-HEX_HEIGHT/4.0, HEX_WIDTH/2.0],
        [-HEX_HEIGHT/2.0, 0.0],
        [-HEX_HEIGHT/4.0, -HEX_WIDTH/2.0],
                    [0.0, -HEX_WIDTH/2.0],
         [HEX_HEIGHT/4.0, -HEX_WIDTH/2.0],
    ]
}

// fn draw_atom(atom: &Atom) {
//     match *atom {
//         Atom::BaseElement(element) => {},
//         Atom::BaseMetal(metal) => {},
//         Atom::Januae(januae) => {},
//         Atom::Quicksilver => {},
//         Atom::Salt => {},
//     }
// }
