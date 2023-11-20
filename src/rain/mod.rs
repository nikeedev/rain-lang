#![allow(dead_code)]

use std::str::Chars;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    Number, // 1, 2, 3, 4, 5...
    Whitespace,
    Comment, // //
    String { is_terminated: bool },

    Dot, // for getting variables and functions out of structs or,
    // defining a float
    Function, // () {}
    Ident,    // x : =
    If,
    Else,
    For,

    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /

	Increment, // ++
	Decrement, // --

    OpenParen,   // (
    ClosedParen, // )
    OpenBrace,   // {
    ClosedBrace, // }
	OpenBracket,   // [
	ClosedBracket, // ]

    Equals,            // =
    Colon,             // :
    InitOp,            // :=
    ConstInitOp,       // ::
    Semicolon,         // ;
    EqualsCondition,   // ==
    GreaterThan,       // >
    LessThan,          // <
    EqualsGreaterThan, // >=
    EqualsLessThan,    // <=

    True, // true
    False, // false
    Not, // !
    NotCondition, // !=
    Or, // ||
	OrBitwise, // |
    And, // &&
	AndBitwise, // &

    Struct, // struct
    Object, // object
    Public, // pub
    Enum, // enum

    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub value: &'a str,
    pub token_type: TokenType,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    pub tokens: Vec<Token<'a>>,
    pos: usize,
    src: &'a str,
    chars: Chars<'a>,
    pub idents: Vec<Token<'a>>,
}

const EOF: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
			tokens: Vec::new(),
			pos: 0,
			src,
			chars: src.chars(),
			idents: Vec::new(),
		}
	}

	pub fn lex(&mut self) {
		for _ in 0..self.src.len() {
			self.token();
		}
	}

    pub fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or_default()
    }

    pub fn bump(&mut self) -> char {
        match self.chars.next() {
            Some(c) => {
                self.pos += c.len_utf8();
                c
            }
            None => '\0',
        }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn src(&self) -> &'a str {
        self.src
    }

    pub fn token(&mut self) -> Option<Token<'a>> {
        let start = self.pos;

        let token_type = match self.bump() {
            EOF => return None, // aka. '\0'

            c if is_digit(c) => TokenType::Number,
            c if is_whitespace(c) => TokenType::Whitespace,
            c if is_id_start(c) => self.ident(c),
            '.' => TokenType::Dot,

            '+' => match self.peek() {
				'+' => {
					self.bump();
					TokenType::Increment // ++
				}
				_ => TokenType::Plus,  // +
			}
            '-' => match self.peek() {
				'+' => {
					self.bump();
					TokenType::Decrement// --
				}
				_ => TokenType::Minus,  // -
			}

            '*' => TokenType::Star,  // *
            '/' => match self.peek() {
                '/' => self.line_comment(),
                _ => TokenType::Slash, // /
            },

            '(' => TokenType::OpenParen,   // (
            ')' => TokenType::ClosedParen, // )

            '{' => TokenType::OpenBrace,   // {
            '}' => TokenType::ClosedBrace, // }

			'[' => TokenType::OpenBracket,   // [
			']' => TokenType::ClosedBracket, // ]

            '=' => match self.peek() {
                '=' => self.equals_cond(), // ==
                _ => TokenType::Equals,    // =
            },

            ':' => match self.peek() {
                '=' => self.initop(),  // :=
                ':' => self.constop(), // ::
                _ => TokenType::Colon,
            },
            ';' => TokenType::Semicolon, // ;

            '>' => match self.peek() {
                '=' => TokenType::EqualsGreaterThan, // >=
                _ => TokenType::GreaterThan,         // >
            },

            '<' => match self.peek() {
                '=' => TokenType::EqualsLessThan, // <=
                _ => TokenType::LessThan,         // <
            },

            '"' => self.string(),

			'&' => match self.peek() {
				'&' => {
					self.bump();
					TokenType::And
				},
				_ => TokenType::AndBitwise
			},

			'|' => match self.peek() {
				'|' => {
					self.bump();
					TokenType::Or
				},
				_ => TokenType::OrBitwise
			},

			'!' => match self.peek() {
				'=' => {
					self.bump();
					TokenType::NotCondition
				},
				_ => TokenType::Not
			},

            _ => TokenType::Unknown,
        };

        if let TokenType::Ident = token_type {
            self.idents.push(Token {
                value: &self.src()[start..self.pos()],
                token_type,
            })
        }

		match token_type {
			TokenType::Comment => {},
			_ => {
                self.tokens.push(Token {
            	    value: &self.src()[start..self.pos()],
            	    token_type,
        		})
			}
		}

        Some(Token {
            value: &self.src()[start..self.pos()],
            token_type,
        })
    }

    fn line_comment(&mut self) -> TokenType {
        while !matches!(self.peek(), '\0' | '\n') {
            self.bump();
        }
        TokenType::Comment
    }

    fn initop(&mut self) -> TokenType {
        self.bump();
        TokenType::InitOp
    }

    fn constop(&mut self) -> TokenType {
        self.bump();
        TokenType::ConstInitOp
    }

    fn equals_cond(&mut self) -> TokenType {
        self.bump();
        TokenType::EqualsCondition
    }

    pub fn ident(&mut self, c: char) -> TokenType {
        let mut ident = String::from(c);

        while is_id_continue(self.peek()) {
            ident.push(self.bump());
        }
		
        match ident.as_str() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
			"true" => TokenType::True,
			"false" => TokenType::False,
            "pub" => TokenType::Public,
            "struct" => TokenType::Struct,
            "object" => TokenType::Object,
            "enum" => TokenType::Enum,
            _ => TokenType::Ident,
        }
    }

    fn string(&mut self) -> TokenType {
        let is_terminated = loop {
            match self.bump() {
                '\0' => break false,
                '"' => break true,
                _ => {}
            }
        };
        TokenType::String { is_terminated }
    }
}


fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\n')
}

fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn is_id_continue(c: char) -> bool {
    is_id_start(c) || is_digit(c)
}
