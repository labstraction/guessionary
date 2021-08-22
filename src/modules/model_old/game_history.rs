use super::status::Status;

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

// pub struct Pippo {
//     pub eccolo: fn()-> i32
// }


