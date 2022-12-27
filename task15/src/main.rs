use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;

fn merge_once(input: &Vec<(i32, i32)>) -> (bool, Vec<(i32, i32)>) {
    let mut res: Vec<(i32, i32)> = Vec::new();
    let mut merged: Vec<bool> = vec![false; input.len()];
    let mut did_something: bool = false;
    for i in 0..input.len() {
        if merged[i] {
            continue;
        }
        let (mut ox1, mut ox2) = input[i];
        for j in 0..input.len() {
            if i == j {
                continue;
            }
            if merged[j] {
                continue;
            }

            let (nx1, nx2) = input[j];
            if nx1 > ox2 || ox1 > nx2 {
                continue;
            }
            ox1 = min(nx1, ox1);
            ox2 = max(nx2, ox2);
            merged[j] = true;
            did_something |= true;
        }
        res.push((ox1, ox2));
    }

    return (did_something, res);
}

fn merge_all(initial: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut current: Vec<(i32, i32)> = initial;
    loop {
        let (did_something, output) = merge_once(&current);
        current = output;
        if !did_something {
            break;
        }
    }

    return current;
}

fn get_ranges(
    sensors: &Vec<(i32, i32, i32, i32)>,
    target_y: i32,
    exclude_beacons: bool,
) -> Vec<(i32, i32)> {
    let mut ranges: Vec<(i32, i32)> = Vec::new();
    for &(sx, sy, bx, by) in sensors {
        let distance = (bx - sx).abs() + (by - sy).abs();
        let d = distance - (target_y - sy).abs();
        if d < 0 {
            continue;
        }
        let mut nx1 = sx - d;
        let mut nx2 = sx + d;

        if by == target_y && exclude_beacons {
            if d == 0 {
                continue;
            } else {
                if bx < sx {
                    nx1 += 1;
                } else {
                    nx2 -= 1;
                }
            }
        }
        ranges.push((nx1, nx2));
        ranges = merge_all(ranges);
    }
    return ranges;
}

fn main() {
    let first_part = false;
    let mut sensors: Vec<(i32, i32, i32, i32)> = Vec::new();

    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();

        let (sx, sy, bx, by) = line
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        sensors.push((sx, sy, bx, by));
    }

    if first_part {
        const TARGET_Y: i32 = 2000000;
        let ranges = get_ranges(&sensors, TARGET_Y, true);
        let mut total = 0;
        for (x1, x2) in ranges {
            total += x2 - x1 + 1;
        }
        println!("Result {total}");
    } else {
        const LIMIT: i32 = 4_000_000;

        for y in 0..=LIMIT {
            let ranges = get_ranges(&sensors, y, false);
            if ranges.len() > 1 {
                let x: i128 = ranges[0].1 as i128 + 1;
                println!("{:?}", x * (LIMIT as i128) + y as i128);
            }
        }
    }
}
