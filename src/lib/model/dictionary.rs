
use rand::prelude::IteratorRandom;
use super::word::{self, Word};

static ITA_DICT: &'static str = include_str!("../../../assets/ita.txt");

fn select_dict(language: &str) -> &str{
    if language == "ITA" {
        return ITA_DICT;
    }
    ITA_DICT
}

#[derive(Clone, Debug)]
pub struct Dictionary<'a>{
    words: &'a str,
}


pub fn new_dictionary(language: &str) -> Dictionary{
    Dictionary{
        words: select_dict(language)
    }
}


pub fn get_random_word(dictionary: &Dictionary) -> Option<Word>{
    dictionary.words.split(",")
                    .enumerate()
                    .map(|(u,s)| word::new_word(s, u as i32))
                    .choose(& mut rand::thread_rng())
}

pub fn get_top_word(dictionary: &Dictionary) -> Word{
    dictionary.words.split(",")
                    .enumerate()
                    .map(|(u,s)| word::new_word(s, u as i32))
                    .collect::<Vec<Word>>()[0].clone()
}

pub fn get_bottom_word(dictionary: &Dictionary) -> Word{
    let all_words = dictionary.words.split(",")
                                             .enumerate()
                                             .map(|(u,s)| word::new_word(s, u as i32))
                                             .collect::<Vec<Word>>();
    all_words[all_words.len() - 1].clone()
}

pub fn find_word(dictionary: &Dictionary, text: &str) -> Option<Word>{
    dictionary.words.split(",")
                    .enumerate()
                    .map(|(u,s)| word::new_word(s, u as i32))
                    .find(|w|w.text() == text)

}


