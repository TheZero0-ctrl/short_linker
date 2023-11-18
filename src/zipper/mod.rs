use std::io::prelude::*;
use std::io::{Seek, Write};
use std::iter::Iterator;
use colored::Colorize;
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub fn zip_folder(folder_path: &str) -> zip::result::ZipResult<()> {
    println!("{}", "Zipping files ...".yellow());
    const METHOD_STORED: Option<zip::CompressionMethod> = Some(
        zip::CompressionMethod::Stored,
    );
    let path = Path::new(folder_path);
    if !path.is_dir() {
        return Err(ZipError::FileNotFound);
    }
    let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
    let mut ancestors = path.ancestors();
    ancestors.next();
    let dist_dir = ancestors.next().unwrap().join(file_name + ".zip");


    let file = File::create(dist_dir.clone()).unwrap();
    let walkdir = WalkDir::new(folder_path);
    let it = walkdir.into_iter();
    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        &dist_dir.to_string_lossy(),
        file,
        METHOD_STORED.unwrap(),
        folder_path
    )?;

    Ok(())
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    _prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
    folder_path: &str
) -> zip::result::ZipResult<()>
where T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(folder_path).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!(
                "adding file {} as {} ...", 
                path.to_string_lossy().blue(),
                name.to_string_lossy().blue()
            );
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

