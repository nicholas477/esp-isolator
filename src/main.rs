use std::path::{Path, PathBuf};

use ::log::{error, info};
use clap::Parser;
use std::collections::hash_set::HashSet;
use tes3::esp::{Plugin, Static};
use tes3::nif::TextureSource::External;
use tes3::nif::{NiSourceTexture, NiStream};
use zip::{CompressionMethod, write::FileOptions};

mod log;
mod update;

#[derive(Parser, Debug)]
#[command(version, about = "Grabs the statics from an ESP file, then packages the meshes, textures, and the ESP file into a single zip.", long_about = None)]
struct Args {
    /// ESP file to isolate meshes and textures from
    #[arg(required_unless_present = "update", conflicts_with = "update")]
    file: Option<PathBuf>,

    /// (Optional) Output file path. If not specified, the zip file will be created in the same directory as the input ESP file.
    #[arg(
        short,
        long,
        required_unless_present = "update",
        conflicts_with = "update"
    )]
    output: Option<PathBuf>,

    /// Update the program to the latest version. If specified, the file and output arguments will be ignored.
    #[arg(short, long)]
    update: bool,
}

fn add_file(files: &mut HashSet<String>, file_path: &str) -> bool {
    if !file_path.is_empty() {
        files.insert(file_path.to_string())
    } else {
        false
    }
}

fn add_file_path(files: &mut HashSet<String>, file_path: &Path) -> bool {
    if !file_path.is_empty()
        && let Some(path) = file_path.to_str().map(|s| s.to_string())
    {
        return files.insert(path);
    }

    false
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
            error!("Warning: File not found: {}", path.display());
            return Err(format!("File not found: {}", path.display()).into());
        }
    }

    // Redundant since the zip is finished when the ZipWriter is dropped
    zip.finish()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::init_logger();

    let args = Args::parse();
    if args.update {
        info!("Checking for updates...");
        update::update()?;
        return Ok(());
    }

    // This should be fine since the file argument is required unless update is specified, and we already checked for update above
    let input_file = args.file.unwrap();

    if !input_file.is_file() {
        error!(
            "Error: The provided path is not a file: \"{}\"",
            input_file.display()
        );
        std::process::exit(1);
    }

    if !input_file.exists() {
        error!(
            "Error: The provided file does not exist: \"{}\"",
            input_file.display()
        );
        std::process::exit(1);
    }

    // output path
    let zip_path = args
        .output
        .unwrap_or(Path::new(&input_file).with_extension("zip"));

    // Insert the ESP file itself into the set of files to be zipped
    let mut files = HashSet::new();
    add_file(
        &mut files,
        Path::new(&input_file)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
    );

    let plugin_path = input_file.parent().unwrap();
    let plugin = Plugin::from_path(&input_file)
        .inspect_err(|e| error!("Failed to load plugin {}! Error: {e}", input_file.display()))?;

    // Collect all of the mesh references (and their textures) in the ESP file and add them to the set of files to be zipped
    for object in plugin.objects_of_type::<Static>() {
        let path = Path::new("meshes").join(&object.mesh);
        add_file_path(&mut files, &path);

        let full_path = plugin_path.join(&path);
        if !full_path.exists() {
            error!(
                "Error: ESP references nonexistent mesh file: {}",
                path.display()
            );
            error!("Looked for mesh at path: {}", full_path.display());
            return Err(format!("ESP references nonexistent mesh file: {}", path.display()).into());
        }

        let mut stream = NiStream::new();
        stream.load_path(&full_path)?;
        for object in stream.objects_of_type::<NiSourceTexture>() {
            if let External(file_name) = &object.source {
                let tex_path = Path::new(file_name);

                let tex_full_path = plugin_path.join(tex_path);
                if !tex_full_path.exists() {
                    error!(
                        "Error: Nif file {} references nonexistent texture file: {}",
                        path.display(),
                        tex_path.display()
                    );

                    error!("Looked for texture at path: {}", full_path.display());
                    return Err(format!(
                        "Nif file {} references nonexistent texture file: {}",
                        path.display(),
                        tex_path.display()
                    )
                    .into());
                }

                add_file_path(&mut files, tex_path);
            }
        }
    }

    {
        info!("-- Zipping {} files:", files.len());
        let mut files = files.iter().collect::<Vec<&String>>();
        files.sort();
        for file in &files {
            info!("\t-- {file}");
        }
    }

    info!("Creating zip file at: \"{}\"", zip_path.display());

    zip_files(&files, plugin_path, &zip_path)?;

    info!(
        "Zip file created successfully at: \"{}\"",
        zip_path.display()
    );

    Ok(())
}
