use std::collections::VecDeque;
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let d_x: [i32; 4] = [-1, 1, 0, 0];
    let d_y: [i32; 4] = [0, 0, -1, 1];
    let mut heights: Vec<Vec<u32>> = Vec::new();
    let mut start: Point = Point { x: 0, y: 0 };
    let mut end: Point = Point { x: 0, y: 0 };

    let mut y: usize = 0;
    let mut x: usize;
    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();
        let mut row: Vec<u32> = Vec::new();
        x = 0;
        for mut letter in line.chars() {
            if letter == 'S' {
                letter = 'a';
                start.x = x;
                start.y = y;
            } else if letter == 'E' {
                letter = 'z';
                end.x = x;
                end.y = y;
            }

            row.push((letter as u32) - ('a' as u32));
            x += 1;
        }
        heights.push(row);
        y += 1;
    }

    let width: i32 = heights[0].len() as i32;
    let height: i32 = heights.len() as i32;
    let mut distance: Vec<Vec<u32>> = vec![vec![10000; width as usize]; height as usize];

    let mut queue: VecDeque<Point> = VecDeque::new();
    distance[end.y][end.x] = 0;
    queue.push_back(end);
    while !queue.is_empty() {
        let current: Point = queue.pop_front().unwrap();
        for i in 0..4 {
            let new_x_signed: i32 = current.x as i32 + d_x[i];
            let new_y_signed: i32 = current.y as i32 + d_y[i];
            if new_x_signed < 0
                || new_x_signed >= width
                || new_y_signed < 0
                || new_y_signed >= height
            {
                continue;
            }
            let new_x: usize = new_x_signed as usize;
            let new_y: usize = new_y_signed as usize;
            if new_x == 78 && new_y == 17 {
                let a = 0;
            }
            // println!("{new_x} {new_y}");
            if heights[new_y][new_x] + 1 < heights[current.y][current.x] {
                continue;
            }
            if distance[new_y][new_x] <= distance[current.y][current.x] + 1 {
                continue;
            }
            distance[new_y][new_x] = distance[current.y][current.x] + 1;

            let next: Point = Point {
                x: new_x as usize,
                y: new_y as usize,
            };
            queue.push_back(next);
        }
    }

    let first_part = false;
    if first_part {
        println!("Result: {}", distance[start.y][start.x]);
    } else {
        let mut min: u32 = 10000;
        for y in 0..height as usize {
            for x in 0..width as usize {
                if heights[y][x] == 0 {
                    if min > distance[y][x] {
                        min = distance[y][x];
                    }
                }
            }
        }
        println!("Result: {}", min);
    }
}
