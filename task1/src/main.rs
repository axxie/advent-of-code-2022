use std::collections::BinaryHeap;

fn main() {
    let lines = std::io::stdin().lines();
    let mut current_sum = 0;
    let mut max: i32 = 0;
    let mut max_set: bool = false;
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    for line in lines {
        let line: String = line.unwrap();
        if line != "" {
            let number: i32 = line.trim().parse().unwrap();
            current_sum += number;
            continue;
        }
        // if max_set {
        //     if max < current_sum {
        //         max = current_sum;
        //     }
        // } else {
        //     max_set = true;
        //     max = current_sum;
        // }
        heap.push(current_sum);
        current_sum = 0;
    }
    // if max < current_sum {
    //     max = current_sum;
    // }

    println!("Result: {max}");
}
