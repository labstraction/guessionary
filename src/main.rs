// use std::fs::File;
// use std::io::Write;

mod modules;


use modules::{game_logic::new_game};


fn main() {

    new_game("ita");
    
}

// fn main() {

//     let lines: Vec<&str> = HOST_FILE.lines().map(|line| {
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