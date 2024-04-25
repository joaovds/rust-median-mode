use std::{collections::HashMap, io};

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

    if input_numbers.len() == 0 {
        return println!("No data input!...");
    }

    input_numbers.sort();

    println!("------ ..... ------");
    println!("Numbers: {:?}", input_numbers);
    println!("Median: {}", find_the_median(&input_numbers));
    match find_mode(&input_numbers) {
        Some(n) => println!("Mode: {n}"),
        None => println!("Mode: no exists!"),
    };
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

fn find_mode(numbers: &Vec<u32>) -> Option<u32> {
    let mut frequency = HashMap::new();

    for &n in numbers {
        *frequency.entry(n).or_insert(0) += 1;
    }

    let mut mode = None;
    let mut greater_frequency = 0;

    for (n, &freq) in &frequency {
        if freq > greater_frequency {
            mode = Some(*n);
            greater_frequency = freq;
        } else if freq == greater_frequency {
            mode = None
        }
    }

    mode
}
