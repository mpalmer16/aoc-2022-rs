use aoc_common::fetch_with_transform;

fn main() {
    let transform = |s: String| {
        s.trim()
            .split('\n')
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .map(|cs| {
                assert!(cs.len() == 2);
                (cs[0].to_string(), cs[1].to_string())
            })
            .collect::<Vec<(String, String)>>()
    };

    let input = fetch_with_transform(2, transform);

    let scores = input.iter().map(get_score).collect::<Vec<i32>>();

    println!("answer 1: {}", scores.iter().sum::<i32>());

    let decrypted_input = input
        .iter()
        .map(decrypt_round)
        .collect::<Vec<(String, String)>>();

    let scores = decrypted_input.iter().map(get_score).collect::<Vec<i32>>();

    println!("answer 2: {}", scores.iter().sum::<i32>());
}

fn get_score_for_shape(shape: &str) -> i32 {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn get_score_for_outcome(round: (&str, &str)) -> i32 {
    match round {
        ("A", "X") | ("B", "Y") | ("C", "Z") => 3, // draw
        ("A", "Z") | ("C", "Y") | ("B", "X") => 0, // loss
        ("C", "X") | ("B", "Z") | ("A", "Y") => 6,
        _ => panic!("invalid input"),
    }
}

fn get_score(round: &(String, String)) -> i32 {
    let score_for_shape = get_score_for_shape(round.1.as_str());
    let score_for_outcome = get_score_for_outcome((round.0.as_str(), round.1.as_str()));

    score_for_shape + score_for_outcome
}

fn decrypt_round(round: &(String, String)) -> (String, String) {
    match round.1.as_str() {
        "X" => match round.0.as_str() {
            "A" => (round.0.to_string(), "Z".to_string()),
            "B" => (round.0.to_string(), "X".to_string()),
            "C" => (round.0.to_string(), "Y".to_string()),
            _ => panic!("invalid input"),
        },
        "Y" => match round.0.as_str() {
            "A" => (round.0.to_string(), "X".to_string()),
            "B" => (round.0.to_string(), "Y".to_string()),
            "C" => (round.0.to_string(), "Z".to_string()),
            _ => panic!("invalid input"),
        },
        "Z" => match round.0.as_str() {
            "A" => (round.0.to_string(), "Y".to_string()),
            "B" => (round.0.to_string(), "Z".to_string()),
            "C" => (round.0.to_string(), "X".to_string()),
            _ => panic!("invalid input"),
        },
        _ => panic!("invalid input"),
    }
}

#[cfg(test)]
mod tests {
    use aoc_common::get_test_input;

    use crate::{decrypt_round, get_score};

    const TEST_INPUT_FILE: &str = "inputs/test_input.txt";

    fn test_transform(s: String) -> Vec<(String, String)> {
        s.split('\n')
            .map(|s| s.split(' ').collect::<Vec<&str>>())
            .map(|cs| {
                assert!(cs.len() == 2);
                (cs[0].to_string(), cs[1].to_string())
            })
            .collect::<Vec<(String, String)>>()
    }

    #[test]
    fn can_get_input() {
        let input = get_test_input(TEST_INPUT_FILE, test_transform);

        assert!(
            input
                == vec![
                    (String::from("A"), String::from("Y")),
                    (String::from("B"), String::from("X")),
                    (String::from("C"), String::from("Z"))
                ]
        );
    }

    #[test]
    fn can_get_scores() {
        let input = get_test_input(TEST_INPUT_FILE, test_transform);

        let scores = input.iter().map(get_score).collect::<Vec<i32>>();

        assert!(scores == vec![8, 1, 6]);
        assert!(scores.iter().sum::<i32>() == 15);
    }

    #[test]
    fn can_decrypt_rounds() {
        let input = get_test_input(TEST_INPUT_FILE, test_transform);

        let decrypted_rounds = input
            .iter()
            .map(decrypt_round)
            .collect::<Vec<(String, String)>>();

        assert!(
            decrypted_rounds
                == vec![
                    (String::from("A"), String::from("X")),
                    (String::from("B"), String::from("X")),
                    (String::from("C"), String::from("X"))
                ]
        );
    }
}
