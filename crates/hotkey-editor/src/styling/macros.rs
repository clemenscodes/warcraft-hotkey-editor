/// Build a `&[TailwindClass]` from Tailwind utility literals — the one way a
/// class array is written. `const BASE: &[TailwindClass] = tw!["flex", "m-0"];`
/// expands each literal through [`TailwindClass::new`](crate::styling::TailwindClass::new),
/// so the array's element type is `TailwindClass`, never `&str`. A plain
/// `&[&str]` (prose, labels) is a different type and cannot be passed to
/// `classes!`/`states!`; editor tooling keys on the `tw!` call to scope
/// completion to exactly these lists.
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
