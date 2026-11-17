use super::*;

#[test]
fn names_with_control_chars_are_rejected() {
    for bad in ["", "a\u{0}b", "line\nbreak", "\u{202e}rtl", " lead", &"x".repeat(25)] {
        let m = Msg::Hello { name: bad.into(), color: 0 };
        assert!(m.validate().is_err(), "{bad:?}");
    }
    assert!(Msg::Hello { name: "Ñandú".into(), color: 7 }.validate().is_ok());
}

#[test]
fn unknown_types_fields_and_bad_values_are_rejected() {
    for raw in [
        r#"{"t":"exec","cmd":"x"}"#,
        r#"{"t":"hello","name":"a","color":0,"admin":true}"#,
        r#"{"t":"hello","name":"a","color":9}"#,
        r#"{"t":"pos","map":"../x","x":0,"y":0,"z":0,"heading":0}"#,
        r#"{"t":"pos","map":"demo","x":1e300,"y":0,"z":0,"heading":0}"#,
        r#"{"t":"pos","map":"demo","x":0,"y":0,"z":0,"heading":400}"#,
    ] {
        assert!(Msg::decode(raw.as_bytes()).is_err(), "{raw}");
    }
}
