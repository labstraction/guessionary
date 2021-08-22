use super::status::Status;

#[derive(Clone, Debug)]
pub struct GameHistory {
    records: Vec<Status>
}

impl GameHistory{
    pub fn records(&self) -> &Vec<Status>{
        &self.records
    }
}


pub fn new_history() -> GameHistory {
    GameHistory{
        records: vec![]
    }
}


pub fn record_status(mut history: GameHistory, status: Status) -> GameHistory{
    history.records.push(status);
    history
}


