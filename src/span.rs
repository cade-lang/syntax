#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span(usize, usize);

impl Span {
    pub fn start(&self) -> usize {
        self.0
    }

    pub fn end(&self) -> usize {
        self.1
    }
}
