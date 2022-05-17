mod linked_list;

use std::env;
use std::fs;
use linked_list::LinkedList;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let content = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut numbers: Vec<u32> = content
        .split(",")
        .map(|s| s.parse().expect("parse error"))
        .collect();

    numbers.sort();

    let mut linked_list: LinkedList<u32> = LinkedList::new(numbers[0]);

    for (i, n) in numbers.iter().enumerate() {
        if i > 0 {
            linked_list.push(*n);
        }
    }

    println!("{:?}", linked_list.to_array());

    linked_list.remove(1); // index 1 means 2nd node
    
    println!("{:?}", linked_list.to_array());
}
