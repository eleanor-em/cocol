use nom::error::ParseError;
use nom::IResult;
use nom::lib::std::str::FromStr;
use nom::character::complete::{digit1, alpha1, alphanumeric1, multispace0};
use nom::combinator::{map_res, map};
use nom::sequence::{terminated, delimited, preceded};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::multi::{many1, many0, separated_list};
use crate::lang::common::drop_ws;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier {
    name: String,
}

pub fn identifier<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Identifier, E> {
    let (input, prefix) = preceded(multispace0, alt((alpha1, tag("_"))))(input)?;
    let (input, res) = terminated(many0(alt((tag("_"), alphanumeric1))), multispace0)(input)?;

    let name = prefix.to_owned() + res.concat().as_str();
    Ok((input, Identifier {
        name
    }))
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    // Note that "Int" simply means untagged. It could still get inferred to be uint later.
    Int(i32),
    Uint(u32),
    Array(Vec<Value>),
}

fn uint_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Value, E> {
    map(drop_ws(terminated(map_res(
            digit1,
            FromStr::from_str
        ), tag("u"))),
        |val| Value::Uint(val))(input)
}

fn int_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Value, E> {
    map(drop_ws(map_res(
            digit1,
            FromStr::from_str
        )),
    |val| Value::Int(val))(input)
}

fn array_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Value, E> {
    map(drop_ws(delimited(tag("["),
                                 separated_list(tag(","), drop_ws(any_value)),
                                 tag("]"))),
        |vec| Value::Array(vec))(input)
}

pub fn any_value<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Value, E> {
    drop_ws(alt((
                    array_value,
                    uint_value,
                    int_value))
    )(input)
}

pub fn many_values<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Vec<Value>, E> {
    many1(any_value)(input)
}