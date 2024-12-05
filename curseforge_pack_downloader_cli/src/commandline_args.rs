use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub struct CommandlineArgs {
    /// The Project ID
    #[arg(short, long, conflicts_with = "file", required_unless_present = "file")]
    pub id: Option<u64>,

    /// The input pack archive file
    #[arg(short, long, conflicts_with = "id", required_unless_present = "id")]
    pub file: Option<String>,

    /// This is the output zip path
    #[arg(short, long, default_value = "./")]
    pub output: Option<PathBuf>,

    /// This will validate the downloaded mods based on the provided hash. (Note: this can take significantly longer)
    #[arg(long)]
    pub validate: bool,

    /// The number of mods that will be downloaded in parallel
    /// This can allow for much faster downloading.
    /// Higher may not be better, due to internet speeds.
    #[arg(short, long, default_value_t = 16, value_name = "NUMBER")]
    pub parallel_downloads: u8,

    /// This will only attempt to validate files where the file size is less than this value (in bytes)
    #[arg(long, requires = "validate", value_name = "BYTES")]
    pub validate_if_size_less_than: Option<u64>,
}
