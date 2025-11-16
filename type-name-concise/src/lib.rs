use std::borrow::Cow;
use std::sync::LazyLock;

use indoc::indoc;
use regex::Regex;

#[cfg(test)]
mod tests;

pub fn type_name_concise<T>() -> Cow<'static, str>
where
    T: ?Sized,
{
    simplify_type_name(std::any::type_name::<T>())
}

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        indoc! { r#"
            (?ix)
            # ^- case insensitive and verbose mode.

            # any number of path prefixes:
            ([a-z_][a-z0-9_]*::)*

            # the final target path suffix:
            (?<SUFFIX>[a-z_][a-z0-9_]*)
        "# }
        .trim(),
    )
    .unwrap()
});

fn simplify_type_name(name: &str) -> Cow<'_, str> {
    REGEX.replace_all(name, "${SUFFIX}")
}
