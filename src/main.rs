use std::path::Path;

use clap::Parser;
use egg_esp_lib::record::{Record, RecordHeader, RecordType, SubRecord, parse_records};
// use esplugin::{GameId, ParseOptions, Plugin, plugin::RecordIds, plugins_metadata};

//const GAME_ID: GameId = GameId::Morrowind;

#[derive(Parser, Debug)]
#[command(version, about = "A simple CLI tool example", long_about = None)]
struct Args {
    /// ESP file to isolate resources from
    #[arg(default_value = "i:\\SteamLibrary\\steamapps\\common\\Morrowind\\Data Files\\tr_f_fresco_flower_01.ESP")]
    file: String,
}

// fn print_records(records: &RecordIds) {
//     match records {
//         RecordIds::None => println!("No records found."),
//         RecordIds::FormIds(ids) => {
//             println!("Found {} FormIDs:", ids.len());
//             for id in ids {
//                 println!("  - {}", id);
//             }
//         }

//         RecordIds::NamespacedIds(ids) => {
//             println!("Found {} Namespaced IDs:", ids.len());
//             for id in ids {
//                 println!("  - {:?}", id);
//             }
//         }

//         RecordIds::Resolved(ids) => {
//             println!("Found {} Resolved Record IDs:", ids.len());
//             for id in ids {
//                 println!("  - {:?}", id);
//             }
//         }
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    //let plugin_path = Path::new(&args.file).parent().unwrap();
    let plugin_data = std::fs::read(&args.file)?;

    let records = parse_records(&plugin_data)?;

    println!("Parsed {} records from the ESP file.", records.len());
    for record in records {
        println!("Record header: {:?}", record.header);
        println!("Subrecord: {:?}", record.subrecords);
    }

    Ok(())
}
