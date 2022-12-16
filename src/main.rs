//use the fs module
use std::fs;

//use the day3.rs file
//mod day3;
//mod day4;
//mod day5;
//mod day6;
//mod day7;
mod day16;


fn main() {
    //stage1();
    //stage2();
    //day2stage1();
    //day15::stage1();
    day16::stage2();
}


struct tree_node {
    value: i32,
    children: Vec<Box<tree_node>>,
}

fn day2stage1() {
    println!("Day 2 challenge!");

    let mut tree = tree_node {
        value: 0,
        children: Vec::new(),
    };

    //add the first node
    tree.children.push(Box::new(tree_node {
        value: 1,
        children: Vec::new(),
    }));

    //borrow the first node
    let mut current_node = &mut tree.children[0];

    //add the second node
    current_node.children.push(Box::new(tree_node {
        value: 2,
        children: Vec::new(),
    }));

    //move to the second node
    current_node = &mut current_node.children[0];

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    let mut points = 0;
    
    for line in list.lines() {
        let first_char = line.chars().nth(0).unwrap();
        let last_char = line.chars().last().unwrap();

        //switch for first char
        match last_char {
            'X' => { //need to loose
                points += 0;

                if first_char == 'A' { //enemy is rock
                    //you choose scissors
                    points += 3; 
                }
                if first_char == 'B' { //enemy is paper
                    //you choose rock
                    points += 1;
                }
                if first_char == 'C' { //enemy is scissors
                    //you choose paper
                    points += 2;
                }
            },
            'Y' => { //need to draw
                points += 3;

                if first_char == 'A' { //enemy is rock
                    //you choose rock
                    points += 1; 
                }
                if first_char == 'B' { //enemy is paper
                    //you choose paper
                    points += 2;
                }
                if first_char == 'C' { //enemy is scissors
                    //you choose scissors
                    points += 3;
                }
            },
            'Z' => { //need to win
                points += 6;

                if first_char == 'A' { //enemy is rock
                    //you choose paper
                    points += 2; 
                }
                if first_char == 'B' { //enemy is paper
                    //you choose scissors
                    points += 3;
                }
                if first_char == 'C' { //enemy is scissors
                    //you choose rock
                    points += 1;
                }
            },
            _ => {
                println!("Error");
            }
        }

    }

    println!("Points: {}", points);


}


fn stage2() {
    println!("Day 1 challenge!");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //Greatest totals available
    let mut greatest_totals = [0; 3];

    //current total for the current elf
    let mut current_total = 0;

    let mut lowest_index = 0;


    // Find the greatest total
    for line in list.lines() {
        //we've reached the end of a group
        if line.is_empty() {
            
            //replace the lowest value in the greatest totals
            if current_total > greatest_totals[lowest_index] {
                greatest_totals[lowest_index] = current_total;

                //greatest totals have changed so we need to find the new lowest index
                lowest_index = get_lowest_index(greatest_totals);
            }

            

            // Reset the current total
            current_total = 0;
        } else {
            // Add the number to the current total
            current_total += line.parse::<i32>().unwrap();
        }
    }

    //handle the last number
    if current_total > greatest_totals[lowest_index] {
        greatest_totals[lowest_index] = current_total;
    }
    

    println!("Most: {:?}", greatest_totals);

    // Calculate the final total
    let mut final_total = 0;
    for i in 0..greatest_totals.len() {
        final_total += greatest_totals[i];
    }

    //output the greatest totals
    println!("Final total: {}", final_total);
}

fn get_lowest_index(greatest_totals: [i32; 3]) -> usize {
    let mut lowest_index = 0;
    let mut lowest_value = greatest_totals[0];

    for i in 1..greatest_totals.len() {
        if greatest_totals[i] < lowest_value {
            lowest_index = i;
            lowest_value = greatest_totals[i];
        }
    }

    lowest_index
}





fn stage1() {
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
