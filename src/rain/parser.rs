use crate::rain::*;
use lexer::{Token, TokenType};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current_token: Token<'a>,
    current_index: usize,
}

impl<'a> Parser<'a> {
    pub fn new(&mut self, tokens: Vec<Token<'a>>) {
        self.tokens = tokens;
        self.current_token = self.tokens[self.current_index];
        self.current_index = 0;
    }
}
