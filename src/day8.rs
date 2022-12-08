use std::fs;

const size: usize = 99;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the length of the first line
    let mut lines = list.lines();

    

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
    println!("Day 4 challenge! pt.2");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");




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
    
    let mut vision_scores : [[[[u8;10]; size];size];4] = [[[[1;10]; size]; size]; 4];

    
    //loop through the array downward
    for y in 0..size {

        //loop through the array right
        for x in 1..size {
            vision_scores[0][x][y] = calc_visions(heights[x][y], vision_scores[0][x-1][y]);
        }

        //loop through the array left
        for x in (0..size-1).rev() {
            vision_scores[1][x][y] = calc_visions(heights[x][y], vision_scores[1][x+1][y]);
        }
    }

    //loop through the array right
    for x in 0..size {

        //loop through the array downward
        for y in 1..size {
            vision_scores[2][x][y] = calc_visions(heights[x][y], vision_scores[2][x][y-1]);
        }

        //loop through the array upward
        for y in (0..size-1).rev() {
            vision_scores[3][x][y] = calc_visions(heights[x][y], vision_scores[3][x][y+1]);
        }
    }

    //let mut vision_scores2 : [[[[u8;10]; size];size];4] = [[[[1;10]; size]; size]; 4];


    //shift the vision scores back 1
    for x in (0..size).rev() {
        for y in 0..size {
             for h in 0..9 {
                vision_scores[0][x][y][h] = if x == 0 {0} else {vision_scores[0][x-1][y][h]};
             }
        }
    }
    for x in 0..size {
        for y in 0..size {
             for h in 0..9 {
                vision_scores[1][x][y][h] = if x == size-1 {0} else {vision_scores[1][x+1][y][h]};
             }
        }
    }
    for x in 0..size {
        for y in (0..size).rev() {
             for h in 0..9 {
                vision_scores[2][x][y][h] = if y == 0 {0} else {vision_scores[2][x][y-1][h]};
             }
        }
    }
    for x in 0..size {
        for y in 0..size {
             for h in 0..9 {
                vision_scores[3][x][y][h] = if y == size-1 {0} else {vision_scores[3][x][y+1][h]};
             }
        }
    }

    //print out the visible heights
    for d in 0..4 {
        println!("Direction: {}", d);
        for i in 0..size {
            for j in 0..size {
                print!("{}", vision_scores[d][i][j][5]);
            }
            println!("");
        }
        println!("\n");
    }

    let mut vision_multiplied: [[[u32;10];size];size] = [[[1;10]; size];size];

    //let mut vision_scores3 : [[[u8;10]; size];size] = [[[1;10]; size]; size];
    //compile the visible heights
    for d in 0..4 {
        for i in 0..size {
            for j in 0..size {
                for h in 0..10 {
                    vision_multiplied[i][j][h] *= vision_scores[d][i][j][h] as u32;
                }
            }
        }
    }

     

    let mut final_vision_scores : [[u32;size]; size] = [[0;size]; size];
    let mut best_score = 0;
    
    //get the correct score for each tree
    for x in 0..size {
        for y in 0..size {
            let height = heights[x][y];
            
            final_vision_scores[x][y] = vision_multiplied[x][y][height as usize];

            let score = final_vision_scores[x][y];

            if score > best_score {
                best_score = score;
            }

            print!("{}", score);
        }
        println!("");
    }

    println!("Best score: {}", best_score);

}

fn calc_visions(height: u8, prev: [u8; 10]) -> [u8; 10] {
    let mut visions: [u8; 10] = [0; 10];

    for i in 0..10 {
        if height >= i as u8 {
            visions[i] = 1;
        } else {
            visions[i] = prev[i] + 1;
        }
    }

    visions
}