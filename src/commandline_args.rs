use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CommandlineArgs {
    /// The Project ID
    #[arg(short, long, conflicts_with = "file", required_unless_present = "file")]
    pub id: Option<u64>,

    /// The input pack archive file
    #[arg(short, long, conflicts_with = "id", required_unless_present = "id")]
    pub file: Option<String>,

    /// This is the output zip path
    #[arg(short, long, default_value = "output")]
    pub output: Option<String>,

//     This will only download serverside mods
//    #[arg(long)]
//    pub server_only: bool,
}
