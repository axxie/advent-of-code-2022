use std::collections::HashMap;
use std::collections::HashSet;

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
    distance: Vec<Vec<u32>>,
    non_zero: HashSet<usize>,
    open: Vec<bool>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            node_index: HashMap::new(),
            nodes: Vec::new(),
            distance: Vec::new(),
            non_zero: HashSet::new(),
            open: Vec::new(),
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

    fn floyd_warshall(&mut self) {
        let len = self.nodes.len();
        for _ in 0..self.nodes.len() {
            self.distance.push(vec![(len * len) as u32; len])
        }
        for i in 0..len {
            self.distance[i][i] = 0;
            for &adjacent in &self.nodes[i].adjacent {
                self.distance[i][adjacent] = 1;
            }
        }
        for k in 0..len {
            for i in 0..len {
                for j in 0..len {
                    if self.distance[i][j] > self.distance[i][k] + self.distance[k][j] {
                        self.distance[i][j] = self.distance[i][k] + self.distance[k][j]
                    }
                }
            }
        }
    }

    fn init_non_zero_list(&mut self) {
        for i in 0..self.nodes.len() {
            if self.nodes[i].flow != 0 {
                self.non_zero.insert(i);
            }
        }
    }

    fn get_flow(&self, open: &HashSet<usize>) -> u32 {
        let mut result: u32 = 0;
        for &valve in open {
            result += self.nodes[valve].flow;
        }
        return result;
    }
}

fn dfs(
    limit: u32,
    graph: &Graph,
    time: u32,
    current_total: u32,
    position: usize,
    candidates: u64,
    open: &mut HashSet<usize>,
) -> u32 {
    let mut maximum = current_total + (limit - time) * graph.get_flow(open);

    for (index, &new_position) in graph.non_zero.iter().enumerate() {
        if candidates & (1 << index) == 0 {
            continue;
        }
        let distance = graph.distance[position][new_position] + 1;

        if time + distance > limit {
            continue;
        }

        if open.contains(&new_position) {
            continue;
        }

        let new_total = current_total + distance * graph.get_flow(open);
        open.insert(new_position);
        let result = dfs(
            limit,
            graph,
            time + distance,
            new_total,
            new_position,
            candidates,
            open,
        );
        open.remove(&new_position);
        if maximum < result {
            maximum = result;
            if maximum == 2484 {
                let a = 0;
            }
        }
    }
    return maximum;
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
    graph.open = vec![false; graph.nodes.len()];

    graph.init_non_zero_list();
    graph.floyd_warshall();

    let first = false;
    let limit = if first { 30 } else { 26 };
    let mut best_score: u32 = 0;
    let mut open: HashSet<usize> = HashSet::new();
    let upper: u64 = 2_u64.pow(graph.non_zero.len() as u32);
    let position = graph.get_or_add("AA");

    println!("Size: {}", graph.non_zero.len());

    if first {
        best_score = dfs(limit, &graph, 0, 0, position, upper - 1, &mut open);
    } else {
        for mask in 0..upper {
            if mask % 256 == 0 {
                println!("Done {}%", mask * 100 / upper);
            }
            let my_score = dfs(limit, &graph, 0, 0, position, mask, &mut open);
            let elephant_score = dfs(limit, &graph, 0, 0, position, mask ^ (upper - 1), &mut open);
            if best_score < my_score + elephant_score {
                best_score = my_score + elephant_score;
            }
        }
    }

    println!("Result {best_score}");
}
