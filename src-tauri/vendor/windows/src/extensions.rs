//! Contains extensions to the generated bindings
//!
//! This module contains code that extends the functionality of the bindings generated by `bindgen`.
//! It adds inherent impls, trait impls, etc.
//!
//! This module's visibility is `pub(crate)`, not `pub`, because the `extensions` module path is
//! not intended to be directly visible to user code. Associated functions defined within this
//! module tree use the visibility of the types they are associated with, and so the visibility
//! of this module tree does not constrain the visibility of the associated functions. Similarly,
//! trait impls _themselves_ do not have visibility; their visibility is defined by the types and
//! traits used in the trait impl definition.
//!
//! If it becomes necessary to re-export definitions (types, etc.) defined in this module tree,
//! then the right thing to do is to change the visibility (along those module paths) to `pub(crate)`
//! and then to use `pub` for the specific items that will be re-exported.

#[cfg(feature = "Foundation")]
mod Foundation;
#[cfg(feature = "Win32")]
mod Win32;
