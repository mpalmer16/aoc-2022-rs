#[allow(dead_code)]
pub fn apply_command(directory: &mut Directory, command: &Command) {
    match command {
        Command::LS(files) => {
            for file in files {
                match file {
                    FileType::Dir(location) => directory.directories.push(location.to_string()),
                    FileType::File((size, name)) => directory.files.push((*size, name.to_string())),
                }
            }
        }
        Command::CD(_) => {}
    }
}

#[allow(dead_code)]
pub fn apply_command_to_directories(
    directories: &mut Vec<Directory>,
    command: &Command,
    current_idx: usize,
) -> usize {
    match command {
        Command::CD(location) => {
            if location == "../" {
                return current_idx - 1;
            }
            // what about dirs with the same name in different places? nested dirs
            for (idx, directory) in directories.iter().enumerate() {
                if directory.name == *location {
                    return idx;
                }
            }

            directories.push(Directory::new(location));
            current_idx + 1
        }
        Command::LS(_) => current_idx,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    pub name: String,
    pub directories: Vec<String>,
    pub files: Vec<(i32, String)>,
}
#[allow(dead_code)]
impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            directories: vec![],
            files: vec![],
        }
    }

    pub fn size(&self, directories: &Vec<Directory>) -> i32 {
        let file_size = self.files.iter().map(|f| f.0).sum::<i32>();
        let mut dir_size = 0;
        for name in &self.directories {
            if name != &self.name {
                dir_size += directories
                    .iter()
                    .filter(|d| &d.name == name)
                    .take(1)
                    .map(|d| d.size(directories))
                    .sum::<i32>()
            }
        }
        file_size + dir_size
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    CD(String),
    LS(Vec<FileType>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileType {
    Dir(String),
    File((i32, String)),
}
#[allow(dead_code)]
pub fn transform(s: String) -> Vec<String> {
    s.split("$ ")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
}
#[allow(dead_code)]
pub fn parse_command(s: String) -> Command {
    let parts = s
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if parts.len() == 1 {
        let location = parts
            .first()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .to_string();
        Command::CD(location)
    } else {
        let files = parts
            .iter()
            .skip(1)
            .map(|s| s.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
            .map(|v| {
                if v[0] == "dir" {
                    FileType::Dir(v[1].to_string())
                } else {
                    FileType::File((
                        v[0].parse::<i32>().expect("can not parse file size"),
                        v[1].to_string(),
                    ))
                }
            })
            .collect::<Vec<FileType>>();

        Command::LS(files)
    }
}
