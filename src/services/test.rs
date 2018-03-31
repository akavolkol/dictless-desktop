use super::*;

#[test]
fn test_translation() {
    let translator = services::translation::Translator::new("en".toString(), "uk".toString());
    assert_eq!(
        translator.source_language,
        "en"
    )
}