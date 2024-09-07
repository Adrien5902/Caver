use std::time::Instant;

use crate::file::index::FileIndex;

#[test]
pub fn find_main_rs() {
    let fi_fetch_start = Instant::now();
    let fi = match FileIndex::fetch_from_db().ok() {
        Some(fi) => fi,
        None => FileIndex::create().unwrap(),
    };
    println!("fi fetch time : {:?}", Instant::now() - fi_fetch_start);

    let results = fi.search_str("main.rs content<args>");
    println!("results : {:?}", results.len());

    assert!(results
        .iter()
        .find(|(_, path)| path.to_string_lossy().to_string().contains("caver"))
        .is_some())
}
