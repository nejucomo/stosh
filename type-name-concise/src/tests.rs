use test_case::test_case;

#[test_case("" => "")]
#[test_case("foo" => "foo")]
#[test_case("foo::bar" => "bar")]
#[test_case("ratatui::widgets::Block<'a>" => "Block<'a>")]
#[test_case("ratatui_rseq::thingy_majig::Foo<ratatui_rseq::other_thingy::Bar, Quz>" => "Foo<Bar, Quz>")]
#[test_case("&ratatui_rseq::thingy_majig::Foo<&'_ ratatui_rseq::other_thingy::Bar, &'a Quz>" => "&Foo<&'_ Bar, &'a Quz")]
fn test_simplified(input: &str) -> String {
    crate::simplify_type_name(input)
}
