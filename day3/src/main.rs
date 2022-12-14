use std::ops::RangeInclusive;

use aoc_common::fetch_with_transform;

fn main() {
    let rucksacks: Vec<(Vec<char>, Vec<char>)> = fetch_with_transform(3, |s| {
        s.split('\n')
            .map(|s| {
                let half = s.len() / 2;
                (
                    s.chars().take(half).collect(),
                    s.chars().rev().take(half).collect(),
                )
            })
            .collect()
    });
    let priority_sum: usize = rucksacks
        .iter()
        .map(diff)
        .map(|c| priority(&c))
        .sum::<usize>();

    println!("answer 1: {}", priority_sum);

    let elf_groups: Vec<Vec<String>> = fetch_with_transform(3, |s| {
        let strings: Vec<String> = s.split('\n').map(|s| s.to_string()).collect();
        let mut out_vec: Vec<Vec<String>> = vec![];
        for chunk in strings.chunks_exact(3) {
            let mut inner_vec: Vec<String> = vec![];
            for s in chunk {
                inner_vec.push(s.to_string());
            }
            out_vec.push(inner_vec);
        }
        out_vec
    });

    let priority_sum = elf_groups
        .iter()
        .map(|s| find_badge(s))
        .map(|c| priority(&c))
        .sum::<usize>();

    println!("answer 2: {}", priority_sum)
}

fn diff((s1, s2): &(Vec<char>, Vec<char>)) -> char {
    for c in s1 {
        if s2.contains(c) {
            return *c;
        }
    }
    panic!("there is no item in both compartments!");
}

const LOWERCASE: RangeInclusive<char> = 'a'..='z';
const UPPERCASE: RangeInclusive<char> = 'A'..='Z';

fn priority(c: &char) -> usize {
    for (idx, ch) in LOWERCASE.enumerate() {
        if *c == ch {
            return idx + 1;
        }
    }
    for (idx, ch) in UPPERCASE.enumerate() {
        if *c == ch {
            return idx + 27;
        }
    }
    panic!("not a valid item!");
}

fn find_badge(s: &[String]) -> char {
    assert!(s.len() == 3, "not grouped by 3!");

    let first = s.get(0).unwrap();
    let second = s.get(1).unwrap();
    let third = s.get(2).unwrap();

    for c in first.chars() {
        if second.contains(c) && third.contains(c) {
            return c;
        }
    }
    panic!("no badge found!")
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{diff, find_badge, priority};

    const TEST_FILE: &str = "inputs/test_input.txt";

    fn create_rucksack_compartments(s: String) -> Vec<(Vec<char>, Vec<char>)> {
        s.split('\n')
            .map(|s| {
                let half = s.len() / 2;
                (
                    s.chars().take(half).collect(),
                    s.chars().rev().take(half).collect(),
                )
            })
            .collect()
    }

    fn create_elf_groups(s: String) -> Vec<Vec<String>> {
        let strings: Vec<String> = s.split('\n').map(|s| s.to_string()).collect();
        let mut out_vec: Vec<Vec<String>> = vec![];
        for chunk in strings.chunks_exact(3) {
            let mut inner_vec: Vec<String> = vec![];
            for s in chunk {
                inner_vec.push(s.to_string());
            }
            out_vec.push(inner_vec);
        }
        out_vec
    }

    #[test]
    fn can_get_input() {
        let input = get_test_input(TEST_FILE, create_rucksack_compartments);

        assert!(input.len() == 6);
    }

    #[test]
    fn can_diff_compartments() {
        let shared_items: Vec<char> = get_test_input(TEST_FILE, create_rucksack_compartments)
            .iter()
            .map(diff)
            .collect();

        assert!(shared_items == vec!['p', 'L', 'P', 'v', 't', 's']);
    }

    #[test]
    fn can_get_priority() {
        let priorities: Vec<usize> = get_test_input(TEST_FILE, create_rucksack_compartments)
            .iter()
            .map(diff)
            .map(|c| priority(&c))
            .collect();

        assert!(priorities == vec![16, 38, 42, 22, 20, 19]);
        assert!(priorities.iter().sum::<usize>() == 157);
    }

    #[test]
    fn can_find_badges() {
        let badges: Vec<char> = get_test_input(TEST_FILE, create_elf_groups)
            .iter()
            .map(|s| find_badge(s))
            .collect();

        assert!(badges == vec!['r', 'Z']);

        let priority_sum = badges.iter().map(priority).sum::<usize>();

        assert!(priority_sum == 70);
    }
}
