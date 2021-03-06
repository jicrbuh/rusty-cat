use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn handle_stdin() {
        let stdin = io::stdin();
      for line in stdin.lock().lines() {
        match line {
            Err(_) => break,
            Ok(s) => println!("{}", s),
        }

    }
    
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn buffered_read_print(filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
        match line {
            Ok(line) => {
                println!("{}", line)
            },
            Err(_) => {
                println!("Unexpected error reading file.")
                }
            } 
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {

        handle_stdin();
    
    } else {

        for filename in &args[1..] {
            buffered_read_print(filename); 
        
        }
    }
}
