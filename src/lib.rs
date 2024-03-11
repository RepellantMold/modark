//! This is a library crate for working with the [Mod Archive](https://modarchive.org)
//! website via [its XML API](https://modarchive.org/index.php?xml-api). Please check out the documentation for [`ModInfo`] and its methods for more info,
//! do be sure to look at the examples aswell!
//!
//! ## Example
//! ### Get module info as a struct using a module ID
//! ```rust
//! use modark::ModInfo;
//!
//! fn main() {
//!     let modinfo = ModInfo::get(51772).unwrap();
//!     println!("{:#?}", modinfo);
//! }
//! ```
//!
//! ## Example
//! ### Resolve filename to ID then use ID to get the info as struct
//! ```rust
//! use modark::ModInfo;
//!
//! fn main() {
//!     let modid = ModInfo::resolve_filename("noway.s3m").unwrap()[0].id;
//!     let modinfo = ModInfo::get(modid).unwrap();
//!     println!("{:#?}", modinfo);
//! }
//! ```
//!
//! There are more examples other than these which showcase more, remember
//! to check the `examples` directory!
//!
//! [Mod Archive]: https://modarchive.org
#![allow(clippy::needless_doctest_main)]

/// The base URL for the Mod Archive XML API
const BASEURL: &str = "https://modarchive.org/data/xml-tools.php";

use chrono::prelude::{DateTime, Utc};

// https://stackoverflow.com/a/64148190
fn iso8601_time(st: &std::time::SystemTime) -> String {
    let dt: DateTime<Utc> = (*st).into();
    format!("{}", dt.format("%+"))
}

/// Error enum for functions in the crate that return a [`Result`]
#[derive(Debug)]
pub enum Error {
    NotFound,
    ParsingError,
    RequestError,
}

/// Simple struct to represent a search result, id and filename will be provided in each
#[derive(Debug)]
pub struct ModSearch {
    pub id: u32,
    pub filename: String,
}

/// Struct containing all of the info about a module
#[derive(Debug)]
pub struct ModInfo {
    /// The module ID of the module on Mod Archive
    pub id: u32,
    /// The filename of the module
    pub filename: String,
    /// The title of the module
    pub title: String,
    /// The file size of the module, use the
    /// crate `byte-unit` to convert them to other units
    pub size: String,
    /// The MD5 hash of the module file as a string
    pub md5: String,
    /// The format of the module, for example `XM`, `IT`
    /// or `MOD` and more, basically the extension of the
    /// module file
    pub format: String,
    /// Spotlit module or not
    pub spotlit: bool,
    /// Download count of the module at the time of scraping
    pub download_count: u32,
    /// Times the module has been favourited at the time of scraping
    pub fav_count: u32,
    /// The time when it was scraped
    pub scrape_time: String,
    /// The channel count of the module
    pub channel_count: u32,
    /// The genre of the module
    pub genre: String,
    /// The upload date of the module
    pub upload_date: String,
    /// The instrument text of the module
    pub instrument_text: String,
}

fn inner_request(mod_id: u32, api_key: &str) -> Result<String, crate::Error> {
    let body = ureq::get(
        format!(
            "{BASEURL}?key={api_key}&request=view_by_moduleid&query={mod_id}"
        )
        .as_str(),
    )
    .timeout(std::time::Duration::from_secs(60))
    .call();

    match body {
        Ok(body) => Ok(body.into_string().unwrap_or_default()),
        Err(_) => Err(crate::Error::RequestError),
    }
}

impl ModInfo {
    /// (a helper function to make the code more readable)
    fn find_node_text(descendants: &[roxmltree::Node], tag: &str) -> Option<String> {
        descendants.iter()
            .find(|node| node.has_tag_name(tag))
            .and_then(|node| node.text())
            .map(|s| s.to_string())
    }

    /// Probably the singular most important function in this crate, takes a module ID (can be
    /// generated at random, deliberately entered or acquired by resolving a filename and
    /// picking a search result), and then gives you a full [`ModInfo`] struct.
    pub fn get(mod_id: u32, api_key: &str) -> Result<ModInfo, crate::Error> {
        let body = inner_request(mod_id, api_key);

        if body.is_err() {
            return Err(crate::Error::RequestError);
        }

        let body = body.unwrap();

        let id = mod_id;
        let scrape_time = iso8601_time(&std::time::SystemTime::now());

        let xml = roxmltree::Document::parse(&body);

        if xml.is_err() {
            return Err(crate::Error::ParsingError);
        }

        let xml = xml.unwrap();

        let xml_descendants: Vec<_> = xml.descendants().collect();

        if Self::find_node_text(&xml_descendants, "error").is_some() {
            return Err(crate::Error::NotFound);
        }

        let filename = Self::find_node_text(&xml_descendants, "filename").unwrap_or_default();
        let title = Self::find_node_text(&xml_descendants, "title").unwrap_or_default();
        let size = Self::find_node_text(&xml_descendants, "size").unwrap_or_default();
        let md5 = Self::find_node_text(&xml_descendants, "hash").unwrap_or_default();
        let format = Self::find_node_text(&xml_descendants, "format").unwrap_or_default();
        let spotlit = false; // TODO: implement this
        let download_count = Self::find_node_text(&xml_descendants, "hits").unwrap_or_default();
        let fav_count = Self::find_node_text(&xml_descendants, "favoured").unwrap_or_default();
        let channel_count = Self::find_node_text(&xml_descendants, "channels").unwrap_or_default();
        let genre = Self::find_node_text(&xml_descendants, "genretext").unwrap_or_default();
        let upload_date = Self::find_node_text(&xml_descendants, "date").unwrap_or_default();
        let instrument_text = Self::find_node_text(&xml_descendants, "instruments").unwrap_or_default();

        // Cast some of the values to their correct types in the struct
        let download_count = download_count.parse::<u32>().unwrap_or_default();
        let fav_count = fav_count.parse::<u32>().unwrap_or_default();
        let channel_count = channel_count.parse::<u32>().unwrap_or_default();
        

        Ok(ModInfo {
            id,
            filename,
            title,
            size,
            md5,
            format,
            spotlit,
            download_count,
            fav_count,
            scrape_time,
            channel_count,
            genre,
            upload_date,
            instrument_text,
        })
    }

    /// Returns a Mod Archive download link for the given module, you can get this struct by using
    /// [`ModInfo::get()`], or search using [`ModInfo::resolve_filename()`], if you're using the
    /// resolver function please consider using the [`ModSearch::get_download_link()`] method
    /// instead.
    pub fn get_download_link(&self) -> String {
        format!(
            "https://api.modarchive.org/downloads.php?moduleid={}#{}",
            self.id, self.filename
        )
    }

    /// Searches for your string on Mod Archive and returns the results on the first page (a.k.a
    /// only up to the first 40) as a vector of [`ModSearch`]
    pub fn resolve_filename(filename: &str) -> Result<Vec<ModSearch>, crate::Error> {
        let body: String = ureq::get(
                format!(
                    "https://modarchive.org/index.php?request=search&query={}&submit=Find&search_type=filename",
                    filename
                )
                .as_str(),
            )
            .call()
            .unwrap()
            .into_string()
            .unwrap();

        let dom = tl::parse(&body, tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();

        let status = dom.query_selector("h1.site-wide-page-head-title");

        match status {
            Some(_) => {}
            None => return Err(crate::Error::NotFound),
        };
        // from this point on we can unwrap after each query selector
        // because our info will for sure be present.

        let links: Vec<ModSearch> = dom
            .query_selector("a.standard-link[title]")
            .unwrap()
            .map(|nodehandle| {
                let node = nodehandle.get(parser).unwrap();

                let id = match node.as_tag().unwrap().attributes().get("href") {
                    Some(Some(a)) => a
                        .as_utf8_str()
                        .split("query=")
                        .nth(1)
                        .unwrap()
                        .parse()
                        .unwrap(),
                    Some(None) => unreachable!(),
                    None => unreachable!(),
                };

                let filename = node.inner_text(parser).into();

                ModSearch { id, filename }
            })
            .collect();

        Ok(links)
    }
}

impl ModSearch {
    /// Get the download link of this specific module.
    pub fn get_download_link(&self) -> String {
        format!(
            "https://api.modarchive.org/downloads.php?moduleid={}#{}",
            self.id, self.filename
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ModInfo;
    use std::env;

    #[test]
    fn instr_text() {
        let instr_text = ModInfo::get(61772, &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables")).unwrap().instrument_text;
        assert_eq!(
            instr_text,
            "\n        7th  Dance\n\n             By:\n Jari Ylamaki aka Yrde\n  27.11.2000 HELSINKI\n\n            Finland\n           SITE :\n  www.mp3.com/Yrde"
        );
    }

    #[test]
    fn invalid_modid() {
        let invalid = ModInfo::get(30638, &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables"));
        assert!(invalid.is_err());
    }

    #[test]
    fn valid_modid() {
        let valid = ModInfo::get(99356, &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables"));
        assert!(valid.is_ok());
    }

    /*
    #[test]
    fn spotlit_modid() {
        let module = ModInfo::get(158263, &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables")).unwrap();
        assert!(module.spotlit);
    }
    */

    #[test]
    fn name_resolving() {
        let mod_search = ModInfo::resolve_filename("virtual-monotone.mod");
        let mod_search = &mod_search.unwrap()[0];
        assert_eq!(mod_search.id, 88676);
        assert_eq!(
            mod_search.get_download_link().as_str(),
            "https://api.modarchive.org/downloads.php?moduleid=88676#virtual-monotone.mod"
        );
    }

    #[test]
    fn dl_link_modinfo() {
        let modinfo = ModInfo::get(41070, &env::var("MODARCH_KEY").expect("Expected a Mod Archive API key in the environment variables")).unwrap();
        assert_eq!(
            modinfo.get_download_link().as_str(),
            "https://api.modarchive.org/downloads.php?moduleid=41070#fading_horizont.mod"
        );
    }
}
