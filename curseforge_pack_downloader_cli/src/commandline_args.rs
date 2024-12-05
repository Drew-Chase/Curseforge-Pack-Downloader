use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[clap(
    about,
    name = "Curseforge Modpack Downloader",
    version,
    author,
    long_about = "A command line tool for downloading CurseForge modpacks"
)]
pub struct CommandlineArgs {
    /// The Project ID
    #[arg(short, long, conflicts_with = "file", required_unless_present = "file")]
    pub id: Option<u64>,

    /// The input pack archive file
    #[arg(short, long, conflicts_with = "id", required_unless_present = "id")]
    pub file: Option<String>,

    /// Where the downloaded finalized pack will output.
    ///
    /// You can use `%PACK_NAME%` as a variable for the pack name.
    /// For example: `./packs/%PACK_NAME%` will result in `./packs/All the Mods 10`.
    ///
    /// Here is a list of all possible variables:
    ///
    /// - %PACK_NAME%: the name of the modpack
    ///
    /// - %PACK_VERSION%: the version of the modpack
    ///
    /// - %PACK_AUTHOR%: the primary author of the modpack
    ///
    /// - %TIME%: the current time in milliseconds (great for creating unique paths)
    #[arg(short, long, default_value = "./%PACK_NAME%-%PACK_VERSION%-%TIME%")]
    pub output: PathBuf,

    /// This will validate the downloaded mods based on the provided hash. (Note: this can take significantly longer)
    #[arg(long)]
    pub validate: bool,

    /// The number of mods that will be downloaded in parallel
    /// This can allow for much faster downloading.
    /// Higher may not be better, due to internet speeds.
    /// For unlimited downloads put '0' (not recommended!)
    #[arg(short, long, default_value_t = 16, value_name = "NUMBER")]
    pub parallel_downloads: u8,

    /// This will only attempt to validate files where the file size is less than this value (in bytes)
    #[arg(long, requires = "validate", value_name = "BYTES")]
    pub validate_if_size_less_than: Option<u64>,
}
