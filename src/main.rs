#![allow(dead_code)]
#![allow(unused)]

mod rain;
use colored::*;
use serde::Deserialize;
use std::{collections::HashMap, env, fs, process};

use rain::lexer::*;

#[derive(Debug, Deserialize)]
struct Config {
    project: Project,
    droplets: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    version: String,
    author: Vec<String>,
    description: String,
    raindrop: i32,
    out: String,
}

#[derive(Debug, Deserialize)]
struct Droplet {
    name: String,
    version: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // ###### For storm, but right now just for reading the version file
    let file: String = match fs::read_to_string("rain.toml") {
        Ok(x) => x,
        Err(x) => panic!("Error reading rain.toml file: {}", x),
    };

    let config: Config = match toml::from_str(&file) {
        Ok(x) => x,
        Err(x) => panic!("Error parsing rain.toml file: {}", x),
    };

    let droplets: Vec<Droplet> = config
        .droplets
        .iter()
        .map(|droplet| Droplet {
            name: droplet.0.clone(),
            version: droplet.1.clone(),
        })
        .collect();

    // println!("{:#?}", droplets);
    // ###### END - For storm, but right now just for reading the version file

    let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading source file {}: {}", args[1], x),
        }
    } else {
        println!(
            "{}",
            format!("Rain Lang Compiler - {}", config.project.version.as_str()).blue()
        );
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
