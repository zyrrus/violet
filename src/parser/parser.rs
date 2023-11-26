use crate::lexer::tokens::{CommentType, Token};

pub enum ASTNode<'a> {
    Program(Vec<ASTNode<'a>>),
    Comment(Token<'a>),
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    ASTNode::Comment(Token::Comment(CommentType::Block, "REMOVE"))
}
