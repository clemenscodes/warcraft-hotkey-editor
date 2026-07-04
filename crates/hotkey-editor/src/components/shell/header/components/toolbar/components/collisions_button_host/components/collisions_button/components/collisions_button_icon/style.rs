use crate::{classes, styling::TailwindClass, tw};

// 44% of the button box (a cqi fraction), matching the shared toolbar button, so the
// glyph scales with the button instead of staying a fixed size.
const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "justify-center",
    "leading-none",
    "w-[44cqi]",
    "h-[44cqi]",
    "[&_svg]:block",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
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
