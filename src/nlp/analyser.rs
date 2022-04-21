use super::register::Text;

use std::{collections::HashMap, process};

pub fn start_loop(s: String) {
    let mut text = Text::new();

    text.register_words(s.split_whitespace().map(str::to_string).collect());

    loop {
        println!(
            "
        CONTROLS:
        1: Word statistics
        2: Following word statistics
        3: Random generated sentence from most common words
        Other: Exit the program
        "
        );

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Someting went wrong parsing the input");
        match input.as_str() {
            "1" => word_statistics(&text.count),
            "2" => follow_word_statistics(&text.follow_freq),
            "3" => random_sentence_generator(&text.follow_freq),
            _ => {
                println!("Bye!");
                process::exit(0)
            }
        }
    }
}

fn word_statistics(count: &HashMap<String, usize>) {
    let word_count: usize = count.values().sum();
    let average_len: f64 = count
        .iter()
        .map(|(key, val)| (key.len() * val) as f64)
        .reduce(|a, b| (a + b) / 2.)
        .unwrap();

    todo!()
}

fn follow_word_statistics(follow_freq: &HashMap<String, HashMap<String, usize>>) {
    todo!()
}

fn random_sentence_generator(follow_freq: &HashMap<String, HashMap<String, usize>>) {
    todo!()
}
