fn from_symmetric_5(text: String) -> i64 {
    let mut number: i64 = 0;
    for c in text.chars() {
        let digit: i64 = match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Unknown digit"),
        };
        number *= 5;
        number += digit;
    }
    number
}

fn to_symmetric_5(number: i64) -> String {
    if number == 0 {
        return "0".to_string();
    }

    let mut number = number;
    let mut result = String::new();
    while number > 0 {
        let rem = number % 5;
        number = number / 5;
        result.push(match rem {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                number += 1;
                '='
            }
            4 => {
                number += 1;
                '-'
            }
            _ => panic!("Integer division is broken!"),
        });
    }
    result.chars().rev().collect()
}

fn main() {
    assert_eq!(to_symmetric_5(3), "1=");
    assert_eq!(to_symmetric_5(15), "1=0");
    assert_eq!(to_symmetric_5(20), "1-0");
    assert_eq!(to_symmetric_5(12345), "1-0---0");
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let text: String = line.unwrap();
        let number = from_symmetric_5(text);
        sum += number;
    }
    println!("Result {}", to_symmetric_5(sum));
}
