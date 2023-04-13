use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    let question = &args[1];
    let file = &args[2];
    println!("{} {}", question, file);
}
