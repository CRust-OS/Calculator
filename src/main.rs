extern crate calculator;

use calculator::parse_line;
use std::io;
use std::io::prelude::*;

fn prompt(){
    print!("> ");
    io::stdout().flush();
}

fn main(){
    let reader = io::BufReader::new(io::stdin());
    println!("Basic calculator: \n");
    prompt();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 { break; }

        if let Some(i) = parse_line(line) {
            println!("= {}", i);
        } else {
            println!("ERROR");
        }
        prompt();
    };
}
