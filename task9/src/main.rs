use itertools::Itertools;

fn get_dir(input: &str) -> usize {
    return match input {
        "L" => 0,
        "R" => 1,
        "U" => 2,
        "D" => 3,
        _ => panic!(),
    };
}

fn main() {
    const SIZE: i32 = 2000;
    const KNOTS: usize = 10;
    let mut visited = vec![vec![false; SIZE as usize]; SIZE as usize];

    let mut x: [i32; KNOTS] = [SIZE / 2; KNOTS];
    let mut y: [i32; KNOTS] = [SIZE / 2; KNOTS];
    let d_x: [i32; 4] = [-1, 1, 0, 0];
    let d_y: [i32; 4] = [0, 0, -1, 1];
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let (dir_str, dist_str) = line.split(' ').collect_tuple().unwrap();
        let distance: u32 = dist_str.parse().unwrap();
        let direction: usize = get_dir(dir_str);

        for _ in 0..distance {
            x[0] += d_x[direction];
            y[0] += d_y[direction];

            for i in 1..KNOTS {
                let o_x = x[i].abs_diff(x[i - 1]);
                let o_y = y[i].abs_diff(y[i - 1]);

                if o_x > 1 {
                    if x[i] > x[i - 1] {
                        x[i] -= 1;
                    } else {
                        x[i] += 1;
                    }
                    if o_y > 0 {
                        if y[i] > y[i - 1] {
                            y[i] -= 1;
                        } else {
                            y[i] += 1;
                        }
                    }
                } else if o_y > 1 {
                    if y[i] > y[i - 1] {
                        y[i] -= 1;
                    } else {
                        y[i] += 1;
                    }
                    if o_x > 0 {
                        if x[i] > x[i - 1] {
                            x[i] -= 1;
                        } else {
                            x[i] += 1;
                        }
                    }
                }
            }

            visited[y[KNOTS - 1] as usize][x[KNOTS - 1] as usize] = true;
            
            if false {
                let mut map = vec![vec![50; SIZE as usize]; SIZE as usize];
                for i in 0..KNOTS {
                    map[y[i] as usize][x[i] as usize] = i;
                }

                for y in 0..SIZE as usize {
                    for x in 0..SIZE as usize {
                        if map[y][x] == 50 {
                            print!(".");
                        } else {
                            print!("{}", map[y][x]);
                        }
                    }
                    println!();
                }
            }
        }
    }

    let mut count = 0;
    for y in 0..SIZE {
        for x in 0..SIZE {
            if visited[y as usize][x as usize] {
                count += 1;
            }
        }
    }

    println!("Result: {count}");
}
