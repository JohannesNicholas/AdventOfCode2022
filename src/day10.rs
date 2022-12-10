use std::fs;

const size: usize = 99;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the length of the first line
    let mut lines = list.lines();

    
    let mut regX =  1;
    let mut cycle = 1;

    let mut counter = 0;

    for line in lines {

        let prev_cycle = cycle;
        let prev_regX = regX;

        if line.contains("noop") {
            cycle += 1;
        }
        else if line.starts_with("addx") {
            let value = line.split(" ").collect::<Vec<&str>>()[1];
            let value = value.parse::<i32>().unwrap();
            cycle += 2;
            regX += value;
        }

        let important_cycles = [20, 60, 100, 140, 180, 220];

        //loop through the important cycles
        for i in 0..important_cycles.len() {
            let important_cycle = important_cycles[i];
            if cycle > important_cycle && prev_cycle < important_cycle {
                println!("Cycle: {}, RegX: {}", important_cycle, prev_regX);
                counter += prev_regX * important_cycle;
            }
            if cycle == important_cycle {
                println!("Cycle: {}, RegX: {}", cycle, regX);
                counter += regX * cycle;
            }
        }
    }

    println!("Counter: {}", counter);
    
    
}



pub fn stage2() {
    println!("Day 4 challenge! pt.2");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the length of the first line
    let mut lines = list.lines();

    
    let mut regX =  1;
    let mut cycle = 1;

    let mut counter = 0;

    let mut do_another = false;

    let mut x_pos = 0;

    let mut screen = [[false; 42]; 42];

    for line in lines {

        //regX = 5;

        //regX = 5 + cycle as i32 / 40;
        screen[cycle % 40][cycle / 40] = (regX + 2 >= cycle as i32 % 40) && (regX <= cycle as i32 % 40);
        println!("Cycle: {}, RegX: {}", cycle, regX);

        cycle += 1;
        

        if line.contains("noop") {
            
        }
        else if line.starts_with("addx") {
            let value = line.split(" ").collect::<Vec<&str>>()[1];
            let value = value.parse::<i32>().unwrap();
            //print!("_");
            screen[cycle % 40][cycle / 40] = (regX + 2 >= cycle as i32 % 40) && (regX <= cycle as i32 % 40);
            println!("Cycle: {}, RegX: {}", cycle, regX);

            cycle += 1;
            regX += value;
        }

        

        
        
        
        
    }

    for y in 0..8 {
        for x in 0..42 {
            if screen[x][y] {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }

    println!("Counter: {}", counter);
}
