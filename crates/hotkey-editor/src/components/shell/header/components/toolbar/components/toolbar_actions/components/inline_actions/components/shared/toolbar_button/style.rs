use crate::classes;

const BASE: &[&str] = &[
    "block",
    "h-full",
    "w-auto",
    "max-w-full",
    "aspect-square",
    "shrink-0",
    "[container-type:inline-size]",
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
