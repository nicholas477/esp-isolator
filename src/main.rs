use std::path::Path;

use clap::Parser;
use egg_esp_lib::record::parse_records;
use std::collections::hash_set::HashSet;
use zip::{CompressionMethod, write::FileOptions};

#[derive(Parser, Debug)]
#[command(version, about = "A simple CLI tool example", long_about = None)]
struct Args {
    /// ESP file to isolate resources from
    file: String,
}

fn add_file(files: &mut HashSet<String>, file_path: &str) -> bool {
    if !file_path.is_empty() {
        files.insert(file_path.to_string())
    } else {
        false
    }
}

fn collect_files_from_record(
    record: &egg_esp_lib::record::Record,
    plugin_path: &Path,
    files: &mut HashSet<String>,
) {
    if let Ok(stat) = egg_esp_lib::record::types::Static::try_from(record.clone()) {
        println!("Found a Static record: {:?}", stat);

        let nif_file = plugin_path.join(Path::new("meshes")).join(&stat.model.path);

        add_file(files, Path::new("meshes").join(&stat.model.path).to_str().unwrap());

        niflib::get_nif_texture_filepaths(nif_file.to_str().unwrap())
            .iter()
            .for_each(|texture| {
                add_file(files, texture);
            });
    }
}

fn collect_files_from_records(
    records: &[egg_esp_lib::record::Record],
    plugin_path: &Path,
    files: &mut HashSet<String>,
) {
    for record in records {
        collect_files_from_record(record, plugin_path, files);
    }
}

fn zip_files(
    files: &HashSet<String>,
    data_files_path: &Path,
    output_zip_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(output_zip_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let options: FileOptions<'_, ()> =
        FileOptions::default().compression_method(CompressionMethod::DEFLATE);

    for file_path in files {
        let path = data_files_path.join(Path::new(file_path));
        if path.is_file() {
            zip.start_file(file_path, options)?;
            let mut f = std::fs::File::open(path)?;
            std::io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let plugin_path = Path::new(&args.file).parent().unwrap();
    let plugin_data = std::fs::read(&args.file)?;

    let records = parse_records(&plugin_data)?;

    let mut files = HashSet::new();
    files.insert(Path::new(&args.file).file_name().unwrap().to_str().unwrap().to_string());

    collect_files_from_records(&records, plugin_path, &mut files);

    println!("-- Found {} unique files:", files.len());
    for file in &files {
        println!("Found file: {}", file);
    }

    let zip_path = Path::new(&args.file).with_extension("zip");
    println!("Creating zip file at: {:?}", zip_path);

    zip_files(
        &files,
        plugin_path,
        &zip_path,
    )?;

    println!("Zip file created successfully at: {:?}", zip_path);

    Ok(())
}
