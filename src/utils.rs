use nom::character::complete::{multispace0, multispace1};
use nom::error::ParseError;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::Parser;
use std::error::Error;

pub type SolverResult = Result<(), Box<dyn Error>>;

pub fn whitespace_surrounded<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn lines1<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = Vec<O>, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    separated_list1(multispace1, inner)
}
