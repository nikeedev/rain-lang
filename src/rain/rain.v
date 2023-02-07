module rain

fn load(file string) {
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
