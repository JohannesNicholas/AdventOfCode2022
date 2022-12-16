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

    //set AA to have a combined flow rate of 1
    valves.get_mut(&('A', 'A')).unwrap().combined_flow_rate = 1;

    let mut queue = Vec::new();

    //add the first valve to the queue
    queue.push(('A', 'A'));

    //while there are still valves in the queue
    while queue.len() > 0 {
        //get the first valve
        let current_valve_chars = queue.remove(0);

        

        //get the valve
        let current_valve = valves.get(&current_valve_chars).unwrap();

        let tunnels = current_valve.tunnels.clone();

        let combined_flow_rate = current_valve.combined_flow_rate;

        let current_steps = current_valve.steps;

        println!("Looking at {}{} which has combined flow rate of {}", current_valve_chars.0, current_valve_chars.1,combined_flow_rate);

        //loop through the tunnels
        for tunnel in tunnels {
            //get the valve
            let tunnel_valve = valves.get_mut(&tunnel).unwrap();

            let minutes_remaining = 30 - current_steps - 1;

            let calc_flow = combined_flow_rate + tunnel_valve.flow_rate * minutes_remaining;

            println!("{}{} has flow rate of {} and combined flow rate of {}", tunnel.0, tunnel.1, tunnel_valve.flow_rate, calc_flow);

            if calc_flow > tunnel_valve.combined_flow_rate {

                //if it is a node that has flor
                if tunnel_valve.flow_rate > 0 {
                    //make sure it is not on the path back
                    let mut on_path_back = false;
                    let mut current_valve = current_valve_chars;

                    while !on_path_back && current_valve != (' ', ' ') {
                        //get the valve
                        let valve = valves.get(&current_valve).unwrap();

                        //if the current valve is the same as the tunnel
                        if valve.previous_valve == tunnel {
                            on_path_back = true;
                        } else {
                            //if it is not the same, set the current valve to the previous valve
                            current_valve = valve.previous_valve;
                        }

                    }

                }

                tunnel_valve.combined_flow_rate = calc_flow;
                tunnel_valve.previous_valve = current_valve_chars;
                tunnel_valve.steps = current_steps + 1;

                //add to the queue
                queue.push(tunnel);
            }

            
        }


    }
    

    

    //print out the hashmap
    for (key, value) in &valves {
        println!("{}{}: {:?} - {:?}", key.0, key.1, value.flow_rate, value.tunnels);
    }

}

pub fn stage2() {}