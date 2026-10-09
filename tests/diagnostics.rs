mod common;

use common::make_diag;

#[test]
fn line_col_first_character(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(0), (1, 1));
}

#[test]
fn line_col_on_the_newline_itself(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(2), (1, 3));
}

#[test]
fn line_col_first_character_after_newline(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(3), (2, 1));
}

#[test]
fn line_col_last_line(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(7), (3, 2));
}

#[test]
fn line_col_one_past_end(){
    let diag = make_diag("ab\ncd\nef");
    assert_eq!(diag.borrow().source_map.get_line_col(8), (3, 3));
}

#[test]
fn line_col_empty_source(){
    let diag = make_diag("");
    assert_eq!(diag.borrow().source_map.get_line_col(0), (1, 1));
}
