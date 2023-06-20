module main

import os
import io
import term
import rain

fn main() {
	mut file := ''

	if os.args.len < 2 {

		println(term.blue('Rain Lang Compiler - v0.1.0a'))
		println(term.cyan('\nUsage: rain <source file>.rain'))

		mut rain_sources := []string{}
		files := os.ls('.') !

		for rain_file in files {
			if rain_file.ends_with('.rain') {
				rain_sources << rain_file
			}
		}
		if rain_sources.len > 0 {
			println('Avalable .rain source files in this folder: \n')
			for rain_file in rain_sources {
				println(term.cyan(rain_file))
			}
		}

		exit(1)
	} else {

		for i := 1; i < os.args.len; i++ {
			if !os.exists(os.args[i]) {
				eprintln("Error ${os.args[i]} doesn't exist")
				exit(1)
			} else {
				file = os.args[i]
				break
			}
		}

		rain.load(file)

	}
}
