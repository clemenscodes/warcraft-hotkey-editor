use crate::classes;

const BASE: &[&str] = &[
    "font-friz-quadrata",
    "font-normal",
    "uppercase",
    "whitespace-nowrap",
    "tracking-[0.12em]",
    "[text-shadow:1px_1px_0_rgba(0,0,0,0.6)]",
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
