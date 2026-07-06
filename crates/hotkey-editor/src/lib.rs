// Declare this crate's responsive band vocabulary once; this generates the
// crate-global `classes!` and `states!` macros bound to it. Must precede the
// module tree so both are in textual scope for every `style.rs`.
tw_macro::define_styling! { bands: [mobile, tablet, laptop, desktop, qhd, uhd] }

pub mod components;
pub mod persistence;
pub mod repository;
pub mod services;
