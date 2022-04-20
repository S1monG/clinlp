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

    small_test();
}

fn small_test() {

    let input = "a a a a a b b b b";

    let mut t = nlp::Text::new();

    let vec: Vec<String> = input.split(" ").map(|a| a.to_owned()).collect();

    t.register_words(vec);

    println!("{:?}", t.count);
    println!("{:?}", t.follow_freq);
}
