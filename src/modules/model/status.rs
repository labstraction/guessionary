use super::word::Word;

#[derive(Copy, Clone, Debug)]
pub struct Status<'a>  {
    top_word: Word<'a>,
    guess_word: Option<Word<'a>>,
    bottom_word: Word<'a>
}

impl<'a> Status<'a>{
    pub fn new(top: Word<'a>, bottom: Word<'a>) -> Status<'a> {
        Status{
            top_word: top,
            bottom_word: bottom,
            guess_word: None
        }
    }

    pub fn get_top(self) -> Word<'a> {self.top_word}
    pub fn get_bottom(self) -> Word<'a> {self.bottom_word}


    pub fn next_status(self, guess: Word<'a>) -> (Status<'a>, Status<'a>){
        let old = Status{
            top_word: self.top_word,
            bottom_word: self.bottom_word,
            guess_word: Option::Some(guess)
        };

        let new = Status{
            top_word: self.top_word,
            bottom_word: self.bottom_word,
            guess_word: None
        };

        (old, new)
    }
}