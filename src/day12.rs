use std::fs;


const size_x: usize = 67;
const size_y: usize = 40;

//const size_x: usize = 8;
//const size_y: usize = 5;

pub fn stage1() {
    println!("Day 12 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the lines
    let mut lines = list.lines();
    

    let mut count = 0;

    let mut height_map = [[0; size_x]; size_y];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for y in 0..size_y {
        let characters = list.lines().nth(y).unwrap().chars().collect::<Vec<char>>();
        for x in 0..size_x {
            let character = characters[x];
            if character == 'S' {
                start = (x, y);
            }
            else if character == 'E' {
                end = (x, y);
                height_map[y][x] = 26;
            }
            else {
                //a has height 0, b has height 1, etc.
                let height = character as i32 - 97;
                height_map[y][x] = height;
            }
            

        }
    }



    let mut costs = [[1000; size_x]; size_y];
    let mut previous = [[(0, 0); size_x]; size_y];

    let mut queue = Vec::new();

    queue.push(start);

    costs[start.1][start.0] = 0;

    loop {
        if queue.len() == 0 {
            break;
        }

        let current = queue.remove(0);

        let current_cost = costs[current.1][current.0];

        let current_height = height_map[current.1][current.0];

        let neighbors = vec![
            (current.0 as i32, current.1 as i32 - 1),
            (current.0 as i32, current.1 as i32 + 1),
            (current.0 as i32 - 1, current.1 as i32),
            (current.0 as i32 + 1, current.1 as i32),
        ];

        for neighbor in neighbors {
            //if neighbor is in bounds
            if neighbor.0 >= 0 && neighbor.0 < size_x as i32 && neighbor.1 >= 0 && neighbor.1 < size_y as i32 {
                let neighbor_height = height_map[neighbor.1 as usize][neighbor.0 as usize];

                

                //if neighbor is not too high
                if neighbor_height <= current_height + 1 {

                    //if it is the end
                    if neighbor.1 as usize == end.1 && neighbor.0 as usize == end.0 {
                        let cost = current_cost + 1;

                        if cost < costs[neighbor.1 as usize][neighbor.0 as usize] {
                            costs[neighbor.1 as usize][neighbor.0 as usize] = cost;
                            previous[neighbor.1 as usize][neighbor.0 as usize] = current;
                        }
                    }

                    let cost = current_cost + 1;

                    if cost < costs[neighbor.1 as usize][neighbor.0 as usize] {
                        costs[neighbor.1 as usize][neighbor.0 as usize] = cost;
                        previous[neighbor.1 as usize][neighbor.0 as usize] = current;
                        queue.push((neighbor.0 as usize, neighbor.1 as usize));
                    }
                }
            }
        }
    }

    let mut current = end;
    loop {
        if current == start {
            break;
        }

        count += 1;

        let previous = previous[current.1][current.0];
        println!("({}, {}) -> ({}, {})", previous.0, previous.1, current.0, current.1);
        current = previous;
    }


    println!("TOTAL: {}", count);
    
}



pub fn stage2() {
    println!("Day 12 challenge! pt.2");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the lines
    let mut lines = list.lines();
    

    let mut count = 0;

    let mut height_map = [[0; size_x]; size_y];
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut a_squares = Vec::new();

    for y in 0..size_y {
        let characters = list.lines().nth(y).unwrap().chars().collect::<Vec<char>>();
        for x in 0..size_x {
            let character = characters[x];
            if character == 'S' {
                start = (x, y);
            }
            else if character == 'E' {
                end = (x, y);
                height_map[y][x] = 26;
            }
            else {
                //a has height 0, b has height 1, etc.
                let height = character as i32 - 97;
                height_map[y][x] = height;
            }


            if character == 'a' {
                a_squares.push((x, y));
            }

        }
    }


    


    let mut least_cost = 10000000;
    for a in a_squares {


        let mut costs = [[1000; size_x]; size_y];
        let mut previous = [[(0, 0); size_x]; size_y];

        let mut queue = Vec::new();

        queue.push(a);

        costs[a.1][a.0] = 0;

        let mut end_found = false;

        loop {
            if queue.len() == 0 {
                break;
            }
    
            let current = queue.remove(0);
    
            let current_cost = costs[current.1][current.0];
    
            let current_height = height_map[current.1][current.0];
    
            let neighbors = vec![
                (current.0 as i32, current.1 as i32 - 1), // Top
                (current.0 as i32, current.1 as i32 + 1), // Bottom
                (current.0 as i32 - 1, current.1 as i32), // Left
                (current.0 as i32 + 1, current.1 as i32), // Right
            ];
    
            for neighbor in neighbors {
                //if neighbor is in bounds
                if neighbor.0 >= 0 && neighbor.0 < size_x as i32 && neighbor.1 >= 0 && neighbor.1 < size_y as i32 {
                    let neighbor_height = height_map[neighbor.1 as usize][neighbor.0 as usize];
    
                    
    
                    //if neighbor is not too high
                    if neighbor_height <= current_height + 1 {
    
                        //if it is the end
                        if neighbor.1 as usize == end.1 && neighbor.0 as usize == end.0 {
                            let cost = current_cost + 1;

                            end_found = true;
    
                            if cost < costs[neighbor.1 as usize][neighbor.0 as usize] {
                                costs[neighbor.1 as usize][neighbor.0 as usize] = cost;
                                previous[neighbor.1 as usize][neighbor.0 as usize] = current;
                            }
                        }
    
                        let cost = current_cost + 1;
    
                        if cost < costs[neighbor.1 as usize][neighbor.0 as usize] {
                            costs[neighbor.1 as usize][neighbor.0 as usize] = cost;
                            previous[neighbor.1 as usize][neighbor.0 as usize] = current;
                            queue.push((neighbor.0 as usize, neighbor.1 as usize));
                        }
                    }
                }
            }
        }

        if !end_found {
            continue;
        }
    
        let mut count = 0;
        let mut current = end;
        loop {
            if current == a {
                break;
            }
    
            count += 1;
    
            let previous = previous[current.1][current.0];
            //println!("({}, {}) -> ({}, {})", previous.0, previous.1, current.0, current.1);
            current = previous;
        }

        println!("{:?} has cost {}", a, count);

        if count < least_cost {
            least_cost = count;
        }

    }
    


    println!("TOTAL: {}", least_cost-1);
}
