#[derive(Debug)]
pub enum Token<'a> {
    Whitespace,
    Comment(CommentType, &'a str),
    Keyword(Keyword),
    Identifier,
    Punctuation(),
    String,
    Number,
    Bool,
    EOF,
}

#[derive(Debug)]
pub enum CommentType {
    Inline,
    Block,
}

#[derive(Debug)]
pub enum Keyword {
    Import,
    Fn,
    Let,
    Const,
    If,
    Else,
    Return,
}

#[derive(Debug)]
pub enum Punctuation {
    Dot,
}
