use std::fs;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines as an array
    let lines = list.lines();

    //loop through the lines
    for line in lines {
        println!("Reading line: {}", line);
    }


    
    
}


pub fn stage2() {

}