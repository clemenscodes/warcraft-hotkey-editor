/// Build a `&[TailwindClass]` from Tailwind utility literals — the one way to
/// write a standalone class array. `const OVERLAY: &[TailwindClass] =
/// tw!["opacity-40"];` wraps each literal through
/// [`TailwindClass::new`](crate::TailwindClass::new), so the element type is
/// `TailwindClass`, never `&str`. Keyed `classes!`/`states!` take inline
/// literals directly and do not need `tw!`.
#[macro_export]
macro_rules! tw {
    ($($class:literal),* $(,)?) => {
        &[$($crate::TailwindClass::new($class)),*]
    };
}

/// Compile-time guard for a component's `mod.rs`: assert the component function
/// name matches its directory, capitalization included. `classes!` binds the
/// class to the directory, so this closes the triangle
/// `component == directory == class`.
///
/// ```ignore
/// tw_macro::assert_component!(HelpTopRow);
/// ```
#[macro_export]
macro_rules! assert_component {
    ($name:ident) => {
        const _: () = $crate::internal::assert_component_name(stringify!($name), module_path!());
    };
}

/// Internal: invoke `$body` with a real `$` token bound to its first
/// metavariable, so a macro can generate another macro whose transcriber needs
/// `$`. See [`define_styling!`].
#[doc(hidden)]
#[macro_export]
macro_rules! __tw_with_dollar {
    ($($body:tt)*) => {
        macro_rules! __tw_emit_bound_macros {
            $($body)*
        }
        __tw_emit_bound_macros!($);
    };
}

/// Declare this crate's responsive band vocabulary and generate the keyed
/// `classes!` and `states!` macros bound to it. Invoke **once**, at the
/// consumer crate root:
///
/// ```ignore
/// tw_macro::define_styling! { bands: [mobile, tablet, laptop, desktop, qhd, uhd] }
/// ```
///
/// The declared band names are baked into the generated macros. `classes!` then
/// enforces, all at compile time (see the crate docs):
/// - a `base:` entry carries no declared band prefix (but `after:`/`hover:` etc.
///   are fine),
/// - every other key names a declared band, and each of its utilities carries
///   that band's `key:` prefix,
/// - an undeclared key (a typo) is rejected.
///
/// The generator uses the stable "dollar-passing" trick: `define_styling!`
/// bakes the band list as literal data, then hands a real `$` to a helper so the
/// generated `classes!`/`states!` can carry their own metavariables. This is the
/// one intentionally intricate construct in the crate.
#[macro_export]
macro_rules! define_styling {
    (bands: [ $($band:ident),* $(,)? ]) => {
        $crate::__tw_with_dollar! {
            ($d:tt) => {
                /// Join a component's keyed per-band utility lists into a
                /// compile-time, component-private `pub(super) const CLASS`. The
                /// identity class is derived from the component directory; keys
                /// are optional, order-independent, and validated per band.
                #[macro_export]
                macro_rules! classes {
                    (
                        $d( $d key:ident : $d band:expr ),* $d(,)?
                    ) => {
                        const _: () = {
                            $d(
                                $crate::internal::assert_key(
                                    ::core::stringify!($d key),
                                    &[ $( ::core::stringify!($band) ),* ],
                                    $d band,
                                );
                            )*
                        };

                        const MODULE_PATH: &str = ::core::module_path!();
                        const IDENTITY_LEN: usize = $crate::internal::identity_len(MODULE_PATH);
                        const IDENTITY_BYTES: [u8; IDENTITY_LEN] =
                            $crate::internal::build_identity::<IDENTITY_LEN>(MODULE_PATH);
                        const IDENTITY: &str = match ::core::str::from_utf8(&IDENTITY_BYTES) {
                            ::core::result::Result::Ok(identity) => identity,
                            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 identity"),
                        };

                        const BANDS: &[&[$crate::TailwindClass]] = &[ $d( $d band ),* ];
                        const LEN: usize = $crate::internal::joined_len(IDENTITY, BANDS);
                        const BYTES: [u8; LEN] = $crate::internal::join_into::<LEN>(IDENTITY, BANDS);
                        const CLASS_STR: &str = match ::core::str::from_utf8(&BYTES) {
                            ::core::result::Result::Ok(class) => class,
                            ::core::result::Result::Err(_) => ::core::panic!("non-utf8 class list"),
                        };

                        #[allow(dead_code)]
                        pub(super) const CLASS: $crate::ClassList = $crate::ClassList::new(CLASS_STR);
                    };
                }

                /// Select a component's class by its current visual state. Layers
                /// a flat (non-responsive) overlay per state onto the base
                /// `CLASS_STR` and emits `pub(super) fn class(state) -> ClassList`.
                #[macro_export]
                macro_rules! states {
                    (
                        $d ty:ty,
                        $d( $d variant:ident => $d overlay:expr ),+ $d(,)?
                    ) => {
                        const _: () = {
                            $d(
                                $crate::internal::assert_flat(
                                    $d overlay,
                                    &[ $( ::core::stringify!($band) ),* ],
                                );
                            )+
                        };
                        pub(super) fn class(state: $d ty) -> $crate::ClassList {
                            match state {
                                $d(
                                    <$d ty>::$d variant => {
                                        const OVERLAY: &[$crate::TailwindClass] = $d overlay;
                                        const LEN: usize =
                                            $crate::internal::joined_len(CLASS_STR, &[OVERLAY]);
                                        const BYTES: [u8; LEN] =
                                            $crate::internal::join_into::<LEN>(CLASS_STR, &[OVERLAY]);
                                        const STATE_CLASS: $crate::ClassList = $crate::ClassList::new(
                                            match ::core::str::from_utf8(&BYTES) {
                                                ::core::result::Result::Ok(class) => class,
                                                ::core::result::Result::Err(_) => {
                                                    ::core::panic!("non-utf8 state class")
                                                }
                                            },
                                        );
                                        STATE_CLASS
                                    }
                                )+
                            }
                        }
                    };
                }
            };
        }
    };
}
