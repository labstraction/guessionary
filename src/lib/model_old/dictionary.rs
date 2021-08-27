
use rand::Rng;
use super::word::Word;

static ITA_DICT: &'static str = include_str!("../../../assets/ita.txt");

fn select_dict(language: &str) -> &str{ITA_DICT}

#[derive(Copy, Clone, Debug)]
pub struct Dictionary<'a>{
    words: &'a str,
}

impl<'a> Dictionary<'a>{

    pub fn new(language: &str) -> Dictionary{
        Dictionary{
            words: select_dict(language)
        }
    }

    pub fn words(self) -> Vec<Word<'a>>{
        self.words.split(",").enumerate().map(|(u,s)| Word::new(s, u as i32)).collect()
    }

    pub fn get_top_word(self) -> Option<Word<'a>>{
        let words = self.words();
        if words.len() != 0 {
            return Option::Some(words[0].clone());
        }
        None
    }

    pub fn get_bottom_word(self) -> Option<Word<'a>>{
        let words = self.words();
        if words.len() != 0 {
            return Option::Some(words[words.len()-1]);
        }
        None
    }

    pub fn get_random_word(self) -> Option<Word<'a>>{
        let words = self.words();
        if words.len() != 0 {
            return Option::Some(words[rand::thread_rng().gen_range(0..words.len())]);
        }
        None
    }

}

