use crate::classes;

const BASE: &[&str] = &[
    "fixed",
    "bottom-6",
    "right-6",
    "top-auto",
    "left-auto",
    "w-max",
    "max-w-[calc(100vw-3rem)]",
    "z-[2000]",
    "outline-none",
    "pointer-events-none",
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
