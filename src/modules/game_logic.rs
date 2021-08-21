
use super::dict_selector::get_dict;

use rand::Rng;

pub fn select_word(dict: Vec<(i32, &str)>) -> (i32, &str) {
    dict[rand::thread_rng().gen_range(0..dict.len())]
}



pub fn new_game(country: &str){
    let dict = get_dict(country);
    let selected_word = select_word(dict);

    println!("{}", selected_word.1);
}