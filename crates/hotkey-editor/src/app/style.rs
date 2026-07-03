use super::state::AppLayout;
use crate::{classes, states};

// The app shell centres the workbench in a single column with generous padding that
// tightens toward phones. Old pixel breakpoints fold into the named bands: the former
// `max-[1500px]` top-padding drop and `max-[1024px]` height relaxations become the
// mobile/tablet overrides; the `max-[700px]`/`max-[480px]` padding steps collapse to
// the phone band.
const BASE: &[&str] = &[
    "mx-auto",
    "flex",
    "flex-col",
    "pt-7",
    "pb-12",
    "px-14",
    "min-h-[100dvh]",
    "max-w-[100vw]",
    "overflow-x-clip",
    "gap-8",
];
// Phone/tablet: no top padding, safe-area-aware side/bottom padding (so notches
// never clip the shell), a content-driven height above the dvh floor, and the
// tighter section gap. `env(safe-area-inset-*)` falls back to the min so it is a
// no-op on non-notched screens.
const MOBILE: &[&str] = &[
    "mobile:pt-0",
    "mobile:pl-[max(0.75rem,env(safe-area-inset-left))]",
    "mobile:pr-[max(0.75rem,env(safe-area-inset-right))]",
    "mobile:pb-[max(1rem,env(safe-area-inset-bottom))]",
    "mobile:h-auto",
    "mobile:overflow-y-visible",
    "mobile:gap-[16px]",
];
const TABLET: &[&str] = &[
    "tablet:pt-0",
    "tablet:pl-[max(0.75rem,env(safe-area-inset-left))]",
    "tablet:pr-[max(0.75rem,env(safe-area-inset-right))]",
    "tablet:pb-[max(1rem,env(safe-area-inset-bottom))]",
    "tablet:h-auto",
    "tablet:overflow-y-visible",
    "tablet:gap-[16px]",
];
const LAPTOP: &[&str] = &["laptop:gap-4"];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}

// The collisions view is one full-bleed page and drops the inter-section gap; every
// other view keeps the standard responsive gaps from the bands above. `gap-0!` wins
// over those band gaps regardless of the utility order Tailwind emits.
const STANDARD: &[&str] = &[];
const COLLISIONS: &[&str] = &["gap-0!"];
states! {
    AppLayout, Standard => STANDARD, Collisions => COLLISIONS
}
