use std::fmt::{Debug, Error, Formatter};
use std::result::Result;
use super::{Atom, COORDS_TO_IX};

/// A horizontally-aligned hex grid of vertically-aligned hexes, of side length 6
pub const CELL_COUNT: usize = 91;
pub struct Board(pub [Option<Atom>; CELL_COUNT]);

impl Debug for Board {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        self.0[..].fmt(formatter)
    }
}

impl Board {
    pub fn empty() -> Self {
        Board([None; 91])
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Option<Atom>> {
        COORDS_TO_IX.get(y).and_then(|xs| xs.get(x)).map(|ix| &self.0[*ix])
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Option<Atom>> {
        COORDS_TO_IX.get(y).and_then(|xs| xs.get(x)).map(move |ix| &mut self.0[*ix])
    }

    pub fn parse<'a, S: Into<&'a str>>(string: S) -> Option<Board> {
        unimplemented!()
    }

    // Are there three concurrent free tiles around this one?
    pub fn is_available(&self, x: isize, y: isize) -> bool {
        const NEIGHBOUR_COUNT: usize = 6;
        let neighbour_ixs: [(isize, isize); NEIGHBOUR_COUNT] = [(x, y-1), (x+1,y-1), (x-1,y), (x+1,y), (x,y+1),(x+1,y+1)];
        let available_neighbours = (&neighbour_ixs).into_iter().map(|&(n_x, n_y)| {
            n_x >= 0 && n_y >= 0 && {
                let tile = self.get(n_x as usize, n_y as usize);
                tile.is_none() || tile.unwrap().is_none()
            }
        }).collect::<Vec<_>>(); // TODO: smallvec or array or bitvec this - BENCHMARK always has length 6
        has_seq_3_or_longer(&available_neighbours)
    }
}

fn has_seq_3_or_longer(neighbours: &[bool]) -> bool {
    let mut wrapped_neighbours = [false; 8];
    for i in 0..8 { wrapped_neighbours[i] = neighbours[i%neighbours.len()]; }
    wrapped_neighbours.windows(3).any(|w| w == [true, true, true])
}
