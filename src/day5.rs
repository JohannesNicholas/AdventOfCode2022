use std::fs;



pub fn stage1() {
    println!("Day 4 challenge! pt.1");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    let mut counter = 0;

    
   

    //an array of stacks
    let mut stacks = [[' '; 100]; 10];
    let mut stack_heights = [0; 10];


    //get string before blank line, keep line breaks
    let mut stackLines = lines.clone().take_while(|x| x != &"").collect::<Vec<&str>>();

    //loop through stack lines
    for i in 1..stackLines.len() {
        let index = stackLines.len() - i - 1;
        let line = stackLines[index];

        //output line
        println!("analizing line: {}", line);

        for j in 0..line.len()/4+1 {
            let charIndex = j * 4 + 1;
            let char = line.chars().nth(charIndex).unwrap();

            //if not a space
            if char != ' ' {
                //add to stack
                stacks[j][stack_heights[j]] = char;
                stack_heights[j] += 1;
            }
        }
    }

    //loop through lines
    for line in lines {
        
        //if the line starts with "move"
        if line.starts_with("move ") {
            //lines are layed out with the following format:
            //move N from X to Y
            let n = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            let x = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap() - 1;
            let y = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap() - 1;

            //loop n times
            for _ in 0..n {
                //print out what's moveing
                println!("moving {} from {} to {}", stacks[x][stack_heights[x]-1], x, y);

                //get top of stack
                let top = stacks[x][stack_heights[x] - 1];

                //add to stack
                stacks[y][stack_heights[y]] = top;
                stack_heights[y] += 1;

                //remove from stack
                stack_heights[x] -= 1;
            }
        }
        
    }

    

    print!("Tops: ");
    //print the tops of each stack
    for i in 0..stacks.len() {
        //if stack is not empty
        if stack_heights[i] > 0 {
            print!("{}", stacks[i][stack_heights[i] - 1]);
        }
    }
    println!();

    
}

pub fn stage2() {
    println!("Day 4 challenge! pt.2");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    let mut counter = 0;

    
   

    //an array of stacks
    let mut stacks = [[' '; 100]; 10];
    let mut stack_heights = [0; 10];


    //get string before blank line, keep line breaks
    let mut stackLines = lines.clone().take_while(|x| x != &"").collect::<Vec<&str>>();

    //loop through stack lines
    for i in 1..stackLines.len() {
        let index = stackLines.len() - i - 1;
        let line = stackLines[index];

        //output line
        println!("analizing line: {}", line);

        for j in 0..line.len()/4+1 {
            let charIndex = j * 4 + 1;
            let char = line.chars().nth(charIndex).unwrap();

            //if not a space
            if char != ' ' {
                //add to stack
                stacks[j][stack_heights[j]] = char;
                stack_heights[j] += 1;
            }
        }
    }

    //loop through lines
    for line in lines {
        
        //if the line starts with "move"
        if line.starts_with("move ") {
            //lines are layed out with the following format:
            //move N from X to Y
            let n = line.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
            let x = line.split(" ").nth(3).unwrap().parse::<usize>().unwrap() - 1;
            let y = line.split(" ").nth(5).unwrap().parse::<usize>().unwrap() - 1;

            //loop n times
            for i in 0..n {
                let to_move_index = stack_heights[x] - (n - i);
                //print out what's moveing
                println!("moving {} from {} to {}", stacks[x][to_move_index], x, y);

                //get item to move
                let top = stacks[x][to_move_index];

                //add to stack
                stacks[y][stack_heights[y]] = top;
                stack_heights[y] += 1;
            }

            //reduce stack height
            stack_heights[x] -= n;
        }
        
    }

    

    print!("Tops: ");
    //print the tops of each stack
    for i in 0..stacks.len() {
        //if stack is not empty
        if stack_heights[i] > 0 {
            print!("{}", stacks[i][stack_heights[i] - 1]);
        }
    }
    println!();
}