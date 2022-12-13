use std::{fs, rc::Rc};


struct tree_node {
    value: Option<i32>,
    children: Vec<Box<tree_node>>,
}

pub fn stage1() {
    println!("Day 13 challenge! pt.1");

    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //get the lines
    let mut lines = list.lines();
    
    let mut count = 0;
    

    



    //convert lines into a list
    let mut list = Vec::new();
    for line in lines {
        list.push(line);
    }

    let mut correct_lines = Vec::new();

    for line_num in 0..(list.len() / 3) {
        let first = list[line_num * 3];
        let second = list[line_num * 3 + 1];

        let mut left_tree = tree_node {
            value: None,
            children: Vec::new(),
        };
        let mut right_tree = tree_node {
            value: None,
            children: Vec::new(),
        };

        let mut first_chars = first.chars().collect::<Vec<char>>();
        let mut second_chars = second.chars().collect::<Vec<char>>();

        //remove the first and last characters as these are brackets

        build_tree(first_chars, &mut left_tree);
        build_tree(second_chars, &mut right_tree);

        println!("Left tree: ");
        println!("{}", first);
        println!("{}", print_tree(&left_tree));
        println!("Right tree:");
        println!("{}", second);
        println!("{}", print_tree(&right_tree));
        

        //detect if parsing is wrong
        if first != print_tree(&left_tree) {
            println!("REEEEEEE Left")
        }
        if second != print_tree(&right_tree) {
            println!("REEEEEEE Right")
        }

        println!();

        let result = tree_order_correct(&left_tree, &right_tree);
        //println!("Result: {:?}", result);

        if result.is_some() && result.unwrap() {
            count += line_num + 1;
            
            correct_lines.push(line_num + 1);
        }
        
    }

    for line in correct_lines {
        println!("Line {} is correct", line);
    }

    println!("TOTAL: {}", count);
    
}


fn tree_order_correct(left: &tree_node, right: &tree_node) -> Option<bool> {
    //if both values are ints
    if left.value.is_some() && right.value.is_some() {
        if left.value.unwrap() < right.value.unwrap() {
            return Some(true);
        }
        else if left.value.unwrap() > right.value.unwrap() {
            return Some(false);
        }
        else {
            return None;
        }
    }
    //if both values are trees
    else if !left.value.is_some() && !right.value.is_some() {
        let mut num = left.children.len();
        if right.children.len() > num {
            num = right.children.len();
        }

        //loop through the children
        for i in 0..num {
            //if left has run out, return true
            if i >= left.children.len() && i < right.children.len() {
                return Some(true);
            }
            //if right has run out, return false
            else if i >= right.children.len() && i < left.children.len() {
                return Some(false);
            }

            let left_child = &left.children[i];
            let right_child = &right.children[i];

            let result = tree_order_correct(&left_child, &right_child);
            if result.is_some() {
                return result;
            }
        }
    }
    //One is int but the other is not
    else {
        //if left is int, convert it to a tree
        if left.value.is_some() {
            let mut left_tree = tree_node {
                value: None,
                children: Vec::new(),
            };
            left_tree.children.push(Box::new(tree_node {
                value: Some(left.value.unwrap()),
                children: Vec::new(),
            }));
            return tree_order_correct(&left_tree, right);
        }

        //if right is int, convert it to a tree
        if right.value.is_some() {
            let mut right_tree = tree_node {
                value: None,
                children: Vec::new(),
            };
            right_tree.children.push(Box::new(tree_node {
                value: Some(right.value.unwrap()),
                children: Vec::new(),
            }));
            return tree_order_correct(&left, &right_tree);
        }
    }

    return None;
}

fn print_tree(node: &tree_node) -> String {
    let mut string = String::new();

    if !node.value.is_some() {
        for child in &node.children {
            if child.value.is_some() {
                string.push_str(&child.value.unwrap().to_string());
                //print!("{},", child.value.unwrap());
                //if not the last child, add a comma
                string.push_str(",");
            }
            else {
                string.push_str("[");
                string.push_str(&print_tree(&child));
                string.push_str("],");
            }
            
        }
        //remove the last comma
        string.pop();
    }

    return string;
}

//take an array of chars and a current node, and fill in the tree
fn build_tree(chars: Vec<char>, current_node: &mut tree_node) {

    //println!("Building tree from: {:?}", chars);

    

    let mut counter = 0;
    //loop through the characters in the first line
    while counter < chars.len() {
        let char = chars[counter];
        let next_index = counter + 1;
        if char == ',' {
        }
        else if char == '[' {
            current_node.children.push(Box::new(tree_node {
                value: None,
                children: Vec::new(),
            }));

            //find where the bracket ends
            let mut bracket_count = 1;
            let mut end_index = counter + 1;
            while bracket_count > 0 {
                let char = chars[end_index];
                if char == '[' {
                    bracket_count += 1;
                }
                else if char == ']' {
                    bracket_count -= 1;
                }
                end_index += 1;
            }

            //extract the subarray
            let mut subarray = Vec::new();
            for i in counter..end_index {
                subarray.push(chars[i]);
            }

            //remove the first and last characters as these are brackets
            subarray.remove(0);
            subarray.pop();

            //recurse
            build_tree(subarray, &mut current_node.children.last_mut().unwrap());

            //move the counter to the end of the bracket
            counter = end_index;
        }
        else {

            //add the value to the current node
            //println!("Adding value: {}", char);
            let mut value = char.to_digit(10).unwrap() as i32;

            //if there is a next char
            if next_index < chars.len() {
                let next_char = chars[next_index];
                if next_char != ',' && next_char != ']' {
                    //next char is a number, so add it to the current value
                    value *= 10;
                    value += chars[counter + 1].to_digit(10).unwrap() as i32;
                    counter += 1;
                }
            }
            

            current_node.children.push(Box::new(tree_node {
                value: Some(value),
                children: Vec::new(),
            }));

            
        }

        counter += 1;
    }
}


pub fn stage2() {
    
}
