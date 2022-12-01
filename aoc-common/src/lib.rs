use std::{
    fs::{read_to_string, File},
    path::Path,
};

use std::io::Write;

use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, COOKIE},
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
/// The second parameter is an initial split - the input is almost always
/// delimited by '\n' but there are times when splitting the initial input
/// on something different is useful.
///
/// # Example
/// ```
/// # use crate::aoc_common::fetch;
/// let input = fetch(1, "\n");
///
/// assert!(input.len() == 10);
/// ```
///
pub fn fetch(day: i32, initial_split: &str) -> Vec<String> {
    if Path::new(&format!("day{}/inputs/day_{}.txt", day, day)).exists() {
        fetch_from_file(day, initial_split)
    } else {
        match fetch_from_url(day, initial_split) {
            Ok(content) => content,
            Err(e) => panic!("there was an error fetching content: {}", e),
        }
    }
}

fn fetch_from_file(day: i32, initial_split: &str) -> Vec<String> {
    let filename = format!("day{}/inputs/day_{}.txt", day, day);
    let Ok(content) = read_to_string(filename) else {
        panic!("could not read input file");
    };
    content
        .split(initial_split)
        .map(|s| s.to_string())
        .collect()
}

fn fetch_from_url(day: i32, initial_split: &str) -> Result<Vec<String>, String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);
    let resp = build_client()?
        .get(url)
        .send()
        .and_then(|resp| resp.error_for_status())
        .and_then(|resp| resp.text());

    match resp {
        Ok(text) => {
            write_to_file(day, &text);
            Ok(text
                .trim()
                .split(initial_split)
                .map(|s| s.to_string())
                .collect())
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

    let mut headers = HeaderMap::new();

    headers.insert(COOKIE, cookie_header);
    Client::builder()
        .default_headers(headers)
        .redirect(Policy::none())
        .build()
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{fetch, fetch_from_file, fetch_from_url};

    #[test]
    fn can_fetch_input_from_file() {
        let input = fetch_from_file(1, "\n");

        assert!(input.len() == 10);
    }

    #[test]
    fn can_fetch_input_from_url() {
        let input = fetch_from_url(1, "\n").unwrap();

        assert!(input.len() == 2253);
    }

    #[test]
    fn can_fetch_and_save_to_file() {
        let input = fetch(2, "\n");

        assert!(input.len() == 1000);
    }
}
