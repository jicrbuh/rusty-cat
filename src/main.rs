use std::fs;
use std::env;

fn simple_print_file(filename: &str) {
    
    match fs::read_to_string(filename) {
        Ok(contents) => print!("{}", contents),
        Err(_) => println!("rustcat: {}: no such file or directory", filename),
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
        simple_print_file(filename);
    }
}
