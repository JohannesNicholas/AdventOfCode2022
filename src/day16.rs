use std::{fs, collections::HashMap};

struct valve {
    vec: Vec<(char, char)>,
    flow_rate: i32,
}

pub fn stage1() {
    println!("Day 15 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    

    let mut num_covered = 0; //number of locations a beacon cannot be

}

pub fn stage2() {}