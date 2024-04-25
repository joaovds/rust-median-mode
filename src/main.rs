use std::io;

fn main() {
    println!("Enter the integers one after the other:");

    let mut input_numbers = String::new();

    io::stdin()
        .read_line(&mut input_numbers)
        .expect("Failed to read line!");

    let mut input_numbers: Vec<u32> = input_numbers
        .trim()
        .split_whitespace()
        .filter_map(|number| match number.trim().parse() {
            Ok(n) => Some(n),
            Err(_) => {
                println!("'{}' is invalid number!", number);
                None
            }
        })
        .collect();

    input_numbers.sort();

    println!("{:?}", input_numbers)
}
