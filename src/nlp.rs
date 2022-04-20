use std::collections::HashMap;

struct Word {
    word : String,
    length : usize,
    count : usize,
    follow_words : HashMap<String, u32>,
}

impl Word {

    fn new(word: String, length: usize, next_word: String) -> Word {
        Word { word, 
            length, 
            count: 1, 
            follow_words: HashMap::from([(next_word, 1)]),
        }
    }

    fn update(&mut self, next_word: String) {
        self.count += 1;

        let n = &next_word.clone();
        if self.follow_words.contains_key(n) {
            self.follow_words.insert(next_word, self.follow_words.get(n).unwrap() + 1);
        } else {
            self.follow_words.insert(next_word, 1);
        }
    }
}

// equality only checks for the string
impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
    }
}

struct Text {
    words : Vec<Word>,
}

impl Text {
    fn new() -> Text {
        Text { words : Vec::new() }
    }

    fn register(&self, word: String, next_word: String) {
        
    }

}