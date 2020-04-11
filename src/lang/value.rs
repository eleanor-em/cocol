use nom::{IResult, Offset};
use nom::lib::std::str::FromStr;
use nom::character::complete::{digit1, alpha1, alphanumeric1, multispace0};
use nom::combinator::{map_res, map};
use nom::sequence::{terminated, delimited, preceded};
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom::multi::{many0, separated_list};
use nom_locate::{position, LocatedSpan};
use crate::lang::common::Position;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Identifier {
    pos: Position,
    name: String,
}

pub fn identifier(s: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Identifier> {
    let (s, pos) = position(s)?;
    let (s, prefix) = preceded(multispace0, alt((alpha1, tag("_"))))(s)?;
    let (s, rest) = terminated(many0(alt((tag("_"), alphanumeric1))), multispace0)(s)?;
    let name = prefix.fragment().clone().to_owned()
        + rest.iter().map(|l| l.fragment().to_owned()).collect::<Vec::<&str>>().concat().as_str();
    Ok((s, Identifier { pos: Position::new(pos), name }))
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Value {
    // Note that "Int" simply means untagged. It could still get inferred to be uint later.
    Int {
        pos: Position,
        val: i32,
    },
    Uint {
        pos: Position,
        val: u32,
    },
    // TODO: Array is Expression not Value
    // Array {
    //     pos: Position,
    //     val: Vec<Value>,
    // },
}

fn uint_value(s: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Value> {
    let (s, pos) = position(s)?;
    let (s, digits) = terminated(digit1, tag("u"))(s)?;
    let parsed = FromStr::from_str(digits.fragment()).unwrap();
    Ok((s, Value::Uint {
        pos: Position::new(pos),
        val: parsed
    }))
}

fn int_value(s: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Value> {
    let (s, pos) = position(s)?;
    let (s, digits) = digit1(s)?;
    let parsed = FromStr::from_str(digits.fragment()).unwrap();
    Ok((s, Value::Int{
        pos: Position::new(pos),
        val: parsed
    }))
}

// fn array_value(s: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Value> {
//     let (s, pos) = position(s)?;
//     let (s, vec) =  delimited(tag("["),
//                                  separated_list(tag(","), any_value),
//                                  tag("]"))(s)?;
//     Ok((s, Value::Array{
//         pos: Position::new(pos),
//         val: vec
//     }))
// }

pub fn any_value(s: LocatedSpan<&str>) -> IResult<LocatedSpan<&str>, Value> {
    delimited(multispace0,
              alt((
                    uint_value,
                    int_value)),
        multispace0
    )(s)
}
