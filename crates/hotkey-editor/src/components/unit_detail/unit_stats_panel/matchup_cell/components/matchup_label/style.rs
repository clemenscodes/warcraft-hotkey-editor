use crate::classes;

const BASE: &[&str] = &[
    "text-[#9aa5bd]",
    "font-friz-quadrata",
    "text-[inherit]",
    "min-w-0",
    "overflow-hidden",
    "text-ellipsis",
    "whitespace-nowrap",
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
