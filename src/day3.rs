use std::fs;



pub fn stage1() {
    println!("Day 3 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    let mut total = 0; 

    //loop through lines
    for line in lines {
        let line_length = line.len();
        let left_half = &line[0..(line_length/2)];
        let right_half = &line[(line_length/2)..line_length];

        //a boolean array for all characters
        let mut letters = [false; 256];

        //loop through left half
        for c in left_half.chars() {
            //find the index of the letter for the array
            letters[c as usize] = true;
        }

        //loop through right half
        for c in right_half.chars() {
            //if the letter is in the array, it is a duplicate
            if letters[c as usize] {
                //add the number to the current total
                //
                //Lowercase item types a through z have value 1 through 26.
                //Uppercase item types A through Z have value 27 through 52.

                // 1. Subtract 96 from the lowercase letter

                // 2. Subtract 64 from the uppercase letter

                // 3. Add the result to the total

                // 4. Reset the array

                // 5. Go to the next line

                let value = match c {
                    'a'..='z' => c as i32 - 96,
                    'A'..='Z' => c as i32 - 64 + 26,
                    _ => 0,
                };

                total += value;

                //reset the array
                letters = [false; 256];

                //go to the next line
                break;
            }
        }
    }

    println!("Total: {}", total);

    
}

pub fn stage2() {
    println!("Day 3 challenge! pt.2");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    let mut total = 0; 

    //array for all characters
    let mut letters = [0; 256];

    let mut total = 0;

    //loop through lines
    for line in lines {

        //a boolean array for all characters
        let mut found = [false; 256];

        //loop through each character
        for c in line.chars() {
            //if not found, add to the array
            if !found[c as usize] {
                found[c as usize] = true;
                letters[c as usize] += 1;

                if letters[c as usize] == 3 {
                    let value = match c {
                        'a'..='z' => c as i32 - 96,
                        'A'..='Z' => c as i32 - 64 + 26,
                        _ => 0,
                    };

                    total += value;

                    //reset the array
                    letters = [0; 256];

                    break;

                    //print out the letter
                    println!("Character: {}", c)
                }
                
            }
        }

        
    }

    println!("Total: {}", total);

    
}