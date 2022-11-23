use std::collections::HashMap;
use regex::Regex;

/// The [Analysis] struct is used to generate an analysis of a piece of text.
pub struct Analysis {
    string: String,
    number_of_words: i32,
    number_of_sentences: i32,
    word_counts: HashMap<String, i32>
}

impl Analysis {

    /// Instantiates a new [Analysis] object and returns it.
    pub fn new(string: &str) -> Self {
        let mut analysis = Analysis {
            string: String::from(string),
            number_of_words: 0,
            number_of_sentences: 0,
            word_counts: HashMap::new()
        };
        analysis.populate_stats();
        analysis
    }

    pub fn output_report(&self) {
        println!("----- STRING ANALYSIS -----");
        println!("       Input string: {}", self.string);
        println!("Number of sentences: {}", self.number_of_sentences);
        println!("    Number of words: {}", self.number_of_words);
        for word_tuple in &self.word_counts {
            let word          = word_tuple.0;
            let word_count    = word_tuple.1;
            let times_variant = if word_count == &1i32 { "" } else { "s" };
            println!("        {}: {} time{}", word, word_count, times_variant);
        }
    }

    /// Populates an [Analysis] object's fields.
    fn populate_stats(&mut self) {
        let words = self.string.split_whitespace();
        for word in words {
            let word = word.to_lowercase();
            self.number_of_words += 1;
            let end_of_sentence = word.ends_with(".") ||
                                  word.ends_with("?") ||
                                  word.ends_with("!");
            if end_of_sentence { self.number_of_sentences += 1 }
            let regex = Regex::new("[a-zA-Z'-]+").unwrap();
            let word = match regex.find(&word) {
                None => continue,
                Some(mat) => mat.as_str().to_string()
            };
            self.word_counts.entry(word)
                .and_modify(|value| *value += 1)
                .or_insert(1);
        }
    }
}