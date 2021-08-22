use super::word::Word;

#[derive(Clone, Debug)]
pub struct Status  {
    top_word: Word,
    guess_word: Option<Word>,
    bottom_word: Word
}

impl Status{

    pub fn get_top(&self) -> &Word {&self.top_word}
    pub fn get_bottom(&self) -> &Word {&self.bottom_word}

}

pub fn new_status(top_word:Word, bottom_word:Word) -> Status{
    Status{
        top_word,
        bottom_word,
        guess_word: None
    }
}

pub fn next_status(status: Status, guess: Word) -> (Status, Status){
    let old = Status{
        top_word: status.top_word.clone(),
        bottom_word: status.bottom_word.clone(),
        guess_word: Some(guess)
    };

    let new = Status{
        top_word: status.top_word,
        bottom_word: status.bottom_word,
        guess_word: None
    };

    (old, new)
}