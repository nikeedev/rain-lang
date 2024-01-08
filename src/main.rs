mod rain;
use std::{
	env,
	process,
	fs
};
use colored::*;
use rain::Lexer;



fn main() {
	let args: Vec<String> = env::args().collect();

	let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading file: {}", x)
        }
    } else {
        println!("{}", "Rain Lang Compiler - v0.1.2a".blue());
		println!("{}", "\nUsage: rain <source file>.rain".cyan());
		process::exit(1);
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
