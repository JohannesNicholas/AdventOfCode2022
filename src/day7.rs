use std::fs;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");


    
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines as an array
    let mut lines = list.lines();
    
    let mut counter = 0;

    let mut sizes_buffer = [0; 100000];

    let mut current_index = 0;

    //skip the first line
    lines.next();

    //loop through the lines
    for line in lines {

        print!("line: {} \t ", line);

        if line.starts_with("$ cd ..") {
            //moving up a directory
            let size = sizes_buffer[current_index];

            print!("size: {} \t ", size);

            if size <= 100000{
                counter += size;
                sizes_buffer[current_index] = 0;
                sizes_buffer[current_index-1] += size;
            }
            current_index -= 1;
        }
        else if line.starts_with("$ cd ") {
            //moving down a directory
            current_index += 1;
        }
        else if line.starts_with("dir ") {
            //found a directory
        }
        else if line.starts_with("$ ls") {
            //found a directory
        }
        else {
            //must be a file
            //size is number before space
            let size = line.split(" ").next().unwrap();
            let size = size.parse::<usize>().unwrap();

            sizes_buffer[current_index] += size;
        }

        println!();
    }

    println!("counter: {}", counter);

    
}


pub fn stage2() {
}