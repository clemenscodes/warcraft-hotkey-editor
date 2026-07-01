use crate::classes;
const BASE: &[&str] = &[
    "inline-flex",
    "w-[3.5rem]",
    "h-[3.5rem]",
    "text-warcraft-gold",
    "[&_svg]:w-full",
    "[&_svg]:h-full",
    "[filter:drop-shadow(0_0_10px_rgba(255,206,99,0.45))]",
];
const MOBILE: &[&str] = &[];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];
classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
