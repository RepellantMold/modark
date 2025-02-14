use std::env;

use modark::ModInfo;

fn main() {
    let key = &env::var("MODARCH_KEY")
        .expect("Expected a Mod Archive API key in the environment variables");
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).unwrap_or(&"".into()).as_ref() {
        "get" => {
            let mod_id = ModInfo::resolve_filename(
                args.get(2)
                    .expect("No filename provided as second argument."),
            )
            .unwrap()[0]
                .id;
            let mod_info = ModInfo::get(mod_id, key).unwrap();
            println!("{}", &mod_info.instrument_text);

            println!("\n----------------------------------------\n");

            println!("{:#?}", mod_info);

            println!("\n----------------------------------------\n");

            println!("Download link: {}", mod_info.get_download_link());
        }
        "download" => {
            let mod_id = ModInfo::resolve_filename(
                args.get(2)
                    .expect("No filename provided as second argument."),
            )
            .unwrap()[0]
                .id;
            let mod_info = ModInfo::get(mod_id, key).unwrap();

            let module_bytes = match mod_info.download_module() {
                Ok(module_bytes) => module_bytes,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            println!("Raw bytes:\n{:?}", module_bytes);
        }
        _ => println!("Usage: trackermeta get <filename>"),
    }
}
