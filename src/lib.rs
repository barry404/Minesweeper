mod random;

use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

use random::random_range;

// #[derive(Deubg, Hash, Eq)]
// pub struct Pos(usize, usize);
pub type Pos = (usize, usize);

pub enum OpenResult {
    Mine,
    NoMine(u8),
}

struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Pos>,
    mines: HashSet<Pos>,
    flagged_fields: HashSet<Pos>,
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.width {
            for y in 0..self.height {
                let pos = (x, y);
                if !self.open_fields.contains(&pos) {
                    f.write_str("📗 ")?;
                } else if self.mines.contains(&pos) {
                    f.write_str("💣  ")?;
                } else {
                    write!(f, "{}", self.neighboring_mines(pos))?
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

impl Minesweeper {
    fn new(width: usize, height: usize, mines_count: usize) -> Self {
        Self {
            width,
            height,
            open_fields: HashSet::new(),
            mines: {
                let mut mines = HashSet::new();

                while mines.len() < mines_count {
                    mines.insert((random_range(0, width), random_range(0, height)));
                }
                mines
            },
            flagged_fields: HashSet::new(),
        }
    }

    // pub fn iter_neighbors(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
    //     ((x - 1).max(0)..=(x + 1).min(self.width - 1))
    //         .map(|i| ((y - 1).max(0)..=(y + 1).min(self.height - 1)).map(move |j| (i, j)))
    //         .filter(|pos| pos != (x, y))
    // }
    pub fn iter_neighbors(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
        let width = self.width;
        let height = self.height;

        (x.max(1) - 1..=(x + 1).min(width - 1))
            .flat_map(move |i| (y.max(1) - 1..=(y + 1).min(height - 1)).map(move |j| (i, j)))
            .filter(move |&pos| pos != (x, y))
    }

    pub fn neighboring_mines(&self, pos: Pos) -> u8 {
        self.iter_neighbors(pos)
            .filter(|p| self.mines.contains(p))
            .count() as u8
    }

    pub fn open(&mut self, pos: Pos) -> OpenResult {
        self.open_fields.insert(pos);
        let is_mine = self.mines.contains(&pos);
        if is_mine {
            OpenResult::Mine
        } else {
            OpenResult::NoMine(0)
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ms = Minesweeper::new(10, 10, 5);
        ms.open((2, 3));
        ms.open((3, 3));
        ms.open((5, 1));
        println!("{}", ms);
    }
}
