use std::path::{Path, PathBuf};

use ntfs_reader::volume::Volume;
use sysinfo::Disks;

use crate::error::{CaverError, CaverResult, IntoCaverResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiskLetter(char);

impl ToString for DiskLetter {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<char> for DiskLetter {
    fn from(value: char) -> Self {
        Self(value)
    }
}

impl DiskLetter {
    pub fn new(c: char) -> Self {
        Self(c)
    }

    pub fn volume(&self) -> CaverResult<Volume> {
        let path = format!("\\\\.\\{}:", self.0);
        Volume::new(path).into_caver_result()
    }

    pub fn with_dots(&self) -> String {
        self.to_string() + ":"
    }

    pub fn path_as_str(&self) -> String {
        self.with_dots() + "\\"
    }

    pub fn path(&self) -> PathBuf {
        Path::new(&(self.path_as_str() + "\\")).to_owned()
    }

    pub fn get_all() -> CaverResult<Vec<Self>> {
        Disks::new_with_refreshed_list()
            .iter()
            .map(|disk| {
                let path = disk.mount_point();
                Ok(DiskLetter(
                    path.to_str()
                        .ok_or(CaverError::UnableToConvertPathToString(path.to_owned()))?
                        .chars()
                        .next()
                        .ok_or(CaverError::UnableToConvertPathToString(path.to_owned()))?,
                ))
            })
            .collect()
    }
}
