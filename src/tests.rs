#[cfg(test)]
use crate::ModInfo;
use std::env;

#[test]
fn instr_text() {
    let instr_text = ModInfo::get(
        61772,
        &env
            ::var("MODARCH_KEY")
            .expect("Expected a Mod Archive API key in the environment variables")
    ).unwrap().instrument_text;
    assert_eq!(
        instr_text,
        "\n        7th  Dance\n\n             By:\n Jari Ylamaki aka Yrde\n  27.11.2000 HELSINKI\n\n            Finland\n           SITE :\n  www.mp3.com/Yrde"
    );
}

#[test]
fn invalid_modid() {
    let invalid = ModInfo::get(
        30638,
        &env
            ::var("MODARCH_KEY")
            .expect("Expected a Mod Archive API key in the environment variables")
    );
    assert!(invalid.is_err());
}

#[test]
fn valid_modid() {
    let valid = ModInfo::get(
        99356,
        &env
            ::var("MODARCH_KEY")
            .expect("Expected a Mod Archive API key in the environment variables")
    );
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
    let modinfo = ModInfo::get(
        41070,
        &env
            ::var("MODARCH_KEY")
            .expect("Expected a Mod Archive API key in the environment variables")
    ).unwrap();
    assert_eq!(
        modinfo.get_download_link().as_str(),
        "https://api.modarchive.org/downloads.php?moduleid=41070#fading_horizont.mod"
    );
}
