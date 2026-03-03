#[path = "derive/de.rs"]
mod de;

#[path = "derive/ser.rs"]
mod ser;

#[test]
fn ui_de() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive/ui/de/fail/*.rs");
    t.pass("tests/derive/ui/de/pass/*.rs");
}

#[test]
fn ui_ser() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/derive/ui/ser/fail/*.rs");
    t.pass("tests/derive/ui/ser/pass/*.rs");
}
