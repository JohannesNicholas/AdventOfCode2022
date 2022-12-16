use std::fs;



pub fn stage1() {
    println!("Day 14 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    
    let mut grid = [[false; 500]; 1000];
    
    let mut max_y = 0;

    for line in lines {
        //split into coordinates
        let coords = line.split(" -> ").collect::<Vec<&str>>();

        for i in 0..(coords.len() - 1) {
            let coord = coords[i].split(",").collect::<Vec<&str>>();
            let x = coord[0].parse::<usize>().unwrap();
            let y = coord[1].parse::<usize>().unwrap();

            let next_coord = coords[i + 1].split(",").collect::<Vec<&str>>();

            let next_x = next_coord[0].parse::<usize>().unwrap();
            let next_y = next_coord[1].parse::<usize>().unwrap();

            if next_y > max_y {
                max_y = next_y;
            }
            if y > max_y {
                max_y = y;
            }

            println!("{} {} -> {} {}", x, y, next_x, next_y);

            if x == next_x {
                //vertical line
                if y < next_y {
                    for j in y..next_y {
                        grid[x][j] = true;
                    }
                } else {
                    for j in next_y..y {
                        grid[x][j] = true;
                    }
                }
            } else {
                //horizontal line
                if x < next_x {
                    for x in x..next_x {
                        grid[x][y] = true;
                    }
                } else {
                    for x in next_x..x {
                        grid[x][y] = true;
                    }
                }
            }

            grid[x][y] = true;
            grid[next_x][next_y] = true;


            
            
        }
    }


    //fill in the floor
    let floor = max_y + 2;
    for x in 0..1000 {
        grid[x][floor] = true;
    }

    let mut counter = 0;

    //fall sand
    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;

        let mut settled = false;

        if grid[sand_x][sand_y] {
            break;
        }

        while sand_y < 499 {
            //println!("{} {}", sand_x, sand_y);
            if grid[sand_x][sand_y + 1] {
                //move left or right
                if !grid[sand_x - 1][sand_y + 1] {
                    sand_x -= 1;
                    sand_y += 1;
                } else if !grid[sand_x + 1][sand_y + 1] {
                    sand_x += 1;
                    sand_y += 1;
                }
                else {
                    //stop
                    settled = true;
                    counter += 1;
                    grid[sand_x][sand_y] = true;
                    break;
                }
            } else {
                sand_y += 1;
            }
        }

        //println!("Counter: {}", counter);

        if !settled {
            break;
        }

        if (counter % 3500 == 0 && counter < 20000) || counter == 1 {
            println!("");
            println!("");
            //print out the grid
            for y in 0..(floor+2) {
                for x in 400..600 {
                    if grid[x][y] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }

        }

    }


    println!("");
    println!("");
    //print out the grid
    //for y in 0..(floor+2) {
    //    for x in 400..600 {
    //        if grid[x][y] {
    //            print!("#");
    //        } else {
    //            print!(".");
    //        }
    //    }
    //    println!("");
    //}

    println!("Counter: {}", counter);

    
}

pub fn stage2() {
println!("Day 14 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    
    let mut grid = [[false; 500]; 1000];
    
    let mut max_y = 0;

    for line in lines {
        //split into coordinates
        let coords = line.split(" -> ").collect::<Vec<&str>>();

        for i in 0..(coords.len() - 1) {
            let coord = coords[i].split(",").collect::<Vec<&str>>();
            let x = coord[0].parse::<usize>().unwrap();
            let y = coord[1].parse::<usize>().unwrap();

            let next_coord = coords[i + 1].split(",").collect::<Vec<&str>>();

            let next_x = next_coord[0].parse::<usize>().unwrap();
            let next_y = next_coord[1].parse::<usize>().unwrap();

            if next_y > max_y {
                max_y = next_y;
            }
            if y > max_y {
                max_y = y;
            }

            println!("{} {} -> {} {}", x, y, next_x, next_y);

            if x == next_x {
                //vertical line
                if y < next_y {
                    for j in y..next_y {
                        grid[x][j] = true;
                    }
                } else {
                    for j in next_y..y {
                        grid[x][j] = true;
                    }
                }
            } else {
                //horizontal line
                if x < next_x {
                    for x in x..next_x {
                        grid[x][y] = true;
                    }
                } else {
                    for x in next_x..x {
                        grid[x][y] = true;
                    }
                }
            }

            grid[x][y] = true;
            grid[next_x][next_y] = true;


            
            
        }
    }


    //fill in the floor
    let floor = max_y + 2;
    for x in 0..1000 {
        grid[x][floor] = true;
    }

    let mut counter = 0;

    //fall sand
    loop {
        let mut sand_x = 500;
        let mut sand_y = 0;

        let mut settled = false;

        if grid[sand_x][sand_y] {
            break;
        }

        while sand_y < 499 {
            //println!("{} {}", sand_x, sand_y);
            if grid[sand_x][sand_y + 1] {
                //move left or right
                if !grid[sand_x - 1][sand_y + 1] {
                    sand_x -= 1;
                    sand_y += 1;
                } else if !grid[sand_x + 1][sand_y + 1] {
                    sand_x += 1;
                    sand_y += 1;
                }
                else {
                    //stop
                    settled = true;
                    counter += 1;
                    grid[sand_x][sand_y] = true;
                    break;
                }
            } else {
                sand_y += 1;
            }
        }

        //println!("Counter: {}", counter);

        if !settled {
            break;
        }

        if (counter % 3500 == 0 && counter < 20000) || counter == 1 {
            println!("");
            println!("");
            //print out the grid
            for y in 0..(floor+2) {
                for x in 400..600 {
                    if grid[x][y] {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!("");
            }

        }

    }


    println!("");
    println!("");
    //print out the grid
    //for y in 0..(floor+2) {
    //    for x in 400..600 {
    //        if grid[x][y] {
    //            print!("#");
    //        } else {
    //            print!(".");
    //        }
    //    }
    //    println!("");
    //}

    println!("Counter: {}", counter);

    
}