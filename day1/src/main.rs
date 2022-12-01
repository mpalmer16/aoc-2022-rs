use aoc_common::fetch;

fn main() {
    let input = fetch(1, "\n\n");
    let mut counts = get_elf_counts(input);
    counts.sort();

    println!("answer 1: {}", counts.last().unwrap());

    counts.reverse();
    let top_three_sum = counts.iter().take(3).sum::<i32>();

    println!("answer 2: {}", top_three_sum);
}

fn get_elf_counts(elf_strings: Vec<String>) -> Vec<i32> {
    elf_strings
        .iter()
        .map(|s| {
            s.split('\n')
                .map(|s| s.parse::<i32>().expect("could not parse int"))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;

    fn get_input(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .expect("could not read input file")
            .split("\n\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    #[test]
    fn can_get_elf_counts() {
        let input = get_input("inputs/test_input.txt");
        let counts = get_elf_counts(input);

        assert!(counts == vec![6000, 4000, 11000, 24000, 10000]);
    }

    #[test]
    fn can_find_max() {
        let input = get_input("inputs/test_input.txt");
        let mut counts = get_elf_counts(input);
        counts.sort();

        assert!(*counts.last().unwrap() == 24000)
    }

    #[test]
    fn can_sum_top_three() {
        let input = get_input("inputs/test_input.txt");
        let mut counts = get_elf_counts(input);
        counts.sort();
        counts.reverse();
        let top_three_sum = counts.iter().take(3).sum::<i32>();

        assert!(top_three_sum == 45000);
    }
}
