use std::{
    fs::{read_to_string, File},
    path::Path,
};

use std::io::Write;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, COOKIE, USER_AGENT},
    redirect::Policy,
};

const YEAR: i32 = 2022;

/// Fetches the puzzle input from adventofcode.com
///
/// This will try and save the input as a file in inputs/day_{day}.txt
/// subsequent calls will pull from this file.  If no file is found
/// it will use the client.  This requires the .session.cookie file to be
/// created, along with a valid cookie value it can read.
///
/// The second parameter is an transform function defined as
///     `Fn(String) -> Vec<T>`
///
/// # Example
/// ```ignore
/// # use crate::aoc_common::fetch_with_transform;
/// let transform = |s: String| {
///     s.trim()
///         .split('\n')
///         .map(|s| s.to_string())
///         .collect::<Vec<String>>()
/// };
///
/// let input = fetch_with_transform(1, transform);
///
/// assert!(input.len() == 10);
/// ```
///
pub fn fetch_with_transform<F, T>(day: i32, transform: F) -> T
where
    F: Fn(String) -> T,
{
    if Path::new(&format!("day{}/inputs/day_{}.txt", day, day)).exists() {
        fetch_from_file_with_transform(day, transform)
    } else {
        match fetch_from_url_with_transform(day, transform) {
            Ok(content) => content,
            Err(e) => panic!("there was an error fetching content: {}", e),
        }
    }
}

/// Reads an input file from `input/test_input.txt` that contains the test data.
/// For use in unit tests (helper function)
pub fn get_test_input<F, T>(filename: &str, transform: F) -> T
where
    F: Fn(String) -> T,
{
    let content = read_to_string(filename).expect("can not read test input file");
    transform(content)
}

fn fetch_from_file_with_transform<F, T>(day: i32, transform: F) -> T
where
    F: Fn(String) -> T,
{
    let filename = format!("day{}/inputs/day_{}.txt", day, day);
    let Ok(content) = read_to_string(filename) else {
        panic!("could not read input file");
    };
    transform(content)
}

fn fetch_from_url_with_transform<F, T>(day: i32, transform: F) -> Result<T, String>
where
    F: Fn(String) -> T,
{
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let resp = build_client()?
        .get(url)
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match resp {
        Ok(text) => {
            write_to_file(day, &text);
            Ok(transform(text))
        }
        Err(e) => Err(e.to_string()),
    }
}

fn write_to_file(day: i32, text: &str) {
    let path = format!("day{}/inputs/day_{}.txt", day, day);
    if !Path::new(&path).exists() {
        match File::create(path) {
            Ok(mut output) => write!(output, "{}", text.trim()).expect("Error writing to file"),
            Err(e) => eprintln!("Error creating input file: {}", e),
        }
    }
}

fn build_client() -> Result<Client, String> {
    let session_cookie = read_to_string(".session.cookie").expect("no session cookie found!");

    let cookie_header = HeaderValue::from_str(&format!("session={}", session_cookie.trim()))
        .map_err(|e| format!("Invalid session cookie: {}", e))?;

    let user_agent_header =
        HeaderValue::from_str("github.com/mpalmer16/aoc-2022-rs by mpalmer1661@gmail.com")
            .map_err(|e| format!("Invalid user agent: {}", e))?;

    let mut headers = HeaderMap::new();

    headers.insert(COOKIE, cookie_header);
    headers.insert(USER_AGENT, user_agent_header);
    Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{
        fetch_from_file_with_transform, fetch_from_url_with_transform, fetch_with_transform,
        get_test_input,
    };

    #[test]
    #[ignore]
    fn can_fetch_input_from_file() {
        let transform = |s: String| {
            s.trim()
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };
        let input = fetch_from_file_with_transform(1, transform);

        assert!(input.len() == 10);
    }

    #[test]
    #[ignore]
    fn can_fetch_input_from_url() {
        let transform = |s: String| {
            s.trim()
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };
        let input = fetch_from_url_with_transform(1, transform).unwrap();

        assert!(input.len() == 2253);
    }

    #[test]
    #[ignore]
    fn can_fetch_and_save_to_file() {
        let transform = |s: String| {
            s.trim()
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        };
        let input = fetch_with_transform(2, transform);

        assert!(input.len() == 1000);
    }

    #[test]
    fn can_read_test_input() {
        let transform = |s: String| {
            s.split('\n')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        };

        let input = get_test_input("inputs/test_input.txt", transform);

        assert!(input.len() == 5);
    }
}
