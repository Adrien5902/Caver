use core::str::Chars;
use std::iter::Peekable;

use crate::file::IsValidWindowsFileName;

pub struct SearchParamsTokenizer<'a> {
    pub(crate) iter: Peekable<Chars<'a>>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SearchParamsToken {
    Word(String),
    Or,
    Delimiter(Opening),
    Paren(Opening),
    Inverter,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Opening {
    Opened,
    Closed,
}

impl<'a> SearchParamsTokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            iter: s.chars().peekable(),
        }
    }

    pub fn tokens(&mut self) -> Vec<SearchParamsToken> {
        let mut tokens = Vec::new();
        while let Some(&char) = self.iter.peek() {
            match char {
                ' ' => {
                    self.iter.next();
                }
                '|' => {
                    self.iter.next();
                    tokens.push(SearchParamsToken::Or);
                }
                '<' => {
                    self.iter.next();
                    tokens.push(SearchParamsToken::Delimiter(Opening::Opened));
                }
                '>' => {
                    self.iter.next();
                    tokens.push(SearchParamsToken::Delimiter(Opening::Closed));
                }
                '(' => {
                    self.iter.next();
                    tokens.push(SearchParamsToken::Paren(Opening::Opened));
                }
                ')' => {
                    self.iter.next();
                    tokens.push(SearchParamsToken::Paren(Opening::Closed));
                }
                '?' => {
                    self.iter.next();
                    tokens.push(SearchParamsToken::Inverter);
                }
                _ if char.is_valid_windows_file_name() => {
                    let mut word = String::new();
                    while let Some(&char) = self.iter.peek() {
                        if char.is_valid_windows_file_name() && !char.is_whitespace() {
                            word.push(char);
                            self.iter.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(SearchParamsToken::Word(word.to_string()));
                }
                _ => {
                    self.iter.next();
                }
            }
        }
        tokens
    }
}
