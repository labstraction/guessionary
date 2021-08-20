use std::fs::File;
use std::io::Write;

static HOST_FILE: &'static str = include_str!("../assets/ita.txt");

fn main() {

    let lines: Vec<&str> = HOST_FILE.lines().map(|line| {
        line.split("/")
        .enumerate()
        .filter(|&(i, _ )| i == 0)
        .map(|( _ ,line)| line)
        .collect::<Vec<&str>>()[0]

    }).collect();


    let path = "results.txt";
    let mut output = File::create(path).unwrap();

    for line in lines {
        write!(output, "{},", line).unwrap();  
    }

    
}