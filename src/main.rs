use std::fs;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io;

// TODO: handle case of "rustcat > newfile"
// either create or open a file, delete its content (maybe) and write to it using standard IO
fn read_standard_io() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}

#[allow(unused_must_use)] 
fn simple_print_file(filename: &str) {
    
    match fs::read_to_string(filename) {
        Ok(contents) => print!("{}", contents),
        Err(_) => println!("rustcat: {}: no such file or directory", filename),
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
    println!("debug: args: {:?}", args);
    //let filenames: Vec<String> = 
    if args.len() < 2 {
        println!("rustcat only supports printing the content of files right now");
        return;
    }
    for filename in &args[1..] {
        simple_print_file("fff");
        buffered_read_print(filename); 
    }
}
