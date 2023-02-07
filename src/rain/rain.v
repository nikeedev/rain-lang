module rain

import os


enum TokenTypes {
	number
	identifier
	init_quals
	equals
	open_paran
	closed_paran
	binary_operator
}

pub interface Token {
mut:
	value string
	token_type TokenTypes
}

fn token(value string, token_type TokenTypes) Token {
	return Token{ value, token_type }
}


pub fn load(file string) {
	mut tokens := []Token{}

	mut f := os.read_lines(file) or { panic(err) }

	mut src := ""
	for line in f {
		src += line.str()
	}

	println(src)

	// for line in code {
	// 	println(line)
	// }


}
