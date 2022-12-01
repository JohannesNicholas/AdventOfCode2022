//use the fs module
use std::fs;

fn main() {
    println!("Day 1 challenge!");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    let mut greatest_total = 0;
    let mut current_total = 0;


    // Find the greatest total
    for line in list.lines() {
        //we've reached the end of a group
        if line.is_empty() {
            // If the current total is greater than the greatest total, set the greatest total to the current total
            if current_total > greatest_total {
                greatest_total = current_total;
            }

            // Reset the current total
            current_total = 0;
        } else {
            // Add the number to the current total
            current_total += line.parse::<i32>().unwrap();
        }
    }

    // To handle when the last elf has the most
    if current_total > greatest_total {
        greatest_total = current_total;
    }

    //output the greatest total
    println!("{}", greatest_total);
}
