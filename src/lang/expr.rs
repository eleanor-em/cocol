use nom::IResult;
use crate::lang::value::{Value, Identifier, any_value, identifier};
use nom::branch::alt;
use nom::combinator::map;
use nom_locate::LocatedSpan;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    ValueExpr(Value),
    IdExpr(Identifier),
}

pub fn expression(s: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Expression> {
    alt((
        map(any_value, |val| Expression::ValueExpr(val)),
        map(identifier, |id| Expression::IdExpr(id))
        ))(s)
}