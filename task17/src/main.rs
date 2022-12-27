use std::cmp::max;

static HBAR: &str = "
####";
static CROSS: &str = "
.#.
###
.#.";
static ANGLE: &str = "
..#
..#
###";
static VBAR: &str = "
#
#
#
#";
static SQUARE: &str = "
##
##";

struct Shape {
    positions: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

fn get_shape(input: &str) -> Shape {
    let mut result: Shape = Shape {
        positions: Vec::new(),
        width: 0,
        height: 0,
    };
    for (y, line) in input.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.positions.push((x, y - 1));
                result.width = max(result.width, x + 1);
                result.height = max(result.height, y);
            }
        }
    }
    return result;
}

static SHAPES: [&str; 5] = [HBAR, CROSS, ANGLE, VBAR, SQUARE];

fn get_shapes() -> Vec<Shape> {
    let mut result: Vec<Shape> = Vec::new();
    for shape in SHAPES {
        result.push(get_shape(shape));
    }
    return result;
}

fn is_intersecting(current_shape: &Shape, field: &Vec<[u32; 9]>, x: usize, y: usize) -> bool {
    let mut intersect: bool = false;
    for (sx, sy) in &current_shape.positions {
        if field[y - sy][x + sx] == 1 {
            intersect = true;
            break;
        }
    }
    return intersect;
}

fn main() {
    let shapes = get_shapes();

    let jets: Vec<char> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();

    let mut field: Vec<[u32; 9]> = vec![[1; 9], [1, 0, 0, 0, 0, 0, 0, 0, 1]];
    let mut max_height: usize = 0;
    let mut current_jet: usize = 0;
    let mut current_shape_num: usize = 0;

    for i in 0..100000 {
        println!("Iteration {i} at height {max_height}, {current_jet}, {current_shape_num}");
        if current_jet == 0 && current_shape_num == 0 {
            println!("Sync at iteration {i} at height {max_height}");
        }
        let current_shape = &shapes[current_shape_num];
        let mut x: usize = 3; // shifted by one because of wall
        let mut y: usize = max_height + current_shape.height + 3;
        field.resize_with((y + 1) as usize, || [1, 0, 0, 0, 0, 0, 0, 0, 1]);

        loop {
            let dx: i32 = if jets[current_jet] == '>' { 1 } else { -1 };
            if !is_intersecting(current_shape, &field, (x as i32 + dx) as usize, y) {
                x = ((x as i32) + dx) as usize;
            }
            current_jet = (current_jet + 1) % jets.len();

            if is_intersecting(current_shape, &field, x, y - 1) {
                for (sx, sy) in &current_shape.positions {
                    field[y - sy][x + sx] = 1;
                }
                if max_height < y {
                    max_height = y;
                }
                break;
            }
            y -= 1;
        }
        current_shape_num = (current_shape_num + 1) % shapes.len();
    }

    println!("Result {:?}", max_height);
}

/*

small:

Iteration 30 at height 51, 5, 0
Iteration 65 at height 104, 5, 0
Iteration 100 at height 157, 5, 0

large:

Iteration 1738 at height 2734, 10085, 3
Iteration 3483 at height 5484, 10085, 3
Iteration 5228 at height 8234, 10085, 3
Iteration 6973 at height 10984, 10085, 3

1738 + x*1745 + y = 1000 000 000 000


1738 + 573065901*1745 + 1017 = 1000 000 000 000

height = 2734 + 573065901*2750 + 1592

Iteration 2755 at height 4326, 5881, 0


1738,2734, 10085, 3
3483,5484, 10085, 3
5228,8234, 10085, 3
6973,10984, 10085, 3



*/
