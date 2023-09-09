mod tokens;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::char,
    combinator::{eof, value},
    multi::many0,
    sequence::{pair, terminated, tuple},
    IResult,
};
use std::fs;

pub fn read_file(file_path: &str) -> String {
    // Read the contents of the file into a string
    return fs::read_to_string(file_path).expect("Failed to read the file.");
}

pub fn run_lexer(input: &str) {
    match program(input) {
        Ok((remaining, parsed)) => {
            if remaining.is_empty() {
                println!("Parsed successfully: '{:?}'", parsed);
            } else {
                println!("Parsed partially: '{:?}'", parsed);
                println!("Remaining: '{:?}'", remaining);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    };
}

// === Parsers ===========================================================

fn program(input: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((comment, eof)))(input)
}

fn comment(input: &str) -> IResult<&str, &str> {
    alt((multi_line_comment, single_line_comment))(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, &str> {
    let (rest, _) = value((), (tag("---"), take_until("---"), tag("---")))(input)?;
    Ok((rest, ""))
}

fn single_line_comment(input: &str) -> IResult<&str, &str> {
    let (rest, _) = terminated(pair(tag("--"), take_until("\n")), char('\n'))(input)?;
    Ok((rest, ""))
}
