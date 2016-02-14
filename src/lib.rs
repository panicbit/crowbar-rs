#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
extern crate git2;
#[macro_use]
extern crate mdo;
extern crate walkdir;
extern crate serde;
extern crate serde_json;
extern crate semver;
#[macro_use]
extern crate wrapped_enum;

pub mod index;
pub mod krate;
pub mod error;

pub use index::Index;

#[test]
fn test_parsing() {
    let index = Index::new("/tmp/crates_index").expect("index");
    // for krate in index.crates() {
    //     let krate = krate.expect("crate");
    //     let latest = krate.latest();
    //     println!("name: {}\nvers: {}\n", latest.name(), latest.version());
    // }

    //println!("{:?}", index.crates().count());
}