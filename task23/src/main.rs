use core::panic;
use std::cmp::max;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufRead;

const SIZE: usize = 20;

const SPACE: u8 = 0;
const ELF: u8 = 1;

static CX: [[i32; 3]; 4] = [[-1, 0, 1], [-1, 0, 1], [-1, -1, -1], [1, 1, 1]];
static CY: [[i32; 3]; 4] = [[-1, -1, -1], [1, 1, 1], [-1, 0, 1], [-1, 0, 1]];
static DX: [i32; 4] = [0, 0, -1, 1]; // ↑↓←→
static DY: [i32; 4] = [-1, 1, 0, 0]; // ↑↓←→

fn need_to_move(elves: &BTreeSet<(i32, i32)>, x: i32, y: i32) -> bool {
    let mut result = false;
    for ix in -1..=1 {
        for iy in -1..=1 {
            let n_pos = (x + ix, y + iy);
            if n_pos != (x, y) && elves.contains(&n_pos) {
                result = true;
                break;
            }
        }
        if result {
            break;
        }
    }
    result
}

fn get_cand_position(
    elves: &BTreeSet<(i32, i32)>,
    x: i32,
    y: i32,
    direction_start: usize,
) -> (i32, i32) {
    let mut cand_x = x;
    let mut cand_y = y;
    for dir_offset in 0..4 {
        let direction = (direction_start + dir_offset) % 4;
        let mut all_free = true;
        for i in 0..3 {
            let check_x = x + CX[direction][i];
            let check_y = y + CY[direction][i];
            if elves.contains(&(check_x, check_y)) {
                all_free = false;
                break;
            }
        }
        if !all_free {
            continue; // try another direction
        }
        cand_x = x + DX[direction];
        cand_y = y + DY[direction];
        break;
    }
    (cand_x, cand_y)
}

fn main() {
    let first = false;
    let mut field: Vec<Vec<u8>> = Vec::new();
    for line in std::io::stdin().lines() {
        let text: String = line.unwrap();
        let mut row: Vec<u8> = Vec::new();

        for c in text.chars() {
            let cell: u8 = match c {
                '.' => SPACE,
                '#' => ELF,
                _ => panic!("Incorrect input"),
            };
            row.push(cell);
        }
        field.push(row);
    }
    let height = field.len();
    let width = field[0].len();

    let mut elves: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut new_elves: BTreeSet<(i32, i32)> = BTreeSet::new();

    for y in 0..height {
        for x in 0..width {
            if field[y][x] == ELF {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let mut direction_start = 0;
    let mut iterations = 0;
    loop {
        let mut candidates: BTreeMap<(i32, i32), bool> = BTreeMap::new();
        for &(x, y) in &elves {
            if !need_to_move(&elves, x, y) {
                continue;
            }
            let (cand_x, cand_y) = get_cand_position(&elves, x, y, direction_start);

            match candidates.entry((cand_x, cand_y)) {
                Entry::Occupied(o) => {
                    *(o.into_mut()) = false;
                }
                Entry::Vacant(v) => {
                    v.insert(true);
                }
            }
        }

        for &(x, y) in &elves {
            if !need_to_move(&elves, x, y) {
                new_elves.insert((x, y));
                continue;
            }
            let (mut cand_x, mut cand_y) = get_cand_position(&elves, x, y, direction_start);
            match candidates.get(&(cand_x, cand_y)) {
                None => panic!("Missing candidate position"),
                Some(true) => (),
                Some(false) => {
                    cand_x = x;
                    cand_y = y
                }
            }
            new_elves.insert((cand_x, cand_y));
        }
        if first {
            if iterations == 10 {
                break;
            }
        } else {
            if elves == new_elves {
                iterations += 1;
                break;
            }
        }
        elves = new_elves;
        new_elves = BTreeSet::new();
        direction_start = (direction_start + 1) % 4;
        iterations += 1;
    }

    if first {
        let first_elf = elves.iter().next().unwrap();
        let mut min_x: i32 = first_elf.0;
        let mut max_x: i32 = first_elf.0;
        let mut min_y: i32 = first_elf.1;
        let mut max_y: i32 = first_elf.1;

        for &(x, y) in &elves {
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
        let answer = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32;
        println!("Result {answer}");
    } else {
        println!("Result {iterations}",);
    }
}
