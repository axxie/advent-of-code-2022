use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::io::BufRead;

extern crate png_encode_mini;

fn image_generate_pixels(image_x: usize, image_y: usize) -> Vec<u8> {
    let len = image_x * image_y;
    let mut image: Vec<u8> = Vec::with_capacity(len * 4);

    for y in 0..image_y {
        for x in 0..image_x {
            let r = x as f64 / image_x as f64;
            let g = y as f64 / image_y as f64;
            let b = (1.0 - r) * g;

            image.extend(&[
                (r * 255.0) as u8,
                (g * 255.0) as u8,
                (b * 255.0) as u8,
                255_u8,
            ]);
        }
    }
    return image;
}

fn image_at_size(file: &String, image_x: usize, image_y: usize) {
    let image = image_generate_pixels(image_x, image_y);

    {
        let mut f = std::fs::File::create(file).unwrap();
        match png_encode_mini::write_rgba_from_u8(
            &mut f,
            &image[..],
            image_x as u32,
            image_y as u32,
        ) {
            Ok(_) => {
                // println!("Written image!")
            }
            Err(e) => {
                println!("Error {:?}", e)
            }
        }
    }
}

fn main() {
    let dim = 1000;
    let first_part = false;
    let mut field = vec![vec![0_i32; dim]; dim];
    let mut bottom = 0;

    for line in std::io::stdin().lines() {
        let line: String = line.unwrap();

        let v: Vec<(usize, usize)> = line
            .split("->")
            .map(|x| {
                x.trim()
                    .split(",")
                    .map(|y| y.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();

        let mut old_x = v[0].0;
        let mut old_y = v[0].1;

        if old_y > bottom {
            bottom = old_y;
        }
        for (new_x, new_y) in v[1..].iter() {
            let (lx, ux) = (min(old_x, *new_x), max(old_x, *new_x));
            let (ly, uy) = (min(old_y, *new_y), max(old_y, *new_y));
            if lx == ux {
                for y in ly..uy + 1 {
                    field[y][lx] = -1;
                }
            } else {
                for x in lx..ux + 1 {
                    field[ly][x] = -1;
                }
            }
            old_x = *new_x;
            old_y = *new_y;
            if old_y > bottom {
                bottom = old_y;
            }
        }
    }

    let floor = bottom + 2;
    if !first_part {
        for x in 0..dim {
            field[floor][x] = 1;
        }
    }

    let mut count = 0;
    loop {
        let mut x = 500;
        let mut y = 0;
        if field[y][x] != 0 {
            break;
        }

        loop {
            if field[y + 1][x] == 0 {
                y += 1;
            } else if field[y + 1][x - 1] == 0 {
                y += 1;
                x -= 1;
            } else if field[y + 1][x + 1] == 0 {
                y += 1;
                x += 1;
            } else {
                break;
            }
            if y > 500 {
                break;
            }
        }

        if first_part && y > 500 {
            break;
        }
        field[y][x] = count + 1;
        count += 1;
    }
    println!("Result: {count}");
    let len = dim * dim;
    let mut image: Vec<u8> = Vec::with_capacity(len * 4);
    for y in (0..dim).rev() {
        for x in 0..dim {
            if field[y][x] == -1 {
                image.extend(&[255_u8, 255_u8, 255_u8, 255_u8]);
            } else {
                let mut col: u8 = (field[y][x] / 250 + 100) as u8;
                if field[y][x] == 0 {
                    col = 0;
                }
                image.extend(&[col, col, 0, 255_u8]);
            }
        }
    }

    let mut f = std::fs::File::create(&String::from("test.png")).unwrap();
    png_encode_mini::write_rgba_from_u8(&mut f, &image[..], dim as u32, dim as u32);
}
