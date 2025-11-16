use std::fmt::Debug;

#[derive(Default)]
pub struct Entries<'a>(Vec<Box<dyn Debug + 'a>>);

impl<'a> Entries<'a> {
    pub fn new<T>(entry: T) -> Self
    where
        T: Debug + 'a,
    {
        Self::default().with(entry)
    }

    pub fn with<T>(mut self, entry: T) -> Self
    where
        T: Debug + 'a,
    {
        self.0.push(Box::new(entry));
        self
    }
}

impl<'a> Debug for Entries<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
