use std::fs;



pub fn stage1() {
    println!("Day 4 challenge! pt.1");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into characters
    let mut chars = list.chars();

    //number array representing the last time a letter was seen
    let mut last_seen = [' '; 4];


    for i in 0..chars.as_str().len() {
        let char = chars.next().unwrap();
        
        last_seen[i%4] = char;

        

        //loop through the last_seen array and make sure there is no duplicate
        let mut not_duplicate = true;
        let mut letters_seen =[false; 256];
        for j in 0..last_seen.len() {
            let index = last_seen[j] as usize;
            if last_seen[j] == ' ' {
                not_duplicate = false;
                break;
            }
            if letters_seen[index] {
                //duplicate found
                not_duplicate = false;
                break;
            } else {
                letters_seen[index] = true;
            }
        }

        if not_duplicate {
            println!("found it!");
            println!("last seen: {:?}", last_seen);
            println!("num chars: {}", i+1);
            return;
        }

        
    }

    
}

pub fn stage2() {
    println!("Day 4 challenge! pt.2");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into characters
    let mut chars = list.chars();

    //number array representing the last time a letter was seen
    let mut last_seen = [' '; 14];


    for i in 0..chars.as_str().len() {
        let char = chars.next().unwrap();
        
        last_seen[i%14] = char;

        

        //loop through the last_seen array and make sure there is no duplicate
        let mut not_duplicate = true;
        let mut letters_seen =[false; 256];
        for j in 0..last_seen.len() {
            let index = last_seen[j] as usize;
            if last_seen[j] == ' ' {
                not_duplicate = false;
                break;
            }
            if letters_seen[index] {
                //duplicate found
                not_duplicate = false;
                break;
            } else {
                letters_seen[index] = true;
            }
        }

        if not_duplicate {
            println!("found it!");
            println!("last seen: {:?}", last_seen);
            println!("num chars: {}", i+1);
            return;
        }

        
    }
}