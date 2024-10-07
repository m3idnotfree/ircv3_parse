use nom::{
    branch::alt,
    character::complete::{alpha1, digit1},
    IResult,
};

pub fn command_parse(msg: &str) -> IResult<&str, &str> {
    alt((alpha1, digit1))(msg)
}
