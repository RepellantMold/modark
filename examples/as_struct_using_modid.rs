use std::env;

use modark::ModInfo;

fn main() {
    let key = &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables");
    let modinfo = ModInfo::get(51772, key).unwrap();
    println!("{:#?}", modinfo);
}
