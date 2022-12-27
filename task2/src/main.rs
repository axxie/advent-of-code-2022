// A - rock - 1
// B - paper - 2
// C - scissors - 3

fn main() {
    let lines = std::io::stdin().lines();
    let mut score = 0;
    for line in lines {
        let line: String = line.unwrap();
        let ch1 = line.chars().next().unwrap();
        let ch2 = line.chars().nth(2).unwrap();
        match ch2 {
            'X' => match ch1 {
                'A' => score += 3,
                'B' => score += 1,
                'C' => score += 2,
                _ => (),
            },
            'Y' => {
                score += 3;
                match ch1 {
                    'A' => score += 1,
                    'B' => score += 2,
                    'C' => score += 3,
                        _ => (),
                }
            }
            'Z' => {
                score += 6;
                match ch1 {
                    'A' => score += 2,
                    'B' => score += 3,
                    'C' => score += 1,
                        _ => (),
                }
            }
            _ => panic!("wrong input"),
        }
    }
    println!("score: {score}")
}
