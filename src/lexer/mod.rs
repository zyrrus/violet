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

fn program(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((comment, eof)))(input)
}

fn eof(input: &str) -> IResult<&str, Token> {
    let _ = combinator::eof(input)?;
    Ok((
        "",
        Token {
            token_type: TokenType::EOF,
        },
    ))
}

fn comment(input: &str) -> IResult<&str, Token> {
    alt((multi_line_comment, single_line_comment))(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, Token> {
    let (input, comment) = delimited(tag("---"), take_until("---"), tag("---"))(input)?;
    Ok((
        input,
        Token {
            token_type: TokenType::Comment(comment),
        },
    ))
}

fn single_line_comment(input: &str) -> IResult<&str, Token> {
    let (input, comment) = delimited(
        tag("--"),
        take_till(|c| c == '\r' || c == '\n'),
        line_ending,
    )(input)?;

    Ok((
        input,
        Token {
            token_type: TokenType::Comment(comment),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_comment() {
        let input = "\r\n\r\n-- Import statements\r\nimport std.collections {\r\n    Heap,\r\n    HashMap,\r\n}\r\n\r\n-- Function declaration with types\r\nfn fizz-buzz(i: Num): Str {\r\n    --- This is technically a legal comment ---\r\n    let fb = \"\"\r\n\r\n    if i % 3 == 0 {\r\n        fb = fb ++ \"fizz\"\r\n    }\r\n\r\n    if i % 5 == 0 {\r\n        fb ++= \"buzz\"\r\n    }\r\n\r\n    return i if Str.is-empty(fb) else fb\r\n}\r\n\r\n-- Function declaration without types\r\nfn main() {\r\n    const fb: Str = fizz-buzz()\r\n    print(fb)\r\n}";
        dbg!(single_line_comment(input));
    }
}
