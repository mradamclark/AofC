use std::{collections::VecDeque, fmt::Display};

pub mod handheld;
pub mod parser;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Curr,
    Old,
    Num(u64),
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add((Value, Value)),
    Mult((Value, Value)),
    Div((Value, Value)),
}

#[derive(Clone)]
pub struct Monkey {
    id: u32,
    items: VecDeque<u64>,
    operation: Operation,
    test: Operation,
    true_dest: u32,
    false_dest: u32,
    number_items_inspected: u64,
}
impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Monkey {} - inspections:{}, items:{:?}",
            self.id, self.number_items_inspected, self.items
        )
    }
}
impl Monkey {
    pub fn handle_next(&mut self, worry_level_divisor: u64, magic_number: u64) -> (u64, usize) {
        let item = self.items.pop_front().unwrap();
        self.number_items_inspected += 1;
        let worry = match &self.operation {
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(n) => *n,
                    Value::Curr => item,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(n) => *n,
                    Value::Curr => item,
                };
                (num_a + num_b) % magic_number
            }
            Operation::Mult((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(n) => *n,
                    Value::Curr => item,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(n) => *n,
                    Value::Curr => item,
                };
                (num_a * num_b) % magic_number
            }
            _ => panic!("Not Supported"),
        } / worry_level_divisor;

        let send_to = match &self.test {
            Operation::Div((a, b)) => {
                let num_a = match a {
                    Value::Curr => worry,
                    Value::Num(n) => *n,
                    Value::Old => item,
                };
                let num_b = match b {
                    Value::Curr => worry,
                    Value::Num(n) => *n,
                    Value::Old => item,
                };

                if num_a % num_b == 0 {
                    self.true_dest as usize
                } else {
                    self.false_dest as usize
                }
            }
            _ => panic!("Arg, Don't know how to test"),
        };

        (worry, send_to)
    }
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = parser::parse(input).unwrap();
    let magic_n = get_magic_number(&monkeys);
    for _round in 0..20 {
        for mindex in 0..monkeys.len() {
            for _ in 0..monkeys[mindex].items.len() {
                let monkey = monkeys.get_mut(mindex).unwrap();
                let (item, new_monkey_index) = monkey.handle_next(3, magic_n);
                monkeys[new_monkey_index].items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.number_items_inspected);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.number_items_inspected)
        .product::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = parser::parse(input).unwrap();

    let magic_n = get_magic_number(&monkeys);

    for _round in 0..10000 {
        for mindex in 0..monkeys.len() {
            for _ in 0..monkeys[mindex].items.len() {
                let monkey = monkeys.get_mut(mindex).unwrap();
                let (item, new_monkey_index) = monkey.handle_next(1, magic_n);
                monkeys[new_monkey_index].items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.number_items_inspected);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|monkey| monkey.number_items_inspected)
        .product::<u64>()
        .to_string()
}

fn get_magic_number(monkeys: &Vec<Monkey>) -> u64 {
    let magic_n = monkeys
        .iter()
        .map(|m| m.test)
        .map(|o| match o {
            Operation::Div((_, b)) => match b {
                Value::Num(n) => n,
                _ => panic!("oops"),
            },
            _ => panic!("oops 2"),
        })
        .product::<u64>();
    magic_n
}

#[cfg(test)]
mod tests {

    const INPUT: &str = include_str!("../data/test.txt");
    #[test]
    fn part_1_works() {
        let result = crate::process_part1(INPUT);
        assert_eq!(result, "10605");
    }

    #[test]
    fn part_2_works() {
        let result = crate::process_part2(INPUT);
        assert_eq!(result, "2713310158");
    }
}
