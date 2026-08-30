use std::path::Path;

use clap::Parser;
use egg_esp_lib::record::{Record, RecordHeader, RecordType, SubRecord, parse_records, types::*};

#[derive(Parser, Debug)]
#[command(version, about = "A simple CLI tool example", long_about = None)]
struct Args {
    /// ESP file to isolate resources from
    #[arg(
        default_value = "i:\\SteamLibrary\\steamapps\\common\\Morrowind\\Data Files\\tr_f_fresco_flower_01.ESP"
    )]
    file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let plugin_path = Path::new(&args.file).parent().unwrap();
    let plugin_data = std::fs::read(&args.file)?;

    let records = parse_records(&plugin_data)?;

    println!("Parsed {} records from the ESP file.", records.len());
    // for record in &records {
    //     println!("Record header: {:?}", record.header);
    //     for subrecord in &record.subrecords {
    //         println!("\tSubrecord type: {:?}", subrecord.record_type);
    //         println!("\tSubrecord data: {:?}", subrecord.data);
    //     }
    // }

    for record in &records {
        if let Ok(stat) = egg_esp_lib::record::types::Static::try_from(record.clone()) {
            println!("Found a Static record: {:?}", stat);

            let nif_file = plugin_path.join(Path::new("meshes")).join(&stat.model.path);

            println!("NIF file path: {:?}", nif_file);

            niflib::get_nif_texture_filepaths(nif_file.to_str().unwrap())
                .iter()
                .for_each(|texture| {
                    println!("Texture: {}", texture);
                });
        }
    }

    Ok(())
}
