

#[derive(Copy, Clone, Debug)]
pub struct Word<'a> {
    pub text: &'a str,
    pub position: i32
}

pub fn new_word(text: &str, position: i32) -> Word {
    Word{
        text,
        position
    }
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

