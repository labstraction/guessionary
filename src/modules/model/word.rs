

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

