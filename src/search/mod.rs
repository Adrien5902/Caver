pub mod parse;
#[cfg(test)]
mod test;
pub mod token;

use std::{collections::HashMap, fs, path::PathBuf};

use token::SearchParamsTokenizer;

use crate::file::File;

#[derive(Debug, Clone, PartialEq)]
pub enum SearchExprValue {
    Operation(Box<SearchOperation>),
    Value(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchExpr {
    pub expr: SearchExprValue,
    pub inverted: bool,
}

impl SearchExpr {
    pub fn process(&self, s: &str) -> bool {
        self.inverted
            ^ match &self.expr {
                SearchExprValue::Operation(op) => {
                    let lhs = op.lhs.process(s);
                    match op.operation {
                        SearchOperator::And => {
                            if !lhs {
                                return false;
                            } else {
                                return op.rhs.process(s);
                            }
                        }

                        SearchOperator::Or => {
                            if lhs {
                                return true;
                            } else {
                                return op.rhs.process(s);
                            }
                        }
                    }
                }
                SearchExprValue::Value(value) => s.contains(value),
            }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchOperation {
    pub operation: SearchOperator,
    pub lhs: SearchExpr,
    pub rhs: SearchExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SearchOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Default)]
pub struct SearchParams {
    name: Option<SearchExpr>,
    path: Option<SearchExpr>,
    content: Option<SearchExpr>,
}

impl From<HashMap<SearchField, SearchExpr>> for SearchParams {
    fn from(mut value: HashMap<SearchField, SearchExpr>) -> Self {
        Self {
            name: value.remove(&SearchField::Name),
            path: value.remove(&SearchField::Path),
            content: value.remove(&SearchField::Content),
        }
    }
}

impl SearchParams {
    pub fn process(&self, file: &(&File, PathBuf)) -> bool {
        if let Some(name_expr) = &self.name {
            if !name_expr.process(&file.0.name) {
                return false;
            }
        }

        if let Some(path_expr) = &self.path {
            let Some(s) = file.1.to_str() else {
                return false;
            };

            if !path_expr.process(s) {
                return false;
            }
        }

        if let Some(content_expr) = &self.content {
            if file.0.children.is_empty() {
                let Some(content) = fs::read_to_string(&file.1).ok() else {
                    return false;
                };

                if !content_expr.process(&content) {
                    return false;
                }
            }
        }

        true
    }

    pub fn from_str(s: &str) -> Self {
        SearchParams::parse(SearchParamsTokenizer::new(s).tokens())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Clone, Copy, PartialOrd, Ord)]
pub enum SearchField {
    #[default]
    Name,
    Path,
    Content,
}

impl SearchField {
    pub fn from_string(value: &str) -> Option<Self> {
        Some(match value {
            "name" => Self::Name,
            "content" => Self::Content,
            "path" => Self::Path,
            _ => return None,
        })
    }
}
