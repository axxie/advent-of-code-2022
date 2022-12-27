use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Node {
    name: String,
    flow: u32,
    adjacent: Vec<usize>,
}

#[derive(Debug)]
struct Graph {
    node_index: HashMap<String, usize>,
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            node_index: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn get_or_add(&mut self, name: &str) -> usize {
        match self.node_index.get(name) {
            None => {
                self.nodes.push(Node {
                    name: name.to_string(),
                    flow: 0,
                    adjacent: Vec::new(),
                });
                self.node_index
                    .insert(name.to_string(), self.nodes.len() - 1);
                self.nodes.len() - 1
            }
            Some(i) => *i,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct State {
    valves: Vec<bool>,
    position: usize,
    position2: usize,
}

fn main() {
    let mut graph: Graph = Graph::new();

    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let v: Vec<&str> = line.split(",").collect();
        let name: &str = v[0];
        let flow: u32 = v[1].parse().unwrap();

        let index = graph.get_or_add(name);
        graph.nodes[index].flow = flow;

        for adjacent in &v[2..] {
            let adj_index = graph.get_or_add(adjacent);
            graph.nodes[index].adjacent.push(adj_index);
        }
    }

    let mut current_states: HashMap<State, u32> = HashMap::new();
    let valves = vec![false; graph.nodes.len()];
    current_states.insert(
        State {
            valves,
            position: graph.get_or_add("AA"),
            position2: graph.get_or_add("AA"),
        },
        0,
    );

    for _iteration in 0..26 {
        let mut next_states: HashMap<State, u32> = HashMap::new();
        for (state, &score) in &current_states {
            let mut new_score = score;

            for (index, &valve) in state.valves.iter().enumerate() {
                if valve {
                    new_score += graph.nodes[index].flow;
                }
            }

            let position = state.position;
            for adjacent1 in &graph.nodes[position].adjacent {
                for adjacent2 in &graph.nodes[position].adjacent {
                    let new_state: State = State {
                        valves: state.valves.clone(),
                        position: *adjacent1,
                        position2: *adjacent2,
                    };
                    let new_entry = next_states.entry(new_state).or_insert(new_score);
                    if *new_entry < new_score {
                        *new_entry = new_score;
                    }
                }
            }

            for &adjacent in &graph.nodes[position].adjacent {
                if !state.valves[position] && graph.nodes[position].flow > 0 {
                    let mut new_state: State = State {
                        valves: state.valves.clone(),
                        position,
                        position2: adjacent,
                    };
                    new_state.valves[position] = true;
                    let new_entry = next_states.entry(new_state).or_insert(new_score);
                    if *new_entry < new_score {
                        *new_entry = new_score;
                    }
                }
            }

            let position2 = state.position2;
            for &adjacent in &graph.nodes[position2].adjacent {
                if !state.valves[position2] && graph.nodes[position].flow > 0 {
                    let mut new_state: State = State {
                        valves: state.valves.clone(),
                        position: adjacent,
                        position2,
                    };
                    new_state.valves[position2] = true;
                    let new_entry = next_states.entry(new_state).or_insert(new_score);
                    if *new_entry < new_score {
                        *new_entry = new_score;
                    }
                }
            }

            let mut new_state: State = State {
                valves: state.valves.clone(),
                position,
                position2,
            };
            new_state.valves[position] = true;
            new_state.valves[position2] = true;
            let new_entry = next_states.entry(new_state).or_insert(new_score);
            if *new_entry < new_score {
                *new_entry = new_score;
            }
        }
        current_states = next_states;
    }

    let mut best_score: u32 = 0;
    let mut best_state = &State {
        valves: vec![],
        position: 0,
        position2: 0,
    };
    for (state, &score) in &current_states {
        if score > best_score {
            best_state = state;
            best_score = score;
        }
    }
    println!("Result {best_score}");
}
