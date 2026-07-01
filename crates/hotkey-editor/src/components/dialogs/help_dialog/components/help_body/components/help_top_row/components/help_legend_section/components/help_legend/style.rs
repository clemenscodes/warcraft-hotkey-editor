use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-[0.9rem]",
    "m-0",
    "p-0",
    "list-none",
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
