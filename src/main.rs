//Local Imports
mod args;

//Function Imports
use args::{CliTool, EntityType};
use clap::Parser;
use colored::*;
use regex::Regex;
use std::fs::{self, DirEntry, File};
use std::io::{self, BufRead, Read, Result};
use walkdir::WalkDir;

fn main() {
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
                Ok(created_file_name) => {
                    println!(
                        "{} \nFile Name: {}",
                        "File Created".green(),
                        created_file_name.red()
                    );
                }
                Err(e) => eprintln!("Error: {:?}", e),
            }
        }
    };
}

fn echo_function(some_vector: &Option<Vec<String>>) -> Result<()> {
    match some_vector {
        Some(vector) => {
            println!("{}", vector.join(" ").green())
        }
        None => println!("Nothing goin on here"),
    }
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
                    println!("{:?}", entry.file_name().to_string_lossy());
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

fn concatenate_function(is_directory: bool, files: &Option<Vec<String>>) -> Result<()> {
    let mut contents = String::new();
    match files {
        Some(some_file) => {
            if is_directory {
                let some_file_clone = some_file.clone();
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
                for file in some_file {
                    let mut f = File::open(file)?;
                    let mut temp_content = String::new();
                    f.read_to_string(&mut temp_content)?;
                    contents.push_str(&temp_content);
                }
                fs::write("concat_file.txt", contents)?;
            }
        }
        None => println!("No files are available"),
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

fn grep_function(pattern: &str, filename: &str) -> Result<()> {
    let re = Regex::new(pattern).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line) {
            println!("Line - {}: {}", index + 1, line);
        }
    }

    Ok(())
}

fn create_function(is_directory: bool, file_name: String) -> Result<String> {
    if is_directory {
        println!("{}", "I dunno, create a directory".red());
    } else {
        File::create_new(&file_name)?;
    }
    Ok(file_name)
}
