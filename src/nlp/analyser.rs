use super::register::Text;

use std::{collections::HashMap, process};

use rand::Rng;

const SENTENCE_LENGTH: usize = 10;

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
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "1" => word_statistics(&text.count),
            "2" => follow_word_statistics(&text.follow_freq),
            "3" => random_sentence_generator(&text.follow_freq),
            _ => {
                println!("Bye!");
                process::exit(0);
            }
        }
    }
}

fn word_statistics(count: &HashMap<String, usize>) {
    let word_count: usize = count.values().sum();
    let most_common_word = count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let different_words: usize = count.len();
    let average_len: f64 = count
        .iter()
        .map(|(key, val)| (key.len() * val) as f64)
        .reduce(|a, b| a + b)
        .unwrap()
        / word_count as f64;

    println!("\nThere are {} total words in the text", word_count);
    println!("{} different words in the text.", different_words);
    println!(
        "The most common word is '{}' which occur {} times",
        most_common_word.0, most_common_word.1
    );
    println!(
        "and the average length of a word is {:.2} characters",
        average_len
    );
}

fn follow_word_statistics(follow_freq: &HashMap<String, HashMap<String, usize>>) {
    println!("Which word would you like to see?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    if follow_freq.contains_key(input.trim()) {
        let mut vec_hash: Vec<(&String, &usize)> =
            follow_freq.get(input.trim()).unwrap().iter().collect();

        vec_hash.sort_by(|a, b| b.1.cmp(a.1));

        let loop_count = if vec_hash.len() < 10 {
            vec_hash.len()
        } else {
            10
        };

        println!("Most common words after '{}':", input.trim());

        for item in vec_hash.iter().take(loop_count) {
            println!("{:?}", item);
        }
    } else {
        println!("No such word.");
    }
}

fn random_sentence_generator(follow_freq: &HashMap<String, HashMap<String, usize>>) {
    let mut rng = rand::thread_rng();

    // starts with a random String from the hashmap
    let mut res = follow_freq.keys().next().unwrap().to_owned();

    let mut next_word = &res;

    for _ in 0..SENTENCE_LENGTH {
        let mut next: Vec<(&String, &usize)> = follow_freq.get(next_word).unwrap().iter().collect();

        next.sort_by(|a, b| b.1.cmp(a.1));
        next_word = if next.len() < 2 {
            next[0].0
        } else {
            next[rng.gen_range(0..=2)].0
        };
        res.push(' ');
        res.push_str(next_word);
    }

    println!("Randomly generated sentence:\n{}", res);
}
