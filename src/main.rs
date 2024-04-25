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

    let median = find_the_median(&input_numbers);

    println!("{:?}", input_numbers);
    println!("{:?}", median);
}

fn find_the_median(numbers: &Vec<u32>) -> u32 {
    let center_index = numbers.len() / 2;

    if numbers.len() % 2 == 0 {
        let center_left = numbers[center_index - 1];
        let center_right = numbers[center_index];

        return (center_left + center_right) / 2;
    }

    return numbers[center_index];
}
