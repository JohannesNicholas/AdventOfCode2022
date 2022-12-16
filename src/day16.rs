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
        //let mut tunnels_for_2: Vec<(char, char)> = tunnels.iter().map(|x| (x.chars().nth(0).unwrap(), x.chars().nth(1).unwrap())).collect();
        
        //let on_node = (valve_name.0.to_ascii_lowercase(), valve_name.1.to_ascii_lowercase());

        valves.insert(valve_name, valve {
            tunnels: tunnels_for_1,
            flow_rate: flow_rate,
            combined_flow_rate: -1,
            previous_valve: (' ', ' '),
            steps: 0,
        });

        
        //tunnels_for_2.push(on_node);
//
        ////add to the hashmap
        //valves.insert(valve_name, valve {
        //    tunnels: tunnels_for_2,
        //    flow_rate: 0,
        //    combined_flow_rate: -1,
        //    previous_valve: (' ', ' '),
        //    steps: 0,
        //});
    }

    

    //print out the hashmap
    for (key, value) in &valves {
        println!("{}{}: {:?} - {:?}", key.0, key.1, value.flow_rate, value.tunnels);
    }

    
    //let shortest = shortest_path(&valves, ('A', 'A'), ('I', 'I'));
    //println!("Shortest path: {}", shortest);


    //find the most valuable valve
    let mut most_valuable_valve = (' ', ' ');
    let mut most_valuable_value = 0;

    for (key, value) in &valves {

        let minutes = 30 - shortest_path(&valves, ('A','A'), *key);
        let value = value.flow_rate * minutes;

        if value > most_valuable_value {
            most_valuable_valve = *key;
            most_valuable_value = value;
        }
    }

    println!("Most valuable valve: {}{} - {}", most_valuable_valve.0, most_valuable_valve.1, most_valuable_value);




    let mut valuable_locations = HashMap::new();
    for (key, valve) in &valves {
        if valve.flow_rate > 0 {
            valuable_locations.insert(*key, true);
        }
    }

    let most_flow = most_flow(&valves, valuable_locations, 30, ('A', 'A'));

    println!("Most flow: {}", most_flow);


}


struct flow_cache_key{
    number_of_valves: i32,
    valves: [(char, char); 15],
    valve: (char, char),
}
static mut flow_cache: Option<HashMap<[(char,char);16], i32>> = None;

fn most_flow(values : &HashMap<(char, char), valve>, valuable_locations: HashMap<(char, char), bool>, minutes: i32, valve: (char, char)) -> i32 {

    //generate the key
    let mut key = [(' ', ' '); 16];
  
    for i in 0..valuable_locations.len() {
        key[i + 1] = *valuable_locations.keys().nth(i).unwrap();
    }
    key[0] = valve;

    //check if the value is in the cache
    unsafe {
        if let Some(cache) = &flow_cache {
            if let Some(value) = cache.get(&key) {
                return *value;
            }
        }
    }


    let mut most_flow_amonut = 0;
    let mut most_minutes = 0;
    let mut most_valve = (' ', ' ');

    let mut valuable_locations = valuable_locations.clone();
    valuable_locations.remove(&valve);

    //loop through all the valuable locations
    for valuable_location in valuable_locations.keys() {

        let distance = shortest_path(values, valve, *valuable_location);

        let flow = most_flow(values, valuable_locations.clone(), minutes - distance - 1, *valuable_location);

        if flow > most_flow_amonut {
            most_flow_amonut = flow;
            most_minutes = minutes;
            most_valve = *valuable_location;
        }
    }


    if valuable_locations.len() > 9 {
        println!("Looking at {:?}", valuable_locations.keys());
    }

    //add the value to the cache
    unsafe {
        if flow_cache.is_none() {
            flow_cache = Some(HashMap::new());
        }
        let cache = flow_cache.as_mut().unwrap();
        cache.insert(key, most_flow_amonut + minutes * values.get(&valve).unwrap().flow_rate);

        flow_cache = Some(cache.clone());
    }
    

    return most_flow_amonut + minutes * values.get(&valve).unwrap().flow_rate;
}


static mut distances_cache: Option<HashMap<((char, char),(char,char)), i32>> = None;

//using dycjstra's algorithm to find the shortest path between two nodes
fn shortest_path(valves: &HashMap<(char, char), valve>, start: (char, char), end: (char, char)) -> i32 {
    
    //check if the distance is in the cache
    unsafe {
        if distances_cache.is_some() {
            let distances_cache_local = distances_cache.as_ref().unwrap();
            if distances_cache_local.contains_key(&(start, end)) {
                return *distances_cache_local.get(&(start, end)).unwrap();
            }
        }
    }

    //create a hashmap to store the distances
    let mut distances = HashMap::new();

    //set the start node to 0
    distances.insert(start, 0);

    //create a hashmap to store the previous nodes
    let mut previous = HashMap::new();

    //create a list of nodes to visit
    let mut nodes = vec![start];

    //loop until there are no more nodes to visit
    while nodes.len() > 0 {
        //sort the nodes by distance
        nodes.sort_by(|a, b| distances.get(a).unwrap().cmp(distances.get(b).unwrap()));

        //get the node with the shortest distance
        let current_node = nodes.remove(0);

        //if the current node is the end node, break
        if current_node == end {
            break;
        }

        //get the current distance
        let current_distance = distances.get(&current_node).unwrap();
        let current_distance = *current_distance;

        //get the current node's neighbours
        let neighbours = &valves.get(&current_node).unwrap().tunnels;

        //loop through the neighbours
        for neighbour in neighbours {
            //get the distance to the neighbour
            let distance_to_neighbour = current_distance + 1;

            //if the distance to the neighbour is less than the current distance, update the distance
            if distances.get(neighbour).is_none() || distance_to_neighbour < *distances.get(neighbour).unwrap() {
                distances.insert(*neighbour, distance_to_neighbour);
                previous.insert(*neighbour, current_node);
                nodes.push(*neighbour);
            }
        }
    }

    //get the path
    let mut path = vec![end];
    let mut current_node = end;
    while current_node != start {
        //println!("Current node: {:?}", current_node);
        current_node = *previous.get(&current_node).unwrap();
        path.push(current_node);
    }

    //reverse the path
    path.reverse();

    //print out the path
    //println!("Path: {:?}", path);

    //add the distance to the cache
    unsafe {
        if distances_cache.is_none() {
            distances_cache = Some(HashMap::new());
        }
        let distances_cache_local = distances_cache.as_mut().unwrap();
        distances_cache_local.insert((start, end), *distances.get(&end).unwrap());

        distances_cache = Some(distances_cache_local.clone());
    }

    //return the distance
    *distances.get(&end).unwrap()
}


pub fn stage2() {}