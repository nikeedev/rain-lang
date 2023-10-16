mod rain;

use std::{env, fs};

fn find(array: Vec<&str>, element: &str) -> usize {
	let mut j = 0;

	for elem in array {
		if elem == element {
			j = i
		}
	}

	return j
}

enum TokenTypes {
	InitEquals,
	InitConst,
	Equal,
	Main,
	Number,
	Identifier,
}

const (
	token_type_string = [
		'=',
		'::',
		'=',
		'main'
	]
)

struct Token {
mut:
	value string
	token_type TokenTypes
}

fn token(value string, token_type TokenTypes) Token {
	return Token{ value: value, token_type: token_type }
}


pub fn load(file: String) {
	let mut tokens = []Token{}

	let mut f = fs::read_string(&file);

	let mut src_raw = "";
	for line in f {
		src_raw += line.str().trim_space()
	}

	// println(src_raw)

	let mut src = src_raw.split(' ')

	for line in src {
		if line.trim_space() == " " {
			src.delete(src.index(line))
		}
		if line.trim_space().starts_with("//") {
			src.delete(src.index(line))
		}
	}

	for line in src {
		if find(token_type_string, line) == int(TokenTypes.main)
		{
			tokens << token(line, .main)
		}
		if find(token_type_string, line) == int(TokenTypes.init_const)
		{
			tokens << token(line, .init_const)
		}
		if find(token_type_string, line) == int(TokenTypes.equals)
		{
			tokens << token(line, .equals)
		}
	}

	for token in tokens {
		println(token)
	}

	// for line in code {
	// 	println(line)
	// }


}
