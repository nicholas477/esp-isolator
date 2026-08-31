use std::path::{Path, PathBuf};

use clap::Parser;
use egg_esp_lib::record::parse_records;
use std::collections::hash_set::HashSet;
use zip::{CompressionMethod, write::FileOptions};

#[derive(Parser, Debug)]
#[command(version, about = "Grabs the statics from an ESP file, then packages the meshes, textures, and the ESP file into a single zip.", long_about = None)]
struct Args {
    /// ESP file to isolate meshes and textures from
    file: PathBuf,

    /// (Optional) Output file path. If not specified, the zip file will be created in the same directory as the input ESP file.
    #[arg(short, long)]
    output: Option<PathBuf>,
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

        add_file(
            files,
            Path::new("meshes").join(&stat.model.path).to_str().unwrap(),
        );

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
        } else {
            eprintln!("Warning: File not found: {}", path.display());
            return Err(format!("File not found: {}", path.display()).into());
        }
    }

    // Redundant since the zip is finished when the ZipWriter is dropped
    zip.finish()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.file.is_file() {
        eprintln!(
            "Error: The provided path is not a file: \"{}\"",
            args.file.display()
        );
        std::process::exit(1);
    }

    let zip_path = args
        .output
        .unwrap_or(Path::new(&args.file).with_extension("zip"));

    let plugin_path = args.file.parent().unwrap();
    let plugin_data = std::fs::read(&args.file)?;

    let records = parse_records(&plugin_data)
        .inspect_err(|e| eprintln!("Failed to parse records from file: {}", e))?;

    // Insert the ESP file itself into the set of files to be zipped
    let mut files = HashSet::new();
    files.insert(
        Path::new(&args.file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );

    collect_files_from_records(&records, plugin_path, &mut files);

    {
        println!("-- Zipping {} files:", files.len());
        let mut files = files.iter().collect::<Vec<&String>>();
        files.sort();
        for file in &files {
            println!("\t-- {}", file);
        }
    }

    println!("Creating zip file at: \"{}\"", zip_path.display());

    zip_files(&files, plugin_path, &zip_path)?;

    println!(
        "Zip file created successfully at: \"{}\"",
        zip_path.display()
    );

    Ok(())
}
