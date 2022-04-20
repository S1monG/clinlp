use std::collections::HashMap;

pub struct Text {
    pub follow_freq : HashMap<String, HashMap<String, usize>>,
    pub count : HashMap<String, usize>,
}

impl Text {
    pub fn new() -> Text {
        Text { follow_freq: HashMap::new(), count: HashMap::new() }
    }

    fn register_count(&mut self, word: String) {
        if let Some(x) = self.count.get_mut(&word) {
            *x = x.clone() + 1;
        } else {
            self.count.insert(word.clone(), 1);
        }
    }

    fn register_freq(&mut self, word: String, follow_word: String) {
        if let Some(x) = self.follow_freq.get_mut(&word) {
            if let Some(y) = x.get_mut(&follow_word) {
                *y = y.clone() + 1;
            } else {
                x.insert(follow_word, 1);
            }
        } else {
            self.follow_freq.insert(word, HashMap::from([(follow_word, 1)]));
        }
    }

    pub fn register_words(&mut self, words: Vec<String>) {
        for i in 0..words.len()-1 {
            let word = words.get(i).unwrap();
            let follow_word = words.get(i+1).unwrap();
            self.register_count(word.to_owned());
            self.register_freq(word.to_owned(), follow_word.to_owned());
        }

        self.register_count(words.get(words.len()-1).unwrap().to_owned());
    }
}
