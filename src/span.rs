use crate::position::Position;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[rustfmt::skip]
pub struct Span {
    start: Position,
    end:   Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }

    pub fn start(&self) -> Position {
        self.start
    }

    pub fn end(&self) -> Position {
        self.end
    }
}
