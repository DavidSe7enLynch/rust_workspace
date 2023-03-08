use add_one;
use add_two;

fn main() {
    println!("add_one({}) = {}", 1, add_one::add_one(1));
    println!("add_two({}) = {}", 3, add_two::add_two(3));
    println!("Hello, world!");
}
