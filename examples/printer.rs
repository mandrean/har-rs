use har::{from_path, to_json};

fn main() {
    let Some(path) = std::env::args_os().nth(1) else {
        eprintln!("usage: cargo run --example printer -- <path-to.har>");
        std::process::exit(2);
    };

    match from_path(&path) {
        Ok(har) => {
            let json = to_json(&har).expect("serializing parsed HAR as JSON should succeed");
            println!("{json}");
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
