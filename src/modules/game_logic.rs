

use crate::modules::model::game_history::record_status;

use super::model::{dictionary::{get_bottom_word, get_random_word, get_top_word, new_dictionary}, game_history:: new_history, status::{new_status, next_status}};


pub fn new_game(country: &str){

    let dict = new_dictionary(country);

    let mut hist = new_history();

    let actual_status = new_status(get_top_word(&dict), get_bottom_word(&dict));

    let random_word = get_random_word(&dict);

    let (old, new) = next_status(actual_status, random_word.unwrap());

    hist = record_status(hist, old);

    print!("{:?}", hist)
}


// pub fn new_game(country: &str){
//     let dict = Dictionary::new(country);

//     let mut hist = GameHistory::new();

//     let mut _actual_status = Status::new(dict.get_top_word().unwrap(), dict.get_bottom_word().unwrap());

//     let (old, new) = _actual_status.next_status(Word::new("pippo", -1));

//     hist.record(old);

//     _actual_status = new;

//     // let eccolo = ||34;

//     // let pippo = Pippo{eccolo: eccolo};

//     print!("{:?}", hist)
// }

