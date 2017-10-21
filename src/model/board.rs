use std::fmt::{Debug, Error, Formatter};
use std::result::Result;

use super::Atom;

/// A horizontally-aligned hex grid of vertically-aligned hexes, of side length 6
pub struct Board([Option<Atom>; 91]);

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.0[..].eq(&other.0[..])
    }
}

impl Debug for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.0[..].fmt(formatter)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board([None; 91])
    }
}


const ROW_COUNT: usize = 11;
const ROW_WIDTHS: [u8; ROW_COUNT] = [6, 7, 8, 9, 10, 11, 10, 9, 8, 7, 6];

impl Board {
    pub fn valid_ix(x: u8, y: u8) -> bool {
        // y = 0, 10 => width = 6, 
        // y = 1, 9 => width = 7
        let from_horizontal = (x as i8 - ROW_COUNT as i8).abs();
        let width = ROW_COUNT as i8 - from_horizontal;
        if width < 0 { panic!("Somehow calculated a negative width board {:?}", width);}
        y < ROW_COUNT as u8 && x < (width as u8)
    }


    pub fn get(&self, x: u8, y: u8) -> Option<Option<Atom>> {
        if !Self::valid_ix(x, y) { return None }
        let mut current_index = 0;
        for i in 0 .. ROW_COUNT {
            if x == i as u8 {
                let board_index = current_index as u8 + y;
                return Some(self.0[board_index as usize]);
            } else {
                current_index += ROW_WIDTHS[i];
            }
            panic!("Index exceeded number of rows: {:?}", i);
        }
        None
    }

    pub fn parse<'a, S: Into<&'a str>>(string: S) -> Option<Board> {
        let string = string.into();
        let stripped_string = string.chars().filter(|&c| !char::is_whitespace(c));
        let parsed_atoms = stripped_string.map(|c| Atom::parse(c).unwrap_or_else(|| {return None;} ));
        let parsed_atoms_vec = parsed_atoms.collect::<Vec<Option<Atom>>>();
        if parsed_atoms_vec.len() != 91 { return None; }
        let mut board = Board::default();
        board.0.copy_from_slice(parsed_atoms_vec.as_slice());
        Some(board)
    }
}
