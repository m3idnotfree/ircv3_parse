#[test]
fn ui_de() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/de/fail/*.rs");
    t.pass("tests/ui/de/pass/*.rs");
}

#[test]
fn ui_ser() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/ser/fail/*.rs");
    t.pass("tests/ui/ser/pass/*.rs");
}
