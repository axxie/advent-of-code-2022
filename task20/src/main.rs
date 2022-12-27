fn mix_and_get_sum(
    rounds: usize,
    key: i64,
    numbers: &Vec<i64>,
    reverse_mapping: &mut Vec<usize>,
) -> i64 {
    let count = numbers.len();
    for _ in 0..rounds {
        for (index, &n) in numbers.iter().enumerate() {
            let old_position = reverse_mapping.iter().position(|&x| x == index).unwrap();
            let new_position =
                (old_position as i64 + n * key).rem_euclid((count - 1) as i64) as usize;

            if new_position > old_position {
                reverse_mapping[old_position..new_position + 1].rotate_left(1);
            } else if new_position < old_position {
                reverse_mapping[new_position..old_position + 1].rotate_right(1);
            }

            // for &el in &reverse_mapping {
            //     print!("{} ", numbers[el]);
            // }
            // println!();
        }
    }

    let zero_original_position = numbers.iter().position(|&x| x == 0).unwrap();
    let zero_new_position = reverse_mapping
        .iter()
        .position(|&x| x == zero_original_position)
        .unwrap();
    let a1 = key * numbers[reverse_mapping[(zero_new_position + 1000) % numbers.len()]];
    let a2 = key * numbers[reverse_mapping[(zero_new_position + 2000) % numbers.len()]];
    let a3 = key * numbers[reverse_mapping[(zero_new_position + 3000) % numbers.len()]];
    return a1 + a2 + a3;
}

fn main() {
    let mut numbers: Vec<i64> = Vec::new();
    let mut reverse_mapping: Vec<usize> = Vec::new();
    let mut count = 0;
    for line in std::io::stdin().lines() {
        let text: String = line.unwrap();
        let number: i64 = text.parse().unwrap();
        numbers.push(number);
        reverse_mapping.push(count);
        count += 1;
    }

    let result = if false {
        mix_and_get_sum(1, 1, &numbers, &mut reverse_mapping)
    } else {
        mix_and_get_sum(10, 811589153, &numbers, &mut reverse_mapping)
    };

    println!("Result {}", result);
}
