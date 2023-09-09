#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
}

#[derive(Debug)]
pub enum TokenType<'a> {
    Whitespace,
    Comment(&'a str),
    Keyword,
    Type,
    Identifier,
    Literal,
    EOF,
}
