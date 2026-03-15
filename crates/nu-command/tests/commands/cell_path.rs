use nu_test_support::nu;

// -- Dynamic cell path subexpressions: $var.(expr) --

#[test]
fn dynamic_string_key_record() {
    let actual = nu!(r#"let key = "a"; {a: 1, b: 2}.($key)"#);
    assert_eq!(actual.out, "1");
}

#[test]
fn dynamic_int_index_list() {
    let actual = nu!(r#"let i = 2; [10 20 30].($i)"#);
    assert_eq!(actual.out, "30");
}

#[test]
fn dynamic_expr_in_middle_of_path() {
    let actual = nu!(r#"let key = "b"; {a: {b: 42}}.a.($key)"#);
    assert_eq!(actual.out, "42");
}

#[test]
fn dynamic_expr_with_static_tail() {
    let actual = nu!(r#"let key = "a"; {a: {b: 99}}.($key).b"#);
    assert_eq!(actual.out, "99");
}

#[test]
fn dynamic_optional_missing_key_returns_nothing() {
    let actual = nu!(r#"let key = "z"; {a: 1}.($key)? | to nuon"#);
    assert_eq!(actual.out, "null");
}

#[test]
fn dynamic_string_key_from_expression() {
    let actual = nu!(r#"{"hello world": 7}.("hello" + " " + "world")"#);
    assert_eq!(actual.out, "7");
}

#[test]
fn dynamic_int_index_from_expression() {
    let actual = nu!(r#"[10 20 30].(1 + 1)"#);
    assert_eq!(actual.out, "30");
}
