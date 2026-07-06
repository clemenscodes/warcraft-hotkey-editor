//! Native proof that `define_styling!` generates working, vocabulary-bound
//! keyed macros, and that the validators the generated macros expand into reject
//! every mis-styled input. In a `style.rs` these rejections fire at compile time
//! (the validator runs in a `const _: () = { … }` block); here they are exercised
//! directly at runtime, so `#[should_panic]` can assert each one.

use crate::TailwindClass;
use crate::internal::{assert_flat, assert_key};

define_styling! { bands: [mobile, tablet, laptop, desktop, qhd, uhd] }

#[derive(Clone, Copy)]
enum Tone {
    Idle,
    Active,
}

mod header {
    pub mod style {
        classes! {
            base: tw!["@container", "relative"],
            mobile: tw!["mobile:flex"],
            laptop: tw!["laptop:grid", "laptop:items-stretch"],
        }

        states! {
            super::super::Tone,
            Idle => tw![],
            Active => tw!["opacity-100", "ring-2"],
        }
    }

    // The parent of `style` — where `pub(super) CLASS`/`class` are visible, the
    // same relationship a component's `mod.rs` has to its `style.rs`.
    pub fn base_class() -> crate::ClassList {
        style::CLASS
    }

    pub fn state_class(tone: super::Tone) -> crate::ClassList {
        style::class(tone)
    }
}

#[test]
fn generated_keyed_classes_and_states_compile_and_resolve() {
    let _base = header::base_class();
    let _idle = header::state_class(Tone::Idle);
    let _active = header::state_class(Tone::Active);
}

#[test]
fn joined_len_counts_identity_plus_space_prefixed_utilities() {
    let base: &[TailwindClass] = tw!["@container", "relative"];
    let mobile: &[TailwindClass] = tw!["mobile:flex"];
    let bands: &[&[TailwindClass]] = &[base, mobile];
    // "header" (6) + " @container" (11) + " relative" (9) + " mobile:flex" (12)
    assert_eq!(
        crate::internal::joined_len("header", bands),
        6 + 11 + 9 + 12
    );
}

const VOCAB: &[&str] = &["mobile", "tablet", "laptop", "desktop", "qhd", "uhd"];

#[test]
fn base_accepts_variant_prefixes_but_not_band_prefixes() {
    let base: &[TailwindClass] = tw!["flex", "after:absolute", "hover:text-white"];
    assert_key("base", VOCAB, base);
}

#[test]
#[should_panic(expected = "base class carries a responsive band prefix")]
fn base_rejects_a_band_prefix() {
    let base: &[TailwindClass] = tw!["mobile:flex"];
    assert_key("base", VOCAB, base);
}

#[test]
#[should_panic(expected = "class key is not a declared responsive band")]
fn rejects_an_undeclared_band_key() {
    let entries: &[TailwindClass] = tw!["moble:flex"];
    assert_key("moble", VOCAB, entries);
}

#[test]
#[should_panic(expected = "class is not prefixed with its band")]
fn band_rejects_an_unprefixed_utility() {
    let mobile: &[TailwindClass] = tw!["flex"];
    assert_key("mobile", VOCAB, mobile);
}

#[test]
#[should_panic(expected = "state overlay carries a responsive band prefix")]
fn state_overlay_rejects_a_band_prefix() {
    let overlay: &[TailwindClass] = tw!["mobile:opacity-40"];
    assert_flat(overlay, VOCAB);
}
