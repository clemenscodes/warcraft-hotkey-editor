use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "items-center",
    "justify-end",
    "py-3",
    "px-4",
    "border-b",
    "border-b-[rgba(255,206,99,0.12)]",
    "shrink-0",
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
