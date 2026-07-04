use crate::classes;

// The title's font size is a container-query length off the brand host, and the line
// never wraps — so title, gap, and flourishes are one fixed proportional composition
// that scales with the host. It shrinks to fit instead of truncating; no ellipsis, no
// per-band font clamps.
const BASE: &[&str] = &[
    "m-0",
    "font-friz-quadrata",
    "font-normal",
    "text-[length:5cqi]",
    "leading-[1.1]",
    "tracking-[0.04em]",
    "text-warcraft-gold",
    "whitespace-nowrap",
    "text-left",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.92),0_0_14px_rgba(255,206,99,0.18)]",
];

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];

const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
