use test_case::test_case;

#[test_case(
    ""
    => "".to_string()
)]
#[test_case(
    "foo"
    => "foo".to_string()
)]
#[test_case(
    "foo::bar"
    => "bar".to_string()
)]
#[test_case(
    "ratatui::widgets::Block<'a>"
    => "Block<'a>".to_string()
)]
#[test_case(
    "ratatui_rseq::thingy_majig::Foo<ratatui_rseq::other_thingy::Bar, Quz>"
    => "Foo<Bar, Quz>".to_string()
)]
#[test_case(
    "&ratatui_rseq::thingy_majig::Foo<&'_ ratatui_rseq::other_thingy::Bar, &'a Quz>"
    => "&Foo<&'_ Bar, &'a Quz>".to_string()
)]
fn test_simplified(input: &str) -> String {
    crate::simplify_type_name(input).to_string()
}
