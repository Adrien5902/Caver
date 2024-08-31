use std::{fs, path::PathBuf};

use crate::file::File;

pub struct SearchParams {
    pub contains: Option<SearchJoinParam>,
    pub content_contains: Option<SearchJoinParam>,
}

impl SearchParams {
    pub fn process(&self, (file, path): &(&File, PathBuf)) -> Option<()> {
        if let Some(contains) = &self.contains {
            let path_str = path.to_str().unwrap();
            contains.process(&path_str, &|s, val| s.contains(&val.s).then_some(()))?
        }

        if let Some(content_contains) = &self.content_contains {
            if !file.children.is_empty() {
                return None;
            }

            content_contains.process(&path, &|path, val| {
                let string = fs::read_to_string(path).ok()?;

                string.contains(&val.s).then_some(())
            })?;
        }

        Some(())
    }
}

pub enum SearchJoinParam {
    Join(Box<SearchJoin>),
    Value(SearchValue),
}

impl SearchJoinParam {
    pub fn process<F, P>(&self, s: &P, f: &F) -> Option<()>
    where
        F: Fn(&P, &SearchValue) -> Option<()>,
    {
        match self {
            SearchJoinParam::Join(join) => match join.operation {
                SearchJoinOperation::And => join.lhs.process(s, f).and(join.rhs.process(s, f)),
                SearchJoinOperation::Or => join.lhs.process(s, f).or(join.rhs.process(s, f)),
            },
            SearchJoinParam::Value(value) => f(s, value).xor(value.invert.then_some(())),
        }
    }
}

pub struct SearchJoin {
    pub operation: SearchJoinOperation,
    pub lhs: SearchJoinParam,
    pub rhs: SearchJoinParam,
}

pub enum SearchJoinOperation {
    And,
    Or,
}

pub struct SearchValue {
    pub s: String,
    pub invert: bool,
}
