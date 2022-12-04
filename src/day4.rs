use std::fs;



pub fn stage1() {
    println!("Day 4 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    let mut counter = 0;

    //loop through lines
    for line in lines {
        //get number before "-"
        let number1 = line.split("-").next().unwrap();
        //get number after "-" but before ","
        let number2 = line.split("-").nth(1).unwrap().split(",").next().unwrap();
        //get number after "," but before "-"
        let number3 = line.split(",").nth(1).unwrap().split("-").next().unwrap();
        //get number after second "-"
        let number4 = line.split("-").nth(2).unwrap();

        //convert numbers to u32
        let number1 = number1.parse::<u32>().unwrap();
        let number2 = number2.parse::<u32>().unwrap();
        let number3 = number3.parse::<u32>().unwrap();
        let number4 = number4.parse::<u32>().unwrap();

        //if the range of number1 to number 2 is inside the range of number3 to number4
        if number1 >= number3 && number1 <= number4 {
            if number2 >= number3 && number2 <= number4 {
                
                counter += 1;
                continue;
            }
        }

        //if the range of number3 to number 4 is inside the range of number1 to number2
        if number3 >= number1 && number3 <= number2 {
            if number4 >= number1 && number4 <= number2 {
                
                counter += 1;
            }
        }


        
    }

    println!("Total: {}", counter);

    
}

pub fn stage2() {
    println!("Day 4 challenge! pt.2");
   

    
}