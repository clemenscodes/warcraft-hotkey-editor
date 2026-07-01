use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-center",
    "flex-[1_1_0]",
    "min-w-0",
    "self-stretch",
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
