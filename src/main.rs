mod rain;
use std::{
	env,
	process,
	fs
};
use colored::*;
use toml;
use serde::Deserialize;

use rain::lexer::*;

#[derive(Debug, Deserialize)]
struct Config {
	about: About,
}

#[derive(Debug, Deserialize)]
struct About {
	name: String,
	version: String,
	author: Vec<String>,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let file: String =
        match fs::read_to_string("rain.toml") {
            Ok(x) => x,
            Err(x) => panic!("Error reading rain.toml file: {}", x)
        };

	let config: Config = match toml::from_str(&file) {
		Ok(x) => x,
		Err(x) => panic!("Error parsing rain.toml file: {}", x)
	};

	let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading file: {}", x)
        }
    } else {
        println!("{}", format!("Rain Lang Compiler - {}", config.about.version.as_str()).blue());
		println!("{}", "\nUsage: rain <source file>.rain".cyan());
		process::exit(0);
    };

	let mut _lexer = Lexer::new(file.as_str()).clone();

	_lexer.lex();

	let _tokens = _lexer.tokens;

//	for token in tokens {
//		println!("{:#?}", token);
//	}

	let idents = _lexer.idents;

	for ident in &idents {
		println!("{:#?}", ident);
	}

}
