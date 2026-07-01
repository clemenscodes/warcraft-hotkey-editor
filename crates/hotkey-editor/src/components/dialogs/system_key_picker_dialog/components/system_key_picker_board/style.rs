use crate::classes;

const BASE: &[&str] = &["flex", "flex-row", "items-end", "gap-[0.8rem]"];
const MOBILE: &[&str] = &[
    "mobile:flex-col",
    "mobile:items-center",
    "mobile:gap-[0.4rem]",
];
const TABLET: &[&str] = &[];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
