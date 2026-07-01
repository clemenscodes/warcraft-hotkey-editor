use crate::classes;

// The grid glyph beside the label; the injected SVG fills the fixed square box.
const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "w-[2.2rem]",
    "h-[2.2rem]",
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

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
