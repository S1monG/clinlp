mod nlp;

use std::{fs, process};

fn main() {
    let dir = clinlp::arg_parsing();

    let contents = fs::read_to_string(dir);

    if let Err(err) = contents {
        eprintln!("Someting went wrong accesing the file: {}", err);
        process::exit(1);
    } else {
        nlp::analyser::start_loop(contents.unwrap());
    }
}
