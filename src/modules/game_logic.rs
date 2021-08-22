
use crate::modules::model::Status;

use super::{dictionary::Dictionary, model::{GameHistory, Word}};




pub fn new_game(country: &str){
    let dict = Dictionary::new(country);

    let mut hist = GameHistory::new();

    let mut _actualStatus = Status::new(dict.get_top_word().unwrap(), dict.get_bottom_word().unwrap());

    let (old, new) = _actualStatus.nextStatus(Word::new("pippo", -1));

    hist.record(old);

    _actualStatus = new;

    print!("{:?}", hist)
}

