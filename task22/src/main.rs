use core::panic;
use std::cmp::max;
use std::io::BufRead;

const EDGE: u8 = 0;
const SPACE: u8 = 1;
const WALL: u8 = 2;

static DX: [i32; 4] = [1, 0, -1, 0]; // →↓←↑
static DY: [i32; 4] = [0, 1, 0, -1]; // →↓←↑

enum Command {
    Left,
    Right,
    Forward(u32),
}

fn main() {
    let first = true;
    let mut field: Vec<Vec<u8>> = vec![vec![EDGE]];
    let mut lines = std::io::stdin().lock().lines();
    loop {
        let text: String = lines.next().unwrap().unwrap();

        if text.is_empty() {
            break; // next line is commands
        }
        let mut row: Vec<u8> = vec![EDGE];

        for c in text.chars() {
            let cell: u8 = match c {
                ' ' => EDGE,
                '.' => SPACE,
                '#' => WALL,
                _ => 0,
            };
            row.push(cell);
        }
        row.push(EDGE);
        field.push(row);
    }
    field.push(vec![EDGE, 0]);

    let commands_text = lines.next().unwrap().unwrap();
    let mut commands: Vec<Command> = Vec::new();
    let mut current_number = String::new();
    for c in commands_text.chars() {
        if c == 'R' {
            commands.push(Command::Forward(current_number.parse().unwrap()));
            current_number.truncate(0);
            commands.push(Command::Right);
        } else if c == 'L' {
            commands.push(Command::Forward(current_number.parse().unwrap()));
            current_number.truncate(0);
            commands.push(Command::Left);
        } else {
            current_number.push(c);
        }
    }
    if !current_number.is_empty() {
        commands.push(Command::Forward(current_number.parse().unwrap()));
    }

    let width = field
        .iter()
        .max_by(|&a, &b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    let height = field.len();
    for y in 0..height {
        let row: &mut Vec<u8> = &mut field[y];
        row.resize(width, EDGE);
    }

    let mut wrap: Vec<Vec<(usize, usize, i32)>> = vec![vec![(0, 0, 0); width]; height];
    let mut x_begin: usize = 0;

    if first {
        for y in 0..height {
            let mut start_x: Option<usize> = None;
            for x in 0..width {
                if field[y][x] != EDGE && start_x.is_none() {
                    if x_begin == 0 {
                        x_begin = x;
                    }
                    start_x = Some(x);
                } else if field[y][x] == EDGE && start_x.is_some() {
                    wrap[y][start_x.unwrap() - 1] = (x - 1, y, 0);
                    wrap[y][x] = (start_x.unwrap(), y, 0);
                    break;
                }
            }
        }

        for x in 0..width {
            let mut start_y: Option<usize> = None;
            for y in 0..height {
                if field[y][x] != EDGE && start_y.is_none() {
                    start_y = Some(y);
                } else if field[y][x] == EDGE && start_y.is_some() {
                    wrap[start_y.unwrap() - 1][x] = (x, y - 1, 0);
                    wrap[y][x] = (x, start_y.unwrap(), 0);
                    break;
                }
            }
        }
    } else {
        x_begin = 51;
        // silly hardcode :(
        for i in 1..=50 {
            wrap[0][i + 50] = (1, i + 150, 1);
            wrap[i + 150][0] = (i + 50, 1, -1);

            wrap[0][i + 100] = (i, 200, 0);
            wrap[201][i] = (i + 100, 1, 0);

            wrap[i][151] = (100, 100 + (51 - i), 2);
            wrap[100 + (51 - i)][101] = (150, i, 2);

            wrap[51][i + 100] = (100, i + 50, 1);
            wrap[i + 50][101] = (i + 100, 50, -1);

            wrap[151][i + 50] = (50, i + 150, 1);
            wrap[i + 150][51] = (i + 50, 150, -1);

            wrap[i][50] = (1, 100 + (51 - i), 2);
            wrap[i + 100][0] = (51, 51 - i, 2);

            wrap[i + 50][50] = (i, 101, -1);
            wrap[100][i] = (51, i + 50, 1);
        }
        wrap[51][101] = (0, 0, 0);
        wrap[100][50] = (0, 0, 0);
        wrap[151][51] = (0, 0, 0);
    }

    let mut x: usize = x_begin;
    let mut y: usize = 1;
    let mut direction: usize = 0;
    for command in commands {
        match command {
            Command::Left => direction = (direction + 4 - 1) % 4,
            Command::Right => direction = (direction + 1) % 4,
            Command::Forward(distance) => {
                for _ in 0..distance {
                    let mut new_x = (x as i32 + DX[direction]) as usize;
                    let mut new_y = (y as i32 + DY[direction]) as usize;
                    let cell = field[new_y][new_x];
                    if cell == SPACE {
                        x = new_x;
                        y = new_y;
                        continue;
                    } else if cell == WALL {
                        continue;
                    } else {
                        // EDGE
                        let (mut wrapped_x, mut wrapped_y, mut dir_change) = wrap[new_y][new_x];
                        if new_x == 50 && new_y == 100 {
                            if direction == 3 {
                                wrapped_x = 51;
                                wrapped_y = 100;
                                dir_change = 1;
                            } else {
                                wrapped_x = 50;
                                wrapped_y = 101;
                                dir_change = -1;
                            }
                        } else if new_x == 101 && new_y == 51 {
                            if direction == 0 {
                                wrapped_x = 101;
                                wrapped_y = 50;
                                dir_change = -1;
                            } else {
                                wrapped_x = 100;
                                wrapped_y = 51;
                                dir_change = 1;
                            }
                        }
                        if wrapped_x == 0 || wrapped_y == 0 {
                            panic!("Wrong position of the edge");
                        }
                        let new_cell = field[wrapped_y][wrapped_x];
                        if new_cell == EDGE {
                            panic!("Wrapping to the edge");
                        } else if new_cell == WALL {
                            continue;
                        } else {
                            x = wrapped_x;
                            y = wrapped_y;
                            direction = ((direction as i32 + 4 + dir_change) % 4) as usize;
                        }
                    }
                }
            }
        }
    }

    let result = 1000 * y + x * 4 + direction;
    println!("Result {result}");
}
