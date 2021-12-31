use std::fs;
use std::env;

fn simple_print_file(filename: &str) {
    
let contents = fs::read_to_string(filename)
        .expect("rustcat: no such file or directory");

    print!("{}", contents);
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("rustcat only supports printing the content of a single file right now");
        return;
    }
    simple_print_file(&args[1]);
}
