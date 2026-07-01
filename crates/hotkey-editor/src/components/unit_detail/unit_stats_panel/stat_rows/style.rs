use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "gap-[0.5rem]",
    "min-w-0",
    "flex-[1_1_auto]",
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
