# CurseForge ModPack Downloader

This project allows you to download mods from a CurseForge modpack archive or by providing a project ID. It is a tool independent of CurseForge, designed to simplify the downloading process.

## How to Build

Ensure you have Rust and Cargo installed on your system. If not, you can install them from [rustup.rs](https://rustup.rs/).

1. Clone the repository:
   ```sh
   git clone <repository-url>
   cd <repository-name>
   ```

2. Build the project using Cargo:
   ```sh
   cargo build --release
   ```

The resulting binary will be located in the `target/release` directory.

## How to Run

You can run the program with the following options:

- **By Project ID**: Specify the `--id` option with a CurseForge project ID to download the latest pack archive.
- **From Archive File**: Use the `--file` option with the path to a locally available modpack archive file.

The program also supports additional options:

- `--output`: Specify a custom output directory for the downloaded pack (default is `output`).
- `--server-only`: Indicate that only server-side mods should be downloaded.

### Examples

- **Download using Project ID:**

   ```sh
   ./unfuck-curseforge --id 123456
   ```

  This command will download mods for the specified project ID `123456`.

- **Download from a local archive file:**

   ```sh
   ./unfuck-curseforge --file path/to/modpack.zip
   ```

  This command processes the given `modpack.zip` file to download and organize mods.

- **Download with a specific output directory:**

   ```sh
   ./unfuck-curseforge --id 123456 --output my_output_directory
   ```

  This command downloads the pack and saves it in `my_output_directory`.

- **Download server-side mods only (not yet implemented):**

   ```sh
   ./unfuck-curseforge --id 123456 --server-only
   ```

  This command will only download the server-side mods for the specified project.
