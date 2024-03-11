use std::env;

use modark::ModInfo;

fn main() {
    let key = &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables");
    let modid = ModInfo::resolve_filename("noway.s3m").unwrap()[0].id;
    let modinfo = ModInfo::get(modid, key).unwrap();
    println!("{:#?}", modinfo);
}
