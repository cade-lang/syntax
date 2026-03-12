use std::fmt::Display;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[rustfmt::skip]
pub struct Position {
    offset: usize,
    row:    usize,
    column: usize,
}

impl Position {
    pub fn new(offset: usize, row: usize, column: usize) -> Position {
        Self {
            offset,
            row,
            column,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.row, self.column)
    }
}

#[cfg(test)]
#[allow(invalid_value)] // for MaybeUninit
mod tests {
    use super::Position;
    use std::mem::MaybeUninit;

    #[test]
    fn display() {
        assert_eq!(
            "24:0",
            format!(
                "{}",
                Position::new(unsafe { MaybeUninit::uninit().assume_init() }, 24, 0)
            )
        );
    }
}
