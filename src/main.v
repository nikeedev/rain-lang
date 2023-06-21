module main

import os
import term
import rain

fn main() {
	mut file := ''

	if os.args.len < 2 {
		println(term.blue('Rain Lang Compiler - v0.1.2a'))
		println(term.cyan('\nUsage: rain <source file>.rain'))
	} else {
		for i := 1; i < os.args.len; i++ {
			if !os.exists(os.args[i]) {
				eprintln('Error! "${os.args[i]}" doesn\'t exist')
				exit(-1)
			} else {
				file = os.args[i]
				break
			}
		}

		rain.load(file)
	}
}
