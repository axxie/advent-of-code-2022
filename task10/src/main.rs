fn draw(cycle: i32, x: i32) {
    if (x - (cycle % 40 - 1)).abs() <= 1 {
        print!("#");
    } else {
        print!(" ");
    }

    if cycle % 40 == 0 {
        println!();
    }
}

fn main() {
    let mut cycle = 0;
    let mut x: i32 = 1;
    let mut strength: u32 = 0;
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let v: Vec<&str> = line.split(' ').collect();
        if false {
            if v[0] == "noop" {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    strength += (x * cycle) as u32;
                }
            } else {
                let value: i32 = v[1].parse().unwrap();
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    strength += (x * cycle) as u32;
                }
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    strength += (x * cycle) as u32;
                }
                x += value;
            }
        }

        if v[0] == "noop" {
            cycle += 1;
            draw(cycle, x);
        } else {
            cycle += 1;
            draw(cycle, x);
            cycle += 1;
            draw(cycle, x);
            let value: i32 = v[1].parse().unwrap();
            x += value;
        }
    }
    println!("Result: {strength}");
}
