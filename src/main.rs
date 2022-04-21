mod nlp;

use std::process;

fn main() {

    let dir = clinlp::arg_parsing();

    if let Err(err) = clinlp::run(dir) {
        eprintln!("Someting went wrong accesing the file: {}", err);
        process::exit(1);
    }
}
