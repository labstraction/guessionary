

#[derive(Copy, Clone, Debug)]
pub struct Word<'a> {
    text: &'a str,
    position: i32
}

impl<'a> Word<'a>{
    pub fn new(text: &str, position: i32) -> Word {
        Word{
            text,
            position
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn position(&self) -> &i32 {
        &self.position
    }
}

////////////
#[derive(Copy, Clone, Debug)]
pub struct Status<'a>  {
    topWord: Word<'a>,
    guessWord: Option<Word<'a>>,
    bottomWord: Word<'a>
}

impl<'a> Status<'a>{
    pub fn new(top: Word<'a>, bottom: Word<'a>) -> Status<'a> {
        Status{
            topWord: top,
            bottomWord: bottom,
            guessWord: None
        }
    }

    pub fn get_top(self) -> Word<'a> {self.topWord}
    pub fn get_bottom(self) -> Word<'a> {self.bottomWord}


    pub fn nextStatus(self, guess: Word<'a>) -> (Status<'a>, Status<'a>){
        let old = Status{
            topWord: self.topWord,
            bottomWord: self.bottomWord,
            guessWord: Option::Some(guess)
        };

        let new = Status{
            topWord: self.topWord,
            bottomWord: self.bottomWord,
            guessWord: None
        };

        (old, new)
    }
}
#[derive(Debug)]
pub struct GameHistory<'a>  {
    history: Vec<Status<'a>>
}

impl<'a> GameHistory<'a>{
    pub fn new() -> GameHistory<'a> {
        GameHistory{
            history: vec![]
        }
    }

    pub fn record(&mut self, status: Status<'a>){
        self.history.push(status);
    }
}