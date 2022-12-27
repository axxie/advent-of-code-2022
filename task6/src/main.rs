use std::collections::VecDeque;
fn main() {
    let line: String = std::io::stdin().lines().next().unwrap().unwrap();
    let mut index = 0;
    let mut buf: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        index += 1;
        buf.push_back(c);
        if buf.len() > 14 {
            buf.pop_front();
            let mut duplicate: bool = false;
            for i in 0..14 {
                for j in (i + 1)..14 {
                    if buf.get(i) == buf.get(j) {
                        duplicate = true;
                        break;
                    }
                }
                if duplicate {
                    break;
                }
            }
            if !duplicate {
                break;
            }
        }
    }
    println!("Index: {index}",);
}
