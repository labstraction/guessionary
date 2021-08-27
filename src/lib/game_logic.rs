extern crate termion;
use termion::clear;
use crate::lib::model::{dictionary::find_word, game_history::record_status, status::{get_status_distance, next_status}};

use super::model::{dictionary::{get_bottom_word, get_random_word, get_top_word, new_dictionary}, game_history:: new_history, status::{new_status}};
use std::{io::{self, Write}, thread, time::Duration};



pub fn new_game(language: &str){

    let dict = new_dictionary(language);

    let mut hist = new_history();

    let mut actual_status = new_status(get_top_word(&dict), get_bottom_word(&dict));

    let random_word = get_random_word(&dict).unwrap();

    println!("Benvenuta!");

    loading("lasciami pensare", 3);

    println!("Ho individuato la parola adatta, prova a indovinarla!");

    loop {
        println!("La tua parola è tra {} e {} e ci sono {} parole possibili",
                 actual_status.get_top().text().to_uppercase(),
                 actual_status.get_bottom().text().to_uppercase(),
                 get_status_distance(&actual_status));

        let input = get_input("Scrivi la tua parola: ").to_lowercase();
        if input == random_word.text() {
            println!("Bravissima, hai indovinato!");
            println!("Hai impiegato {} tentativi.", hist.records().len()+1);
            break;
        }
        let guessed = match find_word(&dict, &input){
            Some(w) => w,
            None =>{
                println!("Parola non trovata, riprova!");
                continue;
            } 
        };

        let (old, new) = next_status(actual_status, guessed, &random_word);

        hist = record_status(hist, old);

        actual_status = new;
    }

}


fn loading(text:&str,repeat: i32){

    for _ in 0..=repeat{

        print!("{}",text);

        for _ in 0..6 {
            io::stdout().flush().unwrap();
            print!(".",);
            thread::sleep(Duration::from_millis(500));
        }
        print!("\r");
        print!("{}", clear::CurrentLine);
    }

}

fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

// pub fn new_game(language: &str){
//     let dict = Dictionary::new(language);

//     let mut hist = GameHistory::new();

//     let mut _actual_status = Status::new(dict.get_top_word().unwrap(), dict.get_bottom_word().unwrap());

//     let (old, new) = _actual_status.next_status(Word::new("pippo", -1));

//     hist.record(old);

//     _actual_status = new;

//     // let eccolo = ||34;

//     // let pippo = Pippo{eccolo: eccolo};

//     print!("{:?}", hist)
// }

