fn main() {
    let mut forest: Vec<Vec<u32>> = Vec::new();

    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let mut row: Vec<u32> = Vec::new();
        for digit in line.chars() {
            row.push((digit as u32) - ('0' as u32));
        }
        forest.push(row);
    }

    let height = forest.len();
    let width = forest[0].len();

    if false {
        let mut visible = vec![vec![false; width]; height];

        for y in 0..height {
            visible[y][0] = true;
            visible[y][width - 1] = true;
        }
        for x in 0..width {
            visible[0][x] = true;
            visible[height - 1][x] = true;
        }

        for y in 1..height - 1 {
            let mut min: u32 = forest[y][0];
            for x in 1..width {
                if forest[y][x] > min {
                    min = forest[y][x];
                    visible[y][x] = true;
                }
            }
        }

        for y in 1..height - 1 {
            let mut min: u32 = forest[y][width - 1];
            for x in (1..width).rev() {
                if forest[y][x] > min {
                    min = forest[y][x];
                    visible[y][x] = true;
                }
            }
        }

        for x in 1..width - 1 {
            let mut min: u32 = forest[0][x];
            for y in 1..height {
                if forest[y][x] > min {
                    min = forest[y][x];
                    visible[y][x] = true;
                }
            }
        }

        for x in 1..width - 1 {
            let mut min: u32 = forest[height - 1][x];
            for y in (1..height).rev() {
                if forest[y][x] > min {
                    min = forest[y][x];
                    visible[y][x] = true;
                }
            }
        }

        let mut count = 0;
        for y in 0..height {
            for x in 0..width {
                if visible[y][x] {
                    count += 1;
                    print!("x");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!("Result: {count}");
    }

    let mut left_distance = vec![vec![0; width]; height];
    for y in 0..height {
        let mut stack: Vec<(u32, usize)> = Vec::new();
        for x in 0..width {
            let current_val = forest[y][x];
            let current_index = x;
            left_distance[y][x] = get_distance(current_val, current_index, &mut stack);
        }
    }

    let mut right_distance = vec![vec![0; width]; height];
    for y in 0..height {
        let mut stack: Vec<(u32, usize)> = Vec::new();
        for x in (0..width).rev() {
            let current_val = forest[y][x];
            let current_index = width - 1 - x;
            right_distance[y][x] = get_distance(current_val, current_index, &mut stack);
        }
    }
    let mut top_distance = vec![vec![0; width]; height];
    for x in 0..width {
        let mut stack: Vec<(u32, usize)> = Vec::new();
        for y in 0..height {
            let current_val = forest[y][x];
            let current_index = y;
            top_distance[y][x] = get_distance(current_val, current_index, &mut stack);
        }
    }

    let mut bottom_distance = vec![vec![0; width]; height];
    for x in 0..width {
        let mut stack: Vec<(u32, usize)> = Vec::new();
        for y in (0..height).rev() {
            let current_val = forest[y][x];
            let current_index = height - 1 - y;
            bottom_distance[y][x] = get_distance(current_val, current_index, &mut stack);
        }
    }

    let mut max = 0;
    for y in 0..height {
        for x in 0..width {
            let current = left_distance[y][x]
                * right_distance[y][x]
                * top_distance[y][x]
                * bottom_distance[y][x];
            if max < current {
                max = current;
            }
        }
    }

    println!("Result: {max}");
}

fn get_distance(current_val: u32, current_index: usize, stack: &mut Vec<(u32, usize)>) -> usize {
    loop {
        if let None = stack.last() {
            stack.push((current_val, current_index));
            return current_index;
        }

        let (ref mut top_val, ref mut top_index) = stack.last_mut().unwrap();

        if *top_val < current_val {
            stack.pop();
        } else {
            let last_index = *top_index;

            if *top_val == current_val {
                *top_index = current_index;
            } else {
                stack.push((current_val, current_index));
            }
            return current_index - last_index;
        }
    }
}
