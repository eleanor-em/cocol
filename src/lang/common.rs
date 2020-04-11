use nom::error::ParseError;
use nom::{IResult, InputTakeAtPosition, AsChar};
use nom::sequence::delimited;
use nom::character::complete::multispace0;

struct ParserState<'a> {
    remaining: &'a str,
    line: usize,
    col: usize,
}

impl<'a> ParserState<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            remaining: input,
            line: 1,
            col: 0,
        }
    }

    fn update(&mut self, input: &'a str) {
        let delta = self.remaining.len() - input.len();
        let parsed = &self.remaining[0..delta];
        let split = parsed.split('\n');
        let lines = split.clone().count();
        let col = split.last().unwrap_or("").len();

        self.remaining = input;
        self.line += lines;
        self.col = col;
    }
}

// surely we can do better than this hot garbage of a `where` clause?
pub fn drop_ws<'a, I, F, O, E>(parser: F) -> impl Fn(I) -> IResult<I, O, E>
    where
        I: InputTakeAtPosition,
        <I as InputTakeAtPosition>::Item: AsChar + Clone,
        E: ParseError<I>,
        F: Fn(I) -> IResult<I, O, E>
{
    delimited(multispace0, parser, multispace0)
}