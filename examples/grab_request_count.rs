use std::env;

use modark::ModInfo;

fn main() {
        let key = &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables");    
        let request_count = ModInfo::track_requests(key);

        match request_count {
            Ok(count) => println!("{}", count),
            Err(_) => eprintln!("Error"),
        };
}
    