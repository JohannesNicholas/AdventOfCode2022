use std::{fs, collections::HashMap};

struct valve {
    tunnels: Vec<(char, char)>,
    flow_rate: i32,
    combined_flow_rate: i32,
    previous_valve: (char, char),
    steps: i32, //steps to get here
}

pub fn stage1() {
    println!("Day 17 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();


    for line in lines {

    }
}


pub fn stage2() {}