use itertools::Itertools;
use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    let first_part = false;
    let mut cubes = vec![vec![vec![0u8; 26]; 26]; 26];
    let mut area = 0;
    let dx = [-1, 1, 0, 0, 0, 0];
    let dy = [0, 0, -1, 1, 0, 0];
    let dz = [0, 0, 0, 0, -1, 1];

    if first_part {
        for line in std::io::stdin().lines() {
            let line: String = line.unwrap();
            let (mut x, mut y, mut z) = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            x += 1;
            y += 1;
            z += 1;
            if area == 0 {
                cubes[z][y][x] = 1;
                area = 6;
                continue;
            }

            for i in 0..dx.len() {
                let nx = (x as i32 + dx[i]) as usize;
                let ny = (y as i32 + dy[i]) as usize;
                let nz = (z as i32 + dz[i]) as usize;
                if cubes[nz][ny][nx] == 1 {
                    area -= 1;
                } else {
                    area += 1;
                }
            }
            cubes[z][y][x] = 1;
        }

        println!("Result {area}");
    } else {
        for line in std::io::stdin().lines() {
            let line: String = line.unwrap();
            let (x, y, z) = line
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            cubes[z + 2][y + 2][x + 2] = 1;
        }
        let top = cubes.len() - 1;
        for i in 0..cubes.len() {
            for j in 0..cubes.len() {
                cubes[0][i][j] = 2;
                cubes[top][i][j] = 2;
                cubes[i][0][j] = 2;
                cubes[i][top][j] = 2;
                cubes[i][j][0] = 2;
                cubes[i][j][top] = 2;
            }
        }

        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((top - 1, top - 1, top - 1));
        while !queue.is_empty() {
            let (x, y, z) = queue.pop_front().unwrap();

            for i in 0..dx.len() {
                let nx = (x as i32 + dx[i]) as usize;
                let ny = (y as i32 + dy[i]) as usize;
                let nz = (z as i32 + dz[i]) as usize;
                if cubes[nz][ny][nx] == 0 {
                    queue.push_back((nx, ny, nz));
                    cubes[nz][ny][nx] = 3;
                } else if cubes[nz][ny][nx] == 1 {
                    area += 1;
                }
            }
        }
        println!("Result: {area}");
    }
}
