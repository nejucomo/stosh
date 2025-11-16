mod entries;

use std::fmt::Debug;

use debug_concise::DebugElide;
use type_name_concise::type_name_concise;

pub use self::entries::Entries;

pub trait DebugRollup: Debug {
    fn fmt_rollup(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&type_name_concise::<Self>(true))
            .field(&self.rollup_entries())
            .finish()
    }

    fn rollup_entries(&self) -> Entries<'_>;
}

/// implement [Debug] via [DebugRollup]
#[macro_export]
macro_rules! delegate_debug_to_rollup {
    ( $name:ident ) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.fmt_rollup(f)
            }
        }
    };

    ($name:ident < $($gen:tt),* > $(where $($w:tt)*)? ) => {
        impl< $($gen),* > std::fmt::Debug for $name< $($gen),* >
            $(where $($w)*)?
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $crate::DebugRollup::fmt_rollup(self, f)
            }
        }
    };
}

impl<T> DebugRollup for &T
where
    T: DebugRollup,
{
    fn rollup_entries(&self) -> Entries<'_> {
        (*self).rollup_entries()
    }
}

impl<A, B> DebugRollup for (A, B)
where
    A: DebugRollup,
    B: DebugRollup,
{
    fn rollup_entries(&self) -> Entries<'_> {
        let (a, b) = self;
        a.rollup_entries().with(b)
    }
}

impl<A, B, C> DebugRollup for (A, B, C)
where
    A: DebugRollup,
    B: DebugRollup,
    C: DebugRollup,
{
    fn rollup_entries(&self) -> Entries<'_> {
        let (a, b, c) = self;
        a.rollup_entries().with(b).with(c)
    }
}

impl DebugRollup for ratatui::layout::Layout {
    fn rollup_entries(&self) -> Entries<'_> {
        Entries::default()
    }
}

impl DebugRollup for ratatui::style::Style {
    fn rollup_entries(&self) -> Entries<'_> {
        Entries::new(DebugElide(self))
    }
}

impl DebugRollup for ratatui::widgets::Clear {
    fn rollup_entries(&self) -> Entries<'_> {
        Entries::new(DebugElide(self))
    }
}

impl<'a> DebugRollup for ratatui::widgets::Block<'a> {
    fn rollup_entries(&self) -> Entries<'_> {
        Entries::new(DebugElide(self))
    }
}
