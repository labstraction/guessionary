


mod lib;


// use std::fs::File;
// use std::io::Write;

use lib::game_logic::new_game;
use clap::{App, Arg, ArgMatches};
// static ITA_DICT: &'static sr = include_str!("../assets/ita2.txt");


fn main() {

    let matches = get_user_preferences();

    let language = matches.value_of("Language").unwrap_or("ITA");

    let difficulty = matches.value_of("difficulty").unwrap_or("NORMAL");
    

    new_game(language, difficulty);
    
}


fn get_user_preferences() -> ArgMatches<'static>{
    App::new("Guessionary")
        .version("0.1.0")
        .author("labstraction <labstraction@protonmail.com>")
        .about("A simple game about guessing words.")
        .arg(Arg::with_name("language")
                 .short("l")
                 .long("language")
                 .takes_value(true)
                 .help("select the language. Available: ITA - ENG. Default: ITA."))
        .arg(Arg::with_name("difficulty")
                 .short("d")
                 .long("difficulty")
                 .takes_value(true)
                 .help("Select the difficulty level. Available: NORMAL - HARD - IMPOSSIBLE. Default: NORMAL"))
        .get_matches()
}


// fn main() {

//     let lines: Vec<&str> = ITA_DICT.lines().map(|line| {
//         line.split("/")
//         .enumerate()
//         .filter(|&(i, _ )| i == 0)
//         .map(|( _ ,line)| line)
//         .collect::<Vec<&str>>()[0]

//     }).collect();


//     let path = "results.txt";
//     let mut output = File::create(path).unwrap();

//     for line in lines {
//         write!(output, "{},", line).unwrap();  
//     }

    
// }