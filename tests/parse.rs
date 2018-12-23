use cargo_toml::TomlManifest;
use std::fs::read;
use toml;

#[test]
fn own() {
    let m = TomlManifest::from_slice(&read("Cargo.toml").unwrap()).unwrap();
    assert_eq!("cargo_toml", m.package.name);
    let m = TomlManifest::<toml::Value>::from_slice_with_metadata(&read("Cargo.toml").unwrap()).unwrap();
    assert_eq!("cargo_toml", m.package.name);
    assert_eq!(cargo_toml::Edition::E2018, m.package.edition);
}

#[test]
fn opt_level() {
    let m = TomlManifest::from_slice(&read("tests/opt_level.toml").unwrap()).unwrap();
    assert_eq!("byteorder", m.package.name);
    assert_eq!(3, m.profile.bench.unwrap().opt_level.unwrap().as_integer().unwrap());
    assert_eq!(false, m.lib.unwrap().bench.unwrap());
    assert_eq!(cargo_toml::Edition::E2015, m.package.edition);
}
