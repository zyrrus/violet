mod tokens;

use self::tokens::{CommentType, Keyword, Token};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{alpha1, alphanumeric0, alphanumeric1, line_ending, multispace0},
    combinator::recognize,
    error::ParseError,
    multi::many0,
    sequence::{delimited, tuple},
    IResult, Parser,
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
    many0(alt((comment, keyword, identifier)))(input)
}

// === Utils =============================================================

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

// === Tokens ============================================================

fn comment(input: &str) -> IResult<&str, Token> {
    alt((multi_line_comment, single_line_comment))(input)
}

fn multi_line_comment(input: &str) -> IResult<&str, Token> {
    let (input, comment) = ws(delimited(tag("---"), take_until("---"), tag("---")))(input)?;
    Ok((input, Token::Comment(CommentType::Block, comment)))
}

fn single_line_comment(input: &str) -> IResult<&str, Token> {
    let (input, comment) = ws(delimited(
        tag("--"),
        take_till(|c| c == '\r' || c == '\n'),
        line_ending,
    ))(input)?;

    Ok((input, Token::Comment(CommentType::Inline, comment)))
}

fn keyword(input: &str) -> IResult<&str, Token> {
    let (input, keyword) = alt((
        tag("import"),
        tag("fn"),
        tag("let"),
        tag("const"),
        tag("if"),
        tag("else"),
        tag("return"),
    ))(input)?;

    let token = match keyword {
        "import" => Keyword::Import,
        "fn" => Keyword::Fn,
        "let" => Keyword::Let,
        "const" => Keyword::Const,
        "if" => Keyword::If,
        "else" => Keyword::Else,
        "return" => Keyword::Return,
        _ => {
            return Err(nom::Err::Error(nom::error::make_error(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, Token::Keyword(token)))
}

fn identifier(input: &str) -> IResult<&str, Token> {
    let (input, identifier) = alt((
        recognize(tuple((
            alpha1,
            many0(alt((alphanumeric1, tag("-"), tag("_")))),
            alphanumeric1,
        ))),
        recognize(tuple((alpha1, alphanumeric0))),
    ))(input)?;

    Ok((input, Token::Identifier(identifier)))
}

// TODO: fn number(input: &str) -> IResult<&str, Token> {}

// TODO: fn string(input: &str) -> IResult<&str, Token> {}

// TODO: fn operator(input: &str) -> IResult<&str, Token> {}

// TODO: fn punctuation(input: &str) -> IResult<&str, Token> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        assert_eq!(
            comment("--- This is a comment ---"),
            Ok((
                "",
                Token::Comment(CommentType::Block, " This is a comment ")
            ))
        );
        assert_eq!(
            comment("--- This is a\nmultiline comment ---"),
            Ok((
                "",
                Token::Comment(CommentType::Block, " This is a\nmultiline comment ")
            ))
        );
        assert_eq!(
            comment("-- Single line comment\n"),
            Ok((
                "",
                Token::Comment(CommentType::Inline, " Single line comment")
            ))
        );
    }

    #[test]
    fn test_multi_line_comment() {
        assert_eq!(
            multi_line_comment("--- Multi-line\ncomment ---"),
            Ok((
                "",
                Token::Comment(CommentType::Block, " Multi-line\ncomment ")
            ))
        );
    }

    #[test]
    fn test_single_line_comment() {
        assert_eq!(
            single_line_comment("-- Single line comment\n"),
            Ok((
                "",
                Token::Comment(CommentType::Inline, " Single line comment")
            ))
        );

        assert_eq!(
            single_line_comment("-- Single line comment\r\n"),
            Ok((
                "",
                Token::Comment(CommentType::Inline, " Single line comment")
            ))
        );
    }

    #[test]
    fn test_keyword() {
        assert_eq!(keyword("import"), Ok(("", Token::Keyword(Keyword::Import))));
        assert_eq!(keyword("let"), Ok(("", Token::Keyword(Keyword::Let))));
        assert!(keyword("invalid").is_err());
    }

    #[test]
    fn test_identifier() {
        assert_eq!(identifier("a"), Ok(("", Token::Identifier("a"))));
        assert_eq!(identifier("z"), Ok(("", Token::Identifier("z"))));
        assert_eq!(identifier("A"), Ok(("", Token::Identifier("A"))));
        assert_eq!(identifier("Z"), Ok(("", Token::Identifier("Z"))));
        assert_eq!(identifier("ab"), Ok(("", Token::Identifier("ab"))));
        assert_eq!(identifier("Az"), Ok(("", Token::Identifier("Az"))));
        assert_eq!(identifier("Z1"), Ok(("", Token::Identifier("Z1"))));
        assert_eq!(identifier("a9"), Ok(("", Token::Identifier("a9"))));
        assert_eq!(
            identifier("abc_def"),
            Ok(("", Token::Identifier("abc_def")))
        );
        assert_eq!(identifier("x-y-z"), Ok(("", Token::Identifier("x-y-z"))));
        assert_eq!(
            identifier("A0_B1-C2"),
            Ok(("", Token::Identifier("A0_B1-C2")))
        );
        assert_eq!(
            identifier("test123_abc-xyz"),
            Ok(("", Token::Identifier("test123_abc-xyz")))
        );
        assert!(identifier("1abc").is_err());
        assert!(identifier("_abc").is_err());
        assert!(identifier("-abc").is_err());
        assert!(identifier("abc1!").is_err());
        assert!(identifier("abc_").is_err());
        assert!(identifier("abc-").is_err());
    }
}
