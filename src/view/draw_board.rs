use ::model::elements::Atom;
use graphics::*;

const HEX_HEIGHT: f64 = 1.0;
const HEX_WIDTH: f64 = 0.86602540378 * HEX_HEIGHT; // sqrt(3)/2
const HEX_COORDS: &'static [[f64; 2]] = &[
    [ HEX_HEIGHT/2.0,            0.0],
    [ HEX_HEIGHT/4.0,  HEX_WIDTH/2.0],
    [            0.0,  HEX_WIDTH/2.0],
    [-HEX_HEIGHT/4.0,  HEX_WIDTH/2.0],
    [-HEX_HEIGHT/2.0,            0.0],
    [-HEX_HEIGHT/4.0, -HEX_WIDTH/2.0],
    [            0.0, -HEX_WIDTH/2.0],
    [ HEX_HEIGHT/4.0, -HEX_WIDTH/2.0],
];

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub fn draw_hex<G>(c: Context, g: &mut G) where G: Graphics {
    polygon(RED, HEX_COORDS, c.transform, g);
}

const FIRE_SYMBOL: & 'static str = "🜂";
const AIR_SYMBOL: & 'static str = "🜁";
const WATER_SYMBOL: & 'static str = "🜄";
const EARTH_SYMBOL: & 'static str = "🜃";
const LEAD_SYMBOL: & 'static str = "♄";
const TIN_SYMBOL: & 'static str = "♃";
const IRON_SYMBOL: & 'static str = "♂";
const COPPER_SYMBOL: & 'static str = "♀";
const SILVER_SYMBOL: & 'static str = "☽";
const GOLD_SYMBOL: & 'static str = "☉";
const VITAE_SYMBOL: & 'static str = "🜍";
const MORS_SYMBOL: & 'static str = "🜞";
const QUICKSILVER_SYMBOL: & 'static str = "☿";
const SALT_SYMBOL: & 'static str = "🜔";

// fn draw_atom(atom: &Atom) {
//     match *atom {
//         Atom::BaseElement(element) => {},
//         Atom::BaseMetal(metal) => {},
//         Atom::Januae(januae) => {},
//         Atom::Quicksilver => {},
//         Atom::Salt => {},
//     }
// }
