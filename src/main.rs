use std::path::Path;

use clap::Parser;
use esplugin::{GameId, ParseOptions, Plugin, plugin::RecordIds, plugins_metadata};

const GAME_ID: GameId = GameId::Morrowind;

#[derive(Parser, Debug)]
#[command(version, about = "A simple CLI tool example", long_about = None)]
struct Args {
    /// ESP file to isolate resources from
    #[arg(default_value = "i:\\SteamLibrary\\steamapps\\common\\Morrowind\\Data Files\\tr_f_fresco_flower_01.ESP")]
    file: String,
}

fn print_records(records: &RecordIds) {
    match records {
        RecordIds::None => println!("No records found."),
        RecordIds::FormIds(ids) => {
            println!("Found {} FormIDs:", ids.len());
            for id in ids {
                println!("  - {}", id);
            }
        }

        RecordIds::NamespacedIds(ids) => {
            println!("Found {} Namespaced IDs:", ids.len());
            for id in ids {
                println!("  - {:?}", id);
            }
        }

        RecordIds::Resolved(ids) => {
            println!("Found {} Resolved Record IDs:", ids.len());
            for id in ids {
                println!("  - {:?}", id);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let plugin_path = Path::new(&args.file).parent().unwrap();
    let mut plugin = Plugin::new(GAME_ID, Path::new(&args.file));

    plugin.parse_file(ParseOptions::whole_plugin()).unwrap();

    // Load each master plugin too
    let plugins = plugin
        .masters()?
        .iter()
        .map(|master| {
            let master_path = plugin_path.join(Path::new(master));
            Plugin::new(GAME_ID, &master_path)
        })
        .collect::<Vec<_>>();

    // Convert plugins to &[&Plugin] for plugins_metadata
    let plugins_ref = plugins.iter().collect::<Vec<_>>();

    let metadata = plugins_metadata(&plugins_ref).unwrap();

    println!("Before resolving");
    print_records(&plugin.records());

    plugin.resolve_record_ids(metadata.as_slice()).unwrap();

    println!("After resolving");
    print_records(&plugin.records());

    //println!("Description: {:#?}", plugin.header());

    for subrecord in plugin.header().subrecords() {
        println!(
            "Subrecord: \n\ttype={:?}, \n\tdata={:?}, \n\tdata_str={:?}",
            subrecord.subrecord_type_str(),
            subrecord.data(),
            std::str::from_utf8(&subrecord.data()).unwrap_or("<invalid utf8>")
        );
    }

    Ok(())
}
