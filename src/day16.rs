use std::{fs, collections::HashMap};

struct valve {
    tunnels: Vec<(char, char)>,
    flow_rate: i32,
    combined_flow_rate: i32,
    previous_valve: (char, char),
    steps: i32, //steps to get here
}

pub fn stage1() {
    println!("Day 15 challenge!");
    
    //take input from file
    let list = fs::read_to_string("input.txt").expect("Error reading file");

    //split input into lines
    let lines = list.lines();

    let mut valves = HashMap::new();

    for line in lines {
        //lines are layed out like: Valve AA has flow rate=13; tunnels lead to valves DD, II, BB

        //get the valve name
        let valve_name = &line[6..8];
        //convert into (char, char)
        let valve_name: (char,char) = (valve_name.chars().nth(0).unwrap(), valve_name.chars().nth(1).unwrap());

        //get the flow rate
        let flow_rate: i32 = line.split("flow rate=").nth(1).unwrap().split(";").nth(0).unwrap().parse().unwrap();
        

        
        println!("{}", line);
        //get the tunnels
        let tunnels = line.split(" to valve").nth(1).unwrap();
        //remove the s if there is one
        let tunnels = if tunnels.starts_with("s") {
            &tunnels[1..]
        } else {
            tunnels
        };
        //remove the space
        let tunnels = &tunnels[1..];

        let mut tunnels: Vec<&str> = tunnels.split(", ").collect();

        //remove the last one if it is empty
        if tunnels[tunnels.len() - 1] == "" {
            tunnels.pop();
        }

        //convert into (char, char)
        let mut tunnels_for_1: Vec<(char, char)> = tunnels.iter().map(|x| (x.chars().nth(0).unwrap(), x.chars().nth(1).unwrap())).collect();
        let mut tunnels_for_2: Vec<(char, char)> = tunnels.iter().map(|x| (x.chars().nth(0).unwrap(), x.chars().nth(1).unwrap())).collect();
        
        let on_node = (valve_name.0.to_ascii_lowercase(), valve_name.1.to_ascii_lowercase());

        valves.insert(on_node, valve {
            tunnels: tunnels_for_1,
            flow_rate: flow_rate,
            combined_flow_rate: -1,
            previous_valve: (' ', ' '),
            steps: 0,
        });

        
        tunnels_for_2.push(on_node);

        //add to the hashmap
        valves.insert(valve_name, valve {
            tunnels: tunnels_for_2,
            flow_rate: 0,
            combined_flow_rate: -1,
            previous_valve: (' ', ' '),
            steps: 0,
        });
    }

    

    //print out the hashmap
    for (key, value) in &valves {
        println!("{}{}: {:?} - {:?}", key.0, key.1, value.flow_rate, value.tunnels);
    }

    //find the way
    let highest_way = find_way(&mut valves, vec![('A', 'A')], 30);

    println!("Highest way: {}", highest_way.0);

    println!("Path: {:?}", highest_way.1);

}


fn find_way(valves: &mut HashMap<(char, char), valve>, path: Vec<(char, char)>, minutes: i32) -> (i32, Vec<(char, char)>) {

    

    //get the last valve
    let last_valve = path[path.len() - 1];

    if minutes < 0 {
        println!("Path: {:?}", path);
        return (0, vec![]);
    }

    //get the valve
    let mut valve = valves.get(&last_valve).unwrap();

    let flow_rate = valve.flow_rate;
    let tunnels_copy = valve.tunnels.clone();


    //if the valve has already been visited
    let mut already_visited = false;
    for i in 0..(path.len()-1) {
        if flow_rate > 0 {
            let node = &path[i];
            if *node == last_valve {
                already_visited = true;
                break;
            }
        }
        
    }

    //if the valve has already been visited
    if already_visited {
        return (0, vec![]);
    }


    let mut highest_way = (0, vec![]);
    //loop through the tunnels
    for tunnel in tunnels_copy {

        //add the tunnel to the path
        let mut new_path = path.clone();
        new_path.push(tunnel);

        //find the way
        let way = find_way(valves, new_path, minutes-1);

        //if the way is higher than the highest way
        if way.0 > highest_way.0 {
            highest_way = way;
        }
        
    }

    highest_way.1.push(last_valve);

    return (highest_way.0 + flow_rate * minutes, highest_way.1);
}


pub fn stage2() {}