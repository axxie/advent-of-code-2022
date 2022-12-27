use std::collections::BinaryHeap;
use std::collections::VecDeque;

type Num = i64;

struct Monkey {
    items: VecDeque<Num>,
    op: fn(Num) -> Num,
    test: fn(Num) -> bool,
    succ: usize,
    fail: usize,
    count: u32,
}

fn main() {
    let first_subtask: bool = false;
    let mut monkeys: [Monkey; 8] = [
        Monkey {
            items: VecDeque::from(vec![57]),
            op: |x| x * 13,
            test: |x| x % 11 == 0,
            succ: 3,
            fail: 2,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![58, 93, 88, 81, 72, 73, 65]),
            op: |x| x + 2,
            test: |x| x % 7 == 0,
            succ: 6,
            fail: 7,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![65, 95]),
            op: |x| x + 6,
            test: |x| x % 13 == 0,
            succ: 3,
            fail: 5,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![58, 80, 81, 83]),
            op: |x| x * x,
            test: |x| x % 5 == 0,
            succ: 4,
            fail: 5,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![58, 89, 90, 96, 55]),
            op: |x| x + 3,
            test: |x| x % 3 == 0,
            succ: 1,
            fail: 7,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![66, 73, 87, 58, 62, 67]),
            op: |x| x * 7,
            test: |x| x % 17 == 0,
            succ: 4,
            fail: 1,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![85, 55, 89]),
            op: |x| x + 4,
            test: |x| x % 2 == 0,
            succ: 2,
            fail: 0,
            count: 0,
        },
        Monkey {
            items: VecDeque::from(vec![73, 80, 54, 94, 90, 52, 69, 58]),
            op: |x| x + 7,
            test: |x| x % 19 == 0,
            succ: 6,
            fail: 0,
            count: 0,
        },
    ];

    let iterations = if first_subtask { 20 } else { 10000 };
    for _round in 0..iterations {
        for i in 0..monkeys.len() {
            loop {
                let monkey: &mut Monkey = &mut monkeys[i];
                let worry = monkey.items.pop_front();
                if let None = worry {
                    break;
                }
                monkey.count += 1;
                let worry: Num = worry.unwrap();
                let mut worry: Num = (monkey.op)(worry);
                if first_subtask {
                    worry = worry / 3;
                } else {
                    worry = worry % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19);
                }
                let target: usize = if (monkey.test)(worry) {
                    monkey.succ
                } else {
                    monkey.fail
                };
                monkeys[target].items.push_back(worry);
            }
        }
    }

    let mut heap: BinaryHeap<u32> = BinaryHeap::new();

    for monkey in &monkeys {
        heap.push(monkey.count);
    }
    let (top1, top2) = (heap.pop().unwrap(), heap.pop().unwrap());
    println!("Result: {}", (top1 as i128) * (top2 as i128));
}
