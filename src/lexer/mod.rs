mod tokens;

use self::tokens::{Token, TokenType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{line_ending, not_line_ending},
    combinator,
    multi::many0,
    sequence::delimited,
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

fn eof(input: &str) -> IResult<&str, &str> {
    let _ = combinator::eof(input)?;
    Ok(("", "EOF"))
}

fn comment(input: &str) -> IResult<&str, &str> {
    alt((multi_line_comment, single_line_comment))(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, &str> {
    let (input, comment) = delimited(tag("---"), take_until("---"), tag("---"))(input)?;
    Ok((input, comment))
}

fn single_line_comment(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("--"),
        take_till(|c| c == '\r' || c == '\n'),
        line_ending,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment() {
        assert_eq!(
            single_line_comment("-- This is a comment\n"),
            Ok(("", " This is a comment"))
        );
        assert_eq!(
            single_line_comment("-- This is a comment\r\n"),
            Ok(("", " This is a comment"))
        );
        dbg!(single_line_comment("-- This is not a comment"));
        assert_eq!(
            single_line_comment("-- This is not a comment"),
            Err(nom::Err::Error((" abcdefg", nom::error::ErrorKind::IsNot)))
        );
    }
}
