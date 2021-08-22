

#[derive(Clone, Debug)]
pub struct Word{
    text: String,
    position: i32
}

impl Word{
    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn position(&self) -> &i32 {
        &self.position
    }
}

pub fn new_word(text: &str, position: i32) -> Word {
    Word{
        text: String::from(text),
        position
    }
}


