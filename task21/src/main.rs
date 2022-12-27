use core::slice::Iter;
use std::cmp::max;

use std::collections::HashMap;
use std::collections::HashSet;

const OP_ADD: usize = 0;
const OP_SUB: usize = 1;
const OP_MUL: usize = 2;
const OP_DIV: usize = 3;

#[derive(Clone, Copy, Debug)]
struct Operation {
    op1: usize,
    op2: usize,
    operator: usize,
}

#[derive(Clone, Copy, Debug)]
enum NodeValue {
    Value(u128),
    Operation(Operation),
}

#[derive(Clone, Debug)]
struct Node {
    name: String,
    value: NodeValue,
}

impl Node {
    fn get_operation(&self) -> Operation {
        match &self.value {
            NodeValue::Value(_) => panic!("No operation"),
            NodeValue::Operation(operation) => *operation,
        }
    }
}

#[derive(Clone, Debug)]
enum Expression {
    Value(u128),
    Formula(String),
}

impl Expression {
    fn get_string(&self) -> String {
        match self {
            Expression::Value(value) => value.to_string(),
            Expression::Formula(text) => text.clone(),
        }
    }
}

#[derive(Debug)]
struct Graph {
    node_index: HashMap<String, usize>,
    nodes: Vec<Node>,
    calculated: Vec<Option<u128>>,
    humn: usize,
}

impl Graph {
    fn new() -> Self {
        Graph {
            node_index: HashMap::new(),
            nodes: Vec::new(),
            calculated: Vec::new(),
            humn: 0,
        }
    }

    fn get_or_add(&mut self, name: &str) -> usize {
        match self.node_index.get(name) {
            None => {
                self.nodes.push(Node {
                    name: name.to_string(),
                    value: NodeValue::Value(0),
                });
                self.calculated.push(None);
                self.node_index
                    .insert(name.to_string(), self.nodes.len() - 1);
                self.nodes.len() - 1
            }
            Some(i) => *i,
        }
    }

    fn calculate(&mut self, index: usize) -> u128 {
        let mut node = &mut self.nodes[index];
        if let NodeValue::Value(value) = node.value {
            self.calculated[index] = Some(value);
            return value;
        }
        if let Some(value) = self.calculated[index] {
            return value;
        }

        let Operation { op1, op2, operator } = node.get_operation();
        let val1 = self.calculate(op1);
        let val2 = self.calculate(op2);
        let value = match operator {
            OP_ADD => val1 + val2,
            OP_SUB => val1 - val2,
            OP_MUL => val1 * val2,
            OP_DIV => val1 / val2,
            _ => panic!("Unknown operator"),
        };
        self.calculated[index] = Some(value);
        return value;
    }

    fn get_value(&self, index: usize) -> Option<u128> {
        if index == self.humn {
            return None;
        }

        let node = &self.nodes[index];

        if let NodeValue::Value(value) = node.value {
            return Some(value);
        }

        let Operation { op1, op2, operator } = node.get_operation();
        let val1 = self.get_value(op1);
        let val2 = self.get_value(op2);
        if val1.is_none() || val2.is_none() {
            return None;
        }
        let val1 = val1.unwrap();
        let val2 = val2.unwrap();

        let value = match operator {
            OP_ADD => val1 + val2,
            OP_SUB => val1 - val2,
            OP_MUL => val1 * val2,
            OP_DIV => val1 / val2,
            _ => panic!("Unknown operator"),
        };
        return Some(value);
    }

    fn get_expression(&self, current: usize, humn: usize) -> Expression {
        if current == humn {
            return Expression::Formula("humn".to_string());
        }
        let node = &self.nodes[current];
        if let NodeValue::Value(value) = node.value {
            return Expression::Value(value);
        }
        let Operation { op1, op2, operator } = node.get_operation();
        let exp1: Expression = self.get_expression(op1, humn);
        let exp2: Expression = self.get_expression(op2, humn);
        if let (Expression::Value(val1), Expression::Value(val2)) = (exp1.clone(), exp2.clone()) {
            let result = match operator {
                OP_ADD => val1 + val2,
                OP_SUB => val1 - val2,
                OP_MUL => val1 * val2,
                OP_DIV => val1 / val2,
                _ => panic!("Unknown operator"),
            };
            return Expression::Value(result);
        }
        let mut result = "(".to_owned();
        result.push_str(&exp1.get_string());
        result += match operator {
            OP_ADD => "+",
            OP_SUB => "-",
            OP_MUL => "*",
            OP_DIV => "/",
            _ => panic!("Unknown operator"),
        };
        result.push_str(&exp2.get_string());
        result.push_str(")");

        return Expression::Formula(result);
    }

    fn print_eq(&self, current: usize, humn: usize) {
        let node = &self.nodes[current];
        if current == humn {
            print!("humn");
            return;
        }
        if let NodeValue::Value(value) = node.value {
            print!("{}", value);
            return;
        }
        let Operation { op1, op2, operator } = node.get_operation();
        print!("(");
        self.print_eq(op1, humn);
        match operator {
            OP_ADD => print!("+"),
            OP_SUB => print!("-"),
            OP_MUL => print!("*"),
            OP_DIV => print!("/"),
            _ => panic!("Unknown operator"),
        }
        self.print_eq(op2, humn);
        print!(")");
    }

    fn solve_for(&self, current: usize, target: u128) -> u128 {
        if current == self.humn {
            return target;
        }
        let node = &self.nodes[current];
        if let NodeValue::Value(value) = node.value {
            panic!("Attempt to solve for constant")
        }
        let Operation { op1, op2, operator } = node.get_operation();
        let val1 = self.get_value(op1);
        if val1.is_some() {
            let val1 = val1.unwrap();
            let new_target = match operator {
                OP_ADD => target - val1, // target = val1 + new_target
                OP_SUB => val1 - target, // target = val1 - new_target
                OP_MUL => target / val1, // target = val1 * new_target
                OP_DIV => val1 / target, // target = val1 / new_target
                _ => panic!("Unknown operator"),
            };
            return self.solve_for(op2, new_target);
        }

        let val2 = self.get_value(op2).unwrap();
        let new_target = match operator {
            OP_ADD => target - val2, // target = new_target + val2
            OP_SUB => target + val2, // target = new_target - val2
            OP_MUL => target / val2, // target = new_target * val2
            OP_DIV => target * val2, // target = new_target / val2
            _ => panic!("Unknown operator"),
        };
        return self.solve_for(op1, new_target);
    }
}

fn main() {
    let mut graph: Graph = Graph::new();

    for line in std::io::stdin().lines() {
        let text: String = line.unwrap();

        // dbpl: 5
        // cczh: sllz + lgvd
        let re =
            regex::Regex::new(r"([a-z]+):\s+((\d+)|([a-z]+)\s+(\+|-|\*|/)\s+([a-z]+))").unwrap();
        let captures = re.captures(&text).unwrap();

        let result = match captures.get(3) {
            Some(number) => Node {
                name: captures[1].to_string(),
                value: NodeValue::Value(number.as_str().parse().unwrap()),
            },
            None => {
                let op1_index = graph.get_or_add(&captures[4]);
                let op2_index = graph.get_or_add(&captures[6]);
                let op = match &captures[5] {
                    "+" => OP_ADD,
                    "-" => OP_SUB,
                    "*" => OP_MUL,
                    "/" => OP_DIV,
                    _ => panic!("Unknown operator"),
                };
                Node {
                    name: captures[1].to_string(),
                    value: NodeValue::Operation(Operation {
                        op1: op1_index,
                        op2: op2_index,
                        operator: op,
                    }),
                }
            }
        };

        let index = graph.get_or_add(&captures[1]);
        graph.nodes[index] = result;
    }
    let first_part = false;
    if first_part {
        let root = graph.get_or_add("root");
        let result = graph.calculate(root);
        println!("Result {}", result);
    } else {
        let root = graph.get_or_add("root");
        graph.humn = graph.get_or_add("humn");
        let val2 = graph.get_value(graph.nodes[root].get_operation().op2);
        let solution = graph.solve_for(graph.nodes[root].get_operation().op1, val2.unwrap());
        println!("res: {}", solution);
    }
}
