use std::{fs, error::Error};

use clap::{Arg, Command};

pub fn arg_parsing() -> String {
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
    let dir = matches.value_of("file").expect("Something went wrong with the input");
    dir.to_owned()
}

pub fn run(dir: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(dir)?;

    println!("With text:\n{}", contents);

    Ok(())
}