use crate::classes;

const BASE: &[&str] = &["m-0", "text-[1.6rem]/[1.45]", "opacity-80"];
const MOBILE: &[&str] = &[
    "mobile:text-[13px]/[1.35]",
    "mobile:text-[#c0c8da]",
    "mobile:opacity-90",
];
const TABLET: &[&str] = &[
    "tablet:text-[13px]/[1.35]",
    "tablet:text-[#c0c8da]",
    "tablet:opacity-90",
];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
