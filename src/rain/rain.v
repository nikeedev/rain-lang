module rain

import os

pub enum TokenType {
    number // 0-9
    whitespace

    plus  // +
    minus // -
    dtar  // *
    slash // /

    equals

    unknown
}


pub struct Token {
mut:
	value string
	token_type TokenType
}

pub fn token(value string, token_type TokenType) Token {
	return Token{ value: value, token_type: token_type }
}

pub struct Lexer {

mut:
	tokens []Token
}

