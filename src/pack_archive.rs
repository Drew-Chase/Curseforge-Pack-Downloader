use crate::pack_manifest::Manifest;
use log::error;
use std::env::temp_dir;
use std::error::Error;
use std::fs;
use std::fs::remove_dir;
use std::path::{Path, PathBuf};

pub async fn process_archive(
    zip_path: impl AsRef<Path>,
    parallel: u8,
    validate: bool,
    validate_if_size_less_than: Option<u64>,
) -> Result<(PathBuf, Manifest), Box<dyn Error>> {
    let path = match extract_zip(zip_path) {
        Ok(path) => path,
        Err(err) => {
            error!("Failed to extract zip archive: {}", err);
            std::process::exit(1);
        }
    };
    let mods_dir = path.join("mods");
    let manifest_file = path.join("manifest.json");
    if !manifest_file.exists() {
        error!("Manifest file not found!");
        std::process::exit(1);
    }

    let manifest = Manifest::new(manifest_file)?;
    if let Err(e) = manifest.download_mods(&mods_dir, parallel, validate,validate_if_size_less_than).await {
        if remove_dir(mods_dir).is_err() {
            error!("Failed to remove mods directory after failure");
        }
        error!("Failed to download mods: {}", e);
        std::process::exit(1);
    }

    Ok((path, manifest))
}

fn extract_zip(zip_path: impl AsRef<Path>) -> Result<PathBuf, Box<dyn Error>> {
    let tmp_dir = temp_dir().join(format!(
        "unfuck-curseforge-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)?
            .as_millis()
    ));
    let zip_file = fs::File::open(zip_path)?;
    let mut zip_archive = zip::ZipArchive::new(zip_file)?;
    zip_archive.extract(tmp_dir.join(""))?;
    Ok(tmp_dir)
}

pub fn copy_to_output(
    mods_dir: impl AsRef<Path>,
    overrides_dir: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(&output_path)?;
    let new_mods = output_path.as_ref().join("mods");
    fs::create_dir_all(&new_mods)?;
    copy_dir_recursive(mods_dir, &new_mods)?;
    if overrides_dir.as_ref().exists() {
        copy_dir_recursive(overrides_dir, &output_path)?;
    }
    Ok(output_path.as_ref().to_path_buf())
}

fn copy_dir_recursive(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> std::io::Result<()> {
    let src = src.as_ref();
    let dest = dest.as_ref();
    if src.is_dir() {
        fs::create_dir_all(dest)?;

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            if file_type.is_dir() {
                copy_dir_recursive(&src_path, &dest_path)?;
            } else {
                fs::copy(&src_path, &dest_path)?;
            }
        }
    }
    Ok(())
}
