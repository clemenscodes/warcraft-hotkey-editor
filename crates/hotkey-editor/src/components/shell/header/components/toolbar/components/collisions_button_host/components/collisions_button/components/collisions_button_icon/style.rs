use crate::classes;

// 44% of the button box (a cqi fraction), matching the shared toolbar button, so the
// glyph scales with the button instead of staying a fixed size.
const BASE: &[&str] = &[
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

const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! {
    BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD
}
