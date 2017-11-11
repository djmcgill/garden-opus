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
    pub fn valid_ix(x: isize, y: isize) -> bool {
        if x < 0 || y < 0 { return false; }
        let x = x as usize;
        let y = y as usize;
        // y = 0, 10 => width = 6, 
        // y = 1, 9 => width = 7
        let half_way = ROW_COUNT/2;
        let from_horizontal = if x > half_way {
            x - half_way
        } else {
            half_way - x
        };

        if from_horizontal > ROW_COUNT {
            panic!("Somehow calculated a negative width board {:?}", from_horizontal);
        }        
        let width = ROW_COUNT - from_horizontal;
        y < ROW_COUNT && x < width
    }


    pub fn get(&self, x: isize, y: isize) -> Option<&Option<Atom>> {
        if !Self::valid_ix(x, y) { return None }
        let x = x as usize; // Valid indexes are always positive
        let y = y as usize;
        let mut current_index: u8 = 0;
        for i in 0 .. ROW_COUNT {
            if x == i {
                let board_index = current_index as usize + y;
                return Some(& self.0[board_index]);
            } else {
                current_index += ROW_WIDTHS[i];
            }
            unreachable!();
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

/* 
        x,y-1    x+1,y-1
     x-1,y    x,y     x+1,y
        x,y+1     x+1,y+1
*/
    // Are there three concurrent free tiles around this one?
    pub fn is_available(&self, x: isize, y: isize) -> bool {
        let neighbour_ixs = &[(x, y-1), (x+1,y-1), (x-1,y), (x+1,y), (x,y+1),(x+1,y+1)];
        let available_neighbours = neighbour_ixs.into_iter().map(|&(n_x, n_y)| {
            let tile = self.get(x, y);
            tile.is_none() || tile.unwrap().is_none()
        }).collect::<Vec<_>>();
        has_seq_3_or_longer(&available_neighbours)
    }
}

fn has_seq_3_or_longer(neighbours: &[bool]) -> bool {
    let mut wrapped_neighbours = [false; 8];
    for i in 0..8 { wrapped_neighbours[i] = neighbours[i%neighbours.len()]; }
    wrapped_neighbours.windows(3).any(|w| w == [true, true, true])
}
