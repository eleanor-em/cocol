use nom_locate::LocatedSpan;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
    pub line: u32,
    pub col: usize,
}
impl Position {
    pub fn new(s: LocatedSpan<&str>) -> Self {
        Self {
            line: s.location_line(),
            col: s.get_column(),
        }
    }
}