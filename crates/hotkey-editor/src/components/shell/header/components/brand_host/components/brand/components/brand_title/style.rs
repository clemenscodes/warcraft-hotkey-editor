use crate::{classes, styling::TailwindClass, tw};

// The title's font size is a container-query length off the brand host, and the line
// never wraps — so title, gap, and flourishes are one fixed proportional composition
// that scales with the host. It shrinks to fit instead of truncating; no ellipsis, no
// per-band font clamps.
const BASE: &[TailwindClass] = tw![
    "m-0",
    "font-friz-quadrata",
    "font-normal",
    "text-[5cqi]",
    "leading-[1.1]",
    "tracking-[0.04em]",
    "text-warcraft-gold",
    "whitespace-nowrap",
    "text-left",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92),0_0_14px_rgba(255,206,99,0.18)]",
];

const MOBILE: &[TailwindClass] = tw![];
const TABLET: &[TailwindClass] = tw![];

const LAPTOP: &[TailwindClass] = tw![];
const DESKTOP: &[TailwindClass] = tw![];
const QHD: &[TailwindClass] = tw![];
const UHD: &[TailwindClass] = tw![];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
