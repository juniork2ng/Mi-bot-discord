use std::path::PathBuf;

#[cfg(feature = "json")]
use serde_json::json;

use super::*;
use crate::prelude::*;

#[test]
fn str_normalize_empty() {
    let input = "";
    let pattern = "";
    let expected = "";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_literals_match() {
    let input = "Hello\nWorld";
    let pattern = "Hello\nWorld";
    let expected = "Hello\nWorld";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_pattern_shorter() {
    let input = "Hello\nWorld";
    let pattern = "Hello\n";
    let expected = "Hello\nWorld";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_input_shorter() {
    let input = "Hello\n";
    let pattern = "Hello\nWorld";
    let expected = "Hello\n";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_all_different() {
    let input = "Hello\nWorld";
    let pattern = "Goodbye\nMoon";
    let expected = "Hello\nWorld";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_middles_diverge() {
    let input = "Hello\nWorld\nGoodbye";
    let pattern = "Hello\nMoon\nGoodbye";
    let expected = "Hello\nWorld\nGoodbye";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_elide_delimited_with_sub() {
    let input = "Hello World\nHow are you?\nGoodbye World";
    let pattern = "Hello [..]\n...\nGoodbye [..]";
    let expected = "Hello [..]\n...\nGoodbye [..]";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_leading_elide() {
    let input = "Hello\nWorld\nGoodbye";
    let pattern = "...\nGoodbye";
    let expected = "...\nGoodbye";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_trailing_elide() {
    let input = "Hello\nWorld\nGoodbye";
    let pattern = "Hello\n...";
    let expected = "Hello\n...";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_middle_elide() {
    let input = "Hello\nWorld\nGoodbye";
    let pattern = "Hello\n...\nGoodbye";
    let expected = "Hello\n...\nGoodbye";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_post_elide_diverge() {
    let input = "Hello\nSun\nAnd\nWorld";
    let pattern = "Hello\n...\nMoon";
    let expected = "Hello\nSun\nAnd\nWorld";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_post_diverge_elide() {
    let input = "Hello\nWorld\nGoodbye\nSir";
    let pattern = "Hello\nMoon\nGoodbye\n...";
    let expected = "Hello\nWorld\nGoodbye\n...";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_inline_elide() {
    let input = "Hello\nWorld\nGoodbye\nSir";
    let pattern = "Hello\nW[..]d\nGoodbye\nSir";
    let expected = "Hello\nW[..]d\nGoodbye\nSir";
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, expected.into_data());
}

#[test]
fn str_normalize_user_literal() {
    let input = "Hello world!";
    let pattern = "Hello [OBJECT]!";
    let mut sub = Redactions::new();
    sub.insert("[OBJECT]", "world").unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, pattern.into_data());
}

#[test]
fn str_normalize_user_path() {
    let input = "input: /home/epage";
    let pattern = "input: [HOME]";
    let mut sub = Redactions::new();
    let sep = std::path::MAIN_SEPARATOR.to_string();
    let redacted = PathBuf::from(sep).join("home").join("epage");
    sub.insert("[HOME]", redacted).unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, pattern.into_data());
}

#[test]
fn str_normalize_user_overlapping_path() {
    let input = "\
a: /home/epage
b: /home/epage/snapbox";
    let pattern = "\
a: [A]
b: [B]";
    let mut sub = Redactions::new();
    let sep = std::path::MAIN_SEPARATOR.to_string();
    let redacted = PathBuf::from(&sep).join("home").join("epage");
    sub.insert("[A]", redacted).unwrap();
    let redacted = PathBuf::from(sep)
        .join("home")
        .join("epage")
        .join("snapbox");
    sub.insert("[B]", redacted).unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, pattern.into_data());
}

#[test]
fn str_normalize_user_disabled() {
    let input = "cargo";
    let pattern = "cargo[EXE]";
    let mut sub = Redactions::new();
    sub.insert("[EXE]", "").unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, pattern.into_data());
}

#[test]
#[cfg(feature = "regex")]
fn str_normalize_user_regex_unnamed() {
    let input = "Hello world!";
    let pattern = "Hello [OBJECT]!";
    let mut sub = Redactions::new();
    sub.insert("[OBJECT]", regex::Regex::new("world").unwrap())
        .unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, pattern.into_data());
}

#[test]
#[cfg(feature = "regex")]
fn str_normalize_user_regex_named() {
    let input = "Hello world!";
    let pattern = "Hello [OBJECT]!";
    let mut sub = Redactions::new();
    sub.insert(
        "[OBJECT]",
        regex::Regex::new("(?<redacted>world)!").unwrap(),
    )
    .unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(input.into(), &pattern.into());
    assert_eq!(actual, pattern.into_data());
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_for_string() {
    let exp = json!({"name": "{...}"});
    let expected = Data::json(exp);
    let actual = json!({"name": "JohnDoe"});
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_eq!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_for_array() {
    let exp = json!({"people": "{...}"});
    let expected = Data::json(exp);
    let actual = json!({
        "people": [
            {
                "name": "JohnDoe",
                "nickname": "John",
            }
        ]
    });
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_eq!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_for_obj() {
    let exp = json!({"people": "{...}"});
    let expected = Data::json(exp);
    let actual = json!({
        "people": {
            "name": "JohnDoe",
            "nickname": "John",
        }
    });
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_eq!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_array_start() {
    let exp = json!({
        "people": [
            "{...}",
            {
                "name": "three",
                "nickname": "3",
            }
        ]
    });
    let expected = Data::json(exp);
    let actual = json!({
        "people": [
            {
                "name": "one",
                "nickname": "1",
            },
            {
                "name": "two",
                "nickname": "2",
            },
            {
                "name": "three",
                "nickname": "3",
            }
        ]
    });
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_eq!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_for_array_start_end() {
    let exp = json!([
        "{...}",
        {
            "name": "two",
            "nickname": "2",
        },
        "{...}"
    ]);
    let expected = Data::json(exp);
    let actual = json!([
        {
            "name": "one",
            "nickname": "1",
        },
        {
            "name": "two",
            "nickname": "2",
        },
        {
            "name": "three",
            "nickname": "3",
        },
        {
            "name": "four",
            "nickname": "4",
        }
    ]);
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_eq!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_for_array_middle_end() {
    let exp = json!([
        {
            "name": "one",
            "nickname": "1",
        },
        "{...}",
        {
            "name": "three",
            "nickname": "3",
        },
        "{...}"
    ]);
    let expected = Data::json(exp);
    let actual = json!([
        {
            "name": "one",
            "nickname": "1",
        },
        {
            "name": "two",
            "nickname": "2",
        },
        {
            "name": "three",
            "nickname": "3",
        },
        {
            "name": "four",
            "nickname": "4",
        },
        {
            "name": "five",
            "nickname": "5",
        }
    ]);
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_eq!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_for_array_mismatch() {
    let exp = json!([
        {
            "name": "one",
            "nickname": "1",
        },
        "{...}",
        {
            "name": "three",
            "nickname": "3",
        },
        "{...}"
    ]);
    let expected = Data::json(exp);
    let actual = json!([
        {
            "name": "one",
            "nickname": "1",
        },
        {
            "name": "two",
            "nickname": "2",
        },
        {
            "name": "four",
            "nickname": "4",
        },
        {
            "name": "five",
            "nickname": "5",
        }
    ]);
    let actual_normalized = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual.clone()), &expected);
    if let DataInner::Json(act) = actual_normalized.inner {
        assert_eq!(act, actual);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_bad_order() {
    let exp = json!({
        "people": ["John", "Jane"]
    });
    let expected = Data::json(exp);
    let actual = json!({
        "people": ["Jane", "John"]
    });
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(Data::json(actual), &expected);
    if let (DataInner::Json(exp), DataInner::Json(act)) = (expected.inner, actual.inner) {
        assert_ne!(exp, act);
    }
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_obj_key() {
    let expected = json!({
        "[A]": "value-a",
        "[B]": "value-b",
        "[C]": "value-c",
    });
    let expected = Data::json(expected);
    let actual = json!({
        "key-a": "value-a",
        "key-b": "value-b",
        "key-c": "value-c",
    });
    let actual = Data::json(actual);
    let mut sub = Redactions::new();
    sub.insert("[A]", "key-a").unwrap();
    sub.insert("[B]", "key-b").unwrap();
    sub.insert("[C]", "key-c").unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(actual, &expected);

    let expected_actual = json!({
        "[A]": "value-a",
        "[B]": "value-b",
        "[C]": "value-c",
    });
    let expected_actual = Data::json(expected_actual);
    assert_eq!(actual, expected_actual);
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_with_missing_obj_key() {
    let expected = json!({
        "a": "[A]",
        "b": "[B]",
        "c": "[C]",
    });
    let expected = Data::json(expected);
    let actual = json!({
        "a": "value-a",
        "c": "value-c",
    });
    let actual = Data::json(actual);
    let mut sub = Redactions::new();
    sub.insert("[A]", "value-a").unwrap();
    sub.insert("[B]", "value-b").unwrap();
    sub.insert("[C]", "value-c").unwrap();
    let actual = NormalizeToExpected::new()
        .redact_with(&sub)
        .normalize(actual, &expected);

    let expected_actual = json!({
        "a": "[A]",
        "c": "[C]",
    });
    let expected_actual = Data::json(expected_actual);
    assert_eq!(actual, expected_actual);
}

#[test]
#[cfg(feature = "json")]
fn json_normalize_glob_obj_key() {
    let expected = json!({
        "a": "value-a",
        "c": "value-c",
        "...": "{...}",
    });
    let expected = Data::json(expected);
    let actual = json!({
        "a": "value-a",
        "b": "value-b",
        "c": "value-c",
    });
    let actual = Data::json(actual);
    let actual = NormalizeToExpected::new()
        .redact()
        .normalize(actual, &expected);

    let expected_actual = json!({
        "a": "value-a",
        "c": "value-c",
        "...": "{...}",
    });
    let expected_actual = Data::json(expected_actual);
    assert_eq!(actual, expected_actual);
}
