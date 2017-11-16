pub mod board;
use self::board::*;

pub mod atom;
use self::atom::*;

mod hex;
pub use self::hex::*;

#[derive(Debug)]
pub struct GameState {
    pub board: Board,
    pub selected_tile: Option<(usize, usize)>, // The hex x, y index from the top left.
    pub active_metal: Option<BaseMetal>, // The metal that can be combined with quicksilver, if any.
}

const WATER: Option<Atom> = Some(Atom::BaseElement(BaseElement::Water));
const AIR: Option<Atom> = Some(Atom::BaseElement(BaseElement::Air));
const FIRE: Option<Atom> = Some(Atom::BaseElement(BaseElement::Fire));
const EARTH: Option<Atom> = Some(Atom::BaseElement(BaseElement::Earth));
const SALT: Option<Atom> = Some(Atom::Salt);
const QS: Option<Atom> = Some(Atom::Quicksilver);
const MORS: Option<Atom> = Some(Atom::Januae(Januae::Mors));
const VITAE: Option<Atom> = Some(Atom::Januae(Januae::Vitae));
const LEAD: Option<Atom> = Some(Atom::BaseMetal(BaseMetal::Lead));
const TIN: Option<Atom> = Some(Atom::BaseMetal(BaseMetal::Tin));
const IRON: Option<Atom> = Some(Atom::BaseMetal(BaseMetal::Iron));
const COPPER: Option<Atom> = Some(Atom::BaseMetal(BaseMetal::Copper));
const SILVER: Option<Atom> = Some(Atom::BaseMetal(BaseMetal::Silver));
const GOLD: Option<Atom> = Some(Atom::BaseMetal(BaseMetal::Gold));
pub const SAMPLE_GAME: GameState = GameState {
    selected_tile: None,
    active_metal: Some(BaseMetal::Lead),
    board: Board(
        [
            None, WATER, None, None, None, None,
            None, SALT, EARTH, AIR, AIR, QS, MORS,
            None, MORS, WATER, FIRE, None, WATER, VITAE, None,
            None, EARTH, None, COPPER, IRON, EARTH, SALT, AIR, None,
            SALT, QS, FIRE, EARTH, None, None, VITAE, None, WATER, None,
            None, AIR, QS, EARTH, None, GOLD, None, QS, VITAE, AIR, None,
            None, SILVER, None, AIR, None, None, FIRE, VITAE, WATER, WATER,
            None, EARTH, MORS, MORS, QS, FIRE, None, LEAD, None,
            None, FIRE, WATER, None, EARTH, SALT, EARTH, None,
            AIR, FIRE, TIN, WATER, FIRE, FIRE, None,
            None, None, None, None, AIR, None,
        ]
    )
};

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: Board::empty(),
            selected_tile: None,
            active_metal: Some(BaseMetal::Lead),
        }
    }

    pub fn select_tile(&mut self, x: usize, y: usize) {
        if let Some(tile) = self.board.get_mut(x as isize, y as isize) {
            if let Some(atom@Atom::BaseMetal(BaseMetal::Gold)) = *tile {
                // Gold was selected
                if self.active_metal == Some(BaseMetal::Gold) {
                    self.active_metal = None;
                    *tile = None;
                }
            } else {
                // Gold was not selected
                if let Some(xy) = self.selected_tile {
                    unimplemented!(); // Check for matches
                } else {
                    // TODO: check for valid selectable tile
                    self.selected_tile = Some((x, y))
                }
            }
        }
    }
}
