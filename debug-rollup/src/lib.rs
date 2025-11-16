use std::fmt::Debug;

use debug_concise::DebugElide;
use type_name_concise::type_name_concise;

pub trait DebugRollup: Debug {
    fn fmt_rollup(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<", type_name_concise::<Self>())?;
        f.debug_list().entries(self.dyn_debugs()).finish()?;
        write!(f, ">")?;
        Ok(())
    }

    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>>;
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
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        (*self).dyn_debugs()
    }
}

impl<A, B> DebugRollup for (A, B)
where
    A: DebugRollup,
    B: DebugRollup,
{
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        let (a, b) = self;
        let mut v = a.dyn_debugs();
        v.push(Box::new(b));
        v
    }
}

impl<A, B, C> DebugRollup for (A, B, C)
where
    A: DebugRollup,
    B: DebugRollup,
    C: DebugRollup,
{
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        let (a, b, c) = self;
        let mut v = a.dyn_debugs();
        v.push(Box::new(b));
        v.push(Box::new(c));
        v
    }
}

impl DebugRollup for ratatui::layout::Layout {
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        vec![]
    }
}

impl DebugRollup for ratatui::style::Style {
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        vec![Box::new(DebugElide(self))]
    }
}

impl DebugRollup for ratatui::widgets::Clear {
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        vec![Box::new(DebugElide(self))]
    }
}

impl<'a> DebugRollup for ratatui::widgets::Block<'a> {
    fn dyn_debugs(&self) -> Vec<Box<dyn Debug + '_>> {
        vec![Box::new(DebugElide(self))]
    }
}
