fn includes(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    x1 >= y1 && x1 <= y2 && x2 >= y1 && x2 <= y2
}

fn includes_total(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    includes(x1, x2, y1, y2) || includes(y1, y2, x1, x2)
}

fn overlaps(x1: i32, x2: i32, y1: i32, y2: i32) -> bool {
    // (x1 >= y1 && x1 <= y2) || (x2 >= y1 && x2 <= y2)
    !(x2 < y1 || y2 < x1)
}

fn main() {
    let mut score = 0;
    let lines = std::io::stdin().lines();
    for line in lines {
        let line: String = line.unwrap();
        let v: Vec<i32> = line
            .split(&['-', ','][..])
            .map(|x| x.parse().unwrap())
            .collect();
        if overlaps(v[0], v[1], v[2], v[3]) {
            score += 1;
        }
    }
    println!("Score {score}");
}
