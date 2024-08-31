#![feature(extract_if)]

pub mod disk;
pub mod error;
pub mod file;
pub mod search;

use std::{env, time::Instant};

use file::index::FileIndex;
use search::{SearchJoin, SearchJoinOperation, SearchJoinParam, SearchParams, SearchValue};

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
        let results = fi.advanced_search(SearchParams {
            contains: Some(SearchJoinParam::Value(SearchValue {
                s: "main.rs".to_string(),
                invert: false,
            })),
            content_contains: Some(SearchJoinParam::Value(SearchValue {
                s: "args".to_string(),
                invert: false,
            })),
        });

        println!("search time : {:?}", Instant::now() - search_start);
        println!("results : {:?}", results.len());
        results
            .iter()
            .for_each(|(_, file)| println!("{}", file.to_str().unwrap()));
    }
}
