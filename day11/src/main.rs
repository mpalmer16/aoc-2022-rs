use std::str::FromStr;

use aoc_common::fetch_with_transform;

fn main() {
    let answer_1 = get_answer(20, true);

    println!("answer 1: {answer_1}");

    let answer_2 = get_answer(10_000, false);

    println!("answer 2: {answer_2}");
}

fn get_answer(how_many_rounds: i32, reduce_worry: bool) -> u64 {
    let mut monkeys: Vec<Monkey> = fetch_with_transform(11, |s| {
        s.split("\n\n")
            .map(|s| s.parse::<Monkey>().unwrap())
            .collect::<_>()
    });
    let divisor_product = monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();
    let mut new_monkeys = rounds(how_many_rounds, &mut monkeys, reduce_worry, divisor_product);

    new_monkeys.sort_by_key(|m| m.inpsected_count());

    let top_two = new_monkeys.iter().rev().take(2).collect::<Vec<&Monkey>>();

    top_two[0].inpsected_count() * top_two[1].inpsected_count()
}

fn monkey_round(monkey: &Monkey, reduce_worry: bool, divisor_product: u64) -> Vec<ItemDestination> {
    let mut items_and_destinations: Vec<ItemDestination> = vec![];

    for mut item in monkey.items.iter().copied() {
        item %= divisor_product;
        let mut worry_level = match monkey.operation.operator {
            Operator::Add => {
                item + match monkey.operation.value.clone() {
                    OperationValue::Value(v) => v,
                    OperationValue::Old => item,
                }
            }
            Operator::Multiply => {
                let value = match monkey.operation.value.clone() {
                    OperationValue::Value(v) => v,
                    OperationValue::Old => item,
                };
                item * value
            }
        };
        if reduce_worry {
            worry_level /= 3;
        }

        let is_divisible = worry_level % monkey.test.divisible_by == 0;

        let destination = if is_divisible {
            monkey.test.if_true
        } else {
            monkey.test.if_false
        };

        items_and_destinations.push(ItemDestination {
            item: worry_level,
            destination: destination.try_into().unwrap(),
        });
    }
    items_and_destinations
}

fn round(
    monkeys: &mut [Monkey],
    idx: usize,
    reduce_worry: bool,
    divisor_product: u64,
) -> Vec<Monkey> {
    if idx == monkeys.len() {
        monkeys.to_vec()
    } else {
        let items_and_destinations = monkey_round(&monkeys[idx], reduce_worry, divisor_product);
        for item in items_and_destinations {
            monkeys[idx].increase_inspected(item.destination);
            monkeys[item.destination].items.push(item.item);
        }
        monkeys[idx].items = vec![];
        round(monkeys, idx + 1, reduce_worry, divisor_product)
    }
}

fn rounds(
    how_many: i32,
    monkeys: &mut [Monkey],
    reduce_worry: bool,
    divisor_product: u64,
) -> Vec<Monkey> {
    if how_many == 0 {
        monkeys.to_vec()
    } else {
        rounds(
            how_many - 1,
            &mut round(monkeys, 0, reduce_worry, divisor_product),
            reduce_worry,
            divisor_product,
        )
    }
}

struct ItemDestination {
    item: u64,
    destination: usize,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    value: OperationValue,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum OperationValue {
    Value(u64),
    Old,
}

#[derive(Debug, Clone)]
struct Test {
    divisible_by: u64,
    if_true: u64,
    if_false: u64,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    inspection_count: Vec<u64>,
}

impl Monkey {
    fn inpsected_count(&self) -> u64 {
        self.inspection_count.iter().sum()
    }

    fn increase_inspected(&mut self, idx: usize) {
        if idx < self.inspection_count.len() {
            self.inspection_count[idx] += 1;
        } else {
            for _ in self.inspection_count.len()..idx {
                self.inspection_count.push(0);
            }
            self.inspection_count.push(1);
        }
    }
}

impl FromStr for Monkey {
    type Err = MonkeyParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().split('\n').collect::<Vec<_>>();
        let items = lines[1]
            .trim()
            .split("Starting items: ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u64>().expect("can not parse item!"))
            .collect::<Vec<u64>>();

        let operation_parts = lines[2]
            .trim()
            .split("Operation: new = old ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>();

        let operator = match operation_parts[0] {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("unknown operator!"),
        };

        let value = match operation_parts[1].parse::<u64>() {
            Ok(v) => OperationValue::Value(v),
            Err(_) => OperationValue::Old,
        };

        let operation = Operation { operator, value };

        let divisible_by = lines[3]
            .trim()
            .split("Test: divisible by ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let if_true = lines[4]
            .trim()
            .split("If true: throw to monkey ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let if_false = lines[5]
            .trim()
            .split("If false: throw to monkey ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let test = Test {
            divisible_by,
            if_true,
            if_false,
        };

        Ok(Self {
            items,
            operation,
            test,
            inspection_count: vec![],
        })
    }
}

#[derive(Debug)]
struct MonkeyParsingError {}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{rounds, Monkey};

    const TEST_FILE: &str = "inputs/test_input.txt";

    fn test_transform(s: String) -> Vec<Monkey> {
        s.split("\n\n")
            .map(|s| s.parse::<Monkey>().unwrap())
            .collect::<_>()
    }

    #[test]
    fn can_read_input() {
        let input: Vec<Monkey> = get_test_input(TEST_FILE, |s| {
            s.split("\n\n")
                .map(|s| s.parse::<Monkey>().unwrap())
                .collect::<_>()
        });

        assert!(input.len() == 4);
    }

    #[test]
    fn can_monkey_round() {
        let mut monkeys = get_test_input(TEST_FILE, test_transform);

        let divisor_product = monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();

        let new_monkeys = rounds(20, &mut monkeys, true, divisor_product);

        assert!(new_monkeys.len() == 4);
    }

    #[test]
    fn can_get_most_active_monkeys() {
        let mut monkeys = get_test_input(TEST_FILE, test_transform);
        let divisor_product = monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();
        let mut new_monkeys = rounds(20, &mut monkeys, true, divisor_product);

        new_monkeys.sort_by_key(|m| m.inpsected_count());

        let top_two = new_monkeys.iter().rev().take(2).collect::<Vec<&Monkey>>();

        let monkey_business = top_two[0].inpsected_count() * top_two[1].inpsected_count();

        assert!(monkey_business == 10605);
    }

    #[test]
    fn can_get_most_active_monkeys_no_worry_reduction() {
        let mut monkeys = get_test_input(TEST_FILE, test_transform);
        let divisor_product = monkeys.iter().map(|m| m.test.divisible_by).product::<u64>();
        let mut new_monkeys = rounds(10_000, &mut monkeys, false, divisor_product);

        new_monkeys.sort_by_key(|m| m.inpsected_count());

        let top_two = new_monkeys.iter().rev().take(2).collect::<Vec<&Monkey>>();

        let monkey_business = top_two[0].inpsected_count() * top_two[1].inpsected_count();

        assert!(monkey_business == 2713310158);
    }
}
