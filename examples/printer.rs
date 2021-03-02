#![allow(dead_code, unused_imports)]
use har::{from_path, to_json, Error};
use std::io::Write;

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        match har::from_path(path) {
            Ok(spec) => {
                /*for (path, op) in spec.paths {
                    println!("{}", path);
                    println!("{:#?}", op);
                }
                for (name, definition) in spec.definitions {
                    println!("{}", name);
                    println!("{:#?}", definition);
                }*/
                println!("{}", har::to_json(&spec).unwrap());
            }
            Err(e) => {
                match e {
                    Error::Io(e) => eprintln!("{}", e),
                    Error::Yaml(e) => eprintln!("{}", e),
                    Error::Json(e) => eprintln!("{}", e),
                }

                ::std::process::exit(1);
            }
        }
    }
}
