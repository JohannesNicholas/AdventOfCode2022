use std::{fs, collections::HashMap};

struct sensor {
    x: i32,
    y: i32,
    range: i32,
}

pub fn stage1() {
    println!("Day 15 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    
    let mut sensors = Vec::new();

    let mut num_covered = 0; //number of locations a beacon cannot be

    for line in lines {
        //lines are layed out like:
        //Sensor at x=0, y=11: closest beacon is at x=2, y=10

        //get the x and y coordinates=
        let coords = line.split("Sensor at x=").collect::<Vec<&str>>();
        let coords = coords[1].split(", y=").collect::<Vec<&str>>();
        let x = coords[0].parse::<i32>().unwrap();
        let y = coords[1].split(":").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();

        //get the coordinates of the closest beacon
        let coords = line.split("closest beacon is at x=").collect::<Vec<&str>>();
        let coords = coords[1].split(", y=").collect::<Vec<&str>>();
        let beacon_x = coords[0].parse::<i32>().unwrap();
        let beacon_y = coords[1].parse::<i32>().unwrap();

        

        //calculate the range
        let range = (beacon_x - x).abs() + (beacon_y - y).abs();

        sensors.push(sensor {x, y, range});

        println!("{} {} -> {} {} : {}", x, y, beacon_x, beacon_y, range);

    }


    //loop through the target row:
    let taget_row = 2000000;

    //hashmap for the locations that it can't be
    let mut covered = HashMap::new();

    let mut num_covered = 0;

    println!();

    for i in 0..sensors.len() {
        let distance_to_target = (sensors[i].y - taget_row).abs();
        //if the range can cover the target row:
        if distance_to_target <= sensors[i].range {
            //calculate where the range starts and ends
            let start = sensors[i].x - sensors[i].range + distance_to_target;
            let end = sensors[i].x + sensors[i].range - distance_to_target;

            println!("{},{} \t with range {} \t goes from-> {} to {}", sensors[i].x, sensors[i].y, sensors[i].range, start, end);

            for j in start..end {
                //if the location is not in the hashmap, add it
                if !covered.contains_key(&j) {
                    covered.insert(j, true);
                    num_covered += 1;
                }
            }

        }
    }

    //print the locations that are covered
    for i in -5..25 {
        if covered.contains_key(&i) {
            print!("X");
        } else {
            print!(".");
        }
    }


    println!("{} locations are covered", num_covered);
}


pub fn stage2() {
    println!("Day 15 challenge!  Pt 2");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    
    let mut sensors = Vec::new();

    let mut num_covered = 0; //number of locations a beacon cannot be

    for line in lines {
        //lines are layed out like:
        //Sensor at x=0, y=11: closest beacon is at x=2, y=10

        //get the x and y coordinates=
        let coords = line.split("Sensor at x=").collect::<Vec<&str>>();
        let coords = coords[1].split(", y=").collect::<Vec<&str>>();
        let x = coords[0].parse::<i32>().unwrap();
        let y = coords[1].split(":").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();

        //get the coordinates of the closest beacon
        let coords = line.split("closest beacon is at x=").collect::<Vec<&str>>();
        let coords = coords[1].split(", y=").collect::<Vec<&str>>();
        let beacon_x = coords[0].parse::<i32>().unwrap();
        let beacon_y = coords[1].parse::<i32>().unwrap();

        

        //calculate the range
        let range = (beacon_x - x).abs() + (beacon_y - y).abs();

        sensors.push(sensor {x, y, range});

        println!("{} {} -> {} {} : {}", x, y, beacon_x, beacon_y, range);

    }


    //loop through every possible location
    let size = 20;

    

    'yloop: for y in 0..4000000 {
        
        if y % 100000 == 0 {
            println!("y: {}", y);
        }
        let mut x = 0;
        'xloop: while x < 4000000 {


            for sensor in &sensors {
                let distance = (sensor.x - x).abs() + (sensor.y - y).abs();
                if distance <= sensor.range{
                    //println!("{} {} is covered by {} {}", x, y, sensor.x, sensor.y);
                    let distance_to_target = (sensor.y - y).abs();
                    let end = sensor.x + sensor.range - distance_to_target;
                    x = end + 1;
                    continue 'xloop; //go to next location
                }
            }
            

            //this is the only location that is not covered by a sensor
            println!("LOCATION: {} {}", x, y);

            println!("tuning frequency: {}", x as i128 * 4000000 + y as i128);

            break 'yloop; //location found, break out of the loops
        }
    }


    println!("{} locations are covered", num_covered);
}