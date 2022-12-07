use std::fs;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");


    
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines as an array
    let mut lines = list.lines();
    
    let mut location = "".to_string();

    let mut currentDirSizes = Vec::new();

    let mut size_counter = 0;

    currentDirSizes.push(0);

    //skip the first line
    lines.next();

    //loop through the lines
    for line in lines {
        
        
        if line.starts_with("$ cd ") {
            let dir_name = line.split(" ").nth(2).unwrap();

            if dir_name == ".." {
                //go up a directory
                //remove the last directory from the location
                let mut new_location = "".to_string();
                
                let locations = location.split("/").collect::<Vec<&str>>();

                for i in 0..locations.len()-2 {
                    new_location = new_location + locations[i] + "/";
                }

                location = new_location;

                
                
                let dir_size = currentDirSizes.pop();
                let dir_size = dir_size.unwrap();

                let old_size = currentDirSizes.pop();
                let old_size = old_size.unwrap();
                let new_size = old_size + dir_size;
                currentDirSizes.push(new_size);
                

                if dir_size <= 100000 {
                    
                    size_counter += dir_size;
                    
                    println!("+({}, {})", dir_size, location)

                }
            } else {
                //go down a directory
                location = location + dir_name + "/";
                currentDirSizes.push(0);
            }
            continue;
        }
        else if line.starts_with("$ ls") {
            continue;
        }
        else if line.starts_with("dir") {
            continue;
        }
        else {
            //file
            let file_size = line.split(" ").nth(0).unwrap();
            let old_size = currentDirSizes.pop();
            let old_size = old_size.unwrap();
            let new_size = old_size + file_size.parse::<usize>().unwrap();
            currentDirSizes.push(new_size);

            
        }

        let dir_size = currentDirSizes.pop();
        let dir_size = dir_size.unwrap();
        currentDirSizes.push(dir_size);

        println!("Location: {}      size: {}      line: {}  ", location, dir_size, line);

        
    }

    //loop back to the root directory
    while currentDirSizes.len() > 1 {
        let dir_size = currentDirSizes.pop();
        let dir_size = dir_size.unwrap();
        if dir_size < 100000 {
            println!("{} {}", location, dir_size);
            size_counter += dir_size;
            let old_size = currentDirSizes.pop();
            let old_size = old_size.unwrap();
            let new_size = old_size + dir_size;
            currentDirSizes.push(new_size);

        }
        let mut new_location = "".to_string();
                
        let locations = location.split("/").collect::<Vec<&str>>();

        for i in 0..locations.len()-2 {
            new_location = new_location + locations[i] + "/";
        }

        location = new_location;
    }

    //add the root directory size if it is less than 100000
    let dir_size = currentDirSizes.pop();
    let dir_size = dir_size.unwrap();
    if dir_size < 100000 {
        println!("Root: {} {}", location, dir_size);
        size_counter += dir_size;
    }


    println!("total size: {}", size_counter);
    
}


pub fn stage2() {

    println!("Day 4 challenge! pt.2");


    
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines as an array
    let mut lines = list.lines();
    
    let mut location = "".to_string();

    let mut currentDirSizes = Vec::new();

    let mut size_counter = 0;

    currentDirSizes.push(0);

    //skip the first line
    lines.next();

    //loop through the lines
    for line in lines {
        
        
        if line.starts_with("$ cd ") {
            let dir_name = line.split(" ").nth(2).unwrap();

            if dir_name == ".." {
                //go up a directory
                //remove the last directory from the location
                let mut new_location = "".to_string();
                
                let locations = location.split("/").collect::<Vec<&str>>();

                for i in 0..locations.len()-2 {
                    new_location = new_location + locations[i] + "/";
                }

                location = new_location;

                
                
                let dir_size = currentDirSizes.pop();
                let dir_size = dir_size.unwrap();

                let old_size = currentDirSizes.pop();
                let old_size = old_size.unwrap();
                let new_size = old_size + dir_size;
                currentDirSizes.push(new_size);
                

                if dir_size <= 100000 {
                    
                    size_counter += dir_size;
                    
                    println!("+({}, {})", dir_size, location)

                }
            } else {
                //go down a directory
                location = location + dir_name + "/";
                currentDirSizes.push(0);
            }
            continue;
        }
        else if line.starts_with("$ ls") {
            continue;
        }
        else if line.starts_with("dir") {
            continue;
        }
        else {
            //file
            let file_size = line.split(" ").nth(0).unwrap();
            let old_size = currentDirSizes.pop();
            let old_size = old_size.unwrap();
            let new_size = old_size + file_size.parse::<usize>().unwrap();
            currentDirSizes.push(new_size);

            
        }

        let dir_size = currentDirSizes.pop();
        let dir_size = dir_size.unwrap();
        currentDirSizes.push(dir_size);

        println!("Location: {}      size: {}      line: {}  ", location, dir_size, line);

        
    }

    //loop back to the root directory
    while currentDirSizes.len() > 1 {
        let dir_size = currentDirSizes.pop();
        let dir_size = dir_size.unwrap();
        if dir_size < 100000 {
            println!("{} {}", location, dir_size);
            size_counter += dir_size;
            let old_size = currentDirSizes.pop();
            let old_size = old_size.unwrap();
            let new_size = old_size + dir_size;
            currentDirSizes.push(new_size);

        }
        let mut new_location = "".to_string();
                
        let locations = location.split("/").collect::<Vec<&str>>();

        for i in 0..locations.len()-2 {
            new_location = new_location + locations[i] + "/";
        }

        location = new_location;
    }

    //add the root directory size if it is less than 100000
    let dir_size = currentDirSizes.pop();
    let dir_size = dir_size.unwrap();
    if dir_size < 100000 {
        println!("Root: {} {}", location, dir_size);
        size_counter += dir_size;
    }


    println!("total size: {}", size_counter);
}