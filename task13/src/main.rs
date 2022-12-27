use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::BufRead;

use core::cmp::Ordering::Equal;
use core::cmp::Ordering::Greater;
use core::cmp::Ordering::Less;

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Number(u32),
    Array(Vec<Item>),
}

impl Item {
    fn array<'a>(&'a self) -> &'a Vec<Item> {
        match self {
            Item::Array(arr) => &arr,
            _ => panic!("Not an array"),
        }
    }
}

fn get_items(line: &str) -> (Item, &str) {
    let first_char = line.chars().next().unwrap();

    if first_char != '[' {
        let delimiter: Option<usize> = line.find(&[',', ']']);
        if let None = delimiter {
            return (Item::Number(line.parse().unwrap()), "");
        }
        let pos: usize = delimiter.unwrap();
        return (Item::Number((&line[0..pos]).parse().unwrap()), &line[pos..]);
    }

    let mut inner: Vec<Item> = Vec::new();
    let mut remainder: &str = &line[1..];
    loop {
        if remainder.is_empty() || &remainder[0..1] == "]" {
            break;
        }
        let (item, new_remainder) = get_items(remainder);

        inner.push(item);
        remainder = new_remainder;
        if !remainder.is_empty() && &remainder[0..1] == "," {
            remainder = &remainder[1..]; // skip comma
        }
    }
    if remainder.is_empty() {
        return (Item::Array(inner), "");
    }
    return (Item::Array(inner), &remainder[1..]); // skip closing bracket
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        assert_eq!(get_items("42"), (Item::Number(42), ""));
        assert_eq!(get_items("1,2"), (Item::Number(1), ",2"));
        assert_eq!(get_items("1,2,3"), (Item::Number(1), ",2,3"));
        assert_eq!(get_items("[1]"), (Item::Array(vec![Item::Number(1)]), ""));
        assert_eq!(
            get_items("[1,2]"),
            (Item::Array(vec![Item::Number(1), Item::Number(2)]), "")
        );
        assert_eq!(get_items("[]"), (Item::Array(vec![]), ""));
        assert_eq!(
            get_items("[[]]"),
            (Item::Array(vec![Item::Array(vec![])]), "")
        );
        assert_eq!(
            get_items("[[],[3]]"),
            (
                Item::Array(vec![
                    Item::Array(vec![]),
                    Item::Array(vec![Item::Number(3)])
                ]),
                ""
            )
        );
    }
}

fn get_diff(a: u32, b: u32) -> i32 {
    if a < b {
        return -1;
    } else if a > b {
        return 1;
    }
    return 0;
}

fn is_less(a: &Item, b: &Item) -> i32 {
    if let (Item::Number(num_a), Item::Number(num_b)) = (a, b) {
        return get_diff(*num_a, *num_b);
    }

    if let Item::Number(num_a) = a {
        return -is_less(b, a);
    }

    if let Item::Number(num_b) = b {
        return is_less(a, &Item::Array(vec![Item::Number(*num_b)]));
    }

    let mut iter_a = a.array().iter();
    let mut iter_b = b.array().iter();
    let mut result = 0;

    while result == 0 {
        result = match (iter_a.next(), iter_b.next()) {
            (Some(item_a), Some(item_b)) => is_less(item_a, item_b),
            (Some(_), None) => 1,
            (None, Some(_)) => -1,
            (None, None) => {
                break;
                0
            }
        };
    }
    return result;
}

fn is_less_ord(a: &Item, b: &Item) -> Ordering {
    match is_less(a, b) {
        -1 => Less,
        0 => Equal,
        1 => Greater,
        _ => panic!(),
    }
}

fn main() {
    let first_part = false;
    if first_part {
        let mut lines = std::io::stdin().lock().lines();
        let mut index: u32 = 1;
        let mut sum: u32 = 0;
        loop {
            let line1: String = lines.next().unwrap().unwrap();
            let line2: String = lines.next().unwrap().unwrap();
            let (item1, _) = get_items(&line1);
            let (item2, _) = get_items(&line2);
            if is_less(&item1, &item2) < 0 {
                sum += index;
            }

            lines.next();
            index += 1;
        }
        println!("Result {}", sum);
    } else {
        let lines = std::io::stdin().lines();
        let (item2, _) = get_items("[[2]]");
        let (item6, _) = get_items("[[6]]");
        let mut items = vec![item2, item6];
        for line in lines {
            let line: String = line.unwrap();
            if line.is_empty() {
                continue;
            }
            let (item, _) = get_items(&line);
            items.push(item);
        }
        items.sort_by(|a, b| is_less_ord(a, b));
        let (item2, _) = get_items("[[2]]");
        let (item6, _) = get_items("[[6]]");
        let i1 = items.iter().position(|r| r == &item2).unwrap();
        let i2 = items.iter().position(|r| r == &item6).unwrap();
        let res = 0;
    }
}
