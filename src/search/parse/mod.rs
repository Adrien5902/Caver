#[cfg(test)]
mod test;

use std::{collections::HashMap, iter::Peekable};

use crate::search::{
    token::{Opening, SearchParamsToken},
    SearchExpr, SearchExprValue, SearchField, SearchOperation, SearchOperator,
};

use super::SearchParams;

impl SearchParams {
    pub fn parse(tokens: Vec<SearchParamsToken>) -> SearchParams {
        let mut map = HashMap::new();
        let mut delimiter_stack = vec![(SearchField::Name, false, Vec::new())];

        pub fn parse_pop(
            delimiter_stack: &mut Vec<(SearchField, bool, Vec<SearchParamsToken>)>,
            map: &mut HashMap<SearchField, SearchExpr>,
        ) {
            pub fn parse_tokens(tokens: Vec<SearchParamsToken>) -> SearchExpr {
                let mut iter = tokens.into_iter().peekable();

                fn parse_expr(
                    iter: &mut Peekable<std::vec::IntoIter<SearchParamsToken>>,
                ) -> SearchExpr {
                    let mut expr = parse_primary_expr(iter);

                    while let Some(token) = iter.peek() {
                        match token {
                            SearchParamsToken::Or => {
                                iter.next(); // Consume the token
                                let rhs = parse_primary_expr(iter);
                                expr = SearchExpr {
                                    expr: SearchExprValue::Operation(Box::new(SearchOperation {
                                        operation: SearchOperator::Or,
                                        lhs: expr,
                                        rhs,
                                    })),
                                    inverted: false,
                                };
                            }
                            _ => break,
                        }
                    }

                    expr
                }

                fn parse_primary_expr(
                    iter: &mut Peekable<std::vec::IntoIter<SearchParamsToken>>,
                ) -> SearchExpr {
                    let mut expr = match iter.next() {
                        Some(SearchParamsToken::Word(word)) => SearchExpr {
                            expr: SearchExprValue::Value(word),
                            inverted: false,
                        },
                        Some(SearchParamsToken::Inverter) => {
                            let inverted_expr = parse_primary_expr(iter);
                            SearchExpr {
                                expr: inverted_expr.expr,
                                inverted: !inverted_expr.inverted,
                            }
                        }
                        Some(SearchParamsToken::Paren(Opening::Opened)) => {
                            let expr = parse_expr(iter);
                            if let Some(SearchParamsToken::Paren(Opening::Closed)) = iter.next() {
                                expr
                            } else {
                                panic!("Unmatched parenthesis");
                            }
                        }
                        _ => panic!("Unexpected token"),
                    };

                    while let Some(token) = iter.peek() {
                        match token {
                            SearchParamsToken::Word(_)
                            | SearchParamsToken::Inverter
                            | SearchParamsToken::Paren(Opening::Opened) => {
                                let rhs = parse_primary_expr(iter);
                                expr = SearchExpr {
                                    expr: SearchExprValue::Operation(Box::new(SearchOperation {
                                        operation: SearchOperator::And,
                                        lhs: expr,
                                        rhs,
                                    })),
                                    inverted: false,
                                };
                            }
                            _ => break,
                        }
                    }
                    expr
                }

                parse_expr(&mut iter)
            }

            if let Some((field, inverted, tokens)) = delimiter_stack.pop() {
                if !tokens.is_empty() {
                    let mut expr = parse_tokens(tokens);

                    if let Some(old) = map.remove(&field) {
                        expr = SearchExpr {
                            expr: SearchExprValue::Operation(Box::new(SearchOperation {
                                operation: SearchOperator::And,
                                lhs: old,
                                rhs: expr,
                            })),
                            inverted,
                        };
                    };

                    map.insert(field, expr);
                }
            }
        }

        for token in tokens.into_iter() {
            match token {
                SearchParamsToken::Delimiter(opening) => match opening {
                    Opening::Opened => {
                        let mut field = SearchField::default();
                        let mut inverted = false;

                        while let Some((_, _, last_scope)) = delimiter_stack.last_mut() {
                            let Some(last_token) = last_scope.last() else {
                                break;
                            };

                            match last_token {
                                SearchParamsToken::Word(_) => {
                                    let SearchParamsToken::Word(word) = last_scope.pop().unwrap()
                                    else {
                                        panic!()
                                    };

                                    if let Some(other_field) = SearchField::from_string(&word) {
                                        field = other_field;
                                    }

                                    break;
                                }

                                SearchParamsToken::Inverter => {
                                    inverted = !inverted;
                                    last_scope.pop();
                                }

                                _ => break,
                            }
                        }

                        delimiter_stack.push((field, inverted, Vec::new()));
                    }
                    Opening::Closed => parse_pop(&mut delimiter_stack, &mut map),
                },
                other => {
                    if let Some((_, _, last_delim_stack)) = delimiter_stack.last_mut() {
                        last_delim_stack.push(other)
                    }
                }
            }
        }

        parse_pop(&mut delimiter_stack, &mut map);

        SearchParams::from(map)
    }
}
