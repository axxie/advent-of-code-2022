use std::io::BufRead;

fn main() {
    let mut score = 0;

    let mut lines = std::io::stdin().lock().lines();
    loop {
        let line1 = lines.next();
        let mut chars1 = [0; 256];
        match line1 {
            None => break,
            Some(res1) => {
                let s1 = res1.unwrap();
                for c in s1.chars() {
                    let i: usize = c as usize;
                    chars1[i] += 1;
                }
            }
        }
        let s2 = lines.next().unwrap().unwrap();
        let mut chars2 = [0; 256];
        for c in s2.chars() {
            let i: usize = c as usize;
            if chars1[i] > 0 {
                chars2[i] += 1;
            }
        }
        let s3 = lines.next().unwrap().unwrap();
        for c in s3.chars() {
            let i: usize = c as usize;
            if chars2[i] > 0 {
                println!("Common char: {c}");
                if c >= 'A' && c <= 'Z' {
                    score += i - 65 + 1 + 26;
                } else {
                    score += i - 97 + 1;
                }
                break;
            }
        }
    }

    println!("Score: {score}")
}

// fn main() {
//     let mut score = 0;
//     let lines = std::io::stdin().lines();
//     for line in lines {
//         let line: String = line.unwrap();
//         let first: &str = &line[0..line.len() / 2];
//         let second: &str = &line[line.len() / 2..];
//         let mut chars = [0; 256];
//         for c in first.chars() {
//             let i: usize = c as usize;
//             chars[i] += 1;
//         }
//         for c in second.chars() {
//             let i: usize = c as usize;
//             if chars[i] > 0 {
//                 if c >= 'A' && c <= 'Z' {
//                     score += i - 65 + 1 + 26;
//                 } else {
//                     score += i - 97 + 1;
//                 }
//                 break;
//             }
//         }
//     }
//     println!("Score: {score}")
// }
