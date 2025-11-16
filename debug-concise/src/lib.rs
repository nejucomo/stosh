//! Utilities to make debug output more concise
use derive_more::{Deref, DerefMut, From};
use derive_new::new;
use type_name_concise::type_name_concise;

/// Attempt to act just like `T` _except_ debugging is a small stub with typename only
#[derive(new, From, Deref, DerefMut)]
pub struct DebugElide<T>(pub T);

impl<T> std::fmt::Debug for DebugElide<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(&type_name_concise::<T>())
            .finish_non_exhaustive()
    }
}
