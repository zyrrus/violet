mod tokens;

use self::tokens::{CommentType, Keyword, Number, Operator, Punctuation, Token};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::complete::{char, digit1, line_ending, multispace0, one_of, satisfy},
    combinator::{map_res, opt, recognize},
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair, tuple},
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
                println!("Parsed successfully:\n'{:?}'", parsed);
            } else {
                println!("Parsed partially:\n'{:?}'", parsed);
                println!();
                println!("Remaining:\n'{:?}'", remaining);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    };
}

// === Parsers ===========================================================

fn program(input: &str) -> IResult<&str, Vec<Token>> {
    many0(alt((
        comment,
        keyword,
        identifier,
        punctuation,
        operator,
        string,
        number,
        bool,
    )))(input)
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
    let (input, identifier) = ws(recognize(pair(
        satisfy(|c| c.is_ascii_alphabetic()),
        recognize(many0(satisfy(|c| {
            c.is_ascii_alphanumeric() || c == '-' || c == '_'
        }))),
    )))(input)?;

    Ok((input, Token::Identifier(identifier)))
}

fn punctuation(input: &str) -> IResult<&str, Token> {
    let (input, punctuation) = ws(one_of(".,:;()[]{}"))(input)?;

    let token = match punctuation {
        '.' => Punctuation::Period,
        ',' => Punctuation::Comma,
        ':' => Punctuation::Colon,
        ';' => Punctuation::SemiColon,
        '(' => Punctuation::RParen,
        ')' => Punctuation::LParen,
        '[' => Punctuation::RBrack,
        ']' => Punctuation::LBrack,
        '{' => Punctuation::RBrace,
        '}' => Punctuation::LBrace,
        _ => {
            return Err(nom::Err::Error(nom::error::make_error(
                input,
                nom::error::ErrorKind::OneOf,
            )))
        }
    };

    Ok((input, Token::Punctuation(token)))
}

fn operator(input: &str) -> IResult<&str, Token> {
    let (input, operator) = ws(alt((
        tag("=="),
        tag("++"),
        tag("="),
        tag("+"),
        tag("-"),
        tag("%"),
    )))(input)?;

    let token = match operator {
        "==" => Operator::IsEqual,
        "++" => Operator::Concat,
        "=" => Operator::Equal,
        "+" => Operator::Plus,
        "-" => Operator::Minus,
        "%" => Operator::Mod,
        _ => {
            return Err(nom::Err::Error(nom::error::make_error(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, Token::Operator(token)))
}

fn string(input: &str) -> IResult<&str, Token> {
    let (input, string) = ws(delimited(tag("\""), take_until("\""), tag("\"")))(input)?;

    Ok((input, Token::String(string)))
}

fn number(input: &str) -> IResult<&str, Token> {
    // Try to parse an integer first
    let (input, integer) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;

    // Check for a possible decimal point
    let (input, decimal) = opt(tuple((char('.'), digit1)))(input)?;

    if let Some((_, fraction)) = decimal {
        // If there is a decimal point, parse the fractional part and construct a float
        let fractional_part = format!("{}.{}", integer, fraction);
        let value = fractional_part
            .parse::<f64>()
            .expect(&format!("Invalid float: '{}.{}'", integer, fraction));
        Ok((input, Token::Number(Number::Float(value))))
    } else {
        // If there is no decimal point, it's an integer
        Ok((input, Token::Number(Number::Int(integer))))
    }
}

fn bool(input: &str) -> IResult<&str, Token> {
    let (input, bool) = ws(alt((tag("true"), tag("false"))))(input)?;

    let token = match bool {
        "true" => true,
        "false" => false,
        _ => {
            return Err(nom::Err::Error(nom::error::make_error(
                input,
                nom::error::ErrorKind::Tag,
            )))
        }
    };

    Ok((input, Token::Bool(token)))
}

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
        assert_eq!(keyword("fn"), Ok(("", Token::Keyword(Keyword::Fn))));
        assert_eq!(keyword("let"), Ok(("", Token::Keyword(Keyword::Let))));
        assert_eq!(keyword("const"), Ok(("", Token::Keyword(Keyword::Const))));
        assert_eq!(keyword("if"), Ok(("", Token::Keyword(Keyword::If))));
        assert_eq!(keyword("else"), Ok(("", Token::Keyword(Keyword::Else))));
        assert_eq!(keyword("return"), Ok(("", Token::Keyword(Keyword::Return))));

        assert!(keyword("invalid_keyword").is_err());
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
    }

    #[test]
    fn test_punctuation() {
        assert_eq!(
            punctuation("."),
            Ok(("", Token::Punctuation(Punctuation::Period)))
        );
        assert_eq!(
            punctuation(","),
            Ok(("", Token::Punctuation(Punctuation::Comma)))
        );
        assert_eq!(
            punctuation(":"),
            Ok(("", Token::Punctuation(Punctuation::Colon)))
        );
        assert_eq!(
            punctuation(";"),
            Ok(("", Token::Punctuation(Punctuation::SemiColon)))
        );
        assert_eq!(
            punctuation("("),
            Ok(("", Token::Punctuation(Punctuation::RParen)))
        );
        assert_eq!(
            punctuation(")"),
            Ok(("", Token::Punctuation(Punctuation::LParen)))
        );
        assert_eq!(
            punctuation("["),
            Ok(("", Token::Punctuation(Punctuation::RBrack)))
        );
        assert_eq!(
            punctuation("]"),
            Ok(("", Token::Punctuation(Punctuation::LBrack)))
        );
        assert_eq!(
            punctuation("{"),
            Ok(("", Token::Punctuation(Punctuation::RBrace)))
        );
        assert_eq!(
            punctuation("}"),
            Ok(("", Token::Punctuation(Punctuation::LBrace)))
        );

        assert!(punctuation("!").is_err());
    }

    #[test]
    fn test_operator() {
        assert_eq!(operator("="), Ok(("", Token::Operator(Operator::Equal))));
        assert_eq!(operator("+"), Ok(("", Token::Operator(Operator::Plus))));
        assert_eq!(operator("-"), Ok(("", Token::Operator(Operator::Minus))));
        assert_eq!(operator("%"), Ok(("", Token::Operator(Operator::Mod))));
        assert_eq!(operator("=="), Ok(("", Token::Operator(Operator::IsEqual))));
        assert_eq!(operator("++"), Ok(("", Token::Operator(Operator::Concat))));

        assert!(operator("!").is_err());
    }

    #[test]
    fn test_string() {
        assert_eq!(
            string("\"Hello, World!\""),
            Ok(("", Token::String("Hello, World!")))
        );

        assert!(string("\"Invalid string").is_err());
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123"), Ok(("", Token::Number(Number::Int(123)))));
        assert_eq!(number("3.14"), Ok(("", Token::Number(Number::Float(3.14)))));

        assert!(number("invalid").is_err());
    }

    #[test]
    fn test_bool() {
        assert_eq!(bool("true"), Ok(("", Token::Bool(true))));
        assert_eq!(bool("false"), Ok(("", Token::Bool(false))));

        assert!(bool("invalid").is_err());
    }
}
