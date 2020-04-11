use crate::lang::value::Identifier;
use crate::lang::expr::Expression;

pub enum Statement {
    Assignment {
        dest: Identifier,
        src: Expression,
    },
}