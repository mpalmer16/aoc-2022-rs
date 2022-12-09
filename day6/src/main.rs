use aoc_common::fetch_with_transform;

fn main() {
    let input = fetch_with_transform(6, |s| s);
    let (_marker, count) = find_marker(&input, 4);

    println!("answer 1: {}", count);

    let (_marker, count) = find_marker(&input, 14);

    println!("answer 2: {}", count);
}

fn find_marker(s: &str, size: usize) -> (String, i32) {
    let mut count = 0;
    let mut marker = "".to_string();

    for c in s.chars() {
        marker = format!("{}{}", marker, c);
        if marker.len() == size {
            if is_unique(&marker) {
                break;
            } else {
                marker = marker.chars().collect::<Vec<char>>()[1..].iter().collect();
            }
        }
        count += 1;
    }
    (marker, count + 1)
}

fn is_unique(s: &str) -> bool {
    let mut letters: Vec<char> = vec![];
    for c in s.chars() {
        if letters.contains(&c) {
            return false;
        } else {
            letters.push(c);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::find_marker;

    fn transform(s: String) -> String {
        s
    }

    #[test]
    fn can_read_input() {
        let input = get_test_input("inputs/test_input.txt", transform);

        assert!(!input.is_empty());
    }

    #[test]
    fn can_find_marker() {
        let input = get_test_input("inputs/test_input.txt", transform);
        let (marker, count) = find_marker(&input, 4);

        assert!(marker == "jpqm");
        assert!(count == 7);
    }
}
