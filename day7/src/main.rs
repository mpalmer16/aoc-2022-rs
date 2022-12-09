mod scratch;

use core::fmt;
use std::{cell::RefCell, rc::Rc};

use aoc_common::fetch_with_transform;
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

use crate::scratch::{apply_command, apply_command_to_directories, transform, Directory};

/// needed help on this one - see [this answer](https://fasterthanli.me/series/advent-of-code-2022/part-7) for more!
fn main() {
    let lines = include_str!("../inputs/test_input.txt") // change this to be real input as needed
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    //ignored
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignored
                    }
                    ".." => {
                        let parent = node.borrow().parent.clone().unwrap();
                        node = parent;
                    }
                    _ => {
                        let child = node.borrow_mut().children.entry(path).or_default().clone();
                        node = child;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(dir) => {
                    let entry = node.borrow_mut().children.entry(dir).or_default().clone();
                    entry.borrow_mut().parent = Some(node.clone());
                }
                Entry::File(size, file) => {
                    let entry = node.borrow_mut().children.entry(file).or_default().clone();
                    entry.borrow_mut().size = size as usize;
                    entry.borrow_mut().parent = Some(node.clone());
                }
            },
        }
    }
    // let sum = all_dirs(root)
    //     .map(|d| d.borrow().total_size())
    //     .filter(|&s| s <= 100_000)
    //     .sum::<u64>();

    // println!("answer 1: {sum}");

    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let removed_dirs_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .min();
    dbg!(removed_dirs_size);
}

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Self::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default, Clone)]
struct Node {
    size: usize,
    children: IndexMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

struct PrettyNode<'a>(&'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}

impl Node {
    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|child| child.borrow().total_size())
            .sum::<u64>()
            + self.size as u64
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

// This is the work from my own solution that passed the tests (see scratch_tests)
// but overflows the stack with the actual input.
fn _scratch_solve() {
    let mut directories: Vec<Directory> = vec![Directory::new("/")];
    let commands = fetch_with_transform(7, transform)
        .iter()
        .map(|s| crate::scratch::parse_command(s.to_string()))
        .collect::<Vec<crate::scratch::Command>>();

    let mut current_idx = 0;

    for command in commands {
        current_idx = apply_command_to_directories(&mut directories, &command, current_idx);
        apply_command(&mut directories[current_idx], &command)
    }

    let dir_sizes = directories
        .iter()
        .map(|d| d.size(&directories))
        .filter(|s| s < &100000)
        .collect::<Vec<i32>>();

    let answer_1 = dir_sizes.iter().sum::<i32>();

    println!("answer 1: {}", answer_1);
}

#[cfg(test)]
mod scratch_tests {

    use crate::scratch::{
        apply_command, apply_command_to_directories, parse_command, transform, Command, Directory,
        FileType,
    };

    use aoc_common::get_test_input;

    const TEST_FILE: &str = "inputs/test_input.txt";

    #[test]
    fn can_read_input() {
        let command_lines = get_test_input(TEST_FILE, transform);

        assert!(command_lines.len() == 10);
    }

    #[test]
    fn can_parse_cd_command() {
        let command_line = "cd /\n".to_string();
        let command = parse_command(command_line);

        assert!(command == Command::CD("/".to_string()))
    }

    #[test]
    fn can_parse_ls_command() {
        let command_line = "ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n".to_string();
        let command = parse_command(command_line);

        assert!(
            command
                == Command::LS(vec![
                    FileType::Dir("e".to_string()),
                    FileType::File((29116, "f".to_string())),
                    FileType::File((2557, "g".to_string())),
                    FileType::File((62596, "h.lst".to_string()))
                ])
        );
    }

    #[test]
    fn can_create_directory() {
        let directory = Directory::new("/");

        assert!(directory.name == "/");
        assert!(directory.directories.is_empty());
        assert!(directory.files.is_empty());
    }

    #[test]
    fn can_apply_commands_to_vec_of_dir() {
        let mut directories: Vec<Directory> = vec![Directory::new("/")];
        let command_1 = Command::CD("/".to_string());
        let command_2 = Command::LS(vec![
            FileType::Dir("foo".to_string()),
            FileType::File((29116, "f".to_string())),
        ]);
        let command_3 = Command::CD("foo".to_string());

        let mut current_idx = 0;

        current_idx = apply_command_to_directories(&mut directories, &command_1, current_idx);

        assert!(current_idx == 0);

        current_idx = apply_command_to_directories(&mut directories, &command_2, current_idx);

        assert!(current_idx == 0);

        current_idx = apply_command_to_directories(&mut directories, &command_3, current_idx);

        assert!(current_idx == 1);
    }

    #[test]
    fn can_apply_command_to_directory() {
        let mut directory = Directory::new("/");
        let command_1 = Command::CD("foo".to_string());
        let command_2 = Command::LS(vec![
            FileType::Dir("foo".to_string()),
            FileType::File((29116, "f".to_string())),
        ]);

        apply_command(&mut directory, &command_1);
        assert!(directory.directories.is_empty());

        apply_command(&mut directory, &command_2);
        assert!(directory.directories.contains(&"foo".to_string()));
    }

    #[test]
    fn can_apply_commands() {
        let mut directories: Vec<Directory> = vec![Directory::new("/")];
        let commands = get_test_input(TEST_FILE, transform)
            .iter()
            .map(|s| parse_command(s.to_string()))
            .collect::<Vec<Command>>();

        let mut current_idx = 0;

        for command in commands {
            current_idx = apply_command_to_directories(&mut directories, &command, current_idx);
            apply_command(&mut directories[current_idx], &command)
        }

        assert!(directories.len() == 5);
    }

    #[test]
    fn can_get_empty_directory_size() {
        let directory = Directory::new("/");

        assert!(directory.size(&vec![]) == 0)
    }

    #[test]
    fn can_get_directory_sizes() {
        let mut directories: Vec<Directory> = vec![Directory::new("/")];
        let commands = get_test_input(TEST_FILE, transform)
            .iter()
            .map(|s| parse_command(s.to_string()))
            .collect::<Vec<Command>>();

        let mut current_idx = 0;

        for command in commands {
            current_idx = apply_command_to_directories(&mut directories, &command, current_idx);
            apply_command(&mut directories[current_idx], &command)
        }

        let dir_sizes = directories
            .iter()
            .map(|d| d.size(&directories))
            .filter(|s| s < &100000)
            .collect::<Vec<i32>>();

        assert!(dir_sizes == [94853, 584, 0]);
    }
}
