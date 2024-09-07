pub mod index;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub name: String,
    pub children: Vec<File>,
}

impl File {
    pub fn iter(&self) -> FileIterator {
        FileIterator::new(self)
    }
}

pub trait IsValidWindowsFileName {
    fn is_valid_windows_file_name(&self) -> bool;
}

impl IsValidWindowsFileName for char {
    fn is_valid_windows_file_name(&self) -> bool {
        match self {
            '\\' | '/' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => false,
            _ => true,
        }
    }
}

impl IsValidWindowsFileName for str {
    fn is_valid_windows_file_name(&self) -> bool {
        self.chars()
            .map(|c| c.is_valid_windows_file_name())
            .reduce(|a, b| a && b)
            .unwrap_or_default() // empty is false
    }
}

pub struct FileIterator<'a> {
    path: PathBuf,
    stack: Vec<std::slice::Iter<'a, File>>,
}

impl<'a> FileIterator<'a> {
    fn new(root: &'a File) -> Self {
        let mut path = PathBuf::with_capacity(512);
        path.push(&root.name);
        Self {
            path,
            stack: vec![root.children.iter()],
        }
    }
}

impl<'a> Iterator for FileIterator<'a> {
    type Item = (&'a File, PathBuf);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(top) = self.stack.last_mut() {
            if let Some(file) = top.next() {
                self.path.push(&file.name);
                self.stack.push(file.children.iter());
                return Some((file, self.path.clone()));
            } else {
                self.path.pop();
                self.stack.pop();
            }
        }
        None
    }
}
