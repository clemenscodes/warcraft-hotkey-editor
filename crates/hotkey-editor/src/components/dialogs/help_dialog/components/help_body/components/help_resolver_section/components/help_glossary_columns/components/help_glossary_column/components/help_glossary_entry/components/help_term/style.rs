use crate::classes;

const BASE: &[&str] = &[
    "m-0",
    "text-[1.8rem]/[1.3]",
    "text-warcraft-gold",
    "[text-shadow:1px_1px_0_#000]",
];
const MOBILE: &[&str] = &["mobile:text-[1.5rem]/[1.3]"];
const TABLET: &[&str] = &["tablet:text-[1.5rem]/[1.3]"];
const LAPTOP: &[&str] = &[];
const DESKTOP: &[&str] = &[];
const QHD: &[&str] = &[];
const UHD: &[&str] = &[];

classes! { BASE, MOBILE, TABLET, LAPTOP, DESKTOP, QHD, UHD }
