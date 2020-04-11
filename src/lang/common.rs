use nom::error::ParseError;
use nom::IResult;
use nom::sequence::delimited;
use nom::bytes::complete::take_while;

pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}
pub fn drop_ws<'a, F, O, E: ParseError<&'a str>>(parser: F) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>
{
    delimited(take_while(is_whitespace), parser, take_while(is_whitespace))
}