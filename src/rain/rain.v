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

struct Token {
mut:
	value string
	token_type TokenTypes
}

fn token(value string, token_type TokenTypes) Token {
	return Token{ value: value, token_type: token_type }
}


pub fn load(file string) {
	mut tokens := []Token{}

	mut f := os.read_lines(file) or { panic(err) }

	mut src_raw := ""
	for line in f {
		src_raw += line.str().trim_space()
	}

	// println(src_raw)

	mut src := src_raw.split(' ')

	for line in src {
		if line.trim_space() == " " {
			src.delete(src.index(line))
		}
		if line.trim_space().starts_with("//") {
			src.delete(src.index(line))
		}
	}

	for line in src {
		println(line)
	}

	// for line in code {
	// 	println(line)
	// }


}
