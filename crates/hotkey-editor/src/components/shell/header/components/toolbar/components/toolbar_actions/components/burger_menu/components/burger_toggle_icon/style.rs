use crate::classes;

// Fixed glyph size: the burger renders at a single button size, so the icon is a plain
// fixed size (~44% of the 36px button) rather than a cqi fraction.
const BASE: &[&str] = &[
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

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
