

use super::model::{
    dictionary::Dictionary,
    word::Word,
    status::Status,
    game_history::GameHistory
};




pub fn new_game(country: &str){
    let dict = Dictionary::new(country);

    let mut hist = GameHistory::new();

    let mut _actual_status = Status::new(dict.get_top_word().unwrap(), dict.get_bottom_word().unwrap());

    let (old, new) = _actual_status.next_status(Word::new("pippo", -1));

    hist.record(old);

    _actual_status = new;

    print!("{:?}", hist)
}

