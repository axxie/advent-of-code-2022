use std::collections::{HashSet, VecDeque};

const UP: usize = 0;
const RIGHT: usize = 1;
const DOWN: usize = 2;
const LEFT: usize = 3;
const BLZ_TYPES: usize = 4;
const SPACE: usize = 4;
const WALL: usize = 5;

type Positions = HashSet<(i32, i32)>;

static DX: [i32; 5] = [0, 0, 0, -1, 1]; // .↑↓←→
static DY: [i32; 5] = [0, -1, 1, 0, 0]; // .↑↓←→

struct Blizzards {
    width: usize,
    height: usize,
    field: Vec<VecDeque<bool>>,
}

impl Blizzards {
    fn new(width: usize, height: usize, field: &Vec<Vec<usize>>) -> Self {
        let mut result = Blizzards {
            width: width - 2,
            height: height - 2,
            field: Vec::new(),
        };
        let total = result.width * result.height;
        for _ in 0..BLZ_TYPES {
            result.field.push(VecDeque::with_capacity(total));
            result.field.last_mut().unwrap().resize(total, false);
        }
        result.fill(field);
        result
    }

    fn fill(self: &mut Self, field: &Vec<Vec<usize>>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = field[y + 1][x + 1];
                if cell < BLZ_TYPES {
                    match cell {
                        UP | DOWN => self.field[cell][y * self.width + x] = true,
                        LEFT | RIGHT => self.field[cell][x * self.height + y] = true,
                        _ => panic!("Unknown bliz_type"),
                    };
                }
            }
        }
    }

    fn step(self: &mut Self) {
        self.field[UP].rotate_left(self.width);
        self.field[DOWN].rotate_right(self.width);
        self.field[LEFT].rotate_left(self.height);
        self.field[RIGHT].rotate_right(self.height);
    }

    fn get(self: &Self, x: usize, y: usize, bliz_type: usize) -> bool {
        match bliz_type {
            UP | DOWN => self.field[bliz_type][y * self.width + x],
            LEFT | RIGHT => self.field[bliz_type][x * self.height + y],
            _ => panic!("Unknown bliz_type"),
        }
    }

    fn is_something_at(self: &Self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.get(x, y, UP) || self.get(x, y, DOWN) || self.get(x, y, LEFT) || self.get(x, y, RIGHT)
    }

    #[allow(dead_code)]
    fn print(self: &Self) {
        for _ in 0..self.width + 2 {
            print!("#");
        }
        println!();
        for y in 0..self.height {
            print!("#");
            for x in 0..self.width {
                if self.get(x, y, UP) {
                    print!("^");
                } else if self.get(x, y, DOWN) {
                    print!("v");
                } else if self.get(x, y, LEFT) {
                    print!("<");
                } else if self.get(x, y, RIGHT) {
                    print!(">");
                } else {
                    print!(" ");
                }
            }
            println!("#");
        }
        for _ in 0..self.width + 2 {
            print!("#");
        }
        println!();
    }
}

fn get_distance(
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
    blizzards: &mut Blizzards,
    field: &Vec<Vec<usize>>,
) -> i32 {
    let height = field.len();
    let mut current = Positions::new();
    let mut next = Positions::new();
    let mut iterations = 0;
    current.insert((start_x, start_y));
    loop {
        iterations += 1;
        blizzards.step();
        for &(x, y) in &current {
            for new_pos_index in 0..DX.len() {
                let new_x = x + DX[new_pos_index];
                let new_y = y + DY[new_pos_index];
                if new_x == end_x && new_y == end_y {
                    return iterations;
                }
                if new_y < 0 || new_y >= height as i32 {
                    // we can exit the map only via start or end point and that would be vertical exit
                    continue;
                }
                if field[new_y as usize][new_x as usize] == WALL {
                    continue;
                }
                if new_y == 0 {
                    // we keep staying in the starting position
                    next.insert((new_x, new_y));
                    continue;
                }
                if blizzards.is_something_at((new_x - 1) as usize, (new_y - 1) as usize) {
                    continue;
                }
                next.insert((new_x, new_y));
            }
        }
        std::mem::swap(&mut current, &mut next);
        next.clear();
    }
}

fn main() {
    let first = false;
    let mut field: Vec<Vec<usize>> = Vec::new();
    for line in std::io::stdin().lines() {
        let text: String = line.unwrap();
        let mut row: Vec<usize> = Vec::new();

        for c in text.chars() {
            let cell = match c {
                '.' => SPACE,
                '#' => WALL,
                '^' => UP,
                '>' => RIGHT,
                'v' => DOWN,
                '<' => LEFT,
                _ => panic!("Incorrect input"),
            };
            row.push(cell);
        }
        field.push(row);
    }

    let height = field.len();
    let width = field[0].len();

    // Find start and end position
    let mut start_x: i32 = 0;
    let start_y: i32 = 0;
    let mut end_x: i32 = 0;
    let end_y: i32 = (height - 1) as i32;
    for x in 0..width {
        if field[0][x] == SPACE {
            start_x = x as i32;
        }
        if field[height - 1][x] == SPACE {
            end_x = x as i32;
        }
    }

    let mut blizzards: Blizzards = Blizzards::new(width, height, &field);
    let forward_trip = get_distance(start_x, start_y, end_x, end_y, &mut blizzards, &field);

    if first {
        println!("Result {forward_trip}");
    } else {
        let backward_trip = get_distance(end_x, end_y, start_x, start_y, &mut blizzards, &field);
        let another_trip = get_distance(start_x, start_y, end_x, end_y, &mut blizzards, &field);
        println!("Result {}", forward_trip + backward_trip + another_trip);
    }
}
