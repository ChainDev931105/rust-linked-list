use std::env;
use std::fs;
mod linked_list;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let content = fs::read_to_string(path).expect("Something went wrong reading the file");
    let split = content.split(",");
    let numbers: Vec<&str> = split.collect();
    
    for s in numbers {
        let n: i32 = s.parse().unwrap();
        println!("{:?} {}", s, n);
    }
}
