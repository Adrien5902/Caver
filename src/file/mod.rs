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

pub struct FileIterator<'a> {
    path: PathBuf,
    stack: Vec<std::slice::Iter<'a, File>>,
}

impl<'a> FileIterator<'a> {
    fn new(root: &'a File) -> Self {
        Self {
            path: PathBuf::from(&root.name),
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
