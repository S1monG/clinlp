mod nlp;

use clap::{Arg, Command};

fn main() {

    // basic app information
    let m = Command::new("Command line nlp")
        .version("1.0")
        .about("Provides different statistics of the input text")
        .author("Simon Gustafsson");

    // Define the name command line option
    let name_option = Arg::new("file")
        .short('f')   // -f
        .long("file") // --file
        .takes_value(true)
        .value_name("FILENAME")
        .help("File to examine")
        .required(true);

    let m = m.arg(name_option);

    let matches = m.get_matches();
    let dir = matches.value_of("file");

    println!("Test 123 {:?}", dir);
}
