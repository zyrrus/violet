#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Whitespace,
    Comment(CommentType, &'a str),
    Keyword(Keyword),
    Identifier(&'a str),
    Punctuation(),
    String,
    Number,
    Bool,
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
    Dot,
}
