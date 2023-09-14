use briefly::key_generator::generate;

#[test]
fn generate_seven_chars_key() {
    let url = "https://www.rust-lang.org";
    let generated = generate(url);
    assert_eq!(generated, "e2XSHcR".to_string())
}
