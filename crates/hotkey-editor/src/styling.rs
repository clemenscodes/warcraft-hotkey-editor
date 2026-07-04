//! Compile-time assembly of a component's Tailwind class list.
//!
//! A component declares one utility list per responsive band as locked consts in
//! its own `style.rs`, then `classes!` joins them into a single
//! `pub(super) const CLASS: &str` at compile time. The caller never writes the
//! identity class: the macro derives it from the component directory.
//!
//! Enforced, all at compile time:
//! - **The identity is derived, not given.** `module_path!()` ends in
//!   `…::<dir>::style`, so the macro reads the component directory and emits the
//!   kebab identity (`help_top_row` → `help-top-row`). It can never drift from
//!   `dir == component == class`, and the caller can never get it wrong.
//! - **Every band is present.** The arity is fixed to the six app bands, so a
//!   component can never silently miss `MOBILE` or `UHD`; an unused band is an
//!   explicit empty `&[]`.
//! - **Every band const is named correctly** (`MOBILE`, `TABLET`, `LAPTOP`,
//!   `DESKTOP`, `QHD`, `UHD`), so `grep MOBILE` finds every component's mobile
//!   styles.
//! - **Every class carries its band's prefix** (`uhd:flex` can never land in
//!   `MOBILE`).
//!
//! `pub(super)` keeps `CLASS` visible only to the component's own module: with
//! the private `mod style;`, no other component can name the path, so styles
//! never leak across components. Every class stays a literal in the band arrays,
//! so Tailwind's source scanner still sees every token.
//!
//! `CLASS` is a [`ClassList`], not a `&str`: it can be handed to a `class:`
//! attribute but deliberately implements neither `Display` nor a public
//! accessor, so it can never be string-interpolated (`class: "{CLASS} other"`
//! does not compile) and its inner string cannot be extracted. A component can
//! only ever wear exactly its own `CLASS` — never another component's class, and
//! nothing can be appended — so styling coupling between components is
//! impossible to express.

use dioxus::core::{AttributeValue, IntoAttributeValue};

/// An opaque, component-private class list. See the module docs: it has no
/// `Display` and no public accessor, so the `class:` attribute can take it but
/// nothing can append to it or read it out.
#[derive(Clone, Copy)]
pub struct ClassList(&'static str);

impl ClassList {
    /// Wrap an already-assembled class string. Only `classes!` calls this.
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }

    /// Bridge for handing this class to a third-party component whose `class`
    /// prop is typed `String` rather than an attribute value (the
    /// `dioxus_primitives` `DialogContent` is the only such case). Crate-internal
    /// and never called from a component body — bodies always use `class: CLASS`,
    /// so the no-interpolation guarantee still holds everywhere it matters.
    pub(crate) fn to_library_class(self) -> String {
        self.0.to_string()
    }
}

impl IntoAttributeValue for ClassList {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.0.to_string())
    }
}

/// A single Tailwind utility class. A named-field newtype over the static class
/// string, built only by the [`tw!`](crate::tw) macro. Making a class list
/// `&[TailwindClass]` rather than a bare `&[&str]` states, in the type, that
/// these strings are Tailwind utilities — an ordinary `&[&str]` of prose is a
/// different type and can never reach the styling machinery. It also gives
/// editor tooling an exact anchor: completion targets `tw![…]` and nothing else.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TailwindClass {
    utility: &'static str,
}

impl TailwindClass {
    /// Wrap a single utility class literal. Only the `tw!` macro calls this.
    pub const fn new(utility: &'static str) -> Self {
        Self { utility }
    }

    /// The underlying utility string.
    pub const fn utility(&self) -> &'static str {
        self.utility
    }
}

const fn to_lower(byte: u8) -> u8 {
    if byte >= b'A' && byte <= b'Z' {
        byte + 32
    } else {
        byte
    }
}

const fn to_upper(byte: u8) -> u8 {
    if byte >= b'a' && byte <= b'z' {
        byte - 32
    } else {
        byte
    }
}

const fn equal(left: &str, right: &str) -> bool {
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return false;
    }
    let mut index = 0;
    while index < left.len() {
        if left[index] != right[index] {
            return false;
        }
        index += 1;
    }
    true
}

/// True when `class` begins with the lowercase `band` followed by `:`.
const fn has_band_prefix(class: &str, band: &str) -> bool {
    let class = class.as_bytes();
    let band = band.as_bytes();
    if class.len() < band.len() + 1 {
        return false;
    }
    let mut index = 0;
    while index < band.len() {
        if to_lower(class[index]) != band[index] {
            return false;
        }
        index += 1;
    }
    class[band.len()] == b':'
}

/// The byte range of the component directory in a module path ending `::style`.
const fn directory_bounds(module_path: &str) -> (usize, usize) {
    let bytes = module_path.as_bytes();
    let suffix = b"::style";
    if bytes.len() < suffix.len() + 1 {
        panic!("classes! must be invoked from a component's style.rs");
    }
    let directory_end = bytes.len() - suffix.len();
    let mut offset = 0;
    while offset < suffix.len() {
        if bytes[directory_end + offset] != suffix[offset] {
            panic!("classes! must be invoked from a component's style.rs");
        }
        offset += 1;
    }
    let mut directory_start = 0;
    let mut position = 0;
    while position + 1 < directory_end {
        if bytes[position] == b':' && bytes[position + 1] == b':' {
            directory_start = position + 2;
        }
        position += 1;
    }
    (directory_start, directory_end)
}

/// Compile-time guard: a band const must be named exactly after its band.
pub const fn assert_named(actual: &str, expected: &str) {
    if !equal(actual, expected) {
        panic!("a band const has the wrong name");
    }
}

/// Compile-time guard: every utility in a band must carry that band's prefix.
pub const fn assert_band(band: &str, classes: &[TailwindClass]) {
    let mut index = 0;
    while index < classes.len() {
        let class = classes[index].utility();
        if !has_band_prefix(class, band) {
            panic!("a class is not prefixed with its band");
        }
        index += 1;
    }
}

const fn is_band_prefixed(class: &str) -> bool {
    has_band_prefix(class, "mobile")
        || has_band_prefix(class, "tablet")
        || has_band_prefix(class, "laptop")
        || has_band_prefix(class, "desktop")
        || has_band_prefix(class, "qhd")
        || has_band_prefix(class, "uhd")
}

/// Compile-time guard: a BASE class is always-on, so it must NOT carry a band
/// prefix (a width-specific style belongs in that band, never in BASE).
pub const fn assert_base(classes: &[TailwindClass]) {
    let mut index = 0;
    while index < classes.len() {
        let class = classes[index].utility();
        if is_band_prefixed(class) {
            panic!("a BASE class carries a band prefix");
        }
        index += 1;
    }
}

/// Compile-time guard: a state overlay (see `states!`) is always-on within its
/// state, so it must NOT carry a band prefix.
#[allow(dead_code)]
pub const fn assert_flat(classes: &[TailwindClass]) {
    let mut index = 0;
    while index < classes.len() {
        let class = classes[index].utility();
        if is_band_prefixed(class) {
            panic!("a state overlay carries a band prefix");
        }
        index += 1;
    }
}

/// Byte length of the kebab identity derived from `module_path`'s directory.
pub const fn identity_len(module_path: &str) -> usize {
    let (start, end) = directory_bounds(module_path);
    end - start
}

/// Build the kebab identity (the component directory with `_` mapped to `-`).
pub const fn build_identity<const N: usize>(module_path: &str) -> [u8; N] {
    let (start, _end) = directory_bounds(module_path);
    let bytes = module_path.as_bytes();
    let mut out = [0u8; N];
    let mut index = 0;
    while index < N {
        let byte = bytes[start + index];
        out[index] = if byte == b'_' { b'-' } else { byte };
        index += 1;
    }
    out
}

/// The byte range of the final segment of a module path (the component module
/// itself, as seen from its `mod.rs`).
const fn last_segment_bounds(module_path: &str) -> (usize, usize) {
    let bytes = module_path.as_bytes();
    let end = bytes.len();
    let mut start = 0;
    let mut position = 0;
    while position + 1 < end {
        if bytes[position] == b':' && bytes[position + 1] == b':' {
            start = position + 2;
        }
        position += 1;
    }
    (start, end)
}

/// Compile-time guard: the PascalCase `component` name must equal the snake_case
/// component directory (the final segment of the module path), capitalization
/// included.
pub const fn assert_component_name(component: &str, module_path: &str) {
    let (start, end) = last_segment_bounds(module_path);
    let directory = module_path.as_bytes();
    let pascal = component.as_bytes();
    let mut directory_index = start;
    let mut pascal_index = 0;
    let mut at_word_start = true;
    while directory_index < end {
        let byte = directory[directory_index];
        if byte == b'_' || byte == b'-' {
            at_word_start = true;
            directory_index += 1;
            continue;
        }
        if pascal_index >= pascal.len() {
            panic!("component name does not match its directory");
        }
        let expected = if at_word_start { to_upper(byte) } else { byte };
        if pascal[pascal_index] != expected {
            panic!("component name does not match its directory");
        }
        pascal_index += 1;
        at_word_start = false;
        directory_index += 1;
    }
    if pascal_index != pascal.len() {
        panic!("component name does not match its directory");
    }
}

/// Length of the joined class string: the identity plus, per band utility, a
/// leading space and its bytes.
pub const fn joined_len(identity: &str, bands: &[&[TailwindClass]]) -> usize {
    let mut len = identity.len();
    let mut band_index = 0;
    while band_index < bands.len() {
        let band = bands[band_index];
        let mut utility_index = 0;
        while utility_index < band.len() {
            let utility = band[utility_index].utility();
            len += 1 + utility.len();
            utility_index += 1;
        }
        band_index += 1;
    }
    len
}

/// Write the identity and every band utility, space separated, into a buffer
/// sized by [`joined_len`].
pub const fn join_into<const N: usize>(identity: &str, bands: &[&[TailwindClass]]) -> [u8; N] {
    let mut out = [0u8; N];
    let mut position = 0;
    let identity_bytes = identity.as_bytes();
    let mut byte_index = 0;
    while byte_index < identity_bytes.len() {
        out[position] = identity_bytes[byte_index];
        position += 1;
        byte_index += 1;
    }
    let mut band_index = 0;
    while band_index < bands.len() {
        let band = bands[band_index];
        let mut utility_index = 0;
        while utility_index < band.len() {
            out[position] = b' ';
            position += 1;
            let utility = band[utility_index].utility();
            let utility_bytes = utility.as_bytes();
            let mut inner = 0;
            while inner < utility_bytes.len() {
                out[position] = utility_bytes[inner];
                position += 1;
                inner += 1;
            }
            utility_index += 1;
        }
        band_index += 1;
    }
    out
}
/// Build a `&[TailwindClass]` from Tailwind utility literals — the one way a
/// class array is written. `const BASE: &[TailwindClass] = tw!["flex", "m-0"];`
/// expands each literal through [`TailwindClass::new`], so the array's element
/// type is `TailwindClass`, never `&str`. A plain `&[&str]` (prose, labels) is a
/// different type and cannot be passed to `classes!`/`states!`; editor tooling
/// keys on the `tw!` call to scope completion to exactly these lists.
///
/// ```ignore
/// use crate::tw;
/// use crate::styling::TailwindClass;
///
/// const BASE: &[TailwindClass] = tw!["relative", "flex"];
/// const MOBILE: &[TailwindClass] = tw![]; // an unused band is an empty list
/// ```
#[macro_export]
macro_rules! tw {
    ($($class:literal),* $(,)?) => {
        &[$($crate::styling::TailwindClass::new($class)),*]
    };
}

/// Join a component's six per-band utility lists into a compile-time,
/// component-private `pub(super) const CLASS: &str`. The identity class is
/// derived from the component directory; the caller passes only the bands.
///
/// Every band is mandatory and must be named after its band; an unused band is
/// an explicit empty slice. See the module docs for the guarantees.
///
/// ```ignore
/// use crate::{classes, tw};
/// use crate::styling::TailwindClass;
///
/// const BASE: &[TailwindClass] = tw!["m-0"];
/// const MOBILE: &[TailwindClass] = tw!["mobile:m-0", "mobile:text-heading-sm", "mobile:text-center"];
/// const TABLET: &[TailwindClass] = tw![];
/// const LAPTOP: &[TailwindClass] = tw!["laptop:text-heading", "laptop:text-left"];
/// const DESKTOP: &[TailwindClass] = tw![];
/// const QHD: &[TailwindClass] = tw![];
/// const UHD: &[TailwindClass] = tw![];
///
/// classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
/// // in dir help_section_title/ → CLASS starts with "help-section-title ..."
/// ```
#[macro_export]
macro_rules! classes {
    (
        $base:ident, $mobile:ident, $tablet:ident, $laptop:ident, $desktop:ident,
        $qhd:ident, $uhd:ident $(,)?
    ) => {
        const _: () = {
            $crate::styling::assert_named(stringify!($base), "BASE");
            $crate::styling::assert_named(stringify!($mobile), "MOBILE");
            $crate::styling::assert_named(stringify!($tablet), "TABLET");
            $crate::styling::assert_named(stringify!($laptop), "LAPTOP");
            $crate::styling::assert_named(stringify!($desktop), "DESKTOP");
            $crate::styling::assert_named(stringify!($qhd), "QHD");
            $crate::styling::assert_named(stringify!($uhd), "UHD");
            $crate::styling::assert_base($base);
            $crate::styling::assert_band("mobile", $mobile);
            $crate::styling::assert_band("tablet", $tablet);
            $crate::styling::assert_band("laptop", $laptop);
            $crate::styling::assert_band("desktop", $desktop);
            $crate::styling::assert_band("qhd", $qhd);
            $crate::styling::assert_band("uhd", $uhd);
        };

        const MODULE_PATH: &str = module_path!();
        const IDENTITY_LEN: usize = $crate::styling::identity_len(MODULE_PATH);

        const IDENTITY_BYTES: [u8; IDENTITY_LEN] =
            $crate::styling::build_identity::<IDENTITY_LEN>(MODULE_PATH);

        const IDENTITY: &str = match ::core::str::from_utf8(&IDENTITY_BYTES) {
            ::core::result::Result::Ok(identity) => identity,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 identity"),
        };

        const BANDS: &[&[$crate::styling::TailwindClass]] =
            &[$base, $mobile, $tablet, $laptop, $desktop, $qhd, $uhd];
        const LEN: usize = $crate::styling::joined_len(IDENTITY, BANDS);
        const BYTES: [u8; LEN] = $crate::styling::join_into::<LEN>(IDENTITY, BANDS);

        const CLASS_STR: &str = match ::core::str::from_utf8(&BYTES) {
            ::core::result::Result::Ok(class) => class,
            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 class list"),
        };

        #[allow(dead_code)]
        pub(super) const CLASS: $crate::styling::ClassList =
            $crate::styling::ClassList::new(CLASS_STR);
    };
}
/// Select a component's class by its current visual state. Used in the same
/// `style.rs` as `classes!`, alongside it: `classes!` produces the base look,
/// `states!` layers a flat (non-responsive) overlay per state on top of it and
/// emits `pub(super) fn class(state: T) -> ClassList`.
///
/// The state type `T` is the component's own enum (in `state.rs`). The match is
/// exhaustive, so every state must be styled; each overlay is validated as
/// always-on (no band prefix). The joined class per state is built at compile
/// time, so the selector is just a match returning a precomputed `ClassList`.
/// The component's `From<&Props>` picks the state and the body places the result,
/// so the body never branches.
///
/// ```ignore
/// use crate::{classes, states, tw};
/// use crate::styling::TailwindClass;
///
/// const BASE: &[TailwindClass] = tw!["relative", "flex"];
/// // ... the six bands ...
/// classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
///
/// const IDLE: &[TailwindClass] = tw![];
/// const DRAG_SOURCE: &[TailwindClass] = tw!["opacity-40", "ring-2", "ring-warcraft-gold"];
///
/// states! { TileState, Idle => IDLE, DragSource => DRAG_SOURCE }
/// // → pub(super) fn class(state: TileState) -> ClassList
/// ```
#[macro_export]
macro_rules! states {
    ($ty:ty, $($variant:ident => $overlay:ident),+ $(,)?) => {
        const _ : () = { $($crate::styling::assert_flat($overlay);)+ }; pub (super) fn
        class(state : $ty) -> $crate::styling::ClassList { match state { $(<$ty
        >::$variant => { const LEN : usize = $crate::styling::joined_len(CLASS_STR, &
        [$overlay]); const BYTES : [u8; LEN] = $crate::styling::join_into::< LEN >
        (CLASS_STR, & [$overlay]); const STATE_CLASS : $crate::styling::ClassList =
        $crate::styling::ClassList::new(match ::core::str::from_utf8(& BYTES) {
        ::core::result::Result::Ok(class) => class, ::core::result::Result::Err(_) => {
        ::core::panic!("non-utf8 state class") } }); STATE_CLASS })+ } }
    };
}
/// Compile-time guard for a component's `mod.rs`: assert the component function
/// name matches its directory, capitalization included. `classes!` already binds
/// the class to the directory, so this closes the triangle
/// `component == directory == class`.
///
/// ```ignore
/// use crate::assert_component;
///
/// assert_component!(HelpTopRow);
///
/// #[component]
/// pub fn HelpTopRow() -> Element { /* body uses style::CLASS */ }
/// ```
#[macro_export]
macro_rules! assert_component {
    ($name:ident) => {
        const _: () = $crate::styling::assert_component_name(stringify!($name), module_path!());
    };
}
