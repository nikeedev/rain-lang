module main

import os
import io
import term

fn main() {
	mut file := ''


	if os.args.len < 2 {

		println(term.blue('Rain Lang - v0.1.0a'))
		println(term.cyan('\nUsage: rain <source file>.rain'))

	} else {
		mut code := []string{}

		for i := 1; i < os.args.len; i++ {
			if !os.exists(os.args[i]) {
				eprintln("Error ${os.args[i]} doesn't exist")
				exit(1)
			} else {
				file = os.args[i]
				break
			}
		}


		mut f := os.open(file) or { panic(err) }
		defer {
			f.close()
		}

		mut r := io.new_buffered_reader(reader: f)
		for {
			l := r.read_line() or { break }
			code << l
		}


		for line in code {
			println(line)
		}

	
	}
}
