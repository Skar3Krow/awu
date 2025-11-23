use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version)]
pub struct CliTool {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
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
}

#[derive(Debug, Args)]
pub struct Repeat {
    /// The sentence to be repeated
    #[arg(default_value = "*cold silence of the universe*")]
    pub repeated_vector: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    /// Lists the directory
    #[arg(default_value = ".")]
    pub directory: String,
    /// Lists all the hidden files as
    #[arg(long, short)]
    pub all: bool,
    /// Lists in a long listing format
    #[arg(long, short)]
    pub long: bool,
}

#[derive(Debug, Args)]
pub struct CatArgs {
    /// Concatenates a directory
    #[arg(long, short)]
    pub dir: bool,
    /// Takes n number of files as input
    pub files: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct FindArgs {
    /// Area where the file needs to be searched
    #[arg(default_value = ".")]
    pub dir_name: String,
    /// File to be searched
    pub file_name: String,
}

#[derive(Debug, Parser)]
pub struct GrepArgs {
    /// Text to be matched
    pub match_text: String,
    /// File in which you want to match text
    pub file_name: Option<Vec<String>>,
}

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Create a directory
    #[arg(short, long)]
    pub directory: bool,
    /// File Name
    pub file_name: String,
}
