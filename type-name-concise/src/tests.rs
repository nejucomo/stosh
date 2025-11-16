use test_case::test_case;

#[test_case("", "", "")]
#[test_case("foo", "foo", "foo")]
#[test_case("foo::bar", "bar", "bar")]
#[test_case("ratatui::widgets::Block<'a>", "Block<…>", "Block<'a>")]
#[test_case(
    "ratatui_rseq::thingy_majig::Foo<ratatui_rseq::other_thingy::Bar, Quz>",
    "Foo<…>",
    "Foo<Bar, Quz>"
)]
#[test_case(
    "&ratatui_rseq::thingy_majig::Foo<&'_ ratatui_rseq::other_thingy::Bar, &'a Quz>",
    "&Foo<…>",
    "&Foo<&'_ Bar, &'a Quz>"
)]
fn test_simplified(input: &str, elided: &str, wparams: &str) {
    assert_eq!(crate::simplify_type_name(input, true), elided);
    assert_eq!(crate::simplify_type_name(input, false), wparams);
}
