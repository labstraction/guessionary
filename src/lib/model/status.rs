

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

pub fn get_status_distance(status: &Status) -> i32{
    status.bottom_word.position() - status.top_word.position() -1
}

pub fn next_status(status: Status, guessed_word: Word, secret_word:&Word) -> (Status, Status){
    let old = Status{
        top_word: status.top_word.clone(),
        bottom_word: status.bottom_word.clone(),
        guess_word: Some(guessed_word.clone())
    };

    let new: Status;

    let is_new_top = guessed_word.position() < secret_word.position() && guessed_word.position() > status.top_word.position();
    let is_new_bottom = guessed_word.position() > secret_word.position() && guessed_word.position() < status.bottom_word.position();

    if is_new_top {
        new = Status{
            top_word: guessed_word,
            bottom_word: status.bottom_word,
            guess_word: None
        };
    } else if is_new_bottom {
        new = Status{
            top_word: status.top_word,
            bottom_word: guessed_word,
            guess_word: None
        };
    } else {
        new = Status{
            top_word: status.top_word,
            bottom_word: status.bottom_word,
            guess_word: None
        };
    }


    

    (old, new)
}