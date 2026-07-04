use crate::{classes, styling::TailwindClass, tw};

// Fixed glyph size: the burger renders at a single button size, so the icon is a plain
// fixed size (~44% of the 36px button) rather than a cqi fraction.
const BASE: &[TailwindClass] = tw![
    "flex",
    "items-center",
    "justify-center",
    "w-4",
    "h-4",
    "leading-none",
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
