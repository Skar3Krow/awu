#[derive(Debug)]
pub enum Command {
    /// Repeats User Input
    Echo(Repeat),
    /// Lists all the files and directories
    List(ListArgs),
    /// Concatenates files
    Cat(CatArgs),
    /// Finds file if exists
    Find(FindArgs),
    /// Matches text in  files
    Grep(GrepArgs),
    /// Creates a directory and files
    Create(CreateArgs),
    /// Exit
    Exit(),
    /// Un-written Command
    Unknown(String),
}

#[derive(Debug)]
pub struct Repeat {
    /// The sentence to be repeated
    pub repeated_vector: Vec<String>,
}

#[derive(Debug)]
pub struct ListArgs {
    /// Lists the directory
    pub directory: String,
    /// Lists all the hidden files as
    pub all: bool,
    /// Lists in a long listing format
    pub long: bool,
}

#[derive(Debug)]
pub struct CatArgs {
    /// Concatenates a directory
    pub dir: bool,
    /// Takes n number of files as input
    pub files: Vec<String>,
}

#[derive(Debug)]
pub struct FindArgs {
    /// Area where the file needs to be searched
    pub dir_name: String,
    /// File to be searched
    pub file_name: String,
}

#[derive(Debug)]
pub struct GrepArgs {
    /// Text to be matched
    pub match_text: String,
    /// File in which you want to match text
    pub file_name: Vec<String>,
}

#[derive(Debug)]
pub struct CreateArgs {
    /// Create a directory
    pub directory: bool,
    /// File Name
    pub file_name: String,
}

/*
#[derive(Debug)]
pub struct Exit {
    exit_status: bool,
}
*/

pub fn parse_command(input: &str) -> Command {
    let mut parts = input
        .split_whitespace()
        .map(String::from)
        .collect::<Vec<_>>();

    if parts.is_empty() {
        return Command::Unknown("empty".into());
    }

    let cmd = parts.remove(0);

    match cmd.as_str() {
        "echo" => Command::Echo(Repeat {
            repeated_vector: parts,
        }),

        "list" => {
            let mut all = false;
            let mut long = false;
            let mut directory = ".".into();

            for p in &parts {
                match p.as_str() {
                    "-a" | "--all" => all = true,
                    "-l" | "--long" => long = true,
                    _ => directory = p.clone(),
                }
            }

            Command::List(ListArgs {
                directory,
                all,
                long,
            })
        }

        "cat" => {
            let mut dir = false;
            let mut files: Vec<String> = Vec::new();

            for p in parts {
                match p.as_str() {
                    "-d" | "--dir" => dir = true,
                    _ => files.push(p),
                }
            }

            Command::Cat(CatArgs { dir, files })
        }

        "find" => {
            if parts.len() < 2 {
                return Command::Unknown("find needs 2 args".into());
            }

            Command::Find(FindArgs {
                dir_name: parts[0].clone(),
                file_name: parts[1].clone(),
            })
        }

        "grep" => {
            if parts.is_empty() {
                return Command::Unknown("grep needs 1+ args".into());
            }

            let match_text = parts.remove(0);
            let file_name = parts;

            Command::Grep(GrepArgs {
                match_text,
                file_name,
            })
        }

        "create" => {
            let mut directory = false;
            let mut file_name = "".to_string();

            for p in parts {
                match p.as_str() {
                    "-d" | "--directory" => directory = true,
                    _ => file_name = p,
                }
            }

            Command::Create(CreateArgs {
                directory,
                file_name,
            })
        }

        "exit" => Command::Exit(),

        other => Command::Unknown(other.into()),
    }
}
