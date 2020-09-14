use crate::cell::Cell;
use core::str::FromStr;

#[derive(Debug)]
pub(crate) struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(shape: &str) -> Result<Self, ()> {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for part in shape.split("\n") {
            height += 1;
            if width == 0 {
                width = part.len() as u32;
            } else if width != part.len() as u32 {
                return Err(());
            }
            for char in part.chars() {
                cells.push(Cell::from_str(&char.to_string())?);
            }
        }
        Ok(Board {
            width,
            height,
            cells,
        })
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row + column * self.height) as usize
    }

    pub fn get_cell(&self, row: u32, column: u32) -> Cell {
        self.cells[self.get_index(row, column)]
    }

    pub fn count_peg(&self) -> usize {
        self.cells.iter().filter(|&n| *n == Cell::Peg).count()
    }

    pub fn is_valid_move(&self, src: (u32, u32), dest: (u32, u32)) -> bool {
        let (i_dest, j_dest) = (dest.0 as i32, dest.1 as i32);
        let (i_src, j_src) = (src.0 as i32, src.1 as i32);
        let (i_middle, j_middle) = if i_dest == i_src {
            if j_dest > j_src {
                (i_dest, j_dest - 1)
            } else {
                (i_dest, j_dest + 1)
            }
        } else {
            if i_dest > i_src {
                (i_dest - 1, j_dest)
            } else {
                (i_dest + 1, j_dest)
            }
        };

        i_src >= 0
            && j_src < self.width as i32
            && i_dest >= 0
            && j_dest < self.height as i32
            && (i_src == i_dest && (j_src == j_dest - 2 || j_src == j_dest + 2)
                || j_src == j_dest && (i_src == i_dest - 2 || i_src == i_dest + 2))
            && self.cells[(i_src + j_src * self.height as i32) as usize] == Cell::Peg
            && self.cells[(i_middle + j_middle * self.height as i32) as usize] == Cell::Peg
            && self.cells[(i_dest + j_dest * self.height as i32) as usize] == Cell::Hole
    }

    pub fn make_move(&mut self, src: (u32, u32), dest: (u32, u32)) {
        let (i_dest, j_dest) = dest;
        let (i_src, j_src) = src;
        let (i_middle, j_middle) = if i_dest == i_src {
            if j_dest > j_src {
                (i_dest, j_dest - 1)
            } else {
                (i_dest, j_dest + 1)
            }
        } else {
            if i_dest > i_src {
                (i_dest - 1, j_dest)
            } else {
                (i_dest + 1, j_dest)
            }
        };

        self.cells[(i_dest + j_dest * self.height) as usize] = Cell::Peg;
        self.cells[(i_src + j_src * self.height) as usize] = Cell::Hole;
        self.cells[(i_middle + j_middle * self.height) as usize] = Cell::Hole;
    }

    pub fn has_valid_move_left(&self) -> bool {
        for i_src in 0..self.width {
            for j_src in 0..self.height {
                for i_dest in 0..self.width {
                    for j_dest in 0..self.height {
                        if i_src != j_src || j_src != j_dest {
                            if self.is_valid_move((i_src, j_src), (i_dest, j_dest)) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}
