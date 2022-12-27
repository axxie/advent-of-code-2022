fn main() {
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['B', 'Z', 'T'],
        vec!['V', 'H', 'T', 'D', 'N'],
        vec!['B', 'F', 'M', 'D'],
        vec!['T', 'J', 'G', 'W', 'V', 'Q', 'L'],
        vec!['W', 'D', 'G', 'P', 'V', 'F', 'Q', 'M'],
        vec!['V', 'Z', 'Q', 'G', 'H', 'F', 'S'],
        vec!['Z', 'S', 'N', 'R', 'L', 'T', 'C', 'W'],
        vec!['Z', 'H', 'W', 'D', 'J', 'N', 'R', 'M'],
        vec!['M', 'Q', 'L', 'F', 'D', 'S'],
    ];
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let v: Vec<&str> = line.split(' ').collect();

        let amount: usize = v[1].parse().unwrap();
        let from: usize = v[3].parse().unwrap();
        let to: usize = v[5].parse().unwrap();
        let at = stacks[from - 1].len() - amount;
        let mut cut = stacks[from - 1].split_off(at);
        stacks[to - 1].append(&mut cut);
        // for _ in 0..amount {
        //     let value: char = stacks[from-1].pop().unwrap();
        //     stacks[to-1].push(value);
        // }
    }
    println!("Stacks: {:?}", stacks);
}
