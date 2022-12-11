use std::fs;


struct monkey {
    items: Vec<i32>,
    operation: String,
    test: i32,
    if_true: i32,
    if_false: i32,
    activity: i32,
}

pub fn stage1() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the lines
    let mut lines = list.lines();

    let mut monkeys = Vec::new();

    for i in 0..8 {
        let mut monkey = monkey {
            items: Vec::new(),
            operation: String::new(),
            test: 0,
            if_true: 0,
            if_false: 0,
            activity: 0,
        };

        lines.next(); //first line is monkey name
        let starting_items = lines.next().unwrap().split("items: ").collect::<Vec<&str>>()[1];

        let starting_items = starting_items.split(", ").collect::<Vec<&str>>();

        for item in starting_items {
            monkey.items.push(item.parse::<i32>().unwrap());
        }

        //get operation, it comes after "Operation: new = "
        let operation = lines.next().unwrap();
        //remove the start
        let operation = operation.split(" = ").collect::<Vec<&str>>()[1];
        monkey.operation = operation.to_string();

        //get test
        let test = lines.next().unwrap();
        let test = test.split("divisible by ").collect::<Vec<&str>>()[1];
        monkey.test = test.parse::<i32>().unwrap();

        //get if_true
        let if_true = lines.next().unwrap();
        let if_true = if_true.split("to monkey ").collect::<Vec<&str>>()[1];
        monkey.if_true = if_true.parse::<i32>().unwrap();

        //get if_false
        let if_false = lines.next().unwrap();
        let if_false = if_false.split("to monkey ").collect::<Vec<&str>>()[1];
        monkey.if_false = if_false.parse::<i32>().unwrap();



        

        //last line is empty
        lines.next();

        monkeys.push(monkey);

    }



    for round in 1..21 {
        //loop through monkeys and keep index
        
        for monkeyID in 0..(monkeys.len()) {
            
            //println!("Monkey {}:", monkeyID);

            let mut numItems = monkeys[monkeyID].items.len();

            //loop through items
            while numItems > 0 {

                

                let item = monkeys[monkeyID].items[0];

                //println!("  Monkey inspects an item with a worry level of {}.", item);

                let old = item as i128;

                let mut worry_level: i128 = 0;

                let monkey = &mut monkeys[monkeyID];

                monkey.activity += 1;

                if monkey.operation.starts_with("old * old") {
                    worry_level = old * old;
                }
                else if monkey.operation.starts_with("old * ") {
                    let value = monkey.operation.split("old * ").collect::<Vec<&str>>()[1];
                    let value = value.parse::<i32>().unwrap();
                    worry_level = old * value as i128;
                }
                else if monkey.operation.starts_with("old + ") {
                    let value = monkey.operation.split("old + ").collect::<Vec<&str>>()[1];
                    let value = value.parse::<i32>().unwrap();
                    worry_level = old + value as i128;
                }



                //println!("    Worry level is {}", worry_level);



                let new_worry = worry_level / 3;

                //println!("    Monkey gets bored with item. Worry to {}.", new_worry);

                let to_monkey: i32 = 
                if new_worry % monkey.test as i128 == 0 {
                    monkey.if_true
                }
                else {
                    monkey.if_false
                };
                //println!("    It gives it to monkey {}.", to_monkey);

                

                monkey.items.remove(0);

                numItems -= 1;

                

                let monkey = &mut monkeys[to_monkey as usize];

                monkey.items.push(new_worry as i32);

                
                
            }

            
        }
    }

    //loop through the monkeys and output activity
    for i in 0..(monkeys.len()) {
        println!("Monkey {} did {} activities.", i, monkeys[i].activity);
    }

    //get the two most active monkeys
    let mut most_active = 0;
    let mut second_most_active = 0;

    for i in 0..(monkeys.len()) {
        if monkeys[i].activity > most_active {
            second_most_active = most_active;
            most_active = monkeys[i].activity;
        }
        else if monkeys[i].activity > second_most_active {
            second_most_active = monkeys[i].activity;
        }
    }

    println!("TOTAL: {}", most_active as i128 * second_most_active as i128);
    
}



pub fn stage2() {
    println!("Day 4 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the lines
    let mut lines = list.lines();

    let mut monkeys = Vec::new();

    let mut safe_wrapper = 1;

    for i in 0..8 {
        let mut monkey = monkey {
            items: Vec::new(),
            operation: String::new(),
            test: 0,
            if_true: 0,
            if_false: 0,
            activity: 0,
        };

        lines.next(); //first line is monkey name
        let starting_items = lines.next().unwrap().split("items: ").collect::<Vec<&str>>()[1];

        let starting_items = starting_items.split(", ").collect::<Vec<&str>>();

        for item in starting_items {
            monkey.items.push(item.parse::<i32>().unwrap());
        }

        //get operation, it comes after "Operation: new = "
        let operation = lines.next().unwrap();
        //remove the start
        let operation = operation.split(" = ").collect::<Vec<&str>>()[1];
        monkey.operation = operation.to_string();

        //get test
        let test = lines.next().unwrap();
        let test = test.split("divisible by ").collect::<Vec<&str>>()[1];
        monkey.test = test.parse::<i32>().unwrap();

        //get if_true
        let if_true = lines.next().unwrap();
        let if_true = if_true.split("to monkey ").collect::<Vec<&str>>()[1];
        monkey.if_true = if_true.parse::<i32>().unwrap();

        //get if_false
        let if_false = lines.next().unwrap();
        let if_false = if_false.split("to monkey ").collect::<Vec<&str>>()[1];
        monkey.if_false = if_false.parse::<i32>().unwrap();



        safe_wrapper = safe_wrapper * monkey.test;

        //last line is empty
        lines.next();

        monkeys.push(monkey);

    }



    for round in 1..10001 {
        //loop through monkeys and keep index
        
        for monkeyID in 0..(monkeys.len()) {
            
            //println!("Monkey {}:", monkeyID);

            let mut numItems = monkeys[monkeyID].items.len();

            //loop through items
            while numItems > 0 {

                

                let item = monkeys[monkeyID].items[0];

                //println!("  Monkey inspects an item with a worry level of {}.", item);

                let old = item as i128;

                let mut worry_level: i128 = 0;

                let monkey = &mut monkeys[monkeyID];

                monkey.activity += 1;

                if monkey.operation.starts_with("old * old") {
                    worry_level = old * old;
                }
                else if monkey.operation.starts_with("old * ") {
                    let value = monkey.operation.split("old * ").collect::<Vec<&str>>()[1];
                    let value = value.parse::<i32>().unwrap();
                    worry_level = old * value as i128;
                }
                else if monkey.operation.starts_with("old + ") {
                    let value = monkey.operation.split("old + ").collect::<Vec<&str>>()[1];
                    let value = value.parse::<i32>().unwrap();
                    worry_level = old + value as i128;
                }



                //println!("    Worry level is {}", worry_level);



                let new_worry = worry_level % safe_wrapper as i128;

                //println!("    Monkey gets bored with item. Worry to {}.", new_worry);

                let to_monkey: i32 = 
                if new_worry % monkey.test as i128 == 0 {
                    monkey.if_true
                }
                else {
                    monkey.if_false
                };
                //println!("    It gives it to monkey {}.", to_monkey);

                

                monkey.items.remove(0);

                numItems -= 1;

                

                let monkey = &mut monkeys[to_monkey as usize];

                monkey.items.push(new_worry as i32);

                
                
            }

            
        }
    }

    //loop through the monkeys and output activity
    for i in 0..(monkeys.len()) {
        println!("Monkey {} did {} activities.", i, monkeys[i].activity);
    }

    //get the two most active monkeys
    let mut most_active = 0;
    let mut second_most_active = 0;

    for i in 0..(monkeys.len()) {
        if monkeys[i].activity > most_active {
            second_most_active = most_active;
            most_active = monkeys[i].activity;
        }
        else if monkeys[i].activity > second_most_active {
            second_most_active = monkeys[i].activity;
        }
    }

    println!("TOTAL: {}", most_active as i128 * second_most_active as i128);


}
