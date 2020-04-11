use nom::error::ParseError;
use nom::IResult;
use crate::lang::value::{Value, Identifier, any_value, identifier};
use nom::branch::alt;
use nom::combinator::map;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    Value(Value),
    Identifier(Identifier),
}

pub fn expression<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Expression, E> {
    alt((
        map(any_value, |val| Expression::Value(val)),
        map(identifier, |id| Expression::Identifier(id))
        ))(input)
}