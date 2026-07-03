use crate::classes;

const BASE: &[&str] = &[
    "fixed",
    "inset-0",
    "z-[70]",
    "bg-[rgba(0,0,0,0.65)]",
    "cursor-pointer",
    "border-none",
    "p-0",
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
