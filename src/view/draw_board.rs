use ::model::elements::Atom;
use graphics::*;

pub const HEX_HEIGHT: f64 = 1.0;
pub const HEX_WIDTH: f64 = 0.86602540378 * HEX_HEIGHT; // sqrt(3)/2
const HEX_COORDS: &'static [[f64; 2]] = &[
    [           0.0,  HEX_HEIGHT/2.0],
    [ HEX_WIDTH/2.0,  HEX_HEIGHT/4.0],
    [ HEX_WIDTH/2.0,             0.0],
    [ HEX_WIDTH/2.0, -HEX_HEIGHT/4.0],
    [           0.0, -HEX_HEIGHT/2.0],
    [-HEX_WIDTH/2.0, -HEX_HEIGHT/4.0],
    [-HEX_WIDTH/2.0,             0.0],
    [-HEX_WIDTH/2.0,  HEX_HEIGHT/4.0],
];

const ORANGE: [f32; 4] = [189.0/255.0, 148.0/255.0, 49.0/255.0, 1.0];
pub fn draw_hex<G>(c: Context, g: &mut G) where G: Graphics {
    polygon(ORANGE, HEX_COORDS, c.transform, g);
}

pub const FIRE_SYMBOL: & 'static str = "🜂";
pub const AIR_SYMBOL: & 'static str = "🜁";
pub const WATER_SYMBOL: & 'static str = "🜄";
pub const EARTH_SYMBOL: & 'static str = "🜃";
pub const LEAD_SYMBOL: & 'static str = "♄";
pub const TIN_SYMBOL: & 'static str = "♃";
pub const IRON_SYMBOL: & 'static str = "♂";
pub const COPPER_SYMBOL: & 'static str = "♀";
pub const SILVER_SYMBOL: & 'static str = "☽";
pub const GOLD_SYMBOL: & 'static str = "☉";
pub const VITAE_SYMBOL: & 'static str = "🜍";
pub const MORS_SYMBOL: & 'static str = "🜞";
pub const QUICKSILVER_SYMBOL: & 'static str = "☿";
pub const SALT_SYMBOL: & 'static str = "🜔";

// fn draw_atom(atom: &Atom) {
//     match *atom {
//         Atom::BaseElement(element) => {},
//         Atom::BaseMetal(metal) => {},
//         Atom::Januae(januae) => {},
//         Atom::Quicksilver => {},
//         Atom::Salt => {},
//     }
// }
