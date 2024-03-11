use std::env;

use modark::ModInfo;

fn main() {
    let key = &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables");
    let modid = ModInfo::resolve_filename("noway.s3m").unwrap()[0].id;
    let modtext = ModInfo::get(modid, key).unwrap();
    println!("{}", modtext.instrument_text);
}
