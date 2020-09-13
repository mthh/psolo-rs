#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cell {
    Peg = 0,
    Hole = 1,
    Unusable = 2,
}

impl core::str::FromStr for Cell {
    type Err = ();
    fn from_str(input: &str) -> Result<Cell, Self::Err> {
        match input {
            "X" => Ok(Cell::Peg),
            "O" => Ok(Cell::Hole),
            " " => Ok(Cell::Unusable),
            _ => Err(()),
        }
    }
}
