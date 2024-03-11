# Modark

[![license](https://img.shields.io/github/license/repellantmold/modark?style=plastic)](LICENSE)
[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=plastic)](https://github.com/RichardLitt/standard-readme)
[![Crates.io](https://img.shields.io/crates/v/modark?style=plastic)](https://crates.io/crates/modark)
![Crates.io](https://img.shields.io/crates/d/modark?style=plastic)

This is a simple library crate that helps with scraping metadata via from the website called [Mod Archive](https://modarchive.org) directly using [the XML API](https://modarchive.org/index.php?xml-api).
This project is forked from [trackermeta by vivyir](https://github.com/vivyir/trackermeta) which has the same general idea but uses HTML parsing instead.

## Table of Contents

- [Install](#install)
- [Usage](#Usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Install

It's basically just like any other Rust crate.

```sh
cargo add modark
```

## Usage

> [!IMPORTANT]
> You'll have to [request an API key from the Mod Archive forums](https://modarchive.org/forums/index.php?topic=1950.0) before you attempt to use this crate, otherwise you'll always get an invalid request error. **Please be sure to also donate to [the Mod Archive's hosting fund](https://www.paypal.com/cgi-bin/webscr?cmd=_s-xclick&hosted_button_id=28NK9DJQRRNGJ) if you use this for any significant amount of time!**

In the following example the program will search for the file name provided by the user and display the data for the closest match.

```rust
use modark::ModInfo;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).unwrap_or(&"".into()).as_ref() {
        "get" => {
            // Returns the first 40 search results, here we'll pick the closest match, if none exist this will panic!
            let mod_id = ModInfo::resolve_filename(
                args.get(2)
                    .expect("No filename provided as second argument."),
            )
            .unwrap()[0].id;

            let mod_info = ModInfo::get(mod_id).unwrap();

            println!("{:#?}", mod_info);
            println!("Download link: {}", mod_info.get_download_link());
        }
        _ => println!("Usage: trackermeta get <filename>"),
    }
}
```

Check out the [examples](examples) directory on the GitHub repo for all examples using the library!

## Maintainers

[@RepellantMold](https://github.com/RepellantMold)

## Contributing

PRs accepted.

Small note: If editing the README, please conform to the
[standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

[MPL 2.0 © 2024 RepellantMold](LICENSE)
