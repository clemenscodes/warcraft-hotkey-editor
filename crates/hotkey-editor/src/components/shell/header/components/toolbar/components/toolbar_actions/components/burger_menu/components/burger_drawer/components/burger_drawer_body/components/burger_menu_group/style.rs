use crate::classes;

const BASE: &[&str] = &[
    "flex",
    "flex-col",
    "items-stretch",
    "gap-[0.4rem]",
    "mt-1",
    "pt-[0.85rem]",
    "border-t",
    "border-t-[rgba(255,206,99,0.12)]",
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
