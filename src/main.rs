#![feature(extract_if)]
#![feature(iter_advance_by)]

pub mod disk;
pub mod error;
pub mod file;
pub mod search;

use std::{env, time::Instant};

use file::index::FileIndex;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.get(1).is_some_and(|s| s == "reset") || !FileIndex::SAVE_PATH.exists() {
        let fi = FileIndex::create().unwrap();
        fi.save().unwrap();
    } else {
        let fi_fetch_start = Instant::now();
        let fi = FileIndex::fetch_from_db().unwrap();
        println!("fi fetch time : {:?}", Instant::now() - fi_fetch_start);

        let search_start = Instant::now();
        let results = fi.search_str("path<minecraft assets>");

        println!("search time : {:?}", Instant::now() - search_start);
        println!("results : {:?}", results.len());
        results
            .iter()
            .for_each(|(_, file)| println!("{}", file.to_str().unwrap()));
    }
}
