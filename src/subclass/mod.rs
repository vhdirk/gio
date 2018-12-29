#![cfg_attr(feature = "cargo-clippy", allow(cast_ptr_alignment))]

pub mod application;

pub mod prelude {
    pub use super::application::ApplicationImpl;
    pub use glib::subclass::prelude::*;
}

use self::prelude::*;
