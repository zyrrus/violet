#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
}
#[derive(Debug)]
enum TokenType {
    Whitespace,
    Keyword,
    Type,
    Identifier,
    Literal,
}
