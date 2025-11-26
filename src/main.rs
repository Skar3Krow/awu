//Local Imports
mod args;
mod lexer;
mod tokens;

//Function Imports
use crate::args::{parse_command, Command};
use colored::*;
use regex::Regex;
use std::fs::{self, create_dir, DirEntry, File};
use std::io::{self, BufRead, Read, Result};
use std::path::Path;
use std::sync::mpsc;
use threadpool::{self, ThreadPool};
use walkdir::WalkDir;

fn main() {
    loop {
        print!("awu> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let command = parse_command(&input.trim());
        match command {
            Command::Echo(some_string) => match echo_function(&some_string.repeated_vector) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            },
            Command::List(list_args) => {
                match list_function(&list_args.directory, list_args.all, list_args.long) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Command::Cat(cat_args) => match concatenate_function(cat_args.dir, &cat_args.files) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {}", e),
            },
            Command::Find(find_args) => {
                match find_file_function(&find_args.dir_name, &find_args.file_name) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Command::Grep(grep_args) => {
                match grep_function(&grep_args.match_text, &grep_args.file_name) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Command::Create(create_args) => {
                match create_function(create_args.directory, create_args.file_name) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Command::Exit() => {
                break;
            }
            Command::Unknown(some_string) => {
                println!("Function {} is not yet written dumbass...", some_string)
            }
        }
    }
    /*
    let matches = CliTool::parse();
    match matches.entity_type {
        EntityType::Echo(some_string) => match echo_function(&some_string.repeated_vector) {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {}", e),
        },
        EntityType::List(some_argument) => match list_function(
            &some_argument.directory,
            some_argument.all,
            some_argument.long,
        ) {
            Ok(_) => (),
            Err(e) => eprintln!("Error : {:?}", e),
        },
        EntityType::Cat(cat_argument) => {
            match concatenate_function(cat_argument.dir, &cat_argument.files) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        EntityType::Find(find_argument) => {
            match find_file_function(&find_argument.dir_name, &find_argument.file_name) {
                Ok(_) => (),
                Err(e) => eprintln!("Error : {}", e),
            }
        }
        EntityType::Grep(grep_argument) => {
            match grep_function(&grep_argument.match_text, &grep_argument.file_name) {
                Ok(_) => (),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
        EntityType::Create(create_argument) => {
            match create_function(create_argument.directory, create_argument.file_name) {
                Ok(()) => (),
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    };
    */
}

fn echo_function(some_vector: &Vec<String>) -> Result<()> {
    println!("{}", some_vector.join(" ").green());
    Ok(())
}

fn list_function(directory: &str, display_all: bool, long_format: bool) -> Result<()> {
    match fs::read_dir(directory) {
        Ok(paths) => {
            for path in paths {
                let entry = path?;
                if !display_all && hidden_file(&entry) {
                    continue;
                }
                if long_format {
                    match print_long_format(&entry) {
                        Ok(_) => (),
                        Err(e) => println!("Error Occured: {:?}", e),
                    };
                } else {
                    println!("{}", entry.file_name().to_string_lossy());
                }
            }
        }
        Err(e) => eprintln!("Error Occured {:?}", e),
    };
    Ok(())
}

fn hidden_file(entry: &DirEntry) -> bool {
    entry.file_name().to_string_lossy().starts_with('.')
}

fn print_long_format(entry: &DirEntry) -> Result<()> {
    let file_metadata = entry.metadata()?;
    let file_type = if file_metadata.is_dir() { "d" } else { "f" };
    let file_size = file_metadata.len();
    println!(
        "{:<3} {:<5} {}",
        file_type.red(),
        file_size,
        entry.file_name().to_string_lossy().green()
    );
    Ok(())
}

fn concatenate_function(is_directory: bool, files: &Vec<String>) -> Result<()> {
    let mut contents = String::new();
    if is_directory {
        let some_file_clone = files.clone();
        let paths = fs::read_dir(&some_file_clone[0])?;
        for path in paths {
            let entry = path?;
            let mut f = File::open(entry.file_name())?;
            let mut temp_content = String::new();
            f.read_to_string(&mut temp_content)?;
            contents.push_str(&temp_content);
        }
        fs::write("concat_dir_file.txt", contents)?;
    } else {
        for file in files {
            let mut f = File::open(file)?;
            let mut temp_content = String::new();
            f.read_to_string(&mut temp_content)?;
            contents.push_str(&temp_content);
        }
        fs::write("concat_file.txt", contents)?;
    }

    Ok(())
}

fn find_file_function(dir_name: &String, file_name: &str) -> Result<()> {
    for entry in WalkDir::new(dir_name).into_iter().filter_map(|e| e.ok()) {
        println!("{:?}", entry.path().display());
        if entry.file_name().to_str() == Some(file_name) {
            println!("{}", "File Found".green());
            break;
        }
    }
    Ok(())
}

fn grep_function(pattern: &str, files: &Vec<String>) -> Result<()> {
    let pool = ThreadPool::new(4);
    let (tx, rx) = mpsc::channel::<String>();
    let re = Regex::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    for file in files {
        let tx = tx.clone();
        let re = re.clone();
        let file = file.clone();
        pool.execute(move || {
            let f = match File::open(&file) {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("File could not be opened...");
                    return;
                }
            };
            let reader = io::BufReader::new(f);
            for (index, line) in reader.lines().enumerate() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => continue,
                };
                if re.is_match(&line) {
                    tx.send(format!("Line - {}: {}", index + 1, line)).unwrap();
                }
            }
        });
    }
    drop(tx);
    for msg in rx {
        println!("{}", msg);
    }

    Ok(())
}

fn create_function(is_directory: bool, file_name: String) -> Result<()> {
    if is_directory {
        let dir_path = Path::new(&file_name);
        create_dir(dir_path)?;
        println!("Directory : {} has been created", &file_name.green());
    } else {
        File::create_new(&file_name)?;
        println!("File {} has been created", file_name.green());
    }
    Ok(())
}

// Tests
#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::Token;

    #[test]
    fn lexer_parsing_test_1() {
        let mut lexer = Lexer::new(r#"grep "hello world" < in.txt | sort > out.txt &"#);
        loop {
            let tok = lexer.next_token();
            println!("{:?}", tok);
            if tok == Token::EOF {
                break;
            }
        }
    }
}
