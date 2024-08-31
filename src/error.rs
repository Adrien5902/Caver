use std::{io, path::PathBuf};

use ntfs_reader::errors::{NtfsReaderError, NtfsReaderResult};

pub type CaverResult<T> = core::result::Result<T, CaverError>;

pub trait IntoCaverResult<T> {
    fn into_caver_result(self) -> CaverResult<T>;
}

impl<T> IntoCaverResult<T> for NtfsReaderResult<T> {
    fn into_caver_result(self) -> CaverResult<T> {
        self.map_err(|e| e.into())
    }
}

impl<T> IntoCaverResult<T> for io::Result<T> {
    fn into_caver_result(self) -> CaverResult<T> {
        self.map_err(|e| e.into())
    }
}

impl<T> IntoCaverResult<T> for bincode::Result<T> {
    fn into_caver_result(self) -> CaverResult<T> {
        self.map_err(|e| (*e).into())
    }
}

#[derive(Debug)]
pub enum CaverError {
    UnableToConvertPathToString(PathBuf),
    IOError(io::Error),
    DeserializeError(bincode::ErrorKind),
    ElevationError,
    Unknown,
}

impl From<NtfsReaderError> for CaverError {
    fn from(value: NtfsReaderError) -> Self {
        match value {
            NtfsReaderError::IOError(e) => Self::IOError(e),
            NtfsReaderError::ElevationError => Self::ElevationError,
            NtfsReaderError::Unknown => Self::Unknown,
            _ => todo!("other errors"),
        }
    }
}

impl From<io::Error> for CaverError {
    fn from(value: io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<bincode::ErrorKind> for CaverError {
    fn from(value: bincode::ErrorKind) -> Self {
        CaverError::DeserializeError(value)
    }
}
