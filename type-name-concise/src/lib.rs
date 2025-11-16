#[cfg(test)]
mod tests;

pub fn type_name_concise<T>() -> String
where
    T: ?Sized,
{
    simplify_type_name(std::any::type_name::<T>())
}

fn simplify_type_name(full: &str) -> String {
    let frag = if let Some((ix, _)) = full
        .char_indices()
        .find(|&(_, c)| !(c == ':' || c == '_' || c.is_ascii_alphanumeric()))
    {
        let (s, _) = full.split_at(ix);
        s
    } else {
        full
    };

    if frag.is_empty() {
        full
    } else {
        frag.rsplit_once("::").map(|(_, s)| s).unwrap_or(frag)
    }
    .to_string()
}
