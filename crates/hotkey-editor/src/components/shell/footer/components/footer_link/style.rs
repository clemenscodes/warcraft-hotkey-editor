use crate::classes;

const BASE: &[&str] = &[
    "inline-flex",
    "items-center",
    "gap-1.5",
    "text-white/60",
    "transition-colors",
    "hover:text-warcraft-gold",
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
