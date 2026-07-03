use crate::classes;

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "justify-center",
    "w-[1.7rem]",
    "h-[1.7rem]",
    "align-[-0.3em]",
    "text-warcraft-gold",
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
