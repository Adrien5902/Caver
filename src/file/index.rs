use std::{
    cell::LazyCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Instant,
};

use ntfs_reader::{
    api::{FIRST_NORMAL_RECORD, ROOT_RECORD},
    mft::Mft,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    disk::DiskLetter,
    error::{CaverResult, IntoCaverResult},
    search::SearchParams,
};

use super::File;

#[derive(Debug)]
pub struct GuardedFile {
    pub name: String,
    pub children: Vec<Arc<Mutex<GuardedFile>>>,
}

impl GuardedFile {
    pub fn unguard(self) -> File {
        File {
            name: self.name,
            children: self
                .children
                .into_iter()
                .map(|file| {
                    Arc::try_unwrap(file)
                        .unwrap()
                        .into_inner()
                        .unwrap()
                        .unguard()
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileIndex {
    pub disks: Vec<File>,
}

impl FileIndex {
    pub const SAVE_PATH: LazyCell<PathBuf> = LazyCell::new(|| Path::new("target/db").to_owned());

    pub fn create() -> CaverResult<Self> {
        Ok(Self {
            disks: DiskLetter::get_all()?
                .par_iter()
                .map(|diskletter| {
                    println!("retrieving mft for {} ...", diskletter.to_string());

                    let mft = Mft::new(diskletter.volume()?).into_caver_result()?;

                    println!("indexing {} ...", diskletter.to_string());

                    let mut children_refs = HashMap::new();
                    let mut roots = Vec::new();
                    let old_files = (FIRST_NORMAL_RECORD..mft.max_record)
                        .into_iter()
                        .map(|index| {
                            if !mft.record_exists(index) {
                                return None;
                            };

                            let file = mft.get_record(index)?;
                            let file_name = file.get_best_file_name(&mft)?;

                            if !file.is_used() {
                                return None;
                            }

                            let parent_index = file_name.parent();
                            if parent_index == ROOT_RECORD {
                                roots.push((index) as usize)
                            } else {
                                children_refs
                                    .entry((parent_index) as usize)
                                    .or_insert(Vec::new())
                                    .push((index) as usize)
                            }

                            Some(file_name.to_string())
                        })
                        .collect::<Vec<_>>();

                    fn build_tree(
                        index: usize,
                        old_files: &[Option<String>],
                        children_cache: &HashMap<usize, Vec<usize>>,
                    ) -> File {
                        let mut file = File {
                            name: old_files[index - FIRST_NORMAL_RECORD as usize]
                                .clone()
                                .unwrap(),
                            children: vec![],
                        };

                        if let Some(children_indices) = children_cache.get(&index) {
                            for &child_index in children_indices {
                                file.children.push(build_tree(
                                    child_index,
                                    old_files,
                                    children_cache,
                                ));
                            }
                        }

                        file
                    }

                    let mut files = Vec::new();
                    for &root_index in &roots {
                        files.push(build_tree(root_index, &old_files, &children_refs));
                    }

                    Ok(File {
                        children: files,
                        name: diskletter.path_as_str(),
                    })
                })
                .collect::<CaverResult<Vec<File>>>()?,
        })
    }

    pub fn save(&self) -> CaverResult<()> {
        fs::write(
            Self::SAVE_PATH.clone(),
            bincode::serialize(self).into_caver_result()?,
        )
        .into_caver_result()?;

        Ok(())
    }

    pub fn fetch_from_db() -> CaverResult<Self> {
        let data = fs::read(Self::SAVE_PATH.clone())?;
        bincode::deserialize(&data).into_caver_result()
    }

    pub fn search(&self, params: SearchParams) -> Vec<(String, PathBuf)> {
        let res = self
            .disks
            .par_iter()
            .flat_map(|disk| {
                disk.iter()
                    .filter_map(|data| {
                        params
                            .process(&data)
                            .then_some((data.0.name.clone(), data.1))
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        res
    }

    pub fn search_str(&self, s: &str) -> Vec<(String, PathBuf)> {
        let params_parse_start = Instant::now();
        let params = SearchParams::from_str(s);
        println!(
            "params parse time {:?}",
            Instant::now() - params_parse_start
        );

        let search_start = Instant::now();
        let res = self.search(params);
        println!("search time {:?}", Instant::now() - search_start);
        res
    }
}
