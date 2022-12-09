use std::fs;

const size: usize = 99;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the length of the first line
    let mut lines = list.lines();

    

    let mut headPosition = (0, 0);
    let mut tailPosition = (0, 0);

    //a hashmap to keep track of unique positions
    let mut positions: std::collections::HashMap<(i32, i32), bool> = std::collections::HashMap::new();

    let mut counter = 0;

    //loop through heach line
    for line in lines {
        //get the first character
        let direction = line.chars().nth(0).unwrap();

        //get the number of steps
        let steps = line[2..].parse::<i32>().unwrap();

        //loop steps times
        for i in 0..steps {
            //move the head
            match direction {
                'U' => headPosition.1 += 1,
                'D' => headPosition.1 -= 1,
                'L' => headPosition.0 -= 1,
                'R' => headPosition.0 += 1,
                _ => println!("Error"),
            }

            let difference: (i32, i32) = ((headPosition.0 - tailPosition.0), (headPosition.1 - tailPosition.1));


            //check if the tail is close enough to the head
            if difference.0.abs() > 1 || difference.1.abs() > 1 {
                //Move the tails x position 1 step closer to the head
                if headPosition.0 > tailPosition.0 {
                    tailPosition.0 += 1;
                } else if headPosition.0 < tailPosition.0 {
                    tailPosition.0 -= 1;
                }

                //Move the tails y position 1 step closer to the head
                if headPosition.1 > tailPosition.1 {
                    tailPosition.1 += 1;
                } else if headPosition.1 < tailPosition.1 {
                    tailPosition.1 -= 1;
                }

                
            }

            //check if the tail is in the hashmap
            if !positions.contains_key(&tailPosition) {
                //add the tail to the hashmap
                positions.insert(tailPosition, true);

                
                counter += 1;
            }

            //output the positions
            //println!("Head: ({}, {}), Tail: ({}, {})", headPosition.0, headPosition.1, tailPosition.0, tailPosition.1);
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

    //a rope 11 segments long
    let mut rope: Vec<(i32, i32)> = Vec::new();

    //create the secments all at 0,0
    for _ in 0..size {
        rope.push((0, 0));
    }

    //a hashmap to keep track of unique positions
    let mut positions: std::collections::HashMap<(i32, i32), bool> = std::collections::HashMap::new();

    let mut counter = 0;

    //loop through heach line
    for line in lines {
        //get the first character
        let direction = line.chars().nth(0).unwrap();

        //get the number of steps
        let steps = line[2..].parse::<i32>().unwrap();

        //loop steps times
        for i in 0..steps {
            //move the head
            match direction {
                'U' => rope[0].1 += 1,
                'D' => rope[0].1 -= 1,
                'L' => rope[0].0 -= 1,
                'R' => rope[0].0 += 1,
                _ => println!("Error"),
            }

            //loop through each rope segment after the head
            for i in 1..11 {
                //get the position of the segment before
                let previousPosition = rope[i - 1];

                //get the position of the next segment
                let tailPosition = rope[i];

                //get the difference between the two
                let difference: (i32, i32) = ((previousPosition.0 - tailPosition.0), (previousPosition.1 - tailPosition.1));

                //check if the segments aren't close enough
                if difference.0.abs() > 1 || difference.1.abs() > 1 {
                    //move the segment closer to the previous segment
                    if previousPosition.0 > tailPosition.0 {
                        rope[i].0 += 1;
                    } else if previousPosition.0 < tailPosition.0 {
                        rope[i].0 -= 1;
                    }

                    //move the segment closer to the previous segment
                    if previousPosition.1 > tailPosition.1 {
                        rope[i].1 += 1;
                    } else if previousPosition.1 < tailPosition.1 {
                        rope[i].1 -= 1;
                    }
                }
                
            }

            let tailPosition = rope[9];

            //check if the tail is in the hashmap
            if !positions.contains_key(&tailPosition) {
                //add the tail to the hashmap
                positions.insert(tailPosition, true);

                
                counter += 1;
            }

            //output the positions
            println!("Head: ({}, {})",rope[9].0, rope[9].1);
        }
    }

    println!("Counter: {}", counter);
}