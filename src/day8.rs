use std::fs;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the length of the first line
    let mut lines = list.lines();

    const size: usize = 99;

    //split input into a 2d array of chars
    let mut heights: [[u8; size];size] = [[0; size]; size];

    //loop through characters and add them to the array
    for (i, line) in list.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            heights[i][j] = c.to_digit(10).unwrap() as u8;
        }
    }

    //output the array
    for i in 0..size {
        for j in 0..size {
            print!("{}", heights[i][j]);
        }
        println!("");
    }
    
    let mut visible_heights : [[[u8; size];size];4] = [[[0; size]; size]; 4];

    //loop through the array downward
    for y in 0..size {

        //loop through the array right
        for x in 1..size {
            visible_heights[0][x][y] = max(heights[x-1][y], visible_heights[0][x-1][y]);
        }

        //loop through the array left
        for x in (0..size-1).rev() {
            visible_heights[1][x][y] = max(heights[x+1][y], visible_heights[1][x+1][y]);
        }
    }

    //loop through the array right
    for x in 0..size {

        //loop through the array downward
        for y in 1..size {
            visible_heights[2][x][y] = max(heights[x][y-1], visible_heights[2][x][y-1]);
        }

        //loop through the array upward
        for y in (0..size-1).rev() {
            visible_heights[3][x][y] = max(heights[x][y+1], visible_heights[3][x][y+1]);
        }
    }

    //print out the visible heights
    for d in 0..4 {
        println!("Direction: {}", d);
        for i in 0..size {
            for j in 0..size {
                print!("{}", visible_heights[d][i][j]);
            }
            println!("");
        }
        println!("\n");
    }

    //compile the visible heights
    for d in 1..4 {
        for i in 0..size {
            for j in 0..size {
                visible_heights[0][i][j] = min(visible_heights[0][i][j], visible_heights[d][i][j]);
                print!("{}", visible_heights[0][i][j]);
            }
            println!("");
        }
        println!("\n");
    }

    let mut canSee = [[true; size]; size];

    let mut counter = 0;

    //loop through the array
    for i in 0..size {
        for j in 0..size {
            if heights[i][j] > visible_heights[0][i][j] || j == 0 || j == size-1 || i == 0 || i == size-1 {
                canSee[i][j] = false;
                counter += 1;
            }
            print!("{}", canSee[i][j] as u32);
        }
        println!("");
    }

    println!("Can see: {}", counter);

    
}

fn max(a: u8, b: u8) -> u8 {
    if a > b {
        a
    } else {
        b
    }
}

fn min (a: u8, b: u8) -> u8 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn stage2() {

}