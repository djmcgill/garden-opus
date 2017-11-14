use std::fmt::{Debug, Error, Formatter};
use std::result::Result;

use super::Atom;

pub const ROW_COUNT: usize = 11;
pub const ROW_WIDTHS: [u8; ROW_COUNT] = [6, 7, 8, 9, 10, 11, 10, 9, 8, 7, 6];

pub const IX_TO_COORDS: &'static[(usize, usize)] = &[
                    ( 0, 0), ( 0, 1), ( 0, 2), ( 0, 3), ( 0, 4), ( 0, 5),
                ( 1, 0), ( 1, 1), ( 1, 2), ( 1, 3), ( 1, 4), ( 1, 5), ( 1, 6), 
            ( 2, 0), ( 2, 1), ( 2, 2), ( 2, 3), ( 2, 4), ( 2, 5), ( 2, 6), ( 2, 7),
        ( 3, 0), ( 3, 1), ( 3, 2), ( 3, 3), ( 3, 4), ( 3, 5), ( 3, 6), ( 3, 7), ( 3, 8),
    ( 4, 0), ( 4, 1), ( 4, 2), ( 4, 3), ( 4, 4), ( 4, 5), ( 4, 6), ( 4, 7), ( 4, 8), ( 4, 9),
( 5, 0), ( 5, 1), ( 5, 2), ( 5, 3), ( 5, 4), ( 5, 5), ( 5, 6), ( 5, 7), ( 5, 8), ( 5, 9), ( 5,10),
    ( 6, 1), ( 6, 2), ( 6, 3), ( 6, 4), ( 6, 5), ( 6, 6), ( 6, 7), ( 6, 8), ( 6, 9), ( 6,10),
        ( 7, 2), ( 7, 3), ( 7, 4), ( 7, 5), ( 7, 6), ( 7, 7), ( 7, 8), ( 7, 9), ( 7,10),
            ( 8, 3), ( 8, 4), ( 8, 5), ( 8, 6), ( 8, 7), ( 8, 8), ( 8, 9), ( 8,10),
                ( 9, 4), ( 9, 5), ( 9, 6), ( 9, 7), ( 9, 8), ( 9, 9), ( 9,10),
                    (10, 5), (10, 6), (10, 7), (10, 8), (10, 9), (10,10),
];

pub const COORDS_TO_IX: &'static[&'static[usize]] = &[
    &[ 0, 1, 2, 3, 4, 5],
    &[ 6, 7, 8, 9,10,11,12],
    &[13,14,15,16,17,18,19,20],
    &[21,22,23,24,25,26,27,28,29],
    &[30,31,32,33,34,35,36,37,38,39],
    &[40,41,42,43,44,45,46,47,48,49,50],
    &[51,52,53,54,55,56,57,58,59,60],
    &[61,62,63,64,65,66,67,68,69],
    &[70,71,72,73,74,75,76,77],
    &[78,79,80,81,82,83,84],
    &[85,86,87,88,89,90],
];

/// A horizontally-aligned hex grid of vertically-aligned hexes, of side length 6
pub const CELL_COUNT: usize = 91;
pub struct Board([Option<Atom>; CELL_COUNT]);

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

impl Board {
    pub fn get<'a>(&'a self, x: isize, y: isize) -> Option<&'a Option<Atom>> {
        if x < 0 || y < 0 { return None; }
        COORDS_TO_IX.get(y as usize).and_then(|xs| xs.get(x as usize)).map(|ix| &self.0[*ix])
    }

    pub fn get_mut<'a>(&'a mut self, x: isize, y: isize) -> Option<&'a mut Option<Atom>> {
        if x < 0 || y < 0 { return None; }
        // COORDS_TO_IX.get(y as usize).and_then(|xs| xs.get(x as usize)).map(|ix| &mut self.0[*ix])
        unimplemented!()
    }

    pub fn parse<'a, S: Into<&'a str>>(string: S) -> Option<Board> {
        let string = string.into();
        let stripped_string = string.chars().filter(|&c| !char::is_whitespace(c));
        let parsed_atoms = stripped_string.map(|c| Atom::parse(c).unwrap_or_else(|| {return None;} ));
        let parsed_atoms_vec = parsed_atoms.collect::<Vec<Option<Atom>>>();
        if parsed_atoms_vec.len() != 91 { return None; }
        let mut board = Board::empty();
        board.0.copy_from_slice(parsed_atoms_vec.as_slice());
        Some(board)
    }

/* 
        x,y-1    x+1,y-1
     x-1,y    x,y     x+1,y
        x,y+1     x+1,y+1
*/
    // Are there three concurrent free tiles around this one?
    pub fn is_available(&self, x: isize, y: isize) -> bool {
        let neighbour_ixs = &[(x, y-1), (x+1,y-1), (x-1,y), (x+1,y), (x,y+1),(x+1,y+1)];
        let available_neighbours = neighbour_ixs.into_iter().map(|&(n_x, n_y)| {
            let tile = self.get(n_x, n_y);
            tile.is_none() || tile.unwrap().is_none()
        }).collect::<Vec<_>>();
        has_seq_3_or_longer(&available_neighbours)
    }

    pub fn empty() -> Self {
        Board([None; 91])
    }
}

fn has_seq_3_or_longer(neighbours: &[bool]) -> bool {
    let mut wrapped_neighbours = [false; 8];
    for i in 0..8 { wrapped_neighbours[i] = neighbours[i%neighbours.len()]; }
    wrapped_neighbours.windows(3).any(|w| w == [true, true, true])
}
