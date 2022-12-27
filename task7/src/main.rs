use std::collections::HashMap;

/*
enum Node {
    File {
        size: u32,
    },
    Dir {
        size: u32,
        children: HashMap<String, Node>,
    },
}

fn main() {

    // let mut h: HashMap<String, i32> = HashMap::new();
    // h.insert("a".to_string(), 1);
    // h.insert("b".to_string(), 2);
    // let mut r1: &mut i32 = h.get_mut(&"a".to_string()).unwrap();
    // *r1 = 15;
    // r1 = h.get_mut(&"b".to_string()).unwrap();



    let mut root: Node = Node::Dir {
        size: 0,
        children: HashMap::new(),
    };
    let mut in_ls: bool = false;
    let mut current: &mut Node = &mut root;
    let mut current_temp: &mut Node;
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let splat: Vec<&str> = line.split(' ').collect();
        let prefix: &str = splat[0];
        if in_ls && prefix == "$" {
            in_ls = false;
        }

        if in_ls {
            let dir_or_size: &str = splat[0];
            let name: &str = splat[1];
            if let Node::Dir {
                ref mut children, ..
            } = current
            {
                let node: Node;
                if dir_or_size == "dir" {
                    node = Node::Dir {
                        size: 0,
                        children: HashMap::new(),
                    };
                } else {
                    let size: u32 = dir_or_size.parse().unwrap();
                    node = Node::File { size };
                }
                children.insert(name.to_string(), node);
            }
        } else {
            let command: &str = splat[1];
            if command == "ls" {
                in_ls = true;
            } else if command == "cd" {
                let dir_name: &str = splat[2];
                if dir_name == ".." {
                } else {
                    if let Node::Dir {
                        ref mut children, ..
                    } = current
                    {
                        if let Some(subdir) = children.get_mut(dir_name) {
                            // drop(current);
                            // current = subdir;
                        }
                    }
                }
            }
        }
    }
    println!("Result:");
}
*/

enum Node {
    File {
        size: u32,
    },
    Dir {
        size: u32,
        children: HashMap<String, usize>,
    },
}

fn main() {
    let mut pool: Vec<Node> = vec![Node::Dir {
        size: 0,
        children: HashMap::new(),
    }];
    let mut stack: Vec<usize> = Vec::new();
    let root: usize = 0;

    let mut in_ls: bool = false;
    let mut current: usize = root;
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let splat: Vec<&str> = line.split(' ').collect();
        let prefix: &str = splat[0];
        if in_ls && prefix == "$" {
            in_ls = false;
        }

        if in_ls {
            let dir_or_size: &str = splat[0];

            let node: Node;
            let mut node_size: u32 = 0;
            if dir_or_size == "dir" {
                node = Node::Dir {
                    size: 0,
                    children: HashMap::new(),
                };
            } else {
                node_size = dir_or_size.parse().unwrap();
                node = Node::File { size: node_size };
            }
            let new_index = pool.len();
            pool.push(node);

            // if let Node::Dir {
            //     ref mut children, ..
            // } = pool[current]
            // {
            //     let name: &str = splat[1];
            //     children.insert(name.to_string(), new_index);
            // }

            if let Node::Dir {
                ref mut size,
                ref mut children,
            } = pool[current]
            {
                let name: &str = splat[1];
                children.insert(name.to_string(), new_index);
                *size += node_size;
            }
            for parent in &stack {
                if let Node::Dir { ref mut size, .. } = pool[*parent] {
                    *size += node_size;
                }
            }
        } else {
            let command: &str = splat[1];
            if command == "ls" {
                in_ls = true;
            } else if command == "cd" {
                let dir_name: &str = splat[2];
                if dir_name == ".." {
                    current = stack.pop().unwrap();
                } else if dir_name != "/" {
                    if let Node::Dir { ref children, .. } = pool[current] {
                        if let Some(subdir) = children.get(dir_name) {
                            stack.push(current);
                            current = *subdir;
                        } else {
                            panic!();
                        }
                    }
                }
            }
        }
    }

    // let mut sum = 0;
    // for node in &pool {
    //     if let Node::Dir { size, .. } = *node {
    //         if size <= 100000 {
    //             sum += size;
    //         }
    //     }
    // }

    // smallest bigger than 913445
    let mut min = 70000000;
    for node in &pool {
        if let Node::Dir { size, .. } = *node {
            if size >= 913445 && size <= min {
                min = size;
            }
        }
    }

    println!("Result:");
}
