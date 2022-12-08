use std::fs;

pub fn stage1() {
    println!("Day 4 challenge! pt.1");


    
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines as an array
    let mut lines = list.lines();

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
                
                
                let dir_size = currentDirSizes.pop();
                let dir_size = dir_size.unwrap();

                let old_size = currentDirSizes.pop();
                let old_size = old_size.unwrap();
                let new_size = old_size + dir_size;
                currentDirSizes.push(new_size);
                

                if dir_size <= 100000 {
                    
                    size_counter += dir_size;
                    
                    

                }
            } else {
                //go down a directory
                
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

        println!("size: {}      line: {}  ", dir_size, line);

        
    }

    //loop back to the root directory
    while currentDirSizes.len() > 1 {
        let dir_size = currentDirSizes.pop();
        let dir_size = dir_size.unwrap();
        if dir_size < 100000 {
            println!("{}", dir_size);
            size_counter += dir_size;
            let old_size = currentDirSizes.pop();
            let old_size = old_size.unwrap();
            let new_size = old_size + dir_size;
            currentDirSizes.push(new_size);

        }
        let mut new_location = "".to_string();
                
        
    }

    //add the root directory size if it is less than 100000
    let dir_size = currentDirSizes.pop();
    let dir_size = dir_size.unwrap();
    if dir_size < 100000 {
        println!("Root: {}", dir_size);
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
    


    let mut currentDirSizes = Vec::new();


    currentDirSizes.push(0);

    

    

    //Find the space remaining
    let mut space_remaining = 70000000;
    
    for line in lines.clone() {
        if line.starts_with("$ cd ") {
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
            space_remaining -= file_size.parse::<usize>().unwrap();
        }
    }

    let space_to_clear = 30000000 - space_remaining;

    println!("Space to clear: {}", space_to_clear);
    
    let mut folder_to_clear_size = 70000000 - space_remaining; //set to the size of the root directory

    let mut lines = list.lines();

    //skip the first line
    lines.next();

    //loop through the lines
    for line in lines {
        
        
        if line.starts_with("$ cd ") {
            let dir_name = line.split(" ").nth(2).unwrap();

            if dir_name == ".." {
                //go up a directory
                
                

                
                
                let dir_size = currentDirSizes.pop();
                let dir_size = dir_size.unwrap();

                let old_size = currentDirSizes.pop();
                let old_size = old_size.unwrap();
                let new_size = old_size + dir_size;
                currentDirSizes.push(new_size);
                

                if dir_size < folder_to_clear_size && dir_size >= space_to_clear {
                    folder_to_clear_size = dir_size;
                }
            } else {
                //go down a directory
                
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

        println!("size: {}      line: {}  ", dir_size, line);

        
    }

    //loop back to the root directory
    while currentDirSizes.len() > 1 {
        let dir_size = currentDirSizes.pop();
        let dir_size = dir_size.unwrap();
        let old_size = currentDirSizes.pop();
        let old_size = old_size.unwrap();
        let new_size = old_size + dir_size;
        currentDirSizes.push(new_size);

        if dir_size < folder_to_clear_size && dir_size >= space_to_clear {
            folder_to_clear_size = dir_size;
        }

        
                
        
    }



    println!("Size to delete: {}", folder_to_clear_size);
}