use std::borrow::Cow;
use std::sync::LazyLock;

use indoc::indoc;
use regex::{Captures, Regex};

#[cfg(test)]
mod tests;

pub fn type_name_concise<T>(elide_params: bool) -> Cow<'static, str>
where
    T: ?Sized,
{
    simplify_type_name(std::any::type_name::<T>(), elide_params)
}

static REGEX_WITH_PARAMS: LazyLock<Regex> = LazyLock::new(|| {
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

static REGEX_ELIDE_PARAMS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        indoc! { r#"
            (?ix)
            # ^- case insensitive and verbose mode.

            # any number of path prefixes:
            ([a-z_][a-z0-9_]*::)*

            # the final target path suffix:
            (?<SUFFIX>[a-z_][a-z0-9_]*)

            # the params:
            (?<PARAMS><.*>)?
        "# }
        .trim(),
    )
    .unwrap()
});

fn simplify_type_name(name: &str, elide_params: bool) -> Cow<'_, str> {
    if elide_params {
        REGEX_ELIDE_PARAMS.replace_all(name, |caps: &Captures| {
            let mut s = caps["SUFFIX"].to_string();
            if caps.name("PARAMS").is_some() {
                s.push_str("<…>");
            }
            s
        })
    } else {
        REGEX_WITH_PARAMS.replace_all(name, "${SUFFIX}")
    }
}
