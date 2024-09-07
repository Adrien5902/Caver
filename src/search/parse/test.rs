use std::collections::HashMap;

use crate::search::{
    SearchExpr, SearchExprValue, SearchField, SearchOperation, SearchOperator, SearchParams,
};

#[test]
fn hard_search_parse() {
    let input = "some ?(word | ?and) other content?<this | ?that woaw>";
    let search_params = SearchParams::from_str(&input);

    let mut map = HashMap::new();
    map.insert(
        SearchField::Name,
        SearchExpr {
            expr: SearchExprValue::Operation(Box::new(SearchOperation {
                operation: SearchOperator::And,
                lhs: SearchExpr {
                    expr: SearchExprValue::Value("some".to_string()),
                    inverted: false,
                },
                rhs: SearchExpr {
                    expr: SearchExprValue::Operation(Box::new(SearchOperation {
                        operation: SearchOperator::And,
                        lhs: SearchExpr {
                            expr: SearchExprValue::Operation(Box::new(SearchOperation {
                                operation: SearchOperator::Or,
                                lhs: SearchExpr {
                                    expr: SearchExprValue::Value("word".to_string()),
                                    inverted: true,
                                },
                                rhs: SearchExpr {
                                    expr: SearchExprValue::Value("and".to_string()),
                                    inverted: false,
                                },
                            })),
                            inverted: true,
                        },
                        rhs: SearchExpr {
                            expr: SearchExprValue::Value("other content".to_string()),
                            inverted: false,
                        },
                    })),
                    inverted: false,
                },
            })),
            inverted: false,
        },
    );

    map.insert(
        SearchField::Content,
        SearchExpr {
            expr: SearchExprValue::Operation(Box::new(SearchOperation {
                operation: SearchOperator::Or,
                lhs: SearchExpr {
                    expr: SearchExprValue::Value("this".to_string()),
                    inverted: false,
                },
                rhs: SearchExpr {
                    expr: SearchExprValue::Value("that woaw".to_string()),
                    inverted: true,
                },
            })),
            inverted: false,
        },
    );

    assert_eq!(search_params, SearchParams::from(map));
}

#[test]
fn easy_search_parse() {
    let input = "some | is and content<this|that woaw>";

    let search_params = SearchParams::from_str(input);

    let mut map = HashMap::new();

    map.insert(
        SearchField::Content,
        SearchExpr {
            expr: SearchExprValue::Operation(Box::new(SearchOperation {
                operation: SearchOperator::Or,
                lhs: SearchExpr {
                    expr: SearchExprValue::Value("this".to_string()),
                    inverted: false,
                },
                rhs: SearchExpr {
                    expr: SearchExprValue::Operation(Box::new(SearchOperation {
                        operation: SearchOperator::And,
                        lhs: SearchExpr {
                            expr: SearchExprValue::Value("that".to_string()),
                            inverted: false,
                        },
                        rhs: SearchExpr {
                            expr: SearchExprValue::Value("woaw".to_string()),
                            inverted: false,
                        },
                    })),
                    inverted: false,
                },
            })),
            inverted: false,
        },
    );
    map.insert(
        SearchField::Name,
        SearchExpr {
            expr: SearchExprValue::Operation(Box::new(SearchOperation {
                operation: SearchOperator::Or,
                lhs: SearchExpr {
                    expr: SearchExprValue::Value("some".to_string()),
                    inverted: false,
                },
                rhs: SearchExpr {
                    expr: SearchExprValue::Operation(Box::new(SearchOperation {
                        operation: SearchOperator::And,
                        lhs: SearchExpr {
                            expr: SearchExprValue::Value("is".to_string()),
                            inverted: false,
                        },
                        rhs: SearchExpr {
                            expr: SearchExprValue::Value("and".to_string()),
                            inverted: false,
                        },
                    })),
                    inverted: false,
                },
            })),
            inverted: false,
        },
    );

    pretty_assertions::assert_eq!(search_params, SearchParams::from(map));
}
