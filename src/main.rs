use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);

    println!("Searching for {}", query);
    println!("... in file {}", filename);

    let contents = fs::read_to_string(filename).expect("unable to read file");
    println!("with text:\n{}", contents)
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
