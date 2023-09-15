#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Comment(CommentType, &'a str),
    Keyword(Keyword),
    Identifier(&'a str),
    Punctuation(Punctuation),
    Operator(Operator),
    String(&'a str),
    Number(Number),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum CommentType {
    Inline,
    Block,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Import,
    Fn,
    Let,
    Const,
    If,
    Else,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Period,    // .
    Comma,     // ,
    Colon,     // :
    SemiColon, // ;
    RParen,    // (
    LParen,    // )
    RBrack,    // [
    LBrack,    // ]
    RBrace,    // {
    LBrace,    // }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal,   // =
    Plus,    // +
    Minus,   // -
    Mod,     // %
    IsEqual, // ==
    Concat,  // ++
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Float(f64),
    Int(i32),
}
